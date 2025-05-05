use chrono::Utc;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Serialize, Clone)]
struct LogMessagePayload {
  message: String,
  jump_to: String,
  created_at: String,
}

impl LogMessagePayload {
  pub fn new(message: &str, jump_to: &str) -> Self {
    let created_at = Utc::now().to_rfc3339();

    LogMessagePayload {
      message: message.to_string(),
      jump_to: jump_to.to_string(),
      created_at,
    }
  }
}

pub fn log_info(message: &str, jump_to: &str, app: &AppHandle) {
  let _ = app.emit("log-info", LogMessagePayload::new(message, jump_to));
}

pub fn log_alert(message: &str, jump_to: &str, app: &AppHandle) {
  let _ = app.emit("log-alert", LogMessagePayload::new(message, jump_to));
}

pub fn log_error(message: &str, jump_to: &str, app: &AppHandle) {
  let _ = app.emit("log-error", LogMessagePayload::new(message, jump_to));
}