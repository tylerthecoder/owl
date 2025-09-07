use clap::{Parser, Subcommand};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;

// =======================================
//            Paths
// =======================================

fn get_owl_path() -> PathBuf {
    let owl_path = "~/owl";
    PathBuf::from(shellexpand::tilde(&owl_path).to_string())
}

fn get_config_path() -> PathBuf {
    let config_path = "~/.config/owl/config.json";
    PathBuf::from(shellexpand::tilde(&config_path).to_string())
}

fn get_owl_rc_path() -> PathBuf {
    let owl_rc_path = "~/.config/owl/rc";
    PathBuf::from(shellexpand::tilde(&owl_rc_path).to_string())
}

fn get_owl_menu_scripts_path() -> PathBuf {
    let owl_menu_scripts_path = "~/.config/owl/menu-scripts";
    PathBuf::from(shellexpand::tilde(&owl_menu_scripts_path).to_string())
}

// =======================================
//            Config
// =======================================
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Config {
    #[serde(default = "get_owl_path")]
    owl_path: PathBuf,
    nest_path: Option<PathBuf>,
}

fn load_config() -> Option<Config> {
    let config_path = get_config_path();
    if !Path::new(&config_path).exists() {
        return None;
    }
    let config_raw = std::fs::read_to_string(&config_path).ok()?;
    let config: Config = serde_json::from_str(&config_raw).ok()?;
    Some(config)
}

fn print_config() {
    let config = get_config();
    print_section("Owl Config");
    print_kv("owl_path", &config.owl_path.display().to_string());
    match &config.nest_path {
        Some(p) => print_kv("active_root", &p.display().to_string()),
        None => println!("  {} {}", "active_root:".white(), "(none)".yellow()),
    }
}

fn prompt_user_for_config() -> Config {
    println!(
        "Enter the absolute path to your owl folder (default: {}): ",
        get_owl_path().display()
    );
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let trimmed = input.trim();
    let owl_path = if trimmed.is_empty() {
        get_owl_path()
    } else {
        PathBuf::from(trimmed)
    };
    Config {
        owl_path,
        nest_path: None,
    }
}

fn get_config() -> Config {
    if let Some(cfg) = load_config() {
        return cfg;
    }
    let cfg = prompt_user_for_config();
    save_config(cfg.clone());
    cfg
}

fn save_config(config: Config) -> Config {
    let config_path = get_config_path();
    if let Some(parent) = config_path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("Unable to create config directory: {}", e);
        }
    }
    let config_raw = serde_json::to_string(&config).expect("Unable to serialize config");
    std::fs::write(config_path, config_raw).expect("Unable to write config file");
    config
}

// =======================================
//            Setup Header
// =======================================

#[derive(Clone)]
struct SetupHeader {
    name: String,
    setup_dir: PathBuf,
    setup_file_path: PathBuf,
}

impl SetupHeader {
    fn new(setup_file_path: PathBuf) -> Result<Self, String> {
        if setup_file_path.extension().unwrap() != "json" {
            return Err("setup file must be a JSON file".to_string());
        }
        let setup_dir = setup_file_path.parent().unwrap();
        let name = setup_dir.file_name().unwrap().to_str().unwrap().to_string();
        if !setup_file_path.exists() {
            return Err("setup file does not exist".to_string());
        }
        Ok(Self {
            name,
            setup_dir: setup_dir.to_path_buf(),
            setup_file_path: setup_file_path.to_path_buf(),
        })
    }
}

fn read_setup_headers_from_dir(dir: &Path) -> Vec<SetupHeader> {
    let mut headers = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let setup_dir = entry.path();
            let setup_file_path = setup_dir.join("setup.json");
            if setup_dir.is_dir() && setup_file_path.exists() {
                headers.push(SetupHeader {
                    name: setup_dir.file_name().unwrap().to_str().unwrap().to_string(),
                    setup_dir,
                    setup_file_path,
                });
            }
        }
    }
    headers
}

