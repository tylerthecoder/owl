pub mod launch_tool_menu {
    use crate::utils::{filter_list, populate_list};
    use gtk::prelude::*;
    use gtk::{
        glib, Application, ApplicationWindow, Box as GtkBox, Entry, Label, ListBox, Orientation,
        ScrolledWindow,
    };
    use std::process::Command;

    const APP_ID: &str = "org.gtk_rs.LaunchToolMenu";

    fn build_ui(app: &Application) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Launch Tool")
            .default_width(500)
            .default_height(600)
            .decorated(true)
            .resizable(false)
            .modal(true)
            .build();

        let vbox = GtkBox::new(Orientation::Vertical, 5);
        vbox.set_margin_top(10);
        vbox.set_margin_bottom(10);
        vbox.set_margin_start(10);
        vbox.set_margin_end(10);

        // Search entry
        let search_entry = Entry::new();
        search_entry.set_placeholder_text(Some("Type to filter tools..."));
        vbox.append(&search_entry);

        // Scrolled window for list items
        let scrolled_window = ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .vexpand(true)
            .build();

        let list_box = ListBox::new();
        list_box.set_selection_mode(gtk::SelectionMode::Single);
        list_box.set_activate_on_single_click(true);
        scrolled_window.set_child(Some(&list_box));

        vbox.append(&scrolled_window);
        window.set_child(Some(&vbox));

        // Get tools and populate
        let tools = get_yard_tools();
        populate_list(&list_box, &tools);

        // Handle search
        let list_box_weak = list_box.downgrade();
        let tools_clone = tools.clone();
        search_entry.connect_changed(move |entry| {
            if let Some(list_box) = list_box_weak.upgrade() {
                let query = entry.text().to_string().to_lowercase();
                filter_list(&list_box, &tools_clone, &query);
            }
        });

        // Handle activation
        let window_weak = window.downgrade();
        list_box.connect_row_activated(move |_, row| {
            if let Some(label) = row.child().and_then(|w| w.downcast::<Label>().ok()) {
                let tool_name = label.text().to_string();
                // Call yard to assemble the tool
                Command::new("yard")
                    .args(["tool", "assemble", &tool_name])
                    .spawn()
                    .ok();

                if let Some(window) = window_weak.upgrade() {
                    window.close();
                }
            }
        });

        // Handle Escape key
        let key_controller = gtk::EventControllerKey::new();
        let window_weak_key = window.downgrade();
        key_controller.connect_key_pressed(move |_, key, _, _| {
            if key == gtk::gdk::Key::Escape {
                if let Some(window) = window_weak_key.upgrade() {
                    window.close();
                }
                return glib::Propagation::Stop;
            }
            glib::Propagation::Proceed
        });
        window.add_controller(key_controller);

        search_entry.grab_focus();
        window.present();
    }

    fn get_yard_tools() -> Vec<String> {
        println!("Running: yard tool list");
        let output = Command::new("yard").args(["tool", "list"]).output();

        match output {
            Ok(o) => {
                let stdout = String::from_utf8_lossy(&o.stdout);
                // Simple parsing: one tool per line
                stdout
                    .lines()
                    .map(|line| line.trim())
                    .filter(|line| !line.is_empty())
                    .map(|s| s.to_string())
                    .collect()
            }
            Err(e) => {
                println!("Error running yard: {}", e);
                Vec::new()
            }
        }
    }

    pub fn run_app() -> glib::ExitCode {
        let app = Application::builder().application_id(APP_ID).build();
        app.connect_activate(build_ui);
        let no_args: [&str; 0] = [];
        app.run_with_args(&no_args)
    }
}
