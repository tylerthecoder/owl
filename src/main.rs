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
    println!("{}", "Owl Config".blue().bold());
    println!(
        "  owl_path: {}",
        config.owl_path.display().to_string().cyan()
    );
    match &config.nest_path {
        Some(p) => println!("  active_root: {}", p.display().to_string().cyan()),
        None => println!("  active_root: {}", "(none)".yellow()),
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
//              Raw Setup
// =======================================
fn resolve_tokenized_path(input: &str, setup_dir: &Path, common_prefix: &str) -> PathBuf {
    let expanded = shellexpand::tilde(input).into_owned();
    let s = expanded.as_str();
    if Path::new(s).is_absolute() {
        return PathBuf::from(s);
    }
    if s.starts_with("common:") {
        let relative = &s[7..];
        return get_config()
            .owl_path
            .join("common")
            .join(common_prefix)
            .join(relative);
    }
    if s.starts_with("local:") {
        let relative = &s[6..];
        return setup_dir.join(relative);
    }
    get_config().owl_path.join(s)
}

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
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    install: Option<String>,
    #[serde(default)]
    links: Option<Vec<SetupFileLinkRaw>>,
    #[serde(default)]
    rc_scripts: Option<Vec<String>>,
    #[serde(default)]
    menu_scripts: Option<Vec<SetupMenuScriptItemRaw>>,
    #[serde(default)]
    services: Option<Vec<SetupServiceRaw>>,
    #[serde(default)]
    dependencies: Option<Vec<String>>,
}

impl SetupFileRaw {
    fn validate(&self, setup_dir: &Path, setup_name: &str) -> Result<Setup, String> {
        let name = self.name.clone().unwrap_or_else(|| setup_name.to_string());

        let links = self
            .links
            .as_ref()
            .unwrap_or(&Vec::new())
            .into_iter()
            .map(|l| ValidatedSetupLink::make(&l, setup_dir))
            .collect::<Result<Vec<_>, _>>()?;

        let rc_scripts = self
            .rc_scripts
            .as_ref()
            .unwrap_or(&Vec::new())
            .into_iter()
            .map(|s| ValidatedRunScript::make(&s, setup_dir, &name))
            .collect::<Result<Vec<_>, _>>()?;

        let menu_scripts = self
            .menu_scripts
            .as_ref()
            .unwrap_or(&Vec::new())
            .into_iter()
            .map(|s| ValidatedSetupMenuScriptItem::make(&s, setup_dir))
            .collect::<Result<Vec<_>, _>>()?;

        let services = self
            .services
            .as_ref()
            .unwrap_or(&Vec::new())
            .into_iter()
            .map(|s| ValidatedSetupService::make(&s, setup_dir))
            .collect::<Result<Vec<_>, _>>()?;

        let dependencies = self
            .dependencies
            .as_ref()
            .unwrap_or(&Vec::new())
            .into_iter()
            .map(|s| ValidatedSetupDependency::make(&s))
            .collect::<Result<Vec<_>, _>>()?;

        let install_script = if let Some(install) = self.install.as_ref() {
            Some(ValidatedSetupInstallScript::make(install, setup_dir)?)
        } else {
            None
        };

        Ok(Setup {
            name,
            origin_dir: setup_dir.to_path_buf(),
            links,
            rc_scripts,
            menu_scripts,
            services,
            dependencies,
            install_script,
        })
    }
}

// =======================================
//              Validated Setup
// =======================================

// ---------- Setup Links ----------
struct ValidatedSetupLink {
    source_path: PathBuf,
    target_path: PathBuf,
    root: bool,
}