// =======================================
//              Raw Setup
// =======================================

#[derive(Debug, Deserialize)]
struct SetupServiceRaw {
    path: String,
    #[serde(rename = "type")] // "system" or "user"
    r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SetupFileLinkRaw {
    source: String,
    target: String,
    #[serde(default)]
    root: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum SetupMenuScriptItemRaw {
    Simple(String),
    Detailed { path: String, name: String },
}

#[derive(Debug, Deserialize)]
struct SetupFileRaw {
    install: Option<String>,
    links: Option<Vec<SetupFileLinkRaw>>,
    rc_scripts: Option<Vec<String>>,
    menu_scripts: Option<Vec<SetupMenuScriptItemRaw>>,
    services: Option<Vec<SetupServiceRaw>>,
    dependencies: Option<Vec<String>>,
}

// =======================================
//              Validated Setup
// =======================================

fn tilde_expand(input: &str) -> String {
    shellexpand::tilde(&input).into_owned()
}

fn tilde_expand_path(input: &str) -> PathBuf {
    PathBuf::from(tilde_expand(input))
}

fn replace_tokens(input: &str, area: &str, setup_dir: &Path) -> PathBuf {
    if input.starts_with("common:") {
        let input = input.split(":").nth(1).unwrap();
        return get_config().owl_path.join("common").join(area).join(input);
    } else if input.starts_with("local:") {
        let input = input.split(":").nth(1).unwrap();
        return setup_dir.join(input);
    } else {
        return get_config().owl_path.join(input);
    }
}

fn ensure_exists(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("path not found: {}", path.display()));
    }
    Ok(())
}

fn get_filename(path: &Path) -> Result<String, String> {
    path.file_name()
        .and_then(|n| n.to_str().map(|s| s.to_string()))
        .ok_or_else(|| format!("invalid filename: {}", path.display()))
}

// ---------- Setup Links ----------
struct ValidatedSetupLink {
    source_path: PathBuf,
    target_path: PathBuf,
    root: bool,
}

impl ValidatedSetupLink {
    fn make(raw: &SetupFileLinkRaw, setup_dir: &Path) -> Result<Self, String> {
        let source_path = replace_tokens(&tilde_expand(&raw.source), "", setup_dir);
        ensure_exists(&source_path)?;
        let target_path = tilde_expand_path(&raw.target);
        Ok(Self {
            source_path,
            target_path,
            root: raw.root.unwrap_or(false),
        })
    }
}

impl Linkable for ValidatedSetupLink {
    fn source_path(&self) -> PathBuf {
        self.source_path.clone()
    }
    fn target_path(&self) -> PathBuf {
        self.target_path.clone()
    }
    fn requires_root(&self) -> bool {
        self.root
    }
    fn display_info() -> &'static str {
        "Links"
    }
}

// ---------- RC Scripts ----------
struct ValidatedRunScript {
    name: String,
    path: PathBuf,
}

impl ValidatedRunScript {
    fn make(raw: &str, setup_dir: &Path, setup_name: &str) -> Result<Self, String> {
        let path = replace_tokens(&tilde_expand(raw), "rc", setup_dir);
        ensure_exists(&path)?;
        let filename = get_filename(&path)?;
        let name = format!("rc-{}-{}", setup_name, filename);
        Ok(Self { name, path })
    }
}

impl Linkable for ValidatedRunScript {
    fn source_path(&self) -> PathBuf {
        self.path.clone()
    }
    fn target_path(&self) -> PathBuf {
        get_owl_rc_path().join(&self.name)
    }
    fn display_info() -> &'static str {
        "RC Scripts"
    }
}

// ---------- Menu Scripts ----------
struct ValidatedSetupMenuScriptItem {
    path: PathBuf,
    name: String,
}

