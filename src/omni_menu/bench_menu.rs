pub mod bench_menu {
    use gtk::prelude::*;
    use gtk::{
        glib, Application, ApplicationWindow, Box as GtkBox, Entry, Label, Orientation, Window,
    };
    use std::process::Command;

    const APP_ID: &str = "org.gtk_rs.BenchMenu";

    fn build_ui(app: &Application) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Bench Menu")
            .default_width(500)
            .default_height(350)
            .decorated(true)
            .resizable(false)
            .modal(true)
            .build();

        let vbox = GtkBox::new(Orientation::Vertical, 5);
        vbox.set_margin_top(10);
        vbox.set_margin_bottom(10);
        vbox.set_margin_start(10);
        vbox.set_margin_end(10);

        // Title label
        let title_label = Label::new(Some("Bench Menu"));
        title_label.set_markup("<span size='large' weight='bold'>Bench Menu</span>");
        title_label.set_halign(gtk::Align::Start);
        vbox.append(&title_label);

        // Instructions label
        let instructions_label = Label::new(Some("Press a hotkey to select an action"));
        instructions_label.set_halign(gtk::Align::Start);
        instructions_label.set_margin_top(10);
        vbox.append(&instructions_label);

        // Menu options box
        let menu_box = GtkBox::new(Orientation::Vertical, 5);
        menu_box.set_margin_top(10);

        let options = vec![
            ("(a) Add window", "Bring a stowed window to current workspace"),
            ("(c) Create from active", "Capture current windows into new bench"),
            ("(s) Switch bench", "Switch to a different bench"),
            ("(t) Launch tool", "Add a tool to current workspace"),
            ("(y) Sync bench", "Save current layout to disk"),
        ];

        for (option, description) in options {
            let option_box = GtkBox::new(Orientation::Vertical, 2);

            let option_label = Label::new(Some(option));
            option_label.set_markup(&format!("<b>{}</b>", option));
            option_label.set_halign(gtk::Align::Start);
            option_box.append(&option_label);

            let desc_label = Label::new(Some(description));
            desc_label.set_halign(gtk::Align::Start);
            desc_label.add_css_class("dim-label");
            desc_label.set_margin_start(20);
            option_box.append(&desc_label);

            menu_box.append(&option_box);
        }

        vbox.append(&menu_box);
        window.set_child(Some(&vbox));

        // Set up keyboard handler
        let key_controller = gtk::EventControllerKey::new();
        let window_weak = window.downgrade();

        key_controller.connect_key_pressed(move |_, key, _, _| {
            let window = match window_weak.upgrade() {
                Some(w) => w,
                None => return glib::Propagation::Proceed,
            };

            // Handle Escape
            if key == gtk::gdk::Key::Escape {
                window.close();
                return glib::Propagation::Stop;
            }

            let exe = std::env::current_exe().unwrap_or_else(|_| "omni-menu".into());

            match key {
                gtk::gdk::Key::a => {
                    Command::new(&exe).arg("add_window").spawn().ok();
                    window.close();
                    return glib::Propagation::Stop;
                }
                gtk::gdk::Key::c => {
                    show_create_dialog(&window);
                    return glib::Propagation::Stop;
                }
                gtk::gdk::Key::s => {
                    Command::new(&exe).arg("switch_bench").spawn().ok();
                    window.close();
                    return glib::Propagation::Stop;
                }
                gtk::gdk::Key::t => {
                    Command::new(&exe).arg("launch_tool").spawn().ok();
                    window.close();
                    return glib::Propagation::Stop;
                }
                gtk::gdk::Key::y => {
                    Command::new("yard").args(["bench", "sync"]).spawn().ok();
                    window.close();
                    return glib::Propagation::Stop;
                }
                _ => {}
            }

            glib::Propagation::Proceed
        });

        window.add_controller(key_controller);
        window.present();
    }

    fn show_create_dialog(parent: &ApplicationWindow) {
        let dialog = Window::builder()
            .title("Create Bench")
            .transient_for(parent)
            .modal(true)
            .default_width(400)
            .default_height(150)
            .build();

        let vbox = GtkBox::new(Orientation::Vertical, 10);
        vbox.set_margin_top(20);
        vbox.set_margin_bottom(20);
        vbox.set_margin_start(20);
        vbox.set_margin_end(20);

        let label = Label::new(Some("Enter a name for the new bench:"));
        label.set_halign(gtk::Align::Start);
        vbox.append(&label);

        let entry = Entry::new();
        entry.set_placeholder_text(Some("bench-name"));
        vbox.append(&entry);

        let hint_label = Label::new(Some("Press Enter to create, Escape to cancel"));
        hint_label.add_css_class("dim-label");
        hint_label.set_halign(gtk::Align::Start);
        hint_label.set_margin_top(10);
        vbox.append(&hint_label);

        dialog.set_child(Some(&vbox));

        // Handle Enter key in entry
        let dialog_weak = dialog.downgrade();
        let parent_weak = parent.downgrade();
        entry.connect_activate(move |entry| {
            let name = entry.text().to_string();
            if !name.is_empty() {
                // Create, focus, and sync the bench
                Command::new("yard")
                    .args(["bench", "create", &name])
                    .spawn()
                    .ok();
                Command::new("yard")
                    .args(["bench", "focus", &name])
                    .spawn()
                    .ok();
                Command::new("yard")
                    .args(["bench", "sync"])
                    .spawn()
                    .ok();
            }
            if let Some(dialog) = dialog_weak.upgrade() {
                dialog.close();
            }
            if let Some(parent) = parent_weak.upgrade() {
                parent.close();
            }
        });

        // Handle Escape key in dialog
        let key_controller = gtk::EventControllerKey::new();
        let dialog_weak = dialog.downgrade();
        key_controller.connect_key_pressed(move |_, key, _, _| {
            if key == gtk::gdk::Key::Escape {
                if let Some(dialog) = dialog_weak.upgrade() {
                    dialog.close();
                }
                return glib::Propagation::Stop;
            }
            glib::Propagation::Proceed
        });
        dialog.add_controller(key_controller);

        dialog.present();
        entry.grab_focus();
    }

    pub fn run_app() -> glib::ExitCode {
        let app = Application::builder().application_id(APP_ID).build();
        app.connect_activate(build_ui);
        let no_args: [&str; 0] = [];
        app.run_with_args(&no_args)
    }
}
