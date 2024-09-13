use clap::{Parser, Subcommand};
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
extern crate skim;
use colored::Colorize;
use std::process::Command;
use std::io::{BufRead, BufReader};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    #[serde(default = "get_default_owl_path")]
    owl_path: PathBuf,
    nest_path: Option<PathBuf>,
}

fn get_default_owl_path() -> PathBuf {
    Path::new("/home/tyler/owl").to_path_buf()
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
        std::fs::create_dir_all(Path::new(&config_path).parent().unwrap()).expect("Unable to create config path");
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

fn verify_config() -> bool {
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
    return true;
}

fn prompt_user_for_nest_path() -> PathBuf {
    println!("Enter the absolute path to your nest folder: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    PathBuf::from(input.trim())
}

fn load_nest() {
    let mut config = get_config();
    let nest_path = config.nest_path;

    if nest_path.is_none() {
        println!("Nest path is not set");
        let nest_path = prompt_user_for_nest_path();
        config.nest_path = Some(nest_path);
        save_config(config);
        return;
    }

    let nest_path = nest_path.unwrap();
    if !nest_path.exists() {
        println!("Nest path {} does not exist", nest_path.display());
        // prompt user to enter a new path
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
}

fn get_nest() -> Nest {
    let nest_path = get_config().nest_path.unwrap();
    let nest_raw = std::fs::read_to_string(nest_path).expect("Unable to read nest file");
    let nest: Nest = serde_json::from_str(&nest_raw).expect("Unable to parse nest file");
    nest
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
    Setup { setup_name: String },
}

fn main() {
    verify_config();
    load_nest();

    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Link) => link_with_setups(),
        Some(Commands::Sync) => {
            println!("Syncing");
            let owl_path = get_config().owl_path;

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
        Some(Commands::Setup { setup_name }) => run_setup(&setup_name),
        None => println!("No command"),
    }
}

#[derive(Debug, Deserialize)]
struct Setup {
    name: String,
    links: Vec<LinkedFile>,
    // vector of script file paths relative to the setup directory
    #[serde(default)]
    actions: Vec<String>,
}

impl Setup {
    pub fn from_file(name: String) -> Setup {
        let setup_path = Path::join(
            &Path::join(Path::new(&get_config().owl_path), Path::new("setups")),
            &Path::join(Path::new(&name), Path::new("links.json")),
        );

        println!("Using setup file: {}", setup_path.display());

        let setup_raw = std::fs::read_to_string(setup_path).expect("Unable to read setup file");
        let setup: Setup = serde_json::from_str(&setup_raw).expect("Unable to parse setup file");
        setup
    }
}

fn run_setup_script(setup: &Setup) {
    let owl_path = get_config().owl_path;

    // print actions to user and have them select one
    if setup.actions.is_empty() {
        println!("No actions to run for {}", setup.name.green());
        return;
    }

    println!("Select an action to run: ");
    for (i, action) in setup.actions.iter().enumerate() {
        println!("{}) {}", i, action);
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap().to_string();
    let index: usize = input.trim().parse().unwrap();

    // this is relative to the setup directory
    let action_path = setup.actions[index].clone();

    let path_parts = [
        Path::new(&owl_path),
        Path::new("setups"),
        Path::new(&setup.name),
        Path::new(&action_path),
    ];

    let full_action_path = path_parts.iter().fold(PathBuf::new(), |acc, &part| acc.join(part));

    // normalize the path
    let full_action_path = full_action_path.canonicalize().expect("Failed to canonicalize path");

    let cmd_str = full_action_path.to_str().expect("Failed to convert path to string");

    println!("Running action: {}", cmd_str);

    let mut cmd = std::process::Command::new("bash");
    cmd.arg("-c").arg(cmd_str);

    // Use spawn and pipe the output
    let mut child = cmd.stdout(std::process::Stdio::piped())
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
        let stderr_reader = std::io::BufReader::new(stderr);
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
    }
}

fn run_setup_link(setup: &Setup) {
    println!("Setting up {}", setup.name.green());
    // setup path is owl path + "/setups/" + setup + "links.json"
    for linked_file in &setup.links {
        linked_file.create_symlink();
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

impl LinkedFile {
    pub fn create_symlink(&self) {
        let absolute_source_path =
            Path::join(Path::new(&get_config().owl_path), Path::new(&self.source_path));

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
    let nests = get_nest();

    for linked_file in nests.links {
        linked_file.create_symlink();
    }

    // read each setup's config file
    for setup in nests.setups {
        let setup = Setup::from_file(setup);
        run_setup_link(&setup);
    }
}

fn run_setup(setup_name: &str) {
    let setup = Setup::from_file(setup_name.to_string());
    run_setup_script(&setup);
    run_setup_link(&setup);
}
