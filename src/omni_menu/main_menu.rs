pub mod main_menu {
    use gtk::prelude::*;
    use gtk::{glib, Application, ApplicationWindow, Box as GtkBox, Label, Orientation};
    use std::process::Command;

    const APP_ID: &str = "org.gtk_rs.OmniMenu";

    fn build_ui(app: &Application) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Omni Menu")
            .default_width(500)
            .default_height(400)
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
        let title_label = Label::new(Some("Main Menu"));
        title_label.set_markup("<span size='large' weight='bold'>Main Menu</span>");
        title_label.set_halign(gtk::Align::Start);
        vbox.append(&title_label);

        // Instructions label
        let instructions_label = Label::new(Some("Press a hotkey to open a submenu"));
        instructions_label.set_halign(gtk::Align::Start);
        instructions_label.set_margin_top(10);
        vbox.append(&instructions_label);

        // Menu options box
        let menu_box = GtkBox::new(Orientation::Vertical, 5);
        menu_box.set_margin_top(10);

        let options = vec![
            ("(t) Launch Tool", "Add a tool to the current workspace"),
            ("(a) Launch App", "Launch any application"),
            ("(b) Switch Bench", "Switch to a different bench"),
            ("(s) Scripts", "Run owl scripts"),
            ("(x) Search", "Search the web"),
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

            let exe = std::env::current_exe().unwrap_or_else(|_| "rust-menu".into());

            match key {
                gtk::gdk::Key::t => {
                    Command::new(&exe).arg("launch_tool").spawn().ok();
                    window.close();
                    return glib::Propagation::Stop;
                }
                gtk::gdk::Key::a => {
                    Command::new("rofi").args(["-show", "drun"]).spawn().ok();
                    window.close();
                    return glib::Propagation::Stop;
                }
                gtk::gdk::Key::b => {
                    Command::new(&exe).arg("switch_bench").spawn().ok();
                    window.close();
                    return glib::Propagation::Stop;
                }
                gtk::gdk::Key::s => {
                    Command::new(&exe).arg("scripts").spawn().ok();
                    window.close();
                    return glib::Propagation::Stop;
                }
                gtk::gdk::Key::x => {
                    Command::new(&exe).arg("search").spawn().ok();
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

    pub fn run_app() -> glib::ExitCode {
        let app = Application::builder().application_id(APP_ID).build();
        app.connect_activate(build_ui);
        let no_args: [&str; 0] = [];
        app.run_with_args(&no_args)
    }
}
