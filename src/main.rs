use std::{path::Path};
use serde::Deserialize;
use std::path::PathBuf;
use clap::{Parser, Subcommand};

extern crate skim;
use skim::prelude::{*, helper::item};
use std::io::Cursor;


fn get_owl_path() -> String {
    match std::env::var("OWL_PATH") {
        Ok(path) => path,
        Err(e) => {
            // ask user for owl path
            println!("Enter the path to owl: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap().to_string()
        },
    }
}

fn get_owl_links_path() -> String {
    match std::env::var("OWL_DEFAULT_LINK") {
        Ok(path) => path,
        Err(e) => {
            // ask user for link path
            println!("Enter the path to link to: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap().to_string()
        },
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
    Add {
        #[arg(short, long)]
        source: PathBuf,
        #[arg(short, long)]
        target: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();



    match cli.command {
        Some(Commands::Link) => link(),
        Some(Commands::Sync) => println!("Syncing"),
        Some(Commands::Edit) => println!("Editing"),
        Some(Commands::Add { source, target }) => println!("Adding {} to {}", source.display(), target.display()),
        None => println!("No command"),
    }


    skim_test();



    let owl_path = std::env::var("OWL_PATH");

    match owl_path {
        Ok(path) => {
            println!("The owl path is: {}", path);
        },
        Err(e) => {
            println!("The owl path is not set: {}", e);
        },
    }


    let action = std::env::args().nth(1);

    match action {
        Some(a) => {

            println!("The command is this: {}", a);

            match a.as_str() {
                "link" => link(),
                _ => println!("Unknown command: {}", a),
            }
        },
        None => println!("Welcome to owl!"),
    }

}

fn add_link() {

    // ask for the source path

}

pub fn skim_test() {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .unwrap();

    let input = "aaaaa\nbbbb\nccc".to_string();

    // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // `SkimItem` was implemented for `AsRef<str>` by default
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    item_reader.of_bufread(source)

    // `run_with` would read and show items from the stream
    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        print!("{}{}", item.output(), "\n");
    }
}


struct OwlConfig {
    links: Vec<LinkedFile>,
    setups: Vec<String>,
}

struct Setup {
    name: String,
    links: Vec<LinkedFile>,
}


#[derive(Debug, Deserialize)]
struct LinkedFile {
    #[serde(rename = "source")]
    source_path: String,
    #[serde(rename = "target")]
    target_path: String,
}

impl LinkedFile {

    pub fn new(source_path: String, target_path: String) -> LinkedFile {
        LinkedFile {
            source_path,
            target_path,
        }
    }

    pub fn create_symlink(&self) {

        let absolute_source_path = Path::join(Path::new(&get_owl_path()), Path::new(&self.source_path));

        let absolute_target_path = shellexpand::tilde(&self.target_path).to_string();

        print!("Linking {} to {}", absolute_source_path.display(), absolute_target_path);

        // remove target file if it exists

        if Path::new(&absolute_target_path).exists() {
            print!("(🗑 old)");
            match std::fs::remove_file(&absolute_target_path) {
                Ok(_) => (),
                Err(e) => println!("Error removing file, {}", e),
            }
        }

        match std::os::unix::fs::symlink(absolute_source_path, absolute_target_path) {
            Ok(_) => println!("✅"),
            Err(e) => println!(" ❌ {}", e),
        }
    }
}


fn link() {

    let link_path = get_owl_links_path();

    println!("The link path is: {}", link_path);

    let linked_file_raw = std::fs::read_to_string(link_path).expect("Unable to read linked file");
    let linked_files: Vec<LinkedFile> = serde_json::from_str(&linked_file_raw).expect("Unable to parse linked file");


    for linked_file in linked_files {
        linked_file.create_symlink();
    }

}
