use clap::{Parser, Subcommand};
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
extern crate skim;
use colored::Colorize;
use std::io::{BufRead, BufReader};
use std::process::Command;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    #[serde(default = "get_default_owl_path")]
    owl_path: PathBuf,
    nest_path: Option<PathBuf>,
}

fn get_default_owl_path() -> PathBuf {
    let owl_path = "~/owl";
    PathBuf::from(shellexpand::tilde(&owl_path).to_string())
}

fn get_config_path() -> String {
    let config_path = "~/.config/owl/config.json";
    shellexpand::tilde(&config_path).to_string()
}

fn get_config() -> Config {
    let config_path = get_config_path();

    // Make file if it doesn't exist
    if !Path::new(&config_path).exists() {
        // create path if it doesn't exist
        std::fs::create_dir_all(Path::new(&config_path).parent().unwrap())
            .expect("Unable to create config path");
        std::fs::File::create(&config_path).expect("Unable to create config file");

        let config = Config {
            owl_path: get_default_owl_path(),
            nest_path: None,
        };

        let config_raw = serde_json::to_string(&config).expect("Unable to serialize config");
        std::fs::write(&config_path, config_raw).expect("Unable to write config file");
    }

    let config_raw = std::fs::read_to_string(&config_path).expect("Unable to read config file");
    let config: Config = serde_json::from_str(&config_raw).expect("Unable to parse config file");
    config
}

fn save_config(config: Config) {
    let config_path = get_config_path();
    let config_raw = serde_json::to_string(&config).expect("Unable to serialize config");
    std::fs::write(config_path, config_raw).expect("Unable to write config file");
}

fn load_config() -> Config {
    let mut config = get_config();
    if !config.owl_path.exists() {
        println!("Owl path {} does not exist", config.owl_path.display());
        // prompt user to enter a new path
        let mut new_path = String::new();
        println!("Enter the absolute path to your owl folder: ");
        std::io::stdin().read_line(&mut new_path).unwrap();
        let new_path = PathBuf::from(new_path.trim());
        config.owl_path = new_path;
        save_config(config);
    }

    let new_config = get_config();
    new_config
}

fn validate_nest_path(path: &PathBuf) -> bool {
    if !path.exists() {
        return false;
    }
    if !path.is_dir() {
        return false;
    }
    let nest_json_path = path.join("nest.json");
    nest_json_path.exists() && nest_json_path.is_file()
}

