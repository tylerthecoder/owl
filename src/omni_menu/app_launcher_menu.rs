pub mod app_launcher_menu {
    use fuzzy_matcher::skim::SkimMatcherV2;
    use fuzzy_matcher::FuzzyMatcher;
    use gtk::gdk::Texture;
    use gtk::prelude::*;
    use gtk::{
        gdk_pixbuf::Pixbuf, glib, Application, ApplicationWindow, Box as GtkBox, Entry,
        IconTheme, Image, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow,
    };
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;
    use std::rc::Rc;

    const APP_ID: &str = "org.gtk_rs.AppLauncherMenu";

    #[derive(Clone, Debug)]
    struct DesktopApp {
        name: String,
        exec: String,
        icon: Option<String>,
        desktop_id: String, // e.g., "chromium" or "org.chromium.Chromium"
    }

    // Apps that should be launched via yard instead of directly
    fn get_yard_overrides() -> HashMap<&'static str, Vec<&'static str>> {
        let mut overrides = HashMap::new();
        // Map app identifiers to yard tool craft arguments
        overrides.insert("chromium", vec!["yard", "tool", "assemble"]);
        overrides.insert("chromium-browser", vec!["yard", "tool", "assemble"]);
        overrides.insert("google-chrome", vec!["yard", "tool", "assemble"]);
        overrides.insert("org.chromium.Chromium", vec!["yard", "tool", "assemble"]);
        overrides
    }

    fn is_yard_override(desktop_id: &str) -> bool {
        let overrides = get_yard_overrides();
        overrides.contains_key(desktop_id)
    }

    fn build_ui(app: &Application) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Launch Application")
            .default_width(600)
            .default_height(500)
            .decorated(true)
            .resizable(false)
            .modal(true)
            .build();

        let vbox = GtkBox::new(Orientation::Vertical, 8);
        vbox.set_margin_top(12);
        vbox.set_margin_bottom(12);
        vbox.set_margin_start(12);
        vbox.set_margin_end(12);

        // Title
        let title_label = Label::new(Some("Launch Application"));
        title_label.set_markup("<span size='large' weight='bold'>Launch Application</span>");
        title_label.set_halign(gtk::Align::Start);
        title_label.set_margin_bottom(8);
        vbox.append(&title_label);

        // Search entry
        let search_entry = Entry::new();
        search_entry.set_placeholder_text(Some("Type to search..."));
        search_entry.set_margin_bottom(8);
        vbox.append(&search_entry);

        // Scrolled window for app list
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

        // Load applications
        let apps = load_desktop_apps();
        let apps_rc = Rc::new(apps);

        // Get icon theme
        let display = gtk::prelude::WidgetExt::display(&window);
        let icon_theme = IconTheme::for_display(&display);

        // Populate list
        populate_app_list(&list_box, &apps_rc, &icon_theme, "");

        // Store apps and list_box for filtering
        let apps_for_filter = apps_rc.clone();
        let list_box_weak = list_box.downgrade();
        let icon_theme_for_filter = icon_theme.clone();

        search_entry.connect_changed(move |entry| {
            if let Some(list_box) = list_box_weak.upgrade() {
                let query = entry.text().to_string();
                populate_app_list(&list_box, &apps_for_filter, &icon_theme_for_filter, &query);
            }
        });

        // Handle activation
        let window_weak = window.downgrade();
        let apps_for_activate = apps_rc.clone();
        let search_entry_for_activate = search_entry.clone();
        list_box.connect_row_activated(move |_list_box, row| {
            let index = row.index() as usize;

            // Get visible apps in current order
            let query = search_entry_for_activate.text().to_string();
            let visible_apps = filter_and_sort_apps(&apps_for_activate, &query);

            if index < visible_apps.len() {
                let app = &visible_apps[index];
                launch_app(app);

                if let Some(window) = window_weak.upgrade() {
                    window.close();
                }
            }
        });

        // Handle keyboard
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

    fn load_desktop_apps() -> Vec<DesktopApp> {
        let mut apps = Vec::new();
        let mut seen = std::collections::HashSet::new();

        // Standard locations for .desktop files
        let locations = vec![
            PathBuf::from("/usr/share/applications"),
            PathBuf::from("/usr/local/share/applications"),
            dirs::data_dir()
                .map(|d| d.join("applications"))
                .unwrap_or_default(),
            dirs::home_dir()
                .map(|d| d.join(".local/share/applications"))
                .unwrap_or_default(),
        ];

        for location in locations {
            if !location.exists() {
                continue;
            }

            if let Ok(entries) = fs::read_dir(&location) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|e| e == "desktop").unwrap_or(false) {
                        if let Some(app) = parse_desktop_file(&path) {
                            // Deduplicate by name
                            if seen.insert(app.name.clone()) {
                                apps.push(app);
                            }
                        }
                    }
                }
            }
        }

        // Sort alphabetically
        apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        apps
    }

    fn parse_desktop_file(path: &PathBuf) -> Option<DesktopApp> {
        let content = fs::read_to_string(path).ok()?;

        let mut name = None;
        let mut exec = None;
        let mut icon = None;
        let mut no_display = false;
        let mut hidden = false;
        let mut in_desktop_entry = false;

        for line in content.lines() {
            let line = line.trim();

            if line == "[Desktop Entry]" {
                in_desktop_entry = true;
                continue;
            }

            if line.starts_with('[') && line != "[Desktop Entry]" {
                in_desktop_entry = false;
                continue;
            }

            if !in_desktop_entry {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                match key.trim() {
                    "Name" if name.is_none() => name = Some(value.trim().to_string()),
                    "Exec" => exec = Some(value.trim().to_string()),
                    "Icon" => icon = Some(value.trim().to_string()),
                    "NoDisplay" => no_display = value.trim().eq_ignore_ascii_case("true"),
                    "Hidden" => hidden = value.trim().eq_ignore_ascii_case("true"),
                    _ => {}
                }
            }
        }

        // Skip hidden/nodisplay apps
        if no_display || hidden {
            return None;
        }

        let name = name?;
        let exec = exec?;

        // Extract desktop ID from filename
        let desktop_id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        Some(DesktopApp {
            name,
            exec,
            icon,
            desktop_id,
        })
    }

    fn filter_and_sort_apps(apps: &[DesktopApp], query: &str) -> Vec<DesktopApp> {
        if query.is_empty() {
            return apps.to_vec();
        }

        let matcher = SkimMatcherV2::default();
        let query_lower = query.to_lowercase();

        let mut scored: Vec<_> = apps
            .iter()
            .filter_map(|app| {
                let score = matcher.fuzzy_match(&app.name.to_lowercase(), &query_lower);
                score.map(|s| (app.clone(), s))
            })
            .collect();

        scored.sort_by(|a, b| b.1.cmp(&a.1));
        scored.into_iter().map(|(app, _)| app).collect()
    }

    fn populate_app_list(list_box: &ListBox, apps: &[DesktopApp], icon_theme: &IconTheme, query: &str) {
        // Clear existing rows
        while let Some(child) = list_box.first_child() {
            list_box.remove(&child);
        }

        let filtered_apps = filter_and_sort_apps(apps, query);

        for app in filtered_apps.iter().take(50) {
            // Limit to 50 for performance
            let row = create_app_row(app, icon_theme);
            list_box.append(&row);
        }

        // Select first row
        if let Some(first_row) = list_box.row_at_index(0) {
            list_box.select_row(Some(&first_row));
        }
    }

    fn create_app_row(app: &DesktopApp, icon_theme: &IconTheme) -> ListBoxRow {
        let hbox = GtkBox::new(Orientation::Horizontal, 12);
        hbox.set_margin_top(6);
        hbox.set_margin_bottom(6);
        hbox.set_margin_start(8);
        hbox.set_margin_end(8);

        // Icon
        let image = Image::new();
        image.set_pixel_size(32);

        if let Some(ref icon_name) = app.icon {
            // Try to load from icon theme first
            if icon_theme.has_icon(icon_name) {
                image.set_icon_name(Some(icon_name));
            } else if PathBuf::from(icon_name).exists() {
                // It's a full path
                if let Ok(pixbuf) = Pixbuf::from_file_at_size(icon_name, 32, 32) {
                    let texture = Texture::for_pixbuf(&pixbuf);
                    image.set_paintable(Some(&texture));
                }
            } else {
                image.set_icon_name(Some("application-x-executable"));
            }
        } else {
            image.set_icon_name(Some("application-x-executable"));
        }

        hbox.append(&image);

        // App info
        let info_box = GtkBox::new(Orientation::Vertical, 2);
        info_box.set_hexpand(true);

        let name_label = Label::new(Some(&app.name));
        name_label.set_halign(gtk::Align::Start);
        name_label.set_markup(&format!("<b>{}</b>", glib::markup_escape_text(&app.name)));
        info_box.append(&name_label);

        // Show if it's a yard-managed app
        if is_yard_override(&app.desktop_id) {
            let yard_label = Label::new(Some("Yard managed"));
            yard_label.set_halign(gtk::Align::Start);
            yard_label.add_css_class("dim-label");
            yard_label.set_markup("<span size='small' style='italic'>via Yard</span>");
            info_box.append(&yard_label);
        }

        hbox.append(&info_box);

        let row = ListBoxRow::new();
        row.set_child(Some(&hbox));
        row
    }

    fn launch_app(app: &DesktopApp) {
        if is_yard_override(&app.desktop_id) {
            // Launch via yard - show tool selection or create new
            // For now, we'll launch the launch_tool menu to let user pick/create a tool
            let exe = std::env::current_exe().unwrap_or_else(|_| "omni-menu".into());
            Command::new(&exe).arg("launch_tool").spawn().ok();
        } else {
            // Parse exec and launch directly
            let exec = parse_exec(&app.exec);
            if let Some((cmd, args)) = exec.split_first() {
                Command::new(cmd).args(args).spawn().ok();
            }
        }
    }

    fn parse_exec(exec: &str) -> Vec<String> {
        // Remove field codes like %u, %U, %f, %F, etc.
        let cleaned: String = exec
            .split_whitespace()
            .filter(|part| !part.starts_with('%'))
            .collect::<Vec<_>>()
            .join(" ");

        // Simple split - doesn't handle quotes perfectly but works for most cases
        shell_words::split(&cleaned).unwrap_or_else(|_| vec![exec.to_string()])
    }

    pub fn run_app() -> glib::ExitCode {
        let app = Application::builder().application_id(APP_ID).build();
        app.connect_activate(build_ui);
        let no_args: [&str; 0] = [];
        app.run_with_args(&no_args)
    }
}
