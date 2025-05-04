use std::{env, path::Path};

#[tauri::command]
pub fn get_bin_dir() -> Result<String, String> {
    let path = env::current_exe().map_err(|e| e.to_string())?;
    let dir = path
        .parent()
        .and_then(|p| p.to_str())
        .ok_or_else(|| "親ディレクトリを取得できませんでした".to_string())?;
    Ok(dir.to_string())
}

#[tauri::command]
pub fn is_valid_dir(path_str: &str) -> bool {
    Path::is_dir(Path::new(path_str))
}