impl ValidatedSetupMenuScriptItem {
    fn make(raw: &SetupMenuScriptItemRaw, setup_dir: &Path) -> Result<Self, String> {
        let path: String = match raw {
            SetupMenuScriptItemRaw::Simple(p) => p.clone(),
            SetupMenuScriptItemRaw::Detailed { path, .. } => path.clone(),
        };
        let path = replace_tokens(&tilde_expand(&path), "menu-scripts", setup_dir);
        ensure_exists(&path)?;
        let name: String = match raw {
            SetupMenuScriptItemRaw::Simple(p) => get_filename(&PathBuf::from(p))?,
            SetupMenuScriptItemRaw::Detailed { name, .. } => name.clone(),
        };
        Ok(Self { path, name })
    }
}

impl Linkable for ValidatedSetupMenuScriptItem {
    fn source_path(&self) -> PathBuf {
        self.path.clone()
    }
    fn target_path(&self) -> PathBuf {
        get_owl_menu_scripts_path().join(&self.name)
    }
    fn display_info() -> &'static str {
        "Menu Scripts"
    }
}

// ---------- Services ----------
#[derive(Debug, Clone, Copy)]
enum ServiceScope {
    System,
    User,
}

impl ServiceScope {
    fn from_str_or_default(v: Option<String>) -> Self {
        match v
            .unwrap_or_else(|| "user".to_string())
            .to_lowercase()
            .as_str()
        {
            "system" => ServiceScope::System,
            _ => ServiceScope::User,
        }
    }
    fn is_root(self) -> bool {
        matches!(self, ServiceScope::System)
    }
    fn get_target_path(&self) -> PathBuf {
        match self {
            ServiceScope::System => PathBuf::from("/etc/systemd/system"),
            ServiceScope::User => {
                PathBuf::from(shellexpand::tilde("~/.config/systemd/user").into_owned())
            }
        }
    }
}

struct ValidatedSetupService {
    path: PathBuf,
    scope: ServiceScope,
    name: String,
    target_path: PathBuf,
}

impl ValidatedSetupService {
    fn make(raw: &SetupServiceRaw, setup_dir: &Path) -> Result<Self, String> {
        let scope = ServiceScope::from_str_or_default(raw.r#type.clone());
        let path = replace_tokens(&tilde_expand(&raw.path), "services", setup_dir);
        ensure_exists(&path)?;
        let name: String = get_filename(&path)?;

        Ok(Self {
            path: path.clone(),
            scope,
            name: name.to_string(),
            target_path: scope.get_target_path().join(name),
        })
    }

    fn enable_and_restart(&self) {
        match self.scope {
            ServiceScope::System => {
                let _ = Command::new("sudo")
                    .arg("systemctl")
                    .arg("daemon-reload")
                    .status();
                let _ = Command::new("sudo")
                    .arg("systemctl")
                    .arg("enable")
                    .arg("--now")
                    .arg(&self.name)
                    .status();
            }
            ServiceScope::User => {
                let _ = Command::new("systemctl")
                    .arg("--user")
                    .arg("daemon-reload")
                    .status();
                let _ = Command::new("systemctl")
                    .arg("--user")
                    .arg("enable")
                    .arg("--now")
                    .arg(&self.name)
                    .status();
            }
        }
    }
}

impl Linkable for ValidatedSetupService {
    fn source_path(&self) -> PathBuf {
        self.path.clone()
    }
    fn target_path(&self) -> PathBuf {
        self.target_path.clone()
    }
    fn requires_root(&self) -> bool {
        self.scope.is_root()
    }
    fn display_info() -> &'static str {
        "Services"
    }
}

// ---------- Dependencies ----------
struct ValidatedSetupDependency {
    name: String,
}

impl ValidatedSetupDependency {
    fn make(raw: &str) -> Result<Self, String> {
        let name = raw.to_string();
        // Ensure the dependency is a valid setup
        if load_setup_by_name(&name).is_err() {
            return Err(format!("dependency not found: {}", name));
        }
        Ok(Self { name })
    }
}

