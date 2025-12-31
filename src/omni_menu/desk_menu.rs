pub mod desk_menu {
    use gtk::prelude::*;
    use gtk::{
        glib, Application, ApplicationWindow, Box as GtkBox, Entry, Label, ListBox,
        Orientation, ScrolledWindow,
    };
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;
    use std::process::Command;
    use crate::utils::{populate_list, filter_list};

    const APP_ID: &str = "org.gtk_rs.DeskMenu";

    fn get_desks_dir() -> PathBuf {
        let home = dirs::home_dir().unwrap();
        // Check if we're in Sway or i3
        if std::env::var("SWAYSOCK").is_ok() {
            home.join(".config/desks-sway")
        } else {
            home.join(".config/desks-i3")
        }
    }

    fn get_desk_scripts() -> Vec<String> {
        let desks_dir = get_desks_dir();
        if !desks_dir.exists() {
            return Vec::new();
        }

        match fs::read_dir(&desks_dir) {
            Ok(entries) => entries
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let file_name = entry.file_name().to_str()?.to_string();
                    // Only include executable files or .sh files
                    if file_name.ends_with(".sh") || entry.metadata().ok()?.permissions().mode() & 0o111 != 0 {
                        Some(file_name)
                    } else {
                        None
                    }
                })
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    fn build_ui(app: &Application) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Select Desk")
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
        search_entry.set_placeholder_text(Some("Type to filter desks..."));
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

        // Get desks and populate
        let desks = get_desk_scripts();
        if desks.is_empty() {
            let label = Label::new(Some("No desk scripts found"));
            label.set_margin_top(20);
            list_box.append(&gtk::ListBoxRow::new());
            let row = list_box.row_at_index(0).unwrap();
            row.set_child(Some(&label));
        } else {
            populate_list(&list_box, &desks);
        }

        // Handle search
        let list_box_weak = list_box.downgrade();
        let desks_clone = desks.clone();
        search_entry.connect_changed(move |entry| {
            if let Some(list_box) = list_box_weak.upgrade() {
                let query = entry.text().to_string().to_lowercase();
                filter_list(&list_box, &desks_clone, &query);
            }
        });

        // Handle activation
        let window_weak = window.downgrade();
        let desks_dir = get_desks_dir();
        list_box.connect_row_activated(move |_, row| {
            if let Some(label) = row.child().and_then(|w| w.downcast::<Label>().ok()) {
                let desk_name = label.text().to_string();
                let desk_path = desks_dir.join(&desk_name);

                // Execute the desk script
                Command::new(&desk_path)
                    .spawn()
                    .ok();

                if let Some(window) = window_weak.upgrade() {
                    window.close();
                }
            }
        });

        // Handle Enter key on search entry
        let list_box_weak = list_box.downgrade();
        search_entry.connect_activate(move |_| {
            if let Some(list_box) = list_box_weak.upgrade() {
                if let Some(first_row) = list_box.first_child() {
                    let row_count = list_box.observe_children().n_items();
                    if row_count == 1 {
                        list_box.emit_by_name::<()>(
                            "row-activated",
                            &[&first_row.downcast_ref::<gtk::ListBoxRow>().unwrap()],
                        );
                    } else if row_count > 1 {
                        list_box.select_row(Some(first_row.downcast_ref::<gtk::ListBoxRow>().unwrap()));
                        list_box.grab_focus();
                    }
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

    pub fn run_app() -> glib::ExitCode {
        let app = Application::builder().application_id(APP_ID).build();
        app.connect_activate(build_ui);
        let no_args: [&str; 0] = [];
        app.run_with_args(&no_args)
    }
}
