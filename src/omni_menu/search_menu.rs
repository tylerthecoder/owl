pub mod search_menu {
    use gtk::prelude::*;
    use gtk::{
        glib, Application, ApplicationWindow, Box as GtkBox, Entry, Justification, Label,
        Orientation,
    };
    use std::process::Command;

    const APP_ID: &str = "org.gtk_rs.SearchMenu";

    struct SearchEngine {
        trigger: &'static str,
        name: &'static str,
        url_template: &'static str,
    }

    static SEARCH_ENGINES: &[SearchEngine] = &[
        SearchEngine {
            trigger: "!g",
            name: "Google",
            url_template: "https://www.google.com/search?q={}",
        },
        SearchEngine {
            trigger: "!c",
            name: "ChatGPT",
            url_template: "https://chat.openai.com/?q={}",
        },
        SearchEngine {
            trigger: "!n",
            name: "Notes",
            url_template: "https://tylertracy.com/notes?search={}",
        },
    ];

    static DEFAULT_SEARCH_ENGINE: &SearchEngine = &SEARCH_ENGINES[0];

    fn launch_search(url_template: &str, query: &str) {
        let encoded_query = glib::uri_escape_string(query, None::<&str>, true);
        let search_url = url_template.replace("{}", &encoded_query);
        Command::new("chromium")
            .arg(&search_url)
            .spawn()
            .expect("Failed to launch browser");
    }

    fn build_ui(app: &Application) {
        println!("Building UI");

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Omni Search")
            .default_width(450)
            .default_height(160)
            .decorated(true)
            .resizable(false)
            .modal(true)
            .build();

        let vbox = GtkBox::new(Orientation::Vertical, 5);
        vbox.set_margin_top(10);
        vbox.set_margin_bottom(10);
        vbox.set_margin_start(10);
        vbox.set_margin_end(10);

        let search_entry = Entry::new();
        vbox.append(&search_entry);

        let mut engine_display_text = String::from("Available Engines:\n");
        for engine in SEARCH_ENGINES.iter() {
            engine_display_text.push_str(&format!("  {}  -  {}\n", engine.trigger, engine.name));
        }
        if engine_display_text.ends_with('\n') {
            engine_display_text.pop();
        }

        let engines_label = Label::new(Some(&engine_display_text));
        engines_label.set_halign(gtk::Align::Start);
        engines_label.set_xalign(0.0);
        engines_label.set_justify(Justification::Left);
        vbox.append(&engines_label);

        window.set_child(Some(&vbox));

        let window_weak = window.downgrade();
        search_entry.connect_activate(move |entry| {
            let original_text = entry.text();
            if original_text.is_empty() {
                return;
            }

            let mut query_to_search = original_text.to_string();
            let mut selected_engine = DEFAULT_SEARCH_ENGINE;
            let mut trigger_found = false;

            for engine in SEARCH_ENGINES.iter() {
                if let Some(_index) = query_to_search.find(engine.trigger) {
                    selected_engine = engine;
                    query_to_search = query_to_search.replacen(engine.trigger, "", 1);
                    query_to_search = query_to_search.trim().to_string();
                    trigger_found = true;
                    break;
                }
            }

            if !query_to_search.is_empty() {
                println!(
                    "Searching with: {} for: '{}'",
                    selected_engine.name, query_to_search
                );
                launch_search(selected_engine.url_template, &query_to_search);
                if let Some(window) = window_weak.upgrade() {
                    window.close();
                }
            } else if trigger_found && query_to_search.is_empty() {
                println!(
                    "Input was a trigger '{}' resulting in an empty query.",
                    original_text
                );
                entry.set_text("");
                entry.set_placeholder_text(Some(&format!(
                    "Enter query for {}",
                    selected_engine.name
                )));
            } else if !trigger_found && original_text.is_empty() {
                println!("Original text was empty or became empty with no trigger.");
            }
        });

        search_entry.grab_focus();
        window.present();
    }

    pub fn run_app() -> glib::ExitCode {
        println!("Running app");
        let app = Application::builder().application_id(APP_ID).build();
        println!("App built");
        app.connect_activate(build_ui);
        println!("App activated");
        let no_args: [&str; 0] = [];
        app.run_with_args(&no_args);
        glib::ExitCode::SUCCESS
    }
}
