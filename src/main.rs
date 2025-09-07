use clap::{Parser, Subcommand};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

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
    let config_raw = serde_json::to_string(&config).expect("Unable to serialize config");
    std::fs::write(config_path, config_raw).expect("Unable to write config file");
    config
}

// =======================================
//              Setups
// =======================================
fn resolve_tokenized_path(
    input: &str,
    setup_dir: &Path,
    config: &Config,
    common_prefix: &str,
) -> PathBuf {
    let expanded = shellexpand::tilde(input).into_owned();
    let s = expanded.as_str();
    if Path::new(s).is_absolute() {
        return PathBuf::from(s);
    }
    if s.starts_with("common:") {
        let relative = &s[7..];
        return config
            .owl_path
            .join("common")
            .join(common_prefix)
            .join(relative);
    }
    if s.starts_with("local:") {
        let relative = &s[6..];
        return setup_dir.join(relative);
    }
    config.owl_path.join(s)
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
    fn validate(
        &self,
        setup_dir: &Path,
        setup_name: &str,
        config: &Config,
    ) -> Result<Setup, String> {
        let name = self.name.clone().unwrap_or_else(|| setup_name.to_string());

        let links = self
            .links
            .as_ref()
            .unwrap_or(&Vec::new())
            .into_iter()
            .map(|l| ValidatedSetupLink::make(&l, setup_dir, config))
            .collect::<Result<Vec<_>, _>>()?;

        let rc_scripts = self
            .rc_scripts
            .as_ref()
            .unwrap_or(&Vec::new())
            .into_iter()
            .map(|s| ValidatedRunScript::make(&s, setup_dir, &name, config))
            .collect::<Result<Vec<_>, _>>()?;

        let menu_scripts = self
            .menu_scripts
            .as_ref()
            .unwrap_or(&Vec::new())
            .into_iter()
            .map(|s| ValidatedSetupMenuScriptItem::make(&s, setup_dir, config))
            .collect::<Result<Vec<_>, _>>()?;

        let services = self
            .services
            .as_ref()
            .unwrap_or(&Vec::new())
            .into_iter()
            .map(|s| ValidatedSetupService::make(&s, setup_dir, config))
            .collect::<Result<Vec<_>, _>>()?;

        let dependencies = self
            .dependencies
            .as_ref()
            .unwrap_or(&Vec::new())
            .into_iter()
            .map(|s| ValidatedSetupDependency::make(&s))
            .collect::<Result<Vec<_>, _>>()?;

        let install_script = if let Some(install) = self.install.as_ref() {
            Some(ValidatedSetupInstallScript::make(
                install, setup_dir, config,
            )?)
        } else {
            None
        };

        Ok(Setup {
            name,
            links,
            rc_scripts,
            menu_scripts,
            services,
            dependencies,
            install_script,
        })
    }
}

struct ValidatedSetupLink {
    source_path: PathBuf,
    target_path: PathBuf,
    root: bool,
}

impl ValidatedSetupLink {
    fn make(raw: &SetupFileLinkRaw, setup_dir: &Path, config: &Config) -> Result<Self, String> {
        let source_path = resolve_tokenized_path(&raw.source, setup_dir, config, "");
        if !source_path.exists() {
            return Err(format!("link source not found: {}", source_path.display()));
        }
        let target_path = resolve_tokenized_path(&raw.target, setup_dir, config, "");
        Ok(Self {
            source_path,
            target_path,
            root: raw.root.unwrap_or(false),
        })
    }

    fn link(&self, quiet: bool) -> Result<(), String> {
        link_paths(&self.source_path, &self.target_path, self.root, quiet)
    }
}

struct ValidatedRunScript {
    name: String,
    path: PathBuf,
}

