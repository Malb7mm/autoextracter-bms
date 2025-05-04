mod fs;
mod config;
use config::Config;
use std::sync::{Arc, Mutex};

struct AppState {
    config: Config,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(AppState {
            config: config::load_config(),
        })))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fs::get_bin_dir, 
            fs::is_valid_dir,
            config::write_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