// ---------- Install Script ----------
struct ValidatedSetupInstallScript {
    path: PathBuf,
}

impl ValidatedSetupInstallScript {
    fn make(raw: &str, setup_dir: &Path) -> Result<Self, String> {
        let path = replace_tokens(&tilde_expand(raw), "", setup_dir);
        ensure_exists(&path)?;
        Ok(Self { path })
    }

    fn install(&self) {
        run_script(&self.path);
    }
}

// =======================================
//              Setup
// =======================================

#[derive(Clone, Copy)]
enum Operation {
    Link,
    Install,
    Systemd,
    Info,
    All,
}

impl Operation {
    fn description(&self) -> &str {
        match self {
            Operation::Link => "🔗 Links",
            Operation::Install => "📦 Installing",
            Operation::Systemd => "🧩 Systemd",
            Operation::Info => "ℹ️  Info",
            Operation::All => "🚀 All",
        }
    }
}

struct Setup {
    name: String,
    origin_dir: PathBuf,
    links: Vec<ValidatedSetupLink>,
    rc_scripts: Vec<ValidatedRunScript>,
    menu_scripts: Vec<ValidatedSetupMenuScriptItem>,
    services: Vec<ValidatedSetupService>,
    dependencies: Vec<ValidatedSetupDependency>,
    install_script: Option<ValidatedSetupInstallScript>,
}

impl Setup {
    fn make(setup_raw: &SetupFileRaw, setup_header: &SetupHeader) -> Result<Self, String> {
        fn validate_vec<T, U>(
            vec: Option<&Vec<T>>,
            make: impl Fn(&T) -> Result<U, String>,
        ) -> Result<Vec<U>, String> {
            vec.unwrap_or(&Vec::new())
                .iter()
                .map(|t| make(t))
                .collect::<Result<Vec<_>, _>>()
        }

        let links = validate_vec(setup_raw.links.as_ref(), |l| {
            ValidatedSetupLink::make(l, &setup_header.setup_dir)
        })?;

        let rc_scripts = validate_vec(setup_raw.rc_scripts.as_ref(), |s| {
            ValidatedRunScript::make(s, &setup_header.setup_dir, &setup_header.name)
        })?;

        let menu_scripts = validate_vec(setup_raw.menu_scripts.as_ref(), |s| {
            ValidatedSetupMenuScriptItem::make(s, &setup_header.setup_dir)
        })?;

        let services = validate_vec(setup_raw.services.as_ref(), |s| {
            ValidatedSetupService::make(s, &setup_header.setup_dir)
        })?;

        let dependencies = validate_vec(setup_raw.dependencies.as_ref(), |s| {
            ValidatedSetupDependency::make(s)
        })?;

        let install_script = setup_raw.install.as_ref().and_then(|install| {
            ValidatedSetupInstallScript::make(install, &setup_header.setup_dir).ok()
        });

        Ok(Setup {
            name: setup_header.name.clone(),
            origin_dir: setup_header.setup_dir.clone(),
            links,
            rc_scripts,
            menu_scripts,
            services,
            dependencies,
            install_script,
        })
    }

    fn edit(&self) {
        let links_path = self.origin_dir.join("setup.json");
        let editor = std::env::var("VISUAL")
            .ok()
            .or_else(|| std::env::var("EDITOR").ok())
            .unwrap_or_else(|| "vim".to_string());
        let mut cmd = Command::new(editor);
        cmd.arg(&links_path);
        match cmd.status() {
            Ok(status) => {
                if !status.success() {
                    eprintln!("Editor exited with non-zero status");
                }
            }
            Err(e) => eprintln!("Failed to launch editor: {}", e),
        }
    }