impl ValidatedRunScript {
    fn make(
        raw: &str,
        setup_dir: &Path,
        setup_name: &str,
        config: &Config,
    ) -> Result<Self, String> {
        let path = resolve_tokenized_path(raw, setup_dir, config, "rc");
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

    fn link_into_rc(&self) -> Result<PathBuf, String> {
        let dest = get_owl_rc_path().join(&self.name);
        link_paths(&self.path, &dest, false, true)?;
        Ok(dest)
    }
}

struct ValidatedSetupMenuScriptItem {
    path: PathBuf,
    name: String,
}

impl ValidatedSetupMenuScriptItem {
    fn make(
        raw: &SetupMenuScriptItemRaw,
        setup_dir: &Path,
        config: &Config,
    ) -> Result<Self, String> {
        match raw {
            SetupMenuScriptItemRaw::Simple(p) => {
                let path = resolve_tokenized_path(p, setup_dir, config, "menu-scripts");
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
                let path = resolve_tokenized_path(path, setup_dir, config, "menu-scripts");
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

    fn link_into_menu(&self) -> Result<PathBuf, String> {
        let dest_dir = get_owl_menu_scripts_path().join(&self.name);
        link_paths(&self.path, &dest_dir, false, true)?;
        Ok(dest_dir)
    }
}

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
    fn make(raw: &SetupServiceRaw, setup_dir: &Path, config: &Config) -> Result<Self, String> {
        let scope = ServiceScope::from_str_or_default(raw.r#type.clone());
        let path = resolve_tokenized_path(&raw.path, setup_dir, config, "services");
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

    fn link(&self, quiet: bool) -> Result<(), String> {
        link_paths(&self.path, &self.target_path, self.scope.is_root(), quiet)
    }

    fn enable_and_restart(&self) {
        match self.scope {
            ServiceScope::System => {
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

struct ValidatedSetupInstallScript {
    path: PathBuf,
}

impl ValidatedSetupInstallScript {
    fn make(raw: &str, setup_dir: &Path, config: &Config) -> Result<Self, String> {
        let path = resolve_tokenized_path(raw, setup_dir, config, "");
        Ok(Self { path })
    }

    fn install(&self) {
        run_script(self.path.clone());
    }
}

struct Setup {
    name: String,
    links: Vec<ValidatedSetupLink>,
    rc_scripts: Vec<ValidatedRunScript>,
    menu_scripts: Vec<ValidatedSetupMenuScriptItem>,
    services: Vec<ValidatedSetupService>,
    dependencies: Vec<ValidatedSetupDependency>,
    install_script: Option<ValidatedSetupInstallScript>,
}

impl Setup {
    // ========= Recursion helper =========
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
    fn link_files(&self) {
        if self.links.is_empty() {
            return;
        }
        println!("  {}", "🔗 Links:".green().bold());
        for link in &self.links {
            match link.link(true) {
                Ok(()) => println!(
                    "    {} → {} {}",
                    link.source_path.display().to_string().blue(),
                    link.target_path.display().to_string().green(),
                    "✅"
                ),
                Err(e) => println!(
                    "    {} → {} {} {}",
                    link.source_path.display().to_string().blue(),
                    link.target_path.display().to_string().red(),
                    "❌",
                    e
                ),
            }
        }
    }

    fn link_rc_scripts(&self) {
        if self.rc_scripts.is_empty() {
            return;
        }
        println!("  {}", "📜 RC Scripts:".yellow().bold());
        for rc in &self.rc_scripts {
            match rc.link_into_rc() {
                Ok(dest) => println!(
                    "    {} → {} {}",
                    rc.path.display().to_string().blue(),
                    dest.display().to_string().green(),
                    "✅"
                ),
                Err(e) => println!(
                    "    {} → {} {} {}",
                    rc.path.display().to_string().blue(),
                    "~/.config/owl-rc".red(),
                    "❌",
                    e
                ),
            }
        }
    }

    fn link_menu_scripts(&self) {
        if self.menu_scripts.is_empty() {
            return;
        }
        println!("  {}", "🧭 Menu Scripts:".cyan().bold());
        for menu in &self.menu_scripts {
            match menu.link_into_menu() {
                Ok(dest) => println!(
                    "    {} → {} {}",
                    menu.path.display().to_string().blue(),
                    dest.display().to_string().green(),
                    "✅"
                ),
                Err(e) => println!(
                    "    {} → {} {} {}",
                    menu.path.display().to_string().blue(),
                    "~/.config/owl/menu-scripts".red(),
                    "❌",
                    e
                ),
            }
        }
    }

    fn link_services(&self) {
        if self.services.is_empty() {
            return;
        }
        println!("  {}", "🧩 Services:".cyan().bold());
        for svc in &self.services {
            match svc.link(true) {
                Ok(()) => println!(
                    "    {} → {} ({}) {}",
                    svc.path.display().to_string().blue(),
                    svc.target_path.display().to_string().green(),
                    match svc.scope {
                        ServiceScope::System => "system",
                        ServiceScope::User => "user",
                    },
                    "✅"
                ),
                Err(e) => println!(
                    "    {} → {} ({}) {} {}",
                    svc.path.display().to_string().blue(),
                    svc.target_path.display().to_string().red(),
                    match svc.scope {
                        ServiceScope::System => "system",
                        ServiceScope::User => "user",
                    },
                    "❌",
                    e
                ),
            }
        }
    }

    // ========= Setup Runners =========
    fn run_enable_and_restart_services(&self) {
        for svc in &self.services {
            svc.enable_and_restart();
        }
    }

    // One-shot runners (no recursion)
    fn install_once(&self) {
        if let Some(script) = &self.install_script {
            println!("Installing {}", self.name.green());
            script.install();
        }
    }

    fn link_once(&self) {
        self.link_files();
        self.link_rc_scripts();
        self.link_menu_scripts();
        self.link_services();
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
        self.link_services();
        self.run_enable_and_restart_services();
    }

    // ========= Recursive runners =========
    fn run_link_recursive(&self) {
        Self::for_each_dep_depth_first(&self.name, |s| {
            println!(
                "{} {} (setups/{}/setup.json)",
                "🚀 Linking".magenta().bold(),
                s.name.cyan().bold(),
                s.name
            );
            s.link_once();
        });
    }

    fn run_install_recursive(&self) {
        Self::for_each_dep_depth_first(&self.name, |s| s.install_once());
    }

    fn run_systemd_recursive(&self) {
        Self::for_each_dep_depth_first(&self.name, |s| s.systemd_once());
    }

    fn run_info_recursive(&self) {
        Self::for_each_dep_depth_first(&self.name, |s| s.info_once());
    }

    fn run_all_recursive(&self) {
        Self::for_each_dep_depth_first(&self.name, |s| {
            s.link_once();
            s.install_once();
            s.systemd_once();
        });
    }

    // ========= Public entry surface expected by CLI =========
    fn run_link(&self, shallow: bool) {
        if shallow {
            self.link_once();
        } else {
            self.run_link_recursive();
        }
    }

    fn run_install(&self, shallow: bool) {
        if shallow {
            self.install_once();
        } else {
            self.run_install_recursive();
        }
    }

    fn run_systemd(&self, shallow: bool) {
        if shallow {
            self.systemd_once();
        } else {
            self.run_systemd_recursive();
        }
    }

    fn run_info(&self, shallow: bool) {
        if shallow {
            self.info_once();
        } else {
            self.run_info_recursive();
        }
    }

    fn run_all(&self, shallow: bool) {
        if shallow {
            self.link_once();
            self.install_once();
            self.systemd_once();
        } else {
            self.run_all_recursive();
        }
    }

    fn run_edit(&self) {
        let config = get_config();
        let links_path = Path::join(
            &config.owl_path,
            Path::new(&format!("setups/{}/setup.json", self.name)),
        );
        let mut cmd = Command::new("vim");
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
}

#[derive(Debug)]
struct SetupLoadError {
    setup_path: PathBuf,
    error: Box<dyn std::error::Error>,
}

fn read_setup_file(setup_json_path: &Path) -> Result<SetupFileRaw, SetupLoadError> {
    let setup_raw = std::fs::read_to_string(setup_json_path).map_err(|e| SetupLoadError {
        setup_path: setup_json_path.to_path_buf(),
        error: Box::new(e),
    })?;
    let parsed: SetupFileRaw = serde_json::from_str(&setup_raw).map_err(|e| SetupLoadError {
        setup_path: setup_json_path.to_path_buf(),
        error: Box::new(e),
    })?;
    Ok(parsed)
}

fn load_setup(name: &str) -> Result<Setup, SetupLoadError> {
    let config = get_config();

    // Check both nests and setups directories
    let nest_dir = config.owl_path.join("nests").join(name);
    let setup_dir = config.owl_path.join("setups").join(name);

    let (chosen_dir, path) = if nest_dir.join("setup.json").exists() {
        (nest_dir.clone(), nest_dir.join("setup.json"))
    } else if setup_dir.join("setup.json").exists() {
        (setup_dir.clone(), setup_dir.join("setup.json"))
    } else {
        return Err(SetupLoadError {
            setup_path: setup_dir.join("setup.json"),
            error: "setup.json not found in either nests or setups directory".into(),
        });
    };

    let raw = read_setup_file(&path)?;
    match raw.validate(&chosen_dir, name, &config) {
        Ok(s) => Ok(s),
        Err(e) => Err(SetupLoadError {
            setup_path: path,
            error: e.into(),
        }),
    }
}

fn get_setup(name: &str) -> Setup {
    match load_setup(name) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "Error loading setup at {}: {}",
                e.setup_path.display(),
                e.error
            );
            std::process::exit(1);
        }
    }
}

fn validate_all_setups() {
    let config = get_config();
    let setups_dir = config.owl_path.join("setups");
    let mut names: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&setups_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.join("setup.json").exists() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    names.push(name.to_string());
                }
            }
        }
    }
    names.sort();

    let mut num_ok = 0usize;
    let mut num_err = 0usize;
    for name in names {
        let setup_dir = setups_dir.join(&name);
        let setup_json = setup_dir.join("setup.json");
        match read_setup_file(&setup_json).and_then(|raw| {
            raw.validate(&setup_dir, &name, &config)
                .map_err(|e| SetupLoadError {
                    setup_path: setup_json.clone(),
                    error: e.into(),
                })
        }) {
            Ok(_) => {
                println!("{} {}", "✓".green(), name.green());
                num_ok += 1;
            }
            Err(e) => {
                eprintln!(
                    "{} {} -> {}",
                    "✗".red(),
                    name.red(),
                    format!("{}", e.error).red()
                );
                num_err += 1;
            }
        }
    }
    println!(
        "\nValidated {} setups: {} ok, {} failed",
        (num_ok + num_err).to_string().bold(),
        num_ok.to_string().green(),
        (if num_err == 0 {
            num_err.to_string().green()
        } else {
            num_err.to_string().red()
        })
    );
}

// =======================================
//              Nests
// =======================================

fn load_nest() -> Option<Setup> {
    let config = get_config();
    if let Some(nest_dir) = config.nest_path.clone() {
        let setup_name = nest_dir.file_name().unwrap().to_str().unwrap().to_string();
        let setup = get_setup(&setup_name);
        return Some(setup);
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

fn switch_nest(path: Option<String>) -> Setup {
    let mut config = get_config();

    let target_path = match path {
        Some(p) => PathBuf::from(p),
        None => {
            let nests = list_nests();
            if nests.is_empty() {
                eprintln!("No nests found under nests/");
                std::process::exit(1);
            }
            println!("Select a nest:");
            for (i, p) in nests.iter().enumerate() {
                println!("{}: {}", i + 1, p.display());
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
        .unwrap()
        .to_str()
        .unwrap()
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
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

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
    Switch { path: Option<String> },
}

fn main() {
    let config = get_config();

    let cli = Cli::parse();
    match cli.command {
        Commands::Config => print_config(),
        Commands::Nest {
            nest_command,
            shallow,
        } => {
            let nest = get_nest();
            match nest_command {
                None | Some(NestCommands::Info) => nest.run_info(shallow),
                Some(NestCommands::Link) => nest.run_link(shallow),
                Some(NestCommands::Install) => nest.run_install(shallow),
                Some(NestCommands::Systemd) => nest.run_systemd(shallow),
                Some(NestCommands::All) => nest.run_all(shallow),
                Some(NestCommands::Edit) => nest.run_edit(),
                Some(NestCommands::Switch { path }) => {
                    let _ = switch_nest(path);
                }
            }
        }
        Commands::Sync => sync(&config),
        Commands::Setup {
            setup_name,
            setup_command,
            shallow,
        } => {
            let s = get_setup(&setup_name);

            match setup_command {
                SetupCommands::Link => s.run_link(shallow),
                SetupCommands::Info => s.run_info(shallow),
                SetupCommands::Edit => s.run_edit(),
                SetupCommands::Install => s.run_install(shallow),
                SetupCommands::Systemd => s.run_systemd(shallow),
                SetupCommands::All => s.run_all(shallow),
            }
        }
        Commands::SetupsValidate => validate_all_setups(),
        Commands::Update => run_update(),
    }
}

fn sync(config: &Config) {
    println!("Syncing");

    let owl_sync_script_path =
        Path::join(&config.owl_path, Path::new("common/scripts/owl-sync.sh"));

    run_script(owl_sync_script_path);
}
fn run_update() {
    let s = get_setup("owl");
    s.run_install(true);
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

    // Read and print stdout
    if let Some(stdout) = child.stdout.take() {
        let stdout_reader = BufReader::new(stdout);
        for line in stdout_reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
    }

    // Read and print stderr
    if let Some(stderr) = child.stderr.take() {
        let stderr_reader = BufReader::new(stderr);
        for line in stderr_reader.lines() {
            if let Ok(line) = line {
                eprintln!("{}", line);
            }
        }
    }

    // Wait for the command to finish and check the status
    let status = child.wait().expect("Failed to wait on child process");
    if !status.success() {
        eprintln!("Command failed with exit code: {:?}", status.code());
    } else {
        println!("Script completed successfully");
    }
}

fn link_paths(
    source_path: &Path,
    target_path: &Path,
    root: bool,
    quiet: bool,
) -> Result<(), String> {
    if target_path.exists() || target_path.is_symlink() {
        if let Err(e) = std::fs::remove_file(target_path) {
            if !quiet {
                eprintln!("remove old: {}", e);
            }
            return Err(format!("remove old: {}", e));
        }
    }

    if let Some(parent) = target_path.parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                if !quiet {
                    eprintln!("mkdir: {}", e);
                }
                return Err(format!("mkdir: {}", e));
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
                if !quiet {
                    eprintln!("{}", msg);
                }
                Err(msg)
            }
            Err(e) => {
                if !quiet {
                    eprintln!("exec sudo ln: {}", e);
                }
                Err(format!("exec sudo ln: {}", e))
            }
        }
    } else {
        std::os::unix::fs::symlink(&source_path, &target_path).map_err(|e| {
            if !quiet {
                eprintln!("{}", e);
            }
            e.to_string()
        })
    }
}
