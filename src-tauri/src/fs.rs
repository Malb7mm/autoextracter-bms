use std::{env, fs, path::{Component, Path, PathBuf, Prefix}};
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub fn get_bin_dir() -> Result<String, String> {
    let path = env::current_exe().map_err(|e| e.to_string())?;
    let dir = path
        .parent()
        .and_then(|p| p.to_str())
        .ok_or_else(|| "Failed to get binary directory".to_string())?;
    Ok(dir.to_string())
}

#[tauri::command]
pub fn is_valid_dir(path_str: &str) -> bool {
    Path::is_dir(Path::new(path_str))
}

fn is_unc(path: &PathBuf) -> bool {
    cfg!(windows) && matches!(
        path.components().next(), 
        Some(Component::Prefix(p)) if !matches!(p.kind(), Prefix::Disk(_))
    )
}

#[tauri::command]
pub fn open_dir(pathStr: String, app: AppHandle) -> Result<(), String> {
    let path = PathBuf::from(pathStr);

    if is_unc(&path) || path.to_string_lossy().starts_with("http") {
        return Err("Remote path is not allowed".into());
    }

    let canonical = path.canonicalize().map_err(|e| e.to_string())?;

    let dir = if fs::metadata(&canonical).map_err(|e| e.to_string())?.is_dir() {
        canonical
    } else {
        canonical
            .parent()
            .ok_or("No parent directory")?
            .to_path_buf()
    };

    app.opener()
        .open_path(dir.to_string_lossy(), None::<&str>)
        .map_err(|e| e.to_string())
}