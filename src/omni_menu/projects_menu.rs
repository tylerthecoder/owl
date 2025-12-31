pub mod projects_menu {
    use fuzzy_matcher::skim::SkimMatcherV2;
    use fuzzy_matcher::FuzzyMatcher;
    use gtk::prelude::*;
    use gtk::{glib, Application, ApplicationWindow, Entry, ListBox, ScrolledWindow};
    use regex::Regex;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;
    use std::rc::Rc;

    const APP_ID: &str = "org.gtk_rs.ProjectsMenu";

    fn get_dev_directories() -> Vec<PathBuf> {
        let home = dirs::home_dir().unwrap();
        let mut directories = Vec::new();
        directories.push(home.join("owl"));
        if let Ok(entries) = fs::read_dir(home.join("dev")) {
            directories.extend(entries.filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_dir() {
                    Some(path)
                } else {
                    None
                }
            }));
        }
        directories
    }

    fn normalize_name(name: &str) -> String {
        let with_spaces = name.replace(['-', '_'], " ");
        let re = Regex::new(r"([a-z])([A-Z])").unwrap();
        let separated = re.replace_all(&with_spaces, "$1 $2");
        separated
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>()
                            + &chars.collect::<String>().to_lowercase()
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn launch_workspace(workspace_name: &str, directory: &PathBuf) {
        Command::new("i3-msg")
            .args(["workspace", workspace_name])
            .spawn()
            .expect("Failed to switch workspace");
        Command::new("i3-msg")
            .args(["layout", "tabbed"])
            .spawn()
            .expect("Failed to set tabbed layout");
        Command::new("terminator")
            .arg("--working-directory")
            .arg(directory)
            .spawn()
            .expect("Failed to launch terminal");
        Command::new("cursor")
            .arg(directory)
            .spawn()
            .expect("Failed to launch editor");
        Command::new("chromium")
            .spawn()
            .expect("Failed to launch browser");
    }

    fn launch_remote_workspace(workspace_name: &str, host: &str, remote_path: &str) {
        Command::new("i3-msg")
            .args(["workspace", workspace_name])
            .spawn()
            .expect("Failed to switch workspace");
        Command::new("i3-msg")
            .args(["layout", "tabbed"])
            .spawn()
            .expect("Failed to set tabbed layout");
        Command::new("terminator")
            .args(["-e", &format!("ssh {}", host)])
            .spawn()
            .expect("Failed to launch remote terminal");
        Command::new("cursor")
            .args(["--remote", &format!("ssh-remote+{}", host), remote_path])
            .spawn()
            .expect("Failed to launch remote editor");
        Command::new("chromium")
            .spawn()
            .expect("Failed to launch browser");
    }

    fn get_remote_dev_directories(host: &str) -> Vec<String> {
        let output = Command::new("ssh")
            .args([host, "ls -1 -d ~/dev/*/ 2>/dev/null || true"])
            .output();

        println!("output: {:?}", output);
        match output {
            Ok(o) => {
                let stdout = String::from_utf8_lossy(&o.stdout);
                stdout
                    .lines()
                    .map(|line| line.trim().trim_end_matches('/').to_string())
                    .filter(|line| !line.is_empty())
                    .collect()
            }
            Err(_) => Vec::new(),
        }
    }

    fn build_ui(app: &Application) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Project Launcher") // Changed title
            .default_width(400)
            .default_height(600)
            .decorated(true)
            .resizable(true)
            .modal(true)
            .build();
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let controls = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        let btn_local = gtk::Button::with_label("Local");
        let btn_remote = gtk::Button::with_label("Remote (ada)");
        let btn_mrwood = gtk::Button::with_label("Remote (mrwood)");
        let search_entry = Entry::new();
        let list_box = ListBox::new();
        list_box.set_vexpand(true);
        list_box.set_selection_mode(gtk::SelectionMode::Single);
        list_box.set_activate_on_single_click(true);
        let scrolled_window = ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .vexpand(true)
            .child(&list_box)
            .build();
        controls.append(&btn_local);
        controls.append(&btn_remote);
        controls.append(&btn_mrwood);
        vbox.append(&controls);
        vbox.append(&search_entry);
        vbox.append(&scrolled_window);
        window.set_child(Some(&vbox));
        let directories = get_dev_directories();
        for dir in &directories {
            let name = dir.file_name().unwrap().to_str().unwrap();
            let normalized = normalize_name(name);
            let label = gtk::Label::new(Some(&normalized));
            label.set_xalign(0.0);
            label.set_margin_start(5);
            label.set_margin_end(5);
            let row = gtk::ListBoxRow::new();
            row.set_child(Some(&label));
            list_box.append(&row);
        }
        let directories_rc = Rc::new(directories);
        let current_remote: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
        let remote_cache: Rc<RefCell<HashMap<String, Vec<String>>>> =
            Rc::new(RefCell::new(HashMap::new()));
        let list_box_weak_for_buttons = list_box.downgrade();
        let search_entry_weak_for_buttons = search_entry.downgrade();
        let directories_rc_for_buttons = directories_rc.clone();
        let current_remote_for_local = current_remote.clone();
        btn_local.connect_clicked(move |_| {
            *current_remote_for_local.borrow_mut() = None;
            if let (Some(list_box), Some(search_entry)) = (
                list_box_weak_for_buttons.upgrade(),
                search_entry_weak_for_buttons.upgrade(),
            ) {
                while let Some(row) = list_box.first_child() {
                    list_box.remove(&row);
                }
                let matcher = SkimMatcherV2::default();
                let query = search_entry.text();
                let mut matches: Vec<_> = directories_rc_for_buttons
                    .iter()
                    .filter_map(|dir| {
                        let name = dir.file_name().unwrap().to_str().unwrap();
                        matcher
                            .fuzzy_match(name, &query)
                            .map(|score| (score, name.to_string(), dir.clone()))
                    })
                    .collect();
                matches.sort_by(|a, b| b.0.cmp(&a.0));
                for (_, name, _dir) in matches {
                    let normalized = normalize_name(&name);
                    let label = gtk::Label::new(Some(&normalized));
                    label.set_xalign(0.0);
                    label.set_margin_start(5);
                    label.set_margin_end(5);
                    let row = gtk::ListBoxRow::new();
                    row.set_child(Some(&label));
                    list_box.append(&row);
                }
            }
        });
        let list_box_weak_for_buttons = list_box.downgrade();
        let search_entry_weak_for_buttons = search_entry.downgrade();
        let current_remote_for_ada = current_remote.clone();
        let remote_cache_for_ada = remote_cache.clone();
        btn_remote.connect_clicked(move |_| {
            let host = "ada".to_string();
            *current_remote_for_ada.borrow_mut() = Some(host.clone());
            if !remote_cache_for_ada.borrow().contains_key(&host) {
                let fetched = get_remote_dev_directories(&host);
                remote_cache_for_ada
                    .borrow_mut()
                    .insert(host.clone(), fetched);
            }
            if let (Some(list_box), Some(search_entry)) = (
                list_box_weak_for_buttons.upgrade(),
                search_entry_weak_for_buttons.upgrade(),
            ) {
                while let Some(row) = list_box.first_child() {
                    list_box.remove(&row);
                }
                let matcher = SkimMatcherV2::default();
                let query = search_entry.text();
                if let Some(remote_dirs) = remote_cache_for_ada.borrow().get(&host) {
                    let mut matches: Vec<_> = remote_dirs
                        .iter()
                        .filter_map(|path| {
                            let name = path
                                .trim_end_matches('/')
                                .rsplit('/')
                                .next()
                                .unwrap_or(path);
                            matcher
                                .fuzzy_match(name, &query)
                                .map(|score| (score, name.to_string(), path.to_string()))
                        })
                        .collect();
                    matches.sort_by(|a, b| b.0.cmp(&a.0));
                    for (_, name, _path) in matches {
                        let normalized = normalize_name(&name);
                        let label = gtk::Label::new(Some(&normalized));
                        label.set_xalign(0.0);
                        label.set_margin_start(5);
                        label.set_margin_end(5);
                        let row = gtk::ListBoxRow::new();
                        row.set_child(Some(&label));
                        list_box.append(&row);
                    }
                }
            }
        });
        let list_box_weak_for_buttons = list_box.downgrade();
        let search_entry_weak_for_buttons = search_entry.downgrade();
        let current_remote_for_mrwood = current_remote.clone();
        let remote_cache_for_mrwood = remote_cache.clone();
        btn_mrwood.connect_clicked(move |_| {
            let host = "mrwood".to_string();
            *current_remote_for_mrwood.borrow_mut() = Some(host.clone());
            if !remote_cache_for_mrwood.borrow().contains_key(&host) {
                let fetched = get_remote_dev_directories(&host);
                remote_cache_for_mrwood
                    .borrow_mut()
                    .insert(host.clone(), fetched);
            }
            if let (Some(list_box), Some(search_entry)) = (
                list_box_weak_for_buttons.upgrade(),
                search_entry_weak_for_buttons.upgrade(),
            ) {
                while let Some(row) = list_box.first_child() {
                    list_box.remove(&row);
                }
                let matcher = SkimMatcherV2::default();
                let query = search_entry.text();
                if let Some(remote_dirs) = remote_cache_for_mrwood.borrow().get(&host) {
                    let mut matches: Vec<_> = remote_dirs
                        .iter()
                        .filter_map(|path| {
                            let name = path
                                .trim_end_matches('/')
                                .rsplit('/')
                                .next()
                                .unwrap_or(path);
                            matcher
                                .fuzzy_match(name, &query)
                                .map(|score| (score, name.to_string(), path.to_string()))
                        })
                        .collect();
                    matches.sort_by(|a, b| b.0.cmp(&a.0));
                    for (_, name, _path) in matches {
                        let normalized = normalize_name(&name);
                        let label = gtk::Label::new(Some(&normalized));
                        label.set_xalign(0.0);
                        label.set_margin_start(5);
                        label.set_margin_end(5);
                        let row = gtk::ListBoxRow::new();
                        row.set_child(Some(&label));
                        list_box.append(&row);
                    }
                }
            }
        });
        let list_box_weak = list_box.downgrade();
        search_entry.connect_activate(move |_entry| {
            let list_box = list_box_weak.upgrade().unwrap();
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
        });
        let list_box_weak = list_box.downgrade();
        let directories_rc_for_search = directories_rc.clone();
        let current_remote_for_search = current_remote.clone();
        let remote_cache_for_search = remote_cache.clone();
        search_entry.connect_changed(move |entry| {
            let list_box = list_box_weak.upgrade().unwrap();
            let matcher = SkimMatcherV2::default();
            let query = entry.text();
            while let Some(row) = list_box.first_child() {
                list_box.remove(&row);
            }
            if let Some(host) = current_remote_for_search.borrow().clone() {
                if !remote_cache_for_search.borrow().contains_key(&host) {
                    let fetched = get_remote_dev_directories(&host);
                    remote_cache_for_search
                        .borrow_mut()
                        .insert(host.clone(), fetched);
                }
                if let Some(remote_dirs) = remote_cache_for_search.borrow().get(&host) {
                    let mut matches: Vec<_> = remote_dirs
                        .iter()
                        .filter_map(|path| {
                            let name = path
                                .trim_end_matches('/')
                                .rsplit('/')
                                .next()
                                .unwrap_or(path);
                            matcher
                                .fuzzy_match(name, &query)
                                .map(|score| (score, name.to_string(), path.to_string()))
                        })
                        .collect();
                    matches.sort_by(|a, b| b.0.cmp(&a.0));
                    for (_, name, _path) in matches {
                        let normalized = normalize_name(&name);
                        let label = gtk::Label::new(Some(&normalized));
                        label.set_xalign(0.0);
                        label.set_margin_start(5);
                        label.set_margin_end(5);
                        let row = gtk::ListBoxRow::new();
                        row.set_child(Some(&label));
                        list_box.append(&row);
                    }
                }
            } else {
                let mut matches: Vec<_> = directories_rc_for_search
                    .iter()
                    .filter_map(|dir| {
                        let name = dir.file_name().unwrap().to_str().unwrap();
                        matcher
                            .fuzzy_match(name, &query)
                            .map(|score| (score, name.to_string(), dir.clone()))
                    })
                    .collect();
                matches.sort_by(|a, b| b.0.cmp(&a.0));
                for (_, name, _dir) in matches {
                    let normalized = normalize_name(&name);
                    let label = gtk::Label::new(Some(&normalized));
                    label.set_xalign(0.0);
                    label.set_margin_start(5);
                    label.set_margin_end(5);
                    let row = gtk::ListBoxRow::new();
                    row.set_child(Some(&label));
                    list_box.append(&row);
                }
            }
        });
        let window_weak = window.downgrade();
        let directories_rc_for_activate = directories_rc.clone();
        let current_remote_for_activate = current_remote.clone();
        let remote_cache_for_activate = remote_cache.clone();
        list_box.connect_row_activated(move |_list_box, row| {
            let label = row.child().unwrap().downcast::<gtk::Label>().unwrap();
            let selected_name = label.text();
            if let Some(host) = current_remote_for_activate.borrow().clone() {
                if let Some(remote_dirs) = remote_cache_for_activate.borrow().get(&host) {
                    if let Some(remote_path) = remote_dirs.iter().find_map(|path| {
                        let name = path
                            .trim_end_matches('/')
                            .rsplit('/')
                            .next()
                            .unwrap_or(path);
                        if normalize_name(name) == selected_name.as_str() {
                            Some(path.clone())
                        } else {
                            None
                        }
                    }) {
                        launch_remote_workspace(&selected_name, &host, &remote_path);
                    }
                }
            } else {
                if let Some(selected_dir) = directories_rc_for_activate.iter().find(|dir| {
                    normalize_name(dir.file_name().unwrap().to_str().unwrap())
                        == selected_name.as_str()
                }) {
                    launch_workspace(&selected_name, selected_dir);
                }
            }
            if let Some(window) = window_weak.upgrade() {
                window.close();
            }
        });
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