impl ValidatedSetupLink {
    fn make(raw: &SetupFileLinkRaw, setup_dir: &Path) -> Result<Self, String> {
        let source_path = resolve_tokenized_path(&raw.source, setup_dir, "");
        if !source_path.exists() {
            return Err(format!("link source not found: {}", source_path.display()));
        }
        let target_path = resolve_tokenized_path(&raw.target, setup_dir, "");
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
    fn display_info() -> String {
        return "🔗 Links".to_string();
    }
}

// ---------- RC Scripts ----------
struct ValidatedRunScript {
    name: String,
    path: PathBuf,
}

impl ValidatedRunScript {
    fn make(raw: &str, setup_dir: &Path, setup_name: &str) -> Result<Self, String> {
        let path = resolve_tokenized_path(raw, setup_dir, "rc");
        if !path.exists() {
            return Err(format!("rc script missing: {}", path.display()));
        }
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| "invalid script filename".to_string())?;
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
    fn display_info() -> String {
        return "📜 RC Scripts".to_string();
    }
}

// ---------- Menu Scripts ----------
struct ValidatedSetupMenuScriptItem {
    path: PathBuf,
    name: String,
}

impl ValidatedSetupMenuScriptItem {
    fn make(raw: &SetupMenuScriptItemRaw, setup_dir: &Path) -> Result<Self, String> {
        match raw {
            SetupMenuScriptItemRaw::Simple(p) => {
                let path = resolve_tokenized_path(p, setup_dir, "menu-scripts");
                if !path.exists() {
                    return Err(format!("menu script missing: {}", path.display()));
                }
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("menu-script")
                    .to_string();
                Ok(Self { path, name })
            }
            SetupMenuScriptItemRaw::Detailed { path, name } => {
                let path = resolve_tokenized_path(path, setup_dir, "menu-scripts");
                if !path.exists() {
                    return Err(format!("menu script missing: {}", path.display()));
                }
                Ok(Self {
                    path,
                    name: name.clone(),
                })
            }
        }
    }
}

impl Linkable for ValidatedSetupMenuScriptItem {
    fn source_path(&self) -> PathBuf {
        self.path.clone()
    }
    fn target_path(&self) -> PathBuf {
        get_owl_menu_scripts_path().join(&self.name)
    }
    fn display_info() -> String {
        return "🧭 Menu Scripts".to_string();
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
        let path = resolve_tokenized_path(&raw.path, setup_dir, "services");
        if !path.exists() {
            return Err(format!("service file missing: {}", path.display()));
        }
        let path_copy = path.clone();
        let name = path_copy
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| "invalid service filename".to_string())?;
        let target_path = match scope {
            ServiceScope::System => PathBuf::from("/etc/systemd/system").join(name),
            ServiceScope::User => {
                PathBuf::from(shellexpand::tilde("~/.config/systemd/user").into_owned()).join(name)
            }
        };
        Ok(Self {
            path,
            scope,
            name: name.to_string(),
            target_path,
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
                    .arg(&self.name)
                    .status();
                let _ = Command::new("sudo")
                    .arg("systemctl")
                    .arg("restart")
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
                    .arg(&self.name)
                    .status();
                let _ = Command::new("systemctl")
                    .arg("--user")
                    .arg("restart")
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
    fn display_info() -> String {
        return "🧩 Services".to_string();
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
        if load_setup(&name).is_err() {
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
        let path = resolve_tokenized_path(raw, setup_dir, "");
        if !path.exists() {
            return Err(format!("install script missing: {}", path.display()));
        }
        Ok(Self { path })
    }

    fn install(&self) {
        run_script(self.path.clone());
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
        println!("  {}", T::display_info().green().bold());
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
        println!("\n{} {}", "Setup:".blue().bold(), self.name.cyan());
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

        println!("{} {} ({} )", op_description_colored, setup_name, setup_dir);
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
    #[error("setup.json not found in either nests or setups directory (looked for {path})")]
    NotFound { path: PathBuf },
}

fn read_setup_file(setup_json_path: &Path) -> Result<SetupFileRaw, SetupLoadError> {
    let setup_raw = std::fs::read_to_string(setup_json_path).map_err(|e| SetupLoadError::Io {
        path: setup_json_path.to_path_buf(),
        source: e,
    })?;
    let parsed: SetupFileRaw =
        serde_json::from_str(&setup_raw).map_err(|e| SetupLoadError::Json {
            path: setup_json_path.to_path_buf(),
            source: e,
        })?;
    Ok(parsed)
}

fn load_setup(name: &str) -> Result<Setup, SetupLoadError> {
    let config = get_config();

    let nest_dir = config.owl_path.join("nests").join(name);
    let setup_dir = config.owl_path.join("setups").join(name);

    let (chosen_dir, path) = if nest_dir.join("setup.json").exists() {
        (nest_dir.clone(), nest_dir.join("setup.json"))
    } else if setup_dir.join("setup.json").exists() {
        (setup_dir.clone(), setup_dir.join("setup.json"))
    } else {
        return Err(SetupLoadError::NotFound {
            path: setup_dir.join("setup.json"),
        });
    };

    let raw = read_setup_file(&path)?;
    match raw.validate(&chosen_dir, name) {
        Ok(s) => Ok(s),
        Err(e) => Err(SetupLoadError::Validation { path, message: e }),
    }
}

fn get_setup(name: &str) -> Setup {
    match load_setup(name) {
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

    fn collect(dir: &Path) -> Vec<String> {
        let mut out = Vec::new();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() && path.join("setup.json").exists() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        out.push(name.to_string());
                    }
                }
            }
        }
        out.sort();
        out
    }

    let mut total_ok = 0usize;
    let mut total_err = 0usize;

    for (label, base_dir, names) in [
        ("setups", setups_dir.as_path(), collect(&setups_dir)),
        ("nests", nests_dir.as_path(), collect(&nests_dir)),
    ] {
        if names.is_empty() {
            continue;
        }
        println!("\n{} {}", "Validating".blue().bold(), label.cyan().bold());
        let mut num_ok = 0usize;
        let mut num_err = 0usize;
        for name in names {
            let dir = base_dir.join(&name);
            let json = dir.join("setup.json");
            match read_setup_file(&json).and_then(|raw| {
                raw.validate(&dir, &name)
                    .map_err(|e| SetupLoadError::Validation {
                        path: json.clone(),
                        message: e,
                    })
            }) {
                Ok(_) => {
                    print_ok_setup(&name);
                    num_ok += 1;
                }
                Err(e) => {
                    print_err_setup(&name, &e);
                    num_err += 1;
                }
            }
        }
        println!(
            "Validated {} {}: {} ok, {} failed",
            (num_ok + num_err).to_string().bold(),
            label,
            num_ok.to_string().green(),
            (if num_err == 0 {
                num_err.to_string().green()
            } else {
                num_err.to_string().red()
            })
        );
        total_ok += num_ok;
        total_err += num_err;
    }
    if total_ok + total_err > 0 {
        println!(
            "\nValidated total {}: {} ok, {} failed",
            (total_ok + total_err).to_string().bold(),
            total_ok.to_string().green(),
            (if total_err == 0 {
                total_err.to_string().green()
            } else {
                total_err.to_string().red()
            })
        );
    }
}

// =======================================
//              Nests
// =======================================

fn load_nest() -> Option<Setup> {
    let config = get_config();
    if let Some(nest_dir) = config.nest_path.clone() {
        if let Some(os) = nest_dir.file_name() {
            if let Some(name) = os.to_str() {
                let setup = get_setup(name);
                return Some(setup);
            }
        }
        eprintln!(
            "Invalid active root setup path in config: {}",
            nest_dir.display()
        );
        return None;
    }
    None
}

fn get_nest() -> Setup {
    match load_nest() {
        Some(s) => s,
        None => {
            eprintln!("No active root setup found!");
            return switch_nest(None);
        }
    }
}

fn switch_nest(name: Option<String>) -> Setup {
    let mut config = get_config();

    let target_path = match name {
        Some(n) => {
            let p = PathBuf::from(&n);
            let resolved = if p.components().count() == 1 {
                Path::join(&config.owl_path, Path::new(&format!("nests/{}", n)))
            } else {
                p
            };
            if !resolved.join("setup.json").exists() {
                eprintln!(
                    "Nest '{}' not found (expected {}/setup.json)",
                    n,
                    resolved.display()
                );
                std::process::exit(1);
            }
            resolved
        }
        None => {
            let nests = list_nests();
            if nests.is_empty() {
                eprintln!("No nests found under nests/");
                std::process::exit(1);
            }
            println!("Select a nest:");
            for (i, p) in nests.iter().enumerate() {
                if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                    println!("{}: {}", i + 1, name);
                } else {
                    println!("{}: {}", i + 1, p.display());
                }
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
            nests[idx - 1].clone()
        }
    };

    let setup_name = target_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_else(|| {
            eprintln!("Invalid nest path: {}", target_path.display());
            std::process::exit(1);
        })
        .to_string();
    let setup = get_setup(&setup_name);

    config.nest_path = Some(target_path.clone());
    save_config(config);
    println!("Switched nest to {}", setup_name.cyan());

    return setup;
}

fn list_nests() -> Vec<PathBuf> {
    let config = get_config();
    let nests_dir = Path::join(&config.owl_path, Path::new("nests"));
    let mut nests = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&nests_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.join("setup.json").exists() {
                nests.push(path);
            }
        }
    }
    nests
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
    Switch { name: Option<String> },
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
                Some(NestCommands::Switch { name }) => {
                    let _ = switch_nest(name);
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
    let owl_sync_script_path =
        Path::join(&config.owl_path, Path::new("common/scripts/owl-sync.sh"));

    run_script(owl_sync_script_path);
}
fn run_update() {
    let s = get_setup("owl");
    s.run_op(Operation::Install, true);
}

// =======================================
//              Utils
// =======================================

fn run_script(script_path: PathBuf) {
    let display_path = script_path.display().to_string();
    if !Path::new(&script_path).exists() {
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

// Small printing helpers used in validation output
fn print_ok_setup(name: &str) {
    println!("{} {}", "✓".green(), name.green());
}
fn print_err_setup(name: &str, err: &SetupLoadError) {
    eprintln!(
        "{} {} -> {}",
        "✗".red(),
        name.red(),
        format!("{}", err).red()
    );
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
    fn display_info() -> String;

    fn link(&self) -> Result<(), LinkingError> {
        let target_path = self.target_path();
        let root = self.requires_root();
        let source_path = self.source_path();

        if target_path.exists() || target_path.is_symlink() {
            if let Err(e) = std::fs::remove_file(target_path.clone()) {
                return Err(LinkingError {
                    message: format!("remove old: {}", e),
                });
            }
        }

        if let Some(parent) = target_path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
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
