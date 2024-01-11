use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
extern crate skim;
use colored::Colorize;
use std::process::Command;

fn get_owl_path() -> String {
    match std::env::var("OWL_PATH") {
        Ok(path) => path,
        Err(_) => {
            // ask user for owl path
            println!("Enter the path to owl: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap().to_string()
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
    Link,
    Sync,
    Edit,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Link) => link_with_setups(),
        Some(Commands::Sync) => {
            println!("Syncing");
            let owl_path = get_owl_path();

            let owl_sync_script_path = Path::join(
                Path::new(&owl_path),
                Path::new("common/scripts/owl-sync.sh"),
            );

            println!(
                "Running owl-sync.sh script at {}",
                owl_sync_script_path.display()
            );

            // Run the owl-sync command
            let mut cmd = std::process::Command::new(owl_sync_script_path);

            cmd.spawn().expect("Unable to run owl-sync.sh");
        }
        Some(Commands::Edit) => println!("Editing"),
        None => println!("No command"),
    }
}

#[derive(Debug, Deserialize)]
struct OwlConfig {
    links: Vec<LinkedFile>,
    setups: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Setup {
    name: String,
    links: Vec<LinkedFile>,
}

impl Setup {
    pub fn from_file(name: String) -> Setup {
        let setup_path = Path::join(
            &Path::join(Path::new(&get_owl_path()), Path::new("setups")),
            &Path::join(Path::new(&name), Path::new("links.json")),
        );

        println!("Using setup file: {}", setup_path.display());

        let setup_raw = std::fs::read_to_string(setup_path).expect("Unable to read setup file");
        let setup: Setup = serde_json::from_str(&setup_raw).expect("Unable to parse setup file");
        setup
    }
}

fn get_owl_config() -> OwlConfig {
    let config_path = match std::env::var("OWL_CONFIG_PATH") {
        Ok(path) => path,
        Err(_) => {
            println!("Enter the path to your owl config file: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap().to_string();
            input
        }
    };

    println!("Using config file: {}", config_path);

    let config_raw = std::fs::read_to_string(config_path).expect("Unable to read config file");
    let config: OwlConfig = serde_json::from_str(&config_raw).expect("Unable to parse config file");
    return config;
}

#[derive(Debug, Deserialize)]
struct LinkedFile {
    #[serde(rename = "source")]
    source_path: String,
    #[serde(rename = "target")]
    target_path: String,
    root: Option<bool>,
}

impl LinkedFile {
    pub fn create_symlink(&self) {
        let absolute_source_path =
            Path::join(Path::new(&get_owl_path()), Path::new(&self.source_path));

        let absolute_target_path = shellexpand::tilde(&self.target_path).to_string();

        print!(
            "Linking {} to {}",
            absolute_source_path.display().to_string().blue(),
            absolute_target_path.red()
        );

        if Path::new(&absolute_target_path).exists() {
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

fn link_with_setups() {
    // get the config
    let config = get_owl_config();

    // link the configs links
    for linked_file in config.links {
        linked_file.create_symlink();
    }

    // read each setup's config file
    for setup in config.setups {
        let setup = Setup::from_file(setup);

        println!("Setting up {}", setup.name.green());

        // setup path is owl path + "/setups/" + setup + "links.json"
        for linked_file in setup.links {
            linked_file.create_symlink();
        }
    }
}