    fn run_linkables<T: Linkable>(items: &[T]) {
        if items.is_empty() {
            return;
        }
        print_subsection(T::display_info());
        for item in items {
            let src = item.source_path();
            let dst = item.target_path();
            let src_display = src.display().to_string().blue();
            let dst_display = dst.display().to_string().green();
            match item.link() {
                Ok(()) => println!("    {} → {} {}", src_display, dst_display, "✅"),
                Err(e) => println!(
                    "    {} → {} {} {}",
                    src_display, dst_display, "❌", e.message
                ),
            }
        }
    }

    fn link_once(&self) {
        Self::run_linkables(&self.links);
        Self::run_linkables(&self.rc_scripts);
        Self::run_linkables(&self.menu_scripts);
        Self::run_linkables(&self.services);
    }

    fn install_once(&self) {
        if let Some(script) = &self.install_script {
            println!("Installing {}", self.name.green());
            script.install();
        }
    }

    fn info_once(&self) {
        print_section("Setup:");
        println!("  {}", self.name.cyan());
        if !self.dependencies.is_empty() {
            println!(
                "Dependencies: {}",
                self.dependencies
                    .iter()
                    .map(|d| d.name.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
                    .yellow()
            );
        }
        if let Some(install) = &self.install_script {
            println!(
                "Install script: {}",
                install.path.display().to_string().green()
            );
        }
        println!("Links: {} files", self.links.len());
        println!("RC Scripts: {}", self.rc_scripts.len());
        println!("Menu Scripts: {}", self.menu_scripts.len());
        println!("Services: {} files", self.services.len());
    }

    fn systemd_once(&self) {
        Self::run_linkables(&self.services);
        for svc in &self.services {
            svc.enable_and_restart();
        }
    }

    fn apply_operation_once(&self, op: Operation) {
        let op_description = op.description();
        let op_description_colored = op_description.magenta().bold();
        let setup_name = self.name.cyan().bold();
        let setup_dir = self
            .origin_dir
            .join("setup.json")
            .display()
            .to_string()
            .green();

        println!("{} {} ({})", op_description_colored, setup_name, setup_dir);
        match op {
            Operation::Link => self.link_once(),
            Operation::Install => self.install_once(),
            Operation::Systemd => self.systemd_once(),
            Operation::Info => self.info_once(),
            Operation::All => {
                self.link_once();
                self.install_once();
                self.systemd_once();
            }
        }
    }

    fn run_op(&self, op: Operation, shallow: bool) {
        if shallow {
            self.apply_operation_once(op);
        } else {
            for_each_dep_depth_first(&self.name, |s| {
                s.apply_operation_once(op);
            });
        }
    }
}

#[derive(Debug, Error)]
enum SetupLoadError {
    #[error("Failed to read {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Invalid JSON in {path}: {source}")]
    Json {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("Validation error in {path}: {message}")]
    Validation { path: PathBuf, message: String },
}

fn load_setup_by_path(setup_path: &Path) -> Result<Setup, SetupLoadError> {
    let setup_header =
        SetupHeader::new(setup_path.to_path_buf()).map_err(|e| SetupLoadError::Validation {
            path: setup_path.to_path_buf(),
            message: e,
        })?;

    let setup_raw = std::fs::read_to_string(setup_path).map_err(|e| SetupLoadError::Io {
        path: setup_path.to_path_buf(),
        source: e,
    })?;

    let raw: SetupFileRaw = serde_json::from_str(&setup_raw).map_err(|e| SetupLoadError::Json {
        path: setup_path.to_path_buf(),
        source: e,
    })?;

    let setup = Setup::make(&raw, &setup_header).map_err(|e| SetupLoadError::Validation {
        path: setup_path.to_path_buf(),
        message: e,
    })?;
    Ok(setup)
}

#[derive(Debug, Error)]
enum SetupLoadByNameError {
    #[error("Failed to load setup by name: {nest_error} and {setup_error}")]
    Error {
        nest_error: SetupLoadError,
        setup_error: SetupLoadError,
    },
}

fn load_setup_by_name(name: &str) -> Result<Setup, SetupLoadByNameError> {
    let config = get_config();

    let nest_dir = config.owl_path.join("nests").join(name).join("setup.json");
    let setup_dir = config.owl_path.join("setups").join(name).join("setup.json");

    let nest_dir_setup = load_setup_by_path(&nest_dir);
    let setup_dir_setup = load_setup_by_path(&setup_dir);

    match (nest_dir_setup, setup_dir_setup) {
        (Ok(nest_dir_setup), Ok(_)) => Ok(nest_dir_setup),
        (Err(_), Ok(setup_dir_setup)) => Ok(setup_dir_setup),
        (Ok(nest_dir_setup), Err(_)) => Ok(nest_dir_setup),
        (Err(e1), Err(e2)) => Err(SetupLoadByNameError::Error {
            nest_error: e1,
            setup_error: e2,
        }),
    }
}

fn get_setup(name: &str) -> Setup {
    match load_setup_by_name(name) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} {}", "Error loading setup:".red(), e);
            std::process::exit(1);
        }
    }
}

