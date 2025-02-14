use tauri_plugin_autostart::MacosLauncher;

mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            core::search_window_switch::create(app);
            core::search_window_switch::create_focus_handler(app);
            #[cfg(all(desktop))]
            {
                let handle = app.handle();
                core::tray::create_tray(handle)?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            core::commands::set_search_toggle,
            core::commands::prompt_search,
            core::commands::open
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
