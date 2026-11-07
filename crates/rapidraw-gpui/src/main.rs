use rapidraw_gpui::run_desktop_app; // <--- Calls the desktop-specific entry point

fn main() {
    // Linux Wayland workaround (remains as is)
    #[cfg(target_os = "linux")]
    {
        if std::env::var("WAYLAND_DISPLAY").is_ok() && std::env::var("RAPIDRAW_X11_FORCED").is_err()
        {
            if let Ok(current_exe) = std::env::current_exe() {
                let status = std::process::Command::new(current_exe)
                    .args(std::env::args().skip(1))
                    .env_remove("WAYLAND_DISPLAY")
                    .env("RAPIDRAW_X11_FORCED", "1")
                    .status();

                if let Ok(code) = status {
                    std::process::exit(code.code().unwrap_or(0));
                }
            }
        }
    }

    run_desktop_app(); // <--- Call the new desktop entry point
}