fn validate_all_setups() {
    let config = get_config();
    let setups_dir = config.owl_path.join("setups");
    let nests_dir = config.owl_path.join("nests");

    let setups_headers = read_setup_headers_from_dir(&setups_dir);
    let nests_headers = read_setup_headers_from_dir(&nests_dir);
    let all_headers = [setups_headers, nests_headers].concat();

    let mut total_ok = 0usize;
    let mut total_err = 0usize;

    for header in all_headers {
        let setup = load_setup_by_path(&header.setup_file_path);
        match setup {
            Ok(_) => {
                println!("{} {}", "✓".green(), header.name.green());
                total_ok += 1;
            }
            Err(e) => {
                println!("{} {} {}", "✗".red(), header.name.red(), e);
                total_err += 1;
            }
        }
    }

    if total_ok + total_err > 0 {
        println!(
            "\nValidated total {}: {} ok, {} failed",
            (total_ok + total_err).to_string().bold(),
            total_ok.to_string().green(),
            total_err.to_string().red()
        );
    }
}

// =======================================
//              Nests
// =======================================

fn get_nest_path() -> Option<PathBuf> {
    let config = get_config();
    config.nest_path.clone().map(|p| p.join("setup.json"))
}

fn load_nest() -> Result<Setup, SetupLoadError> {
    let nest_path = match get_nest_path() {
        Some(p) => p,
        None => {
            return Err(SetupLoadError::Validation {
                path: get_config_path(),
                message: "No active nest found".to_string(),
            });
        }
    };
    return load_setup_by_path(&nest_path);
}

fn get_nest() -> Setup {
    match load_nest() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("No active nest found! {}", e.to_string().red());
            return switch_nest();
        }
    }
}

fn switch_nest() -> Setup {
    let mut config = get_config();

    let nests = list_nests();
    println!("Select a nest:");
    for (i, p) in nests.iter().enumerate() {
        println!("{}: {}", i + 1, p.name.cyan());
    }
    let mut idx: usize;
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        idx = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Invalid selection");
                continue;
            }
        };
        if idx == 0 || idx > nests.len() {
            eprintln!("Invalid selection");
            continue;
        }
        break;
    }

    let setup_header = nests[idx - 1].clone();

    config.nest_path = Some(setup_header.setup_dir.clone());
    save_config(config);
    println!("Switched nest to {}", setup_header.name.cyan());

    return get_setup(&setup_header.name);
}

fn list_nests() -> Vec<SetupHeader> {
    let config = get_config();
    read_setup_headers_from_dir(&config.owl_path.join("nests"))
}

// =======================================
//              CLI
// =======================================

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Config,
    Nest {
        #[command(subcommand)]
        nest_command: Option<NestCommands>,
        #[arg(long, default_value_t = false)]
        shallow: bool,
    },
    Sync,
    Setup {
        setup_name: String,
        #[command(subcommand)]
        setup_command: SetupCommands,
        #[arg(long, default_value_t = false)]
        shallow: bool,
    },
    SetupsValidate,
    Update,
}