fn prompt_user_for_nest_path() -> PathBuf {
    loop {
        println!("Enter the absolute path to your nest directory: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let path = PathBuf::from(input.trim());

        if validate_nest_path(&path) {
            return path;
        } else {
            eprintln!(
                "Error: Invalid nest path. The directory must exist and contain a nest.json file."
            );
            eprintln!("Please try again or create a nest.json file in your nest directory.");
        }
    }
}

fn load_nest() {
    let mut config = get_config();
    let nest_path = config.nest_path.clone();

    if nest_path.is_none() {
        println!("Nest path is not set");
        let nest_path = prompt_user_for_nest_path();
        config.nest_path = Some(nest_path);
        save_config(config);
        return;
    }

    let nest_path = nest_path.unwrap();
    if !validate_nest_path(&nest_path) {
        eprintln!("Error: Invalid nest path {}", nest_path.display());
        eprintln!("The directory must exist and contain a nest.json file.");
        let nest_path = prompt_user_for_nest_path();
        config.nest_path = Some(nest_path);
        save_config(config);
        return;
    }
}

#[derive(Debug, Deserialize)]
struct Nest {
    links: Vec<LinkedFile>,
    setups: Vec<String>,
    #[serde(default)]
    rc_scripts: Vec<String>,
}

fn get_nest() -> Nest {
    let config = get_config();
    let nest_dir = config.nest_path.unwrap();
    let nest_path = nest_dir.join("nest.json");

    if !nest_path.exists() {
        eprintln!("Error: nest.json not found at {}", nest_path.display());
        eprintln!("Please ensure your nest directory contains a valid nest.json file.");
        std::process::exit(1);
    }

    println!("Reading nest from path: {}", nest_path.display());
    let nest_raw = std::fs::read_to_string(&nest_path).expect("Unable to read nest file");
    let nest: Nest = serde_json::from_str(&nest_raw).expect("Unable to parse nest file");
    nest
}

fn print_config() {
    let config = get_config();
    println!("{:?}", config);
}

fn print_nest_info() {
    let config = get_config();
    let nests = get_nest();

    println!("{}", "=== NEST INFO ===".blue().bold());
    println!();

    println!("{}", "📂 NEST LINKS:".green().bold());
    if nests.links.is_empty() {
        println!("  No direct links configured");
    } else {
        for linked_file in &nests.links {
            let source_path = resolve_path(&linked_file.source_path);
            let absolute_source_path = Path::join(&config.owl_path, Path::new(&source_path));
            let absolute_target_path = shellexpand::tilde(&linked_file.target_path).to_string();

            println!(
                "  {} → {}",
                absolute_source_path.display().to_string().blue(),
                absolute_target_path.red()
            );
        }
    }
    println!();

    println!("{}", "🔧 NEST RC SCRIPTS:".yellow().bold());
    if nests.rc_scripts.is_empty() {
        println!("  No rc scripts configured");
    } else {
        for rc_script in &nests.rc_scripts {
            let resolved_path = resolve_path(rc_script);
            let absolute_source_path = Path::join(&config.owl_path, Path::new(&resolved_path));
            let script_name = Path::new(&resolved_path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();
            let prefixed_filename = format!("nest-{}", script_name);
            let target_path = format!("~/.config/owl-rc/{}", prefixed_filename);

            println!(
                "  {} → {}",
                absolute_source_path.display().to_string().blue(),
                target_path.red()
            );
        }
    }
    println!();

    println!("{}", "📦 SETUPS:".magenta().bold());
    if nests.setups.is_empty() {
        println!("  No setups configured");
    } else {
        for setup_name in &nests.setups {
            println!("  📋 {}", setup_name.cyan().bold());

            let setup = Setup::from_file(setup_name.clone());

            // Show setup links
            if !setup.links.is_empty() {
                println!("    Links:");
                for linked_file in &setup.links {
                    let source_path = resolve_path(&linked_file.source_path);
                    // resolved absolute path not needed for display here
                    let absolute_source_path =
                        Path::join(&config.owl_path, Path::new(&source_path));
                    let absolute_target_path =
                        shellexpand::tilde(&linked_file.target_path).to_string();

                    println!(
                        "      {} → {}",
                        absolute_source_path.display().to_string().blue(),
                        absolute_target_path.red()
                    );
                }
            }

            // Show setup rc scripts
            if !setup.rc_scripts.is_empty() {
                println!("    RC Scripts:");
                for rc_script in &setup.rc_scripts {
                    let resolved_path = resolve_path_with_context(rc_script, Some(setup_name));
                    let absolute_source_path =
                        Path::join(&config.owl_path, Path::new(&resolved_path));
                    let script_name = Path::new(&resolved_path)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap();
                    let prefixed_filename = format!("setup-{}-{}", setup_name, script_name);
                    let target_path = format!("~/.config/owl-rc/{}", prefixed_filename);

                    println!(
                        "      {} → {}",
                        absolute_source_path.display().to_string().blue(),
                        target_path.red()
                    );
                }
            }

            // Show setup actions
            if !setup.actions.is_empty() {
                println!("    Actions:");
                for action in &setup.actions {
                    let action_path = Path::join(
                        &config.owl_path,
                        Path::new(&format!("setups/{}/{}", setup_name, action)),
                    );
                    println!("      {}", action_path.display().to_string().yellow());
                }
            }

            // Show install script
            if let Some(install_script) = &setup.install {
                println!("    Install Script:");
                let install_path = Path::join(
                    &config.owl_path,
                    Path::new(&format!("setups/{}/{}", setup_name, install_script)),
                );
                println!("      {}", install_path.display().to_string().yellow());
            }

            // Show services
            if !setup.services.is_empty() {
                println!("    Services:");
                for service in &setup.services {
                    let source_path = resolve_path_with_context(&service.path, Some(setup_name));
                    let service_filename = Path::new(&source_path)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap();
                    let target_path = match service.service_type.as_str() {
                        "system" => format!("/etc/systemd/system/{}", service_filename),
                        _ => format!("~/.config/systemd/user/{}", service_filename),
                    };

                    println!(
                        "      {} → {} ({})",
                        Path::new(&source_path).display().to_string().blue(),
                        target_path.red(),
                        service.service_type
                    );
                }
            }

            // Show dependencies
            if !setup.dependencies.is_empty() {
                println!("    Dependencies:");
                for dep in &setup.dependencies {
                    println!("      {}", dep.cyan());
                }
            }

            println!();
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Config,
    Nest {
        #[command(subcommand)]
        nest_command: NestCommands,
    },
    Nests {
        #[command(subcommand)]
        nests_command: NestsCommands,
    },
    Sync,
    Setup {
        setup_name: String,
    },
    Setups,
    Update,
}

#[derive(Subcommand)]
enum NestCommands {
    Setup,
    Install,
    Info,
    Switch {
        /// Absolute path to a nest directory (optional). If omitted, an interactive selector opens.
        path: Option<String>,
    },
}

#[derive(Subcommand)]
enum NestsCommands {
    Switch {
        /// Absolute path to a nest directory (optional). If omitted, an interactive selector opens.
        path: Option<String>,
    },
}

fn main() {
    let config = load_config();
    load_nest();

    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Config) => print_config(),
        Some(Commands::Nest { nest_command }) => match nest_command {
            NestCommands::Setup => link_with_setups(),
            NestCommands::Install => install_nest_setups(),
            NestCommands::Info => print_nest_info(),
            NestCommands::Switch { path } => switch_nest(path),
        },
        Some(Commands::Nests { nests_command }) => match nests_command {
            NestsCommands::Switch { path } => switch_nest(path),
        },
        Some(Commands::Sync) => sync(&config),
        Some(Commands::Setup { setup_name }) => run_setup(&setup_name),
        Some(Commands::Setups) => interactive_setups(),
        Some(Commands::Update) => run_update(),
        None => println!("No command"),
    }
}

fn sync(config: &Config) {
    println!("Syncing");

    let owl_sync_script_path =
        Path::join(&config.owl_path, Path::new("common/scripts/owl-sync.sh"));

    run_script(owl_sync_script_path);
}

#[derive(Debug)]
struct Setup {
    name: String,
    links: Vec<LinkedFile>,
    // vector of script file paths relative to the setup directory
    actions: Vec<String>,
    rc_scripts: Vec<String>,
    // Install script (separate from actions/setup)
    install: Option<String>,
    // Services to link to systemd
    services: Vec<Service>,
    // Dependencies on other setups
    dependencies: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Service {
    #[serde(rename = "path")]
    path: String,
    #[serde(rename = "type", default = "default_service_type")]
    service_type: String,
}

fn default_service_type() -> String {
    "user".to_string()
}

#[derive(Debug, Default, Deserialize)]
struct SetupFile {
    name: Option<String>,
    links: Option<Vec<LinkedFile>>,
    actions: Option<Vec<String>>,
    rc_scripts: Option<Vec<String>>,
    install: Option<String>,
    services: Option<Vec<Service>>,
    dependencies: Option<Vec<String>>,
}

impl Setup {
    pub fn from_file(name: String) -> Setup {
        let setup_path = Path::join(
            &Path::join(Path::new(&get_config().owl_path), Path::new("setups")),
            &Path::join(Path::new(&name), Path::new("setup.json")),
        );

        println!(
            "Using setup file: {} for setup {}",
            setup_path.display(),
            name
        );

        let setup_raw = std::fs::read_to_string(setup_path).expect("Unable to read setup file");
        let file: SetupFile = serde_json::from_str(&setup_raw).expect("Unable to parse setup file");

        Setup {
            name: file.name.unwrap_or(name),
            links: file.links.unwrap_or_default(),
            actions: file.actions.unwrap_or_default(),
            rc_scripts: file.rc_scripts.unwrap_or_default(),
            install: file.install,
            services: file.services.unwrap_or_default(),
            dependencies: file.dependencies.unwrap_or_default(),
        }
    }
}

fn run_script(script_path: PathBuf) {
    let script_path = script_path
        .canonicalize()
        .expect("Failed to canonicalize path");
    println!("Running script: {}", script_path.display());

    let mut cmd = Command::new("bash");
    cmd.arg("-c").arg(script_path);

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

fn run_setup_script(setup: &Setup) {
    let config = get_config();
    // print actions to user and have them select one
    if setup.actions.is_empty() {
        println!("No actions to run for {}", setup.name.green());
        return;
    }

    let script_path = match setup.actions.len() {
        1 => setup.actions[0].clone(),
        _ => {
            println!("Select an action to run: ");
            for (i, action) in setup.actions.iter().enumerate() {
                println!("{}) {}", i, action);
            }
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap().to_string();
            let index: usize = input.trim().parse().unwrap();

            // this is relative to the setup directory
            setup.actions[index].clone()
        }
    };

    let path_parts = [
        Path::new(&config.owl_path),
        Path::new("setups"),
        Path::new(&setup.name),
        Path::new(&script_path),
    ];

    let full_action_path = path_parts
        .iter()
        .fold(PathBuf::new(), |acc, &part| acc.join(part));

    run_script(full_action_path);
}

fn run_setup_link(setup: &Setup) {
    println!("Setting up {}", setup.name.green());
    for linked_file in &setup.links {
        let resolved_source =
            resolve_path_with_context(&linked_file.source_path, Some(&setup.name));
        let contextual = LinkedFile {
            source_path: resolved_source,
            target_path: linked_file.target_path.clone(),
            root: linked_file.root,
        };
        contextual.create_symlink();
    }
}

#[derive(Debug, Deserialize)]
struct LinkedFile {
    #[serde(rename = "source")]
    source_path: String,
    #[serde(rename = "target")]
    target_path: String,
    root: Option<bool>,
}

fn resolve_path(path: &str) -> String {
    resolve_path_with_context(path, None)
}

fn resolve_path_with_context(path: &str, setup_name: Option<&str>) -> String {
    let config = get_config();

    if path.starts_with("common:") {
        let relative_path = &path[7..]; // Remove "common:" prefix
        format!("common/rc/{}", relative_path)
    } else if path.starts_with("local:") {
        let relative_path = &path[6..]; // Remove "local:" prefix

        // If we have setup context, resolve to setup directory
        if let Some(setup) = setup_name {
            return format!("setups/{}/{}", setup, relative_path);
        }

        // Otherwise resolve to nest directory
        if let Some(nest_path) = config.nest_path {
            if let Some(nest_name) = nest_path.file_name() {
                return format!("nests/{}/{}", nest_name.to_str().unwrap(), relative_path);
            }
        }
        format!("nests/unknown/{}", relative_path)
    } else {
        path.to_string()
    }
}

impl LinkedFile {
    pub fn create_symlink(&self) {
        let source_path = resolve_path(&self.source_path);

        let absolute_source_path =
            Path::join(Path::new(&get_config().owl_path), Path::new(&source_path));

        let absolute_target_path = shellexpand::tilde(&self.target_path).to_string();

        print!(
            "Linking {} to {}",
            absolute_source_path.display().to_string().blue(),
            absolute_target_path.red()
        );

        if Path::new(&absolute_target_path).exists()
            || Path::new(&absolute_target_path).is_symlink()
        {
            print!("(🗑 old)");
            match std::fs::remove_file(&absolute_target_path) {
                Ok(_) => (),
                Err(e) => println!("Error removing file, {}", e),
            }
        }

        let target_path = Path::new(&absolute_target_path);
        if let Some(parent) = target_path.parent() {
            if !parent.exists() {
                match fs::create_dir_all(parent) {
                    Ok(_) => println!("Created parent directory"),
                    Err(e) => println!("Failed to create parent directory: {}", e),
                }
            }
        }

        if self.root.unwrap_or(false) {
            // Running the command with sudo
            let output = Command::new("sudo")
                .arg("ln")
                .arg("-s")
                .arg(&absolute_source_path)
                .arg(&absolute_target_path)
                .output();

            match output {
                Ok(o) => {
                    if o.status.success() {
                        println!("✅ Symlink created with root privileges");
                    } else {
                        eprintln!(
                            "❌ Failed to create symlink: {}",
                            String::from_utf8_lossy(&o.stderr)
                        );
                    }
                }
                Err(e) => eprintln!("Failed to execute command: {}", e),
            }
        } else {
            match std::os::unix::fs::symlink(&absolute_source_path, &absolute_target_path) {
                Ok(_) => println!("✅ Symlink created"),
                Err(e) => println!("❌ {}", e),
            }
        }
    }
}

fn link_rc_script(script_path: &str, target_dir: &str) {
    link_rc_script_with_context(script_path, target_dir, None);
}

fn link_rc_script_with_context(script_path: &str, target_dir: &str, setup_name: Option<&str>) {
    let resolved_path = resolve_path_with_context(script_path, setup_name);
    let script_name = Path::new(&resolved_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    // Generate prefixed filename based on context
    let prefixed_filename = match setup_name {
        Some(setup) => format!("setup-{}-{}", setup, script_name),
        None => format!("nest-{}", script_name),
    };

    let linked_file = LinkedFile {
        source_path: script_path.to_string(),
        target_path: format!("~/.config/{}/{}", target_dir, prefixed_filename),
        root: None,
    };

    // Create target directory if it doesn't exist
    let target_dir_path = shellexpand::tilde(&format!("~/.config/{}", target_dir)).to_string();
    if !Path::new(&target_dir_path).exists() {
        match fs::create_dir_all(&target_dir_path) {
            Ok(_) => println!("Created directory: {}", target_dir_path),
            Err(e) => println!("Failed to create directory {}: {}", target_dir_path, e),
        }
    }

    // Update the linked file to use the resolved path for symlink creation
    let context_linked_file = LinkedFile {
        source_path: resolved_path,
        target_path: linked_file.target_path,
        root: linked_file.root,
    };

    context_linked_file.create_symlink();
}

fn link_with_setups() {
    // get the config
    let nests = get_nest();

    for linked_file in nests.links {
        linked_file.create_symlink();
    }

    // Link nest rc_scripts to owl-rc
    for rc_script in nests.rc_scripts {
        link_rc_script(&rc_script, "owl-rc");
    }

    // Link each setup including dependencies recursively
    let mut visited = std::collections::HashSet::new();
    for setup_name in nests.setups {
        link_setup_with_deps(&setup_name, &mut visited);
    }
}

fn link_setup_with_deps(setup_name: &str, visited: &mut std::collections::HashSet<String>) {
    if visited.contains(setup_name) {
        return;
    }

    let setup = Setup::from_file(setup_name.to_string());

    // First link dependencies
    for dep in &setup.dependencies {
        link_setup_with_deps(dep, visited);
    }

    // Link setup rc_scripts to owl-rc
    for rc_script in &setup.rc_scripts {
        link_rc_script_with_context(rc_script, "owl-rc", Some(setup_name));
    }

    // Link services to systemd user dir
    link_services(&setup);

    // Link files
    run_setup_link(&setup);
    // Run optional actions (not install)
    if !setup.actions.is_empty() {
        run_setup_script(&setup);
    }

    visited.insert(setup_name.to_string());
}

fn run_setup(setup_name: &str) {
    let setup = Setup::from_file(setup_name.to_string());
    run_setup_link(&setup);
    link_services(&setup);
    if !setup.actions.is_empty() {
        run_setup_script(&setup);
    }
}

fn run_update() {
    run_setup("owl");
}

fn install_nest_setups() {
    let nest = get_nest();
    let mut processed = std::collections::HashSet::new();

    for setup_name in &nest.setups {
        install_setup_with_deps(setup_name, &mut processed);
    }
}

fn install_setup_with_deps(setup_name: &str, processed: &mut std::collections::HashSet<String>) {
    if processed.contains(setup_name) {
        return;
    }

    let setup = Setup::from_file(setup_name.to_string());

    // Install dependencies first
    for dep in &setup.dependencies {
        install_setup_with_deps(dep, processed);
    }

    // Install this setup
    if let Some(install_script) = &setup.install {
        println!("Installing {}", setup_name.green());
        let config = get_config();
        let install_path = Path::join(
            &config.owl_path,
            Path::new(&format!("setups/{}/{}", setup_name, install_script)),
        );
        run_script(install_path);
    }

    processed.insert(setup_name.to_string());
}

fn link_services(setup: &Setup) {
    for service in &setup.services {
        let resolved_path = resolve_path_with_context(&service.path, Some(&setup.name));

        let filename = Path::new(&resolved_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let target = match service.service_type.as_str() {
            "system" => format!("/etc/systemd/system/{}", filename),
            _ => format!("~/.config/systemd/user/{}", filename),
        };

        let linked = LinkedFile {
            source_path: resolved_path,
            target_path: target,
            root: Some(service.service_type == "system"),
        };

        linked.create_symlink();
    }
}

fn interactive_setups() {
    use std::io::{self, Write};

    let config = get_config();
    let setups_dir = Path::join(&config.owl_path, Path::new("setups"));

    let mut setups = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&setups_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.file_type().unwrap().is_dir() {
                    if let Some(name) = entry.file_name().to_str() {
                        setups.push(name.to_string());
                    }
                }
            }
        }
    }

    setups.sort();

    loop {
        println!("\n{}", "Available Setups:".blue().bold());
        for (i, setup) in setups.iter().enumerate() {
            println!("{}. {}", i + 1, setup.cyan());
        }
        println!("0. Exit");

        print!("\nSelect setup number: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let choice: usize = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        if choice == 0 {
            break;
        }

        if choice > setups.len() {
            continue;
        }

        let setup_name = &setups[choice - 1];

        loop {
            println!("\n{} {}", "Setup:".blue().bold(), setup_name.cyan());
            println!("1. View");
            println!("2. Edit");
            println!("3. Install");
            println!("0. Back");

            print!("\nSelect action: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let action: usize = match input.trim().parse() {
                Ok(n) => n,
                Err(_) => continue,
            };

            match action {
                0 => break,
                1 => view_setup(setup_name),
                2 => edit_setup(setup_name),
                3 => {
                    let mut processed = std::collections::HashSet::new();
                    install_setup_with_deps(setup_name, &mut processed);
                }
                _ => continue,
            }
        }
    }
}

fn view_setup(setup_name: &str) {
    let setup = Setup::from_file(setup_name.to_string());
    println!("\n{} {}", "Setup:".blue().bold(), setup_name.cyan());

    if !setup.dependencies.is_empty() {
        println!("Dependencies: {}", setup.dependencies.join(", ").yellow());
    }

    if let Some(install) = &setup.install {
        println!("Install script: {}", install.green());
    }

    if !setup.actions.is_empty() {
        println!("Actions: {}", setup.actions.join(", ").magenta());
    }

    if !setup.links.is_empty() {
        println!("Links: {} files", setup.links.len());
    }

    if !setup.services.is_empty() {
        println!("Services: {} files", setup.services.len());
    }

    if !setup.rc_scripts.is_empty() {
        println!("RC Scripts: {}", setup.rc_scripts.join(", ").blue());
    }
}

fn edit_setup(setup_name: &str) {
    let config = get_config();
    let links_path = Path::join(
        &config.owl_path,
        Path::new(&format!("setups/{}/setup.json", setup_name)),
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

fn list_nests() -> Vec<PathBuf> {
    let config = get_config();
    let nests_dir = Path::join(&config.owl_path, Path::new("nests"));
    let mut nests = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&nests_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.join("nest.json").exists() {
                nests.push(path);
            }
        }
    }
    nests
}

fn switch_nest(path: Option<String>) {
    let mut config = get_config();

    let target_path = match path {
        Some(p) => PathBuf::from(p),
        None => {
            // interactive selection
            let nests = list_nests();
            if nests.is_empty() {
                eprintln!("No nests found under nests/");
                return;
            }
            println!("Select a nest:");
            for (i, p) in nests.iter().enumerate() {
                println!("{}: {}", i + 1, p.display());
            }
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let idx: usize = match input.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Invalid selection");
                    return;
                }
            };
            if idx == 0 || idx > nests.len() {
                eprintln!("Invalid selection");
                return;
            }
            nests[idx - 1].clone()
        }
    };

    if !validate_nest_path(&target_path) {
        eprintln!("Invalid nest path: {}", target_path.display());
        return;
    }

    config.nest_path = Some(target_path.clone());
    save_config(config);
    println!("Switched nest to {}", target_path.display());
}
