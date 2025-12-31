use gtk::glib;
use std::env;

mod desk_menu;
mod emoji_menu;
mod launch_tool_menu;
mod main_menu;
mod projects_menu;
mod scripts_menu;
mod search_menu;
mod switch_bench_menu;
mod utils;

fn main() -> glib::ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "main" => main_menu::main_menu::run_app(),
            "search" => search_menu::search_menu::run_app(),
            "projects" => projects_menu::projects_menu::run_app(),
            "launch_tool" => launch_tool_menu::launch_tool_menu::run_app(),
            "switch_bench" => switch_bench_menu::switch_bench_menu::run_app(),
            "scripts" => scripts_menu::scripts_menu::run_app(),
            "desk" => desk_menu::desk_menu::run_app(),
            "emoji" => emoji_menu::emoji_menu::run_app(),
            _ => {
                eprintln!(
                    "Usage: omni-menu [main|search|projects|launch_tool|switch_bench|scripts|desk|emoji]"
                );
                glib::ExitCode::FAILURE
            }
        }
    } else {
        // Default to main menu
        main_menu::main_menu::run_app()
    }
}
