use std::env;

#[tauri::command]
pub fn get_bin_dir() -> Result<String, String> {
  let exe_path = env::current_exe().map_err(|e| e.to_string())?;
  let dir = exe_path
    .parent()
    .and_then(|p| p.to_str())
    .ok_or_else(|| "親ディレクトリを取得できませんでした".to_string())?;
  Ok(dir.to_string())
}