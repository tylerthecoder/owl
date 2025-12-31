pub mod emoji_menu {
    use gtk::glib;
    use std::process::Command;

    pub fn run_app() -> glib::ExitCode {
        // Simply launch rofi in emoji mode
        match Command::new("rofi")
            .args(["-modi", "emoji", "-show", "emoji"])
            .spawn()
        {
            Ok(_) => glib::ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("Failed to launch rofi emoji picker: {}", e);
                glib::ExitCode::FAILURE
            }
        }
    }
}
