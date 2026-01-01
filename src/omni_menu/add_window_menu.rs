pub mod add_window_menu {
    use crate::utils::{filter_list, populate_list};
    use gtk::prelude::*;
    use gtk::{
        glib, Application, ApplicationWindow, Box as GtkBox, Entry, ListBox, Orientation,
        ScrolledWindow,
    };
    use std::process::Command;

    const APP_ID: &str = "org.gtk_rs.AddWindowMenu";

    #[derive(Clone)]
    struct StowedWindow {
        id: String,
        name: String,
        app_id: String,
    }

    fn build_ui(app: &Application) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Add Window")
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
        search_entry.set_placeholder_text(Some("Type to filter windows..."));
        vbox.append(&search_entry);

        // Scrolled window
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

        // Get stowed windows and populate
        let stowed_windows = get_stowed_windows();
        let display_names: Vec<String> = stowed_windows
            .iter()
            .map(|w| format!("{} ({})", w.name, w.app_id))
            .collect();
        populate_list(&list_box, &display_names);

        // Handle search
        let list_box_weak = list_box.downgrade();
        let display_names_clone = display_names.clone();
        search_entry.connect_changed(move |entry| {
            if let Some(list_box) = list_box_weak.upgrade() {
                let query = entry.text().to_string().to_lowercase();
                filter_list(&list_box, &display_names_clone, &query);
            }
        });

        // Handle activation
        let window_weak = window.downgrade();
        let stowed_windows_clone = stowed_windows.clone();
        list_box.connect_row_activated(move |_, row| {
            let index = row.index() as usize;
            if index < stowed_windows_clone.len() {
                let stowed = &stowed_windows_clone[index];
                // Move window to current workspace
                Command::new("swaymsg")
                    .args([
                        &format!("[con_id={}]", stowed.id),
                        "move",
                        "to",
                        "workspace",
                        "current",
                    ])
                    .spawn()
                    .ok();
                // Focus the window
                Command::new("swaymsg")
                    .args([&format!("[con_id={}]", stowed.id), "focus"])
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

    fn get_stowed_windows() -> Vec<StowedWindow> {
        // Get sway tree and find windows in "temp" workspace
        let output = Command::new("swaymsg").args(["-t", "get_tree"]).output();

        match output {
            Ok(o) => {
                let stdout = String::from_utf8_lossy(&o.stdout);
                parse_stowed_windows(&stdout)
            }
            Err(_) => Vec::new(),
        }
    }

    fn parse_stowed_windows(json: &str) -> Vec<StowedWindow> {
        let mut windows = Vec::new();

        // Parse JSON to find windows in temp workspace
        if let Ok(tree) = serde_json::from_str::<serde_json::Value>(json) {
            find_temp_windows(&tree, &mut windows, false);
        }

        windows
    }

    fn find_temp_windows(node: &serde_json::Value, windows: &mut Vec<StowedWindow>, in_temp: bool) {
        // Check if this is the temp workspace
        let is_temp_workspace = node
            .get("type")
            .and_then(|t| t.as_str())
            .map(|t| t == "workspace")
            .unwrap_or(false)
            && node
                .get("name")
                .and_then(|n| n.as_str())
                .map(|n| n == "temp")
                .unwrap_or(false);

        let currently_in_temp = in_temp || is_temp_workspace;

        // If this is a window (con with app_id or window class) and we're in temp, add it
        if currently_in_temp {
            let node_type = node.get("type").and_then(|t| t.as_str()).unwrap_or("");
            let has_app_id = node.get("app_id").is_some()
                && !node
                    .get("app_id")
                    .unwrap()
                    .is_null();
            let has_window_props = node.get("window_properties").is_some();

            if node_type == "con" && (has_app_id || has_window_props) {
                let id = node
                    .get("id")
                    .and_then(|i| i.as_i64())
                    .map(|i| i.to_string())
                    .unwrap_or_default();

                let name = node
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("unnamed")
                    .to_string();

                let app_id = node
                    .get("app_id")
                    .and_then(|a| a.as_str())
                    .map(|s| s.to_string())
                    .or_else(|| {
                        node.get("window_properties")
                            .and_then(|wp| wp.get("class"))
                            .and_then(|c| c.as_str())
                            .map(|s| s.to_string())
                    })
                    .unwrap_or_else(|| "unknown".to_string());

                if !id.is_empty() {
                    windows.push(StowedWindow { id, name, app_id });
                }
            }
        }

        // Recurse into nodes
        if let Some(nodes) = node.get("nodes").and_then(|n| n.as_array()) {
            for child in nodes {
                find_temp_windows(child, windows, currently_in_temp);
            }
        }

        // Also check floating_nodes
        if let Some(floating) = node.get("floating_nodes").and_then(|n| n.as_array()) {
            for child in floating {
                find_temp_windows(child, windows, currently_in_temp);
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
