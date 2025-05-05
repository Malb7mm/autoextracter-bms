use std::sync::{mpsc, Arc, Mutex};
use notify::{Event, Result};
use tauri::{AppHandle, State};

use crate::{log::log_alert, config::ExtractedHandlingMode, fs::is_valid_dir, AppState};

pub fn try_launch(state: State<'_, Arc<Mutex<AppState>>>, app: &AppHandle) {
  if !path_check(state, app) { return; }

  let (tx, rx) = mpsc::channel::<Result<Event>>();
  let mut watcher = notify::recommended_watcher(tx).unwrap();
}

fn path_check(state: State<'_, Arc<Mutex<AppState>>>, app: &AppHandle) -> bool {
  let mut state = state.lock().unwrap();
  let mut result = true;

  if !is_valid_dir(&state.config.directory.monitor) {
    result = false;
    let warned_invalid = &mut state.warned_invalid;
    if !warned_invalid.monitor {
      log_alert("項目「監視するディレクトリ」の値が無効です。\
      有効なディレクトリが指定されるまで監視は開始されません。", "/settings", app);
      warned_invalid.monitor = true;
    }
  }

  if !is_valid_dir(&state.config.directory.output) {
    result = false;
    let warned_invalid = &mut state.warned_invalid;
    if !warned_invalid.output {
      log_alert("項目「展開先ディレクトリ」の値が無効です。\
      有効なディレクトリが指定されるまで監視は開始されません。", "/settings", app);
      warned_invalid.output = true;
    }
  }

  if state.config.behavior.extracted_handling.handling_mode == ExtractedHandlingMode::Move
      && !is_valid_dir(&state.config.behavior.extracted_handling.move_destination) {
    result = false;
    let warned_invalid = &mut state.warned_invalid;
    if !warned_invalid.move_dest {
      log_alert("項目「展開済み圧縮ファイルの削除・移動」の選択肢が「指定ディレクトリへ移動する」に設定されていますが、\
      ディレクトリの値が無効です。有効なディレクトリが指定されるまで監視は開始されません。", "/settings", app);
      warned_invalid.move_dest = true;
    }
  }

  result
}