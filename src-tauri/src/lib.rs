mod fs;
mod config;
mod extracter;
mod command;
use config::Config;
use std::sync::{Arc, Mutex};

struct WarnedInvalid {
    monitor: bool,
    output: bool,
    move_dest: bool,
}

struct AppState {
    config: Config,
    warned_invalid: WarnedInvalid,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(AppState {
            config: config::load_config(),
            warned_invalid: WarnedInvalid {
                monitor: false,
                output: false,
                move_dest: false,
            }
        })))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fs::get_bin_dir, 
            fs::is_valid_dir,
            config::write_config,
            config::load_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