#[derive(Subcommand, Clone, Copy)]
enum SetupCommands {
    Link,
    Info,
    Edit,
    Install,
    Systemd,
    All,
}

#[derive(Subcommand)]
enum NestCommands {
    Link,
    Info,
    Edit,
    Install,
    Systemd,
    All,
    Switch,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Config => print_config(),
        Commands::Nest {
            nest_command,
            shallow,
        } => {
            let nest = get_nest();
            match nest_command {
                None | Some(NestCommands::Info) => nest.run_op(Operation::Info, shallow),
                Some(NestCommands::Link) => nest.run_op(Operation::Link, shallow),
                Some(NestCommands::Install) => nest.run_op(Operation::Install, shallow),
                Some(NestCommands::Systemd) => nest.run_op(Operation::Systemd, shallow),
                Some(NestCommands::All) => nest.run_op(Operation::All, shallow),
                Some(NestCommands::Edit) => nest.edit(),
                Some(NestCommands::Switch) => {
                    let _ = switch_nest();
                }
            }
        }
        Commands::Sync => sync(),
        Commands::Setup {
            setup_name,
            setup_command,
            shallow,
        } => {
            let s = get_setup(&setup_name);

            match setup_command {
                SetupCommands::Link => s.run_op(Operation::Link, shallow),
                SetupCommands::Info => s.run_op(Operation::Info, shallow),
                SetupCommands::Edit => s.edit(),
                SetupCommands::Install => s.run_op(Operation::Install, shallow),
                SetupCommands::Systemd => s.run_op(Operation::Systemd, shallow),
                SetupCommands::All => s.run_op(Operation::All, shallow),
            }
        }
        Commands::SetupsValidate => validate_all_setups(),
        Commands::Update => run_update(),
    }
}

fn sync() {
    println!("Syncing");

    let config = get_config();
    let owl_sync_script_path = config.owl_path.join("common/scripts/owl-sync.sh");

    run_script(&owl_sync_script_path);
}
fn run_update() {
    let s = get_setup("owl");
    s.run_op(Operation::Install, true);
}

// =======================================
//              Utils
// =======================================

fn print_section(title: &str) {
    println!("{}", title.blue().bold());
}

fn print_subsection(title: &str) {
    println!("  {}", title.green().bold());
}

fn print_kv(label: &str, value: &str) {
    println!("  {} {}", format!("{}:", label).white(), value.cyan());
}

fn run_script(script_path: &Path) {
    let display_path = script_path.display().to_string();
    if !script_path.exists() {
        eprintln!("Script not found, skipping: {}", display_path);
        return;
    }

    println!("Running script: {}", display_path);

    let mut cmd = Command::new("bash");
    cmd.arg(&script_path);

    let mut child = cmd
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    // Read and print stdout/stderr concurrently to avoid deadlocks
    let stdout_handle = if let Some(stdout) = child.stdout.take() {
        Some(std::thread::spawn(move || {
            let stdout_reader = BufReader::new(stdout);
            for line in stdout_reader.lines().flatten() {
                println!("{}", line);
            }
        }))
    } else {
        None
    };

    let stderr_handle = if let Some(stderr) = child.stderr.take() {
        Some(std::thread::spawn(move || {
            let stderr_reader = BufReader::new(stderr);
            for line in stderr_reader.lines().flatten() {
                eprintln!("{}", line);
            }
        }))
    } else {
        None
    };

    // Wait for the command to finish and check the status
    let status = child.wait().expect("Failed to wait on child process");
    if let Some(h) = stdout_handle {
        let _ = h.join();
    }
    if let Some(h) = stderr_handle {
        let _ = h.join();
    }
    if !status.success() {
        eprintln!("Command failed with exit code: {:?}", status.code());
    } else {
        println!("Script completed successfully");
    }
}

struct LinkingError {
    message: String,
}

trait Linkable {
    fn source_path(&self) -> PathBuf;
    fn target_path(&self) -> PathBuf;
    fn requires_root(&self) -> bool {
        false
    }
    fn display_info() -> &'static str;

    fn link(&self) -> Result<(), LinkingError> {
        let target_path = self.target_path();
        let root = self.requires_root();
        let source_path = self.source_path();

        // Carefully remove existing targets:
        // - If symlink: remove the symlink only
        // - If file: remove the file
        // - If directory: remove ONLY if empty; otherwise fail with a clear message
        if target_path.is_symlink() {
            if let Err(e) = std::fs::remove_file(&target_path) {
                return Err(LinkingError {
                    message: format!("remove old symlink: {}", e),
                });
            }
        } else if target_path.exists() {
            match std::fs::symlink_metadata(&target_path) {
                Ok(meta) => {
                    if meta.is_file() {
                        if let Err(e) = std::fs::remove_file(&target_path) {
                            return Err(LinkingError {
                                message: format!("remove old file: {}", e),
                            });
                        }
                    } else if meta.is_dir() {
                        match std::fs::read_dir(&target_path) {
                            Ok(mut it) => {
                                let is_empty = it.next().is_none();
                                if is_empty {
                                    if let Err(e) = std::fs::remove_dir(&target_path) {
                                        return Err(LinkingError {
                                            message: format!("remove empty dir: {}", e),
                                        });
                                    }
                                } else {
                                    return Err(LinkingError {
                                        message: format!(
                                            "target is a non-empty directory: {}",
                                            target_path.display()
                                        ),
                                    });
                                }
                            }
                            Err(e) => {
                                return Err(LinkingError {
                                    message: format!("inspect target dir: {}", e),
                                });
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(LinkingError {
                        message: format!("stat target: {}", e),
                    });
                }
            }
        }

        if let Some(parent) = target_path.parent() {
            if !parent.exists() {
                if root {
                    let status = Command::new("sudo")
                        .arg("mkdir")
                        .arg("-p")
                        .arg(parent)
                        .status();
                    match status {
                        Ok(s) if s.success() => {}
                        Ok(s) => {
                            return Err(LinkingError {
                                message: format!("sudo mkdir -p failed with code {:?}", s.code()),
                            })
                        }
                        Err(e) => {
                            return Err(LinkingError {
                                message: format!("exec sudo mkdir -p: {}", e),
                            })
                        }
                    }
                } else if let Err(e) = fs::create_dir_all(parent) {
                    return Err(LinkingError {
                        message: format!("mkdir: {}", e),
                    });
                }
            }
        }

        if root {
            let output = Command::new("sudo")
                .arg("ln")
                .arg("-s")
                .arg(&source_path)
                .arg(&target_path)
                .output();
            match output {
                Ok(o) if o.status.success() => Ok(()),
                Ok(o) => {
                    let msg = format!("sudo ln failed: {}", String::from_utf8_lossy(&o.stderr));
                    Err(LinkingError { message: msg })
                }
                Err(e) => Err(LinkingError {
                    message: format!("exec sudo ln: {}", e),
                }),
            }
        } else {
            std::os::unix::fs::symlink(&source_path, &target_path).map_err(|e| LinkingError {
                message: format!("symlink: {}", e),
            })
        }
    }
}

fn for_each_dep_depth_first<F>(start_name: &str, mut f: F)
where
    F: FnMut(&Setup),
{
    let mut visited = std::collections::HashSet::new();
    fn walk<F>(name: &str, visited: &mut std::collections::HashSet<String>, f: &mut F)
    where
        F: FnMut(&Setup),
    {
        if visited.contains(name) {
            return;
        }
        visited.insert(name.to_string());
        let setup = get_setup(name);
        for dep in &setup.dependencies {
            walk(dep.name.as_str(), visited, f);
        }
        f(&setup);
    }
    walk(start_name, &mut visited, &mut f);
}
