use std::sync::Mutex;

use serde::{Serialize, Deserialize};
use optional_struct::{optional_struct, Applicable};
use tauri::State;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum ExtractedHandlingMode {
  NoAction,
  Delete,
  Move,
}

#[optional_struct]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Directory {
  monitor: String,
  output: String,
}

#[optional_struct]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct ExtractedHandling {
  handling_mode: ExtractedHandlingMode,
  delete_permanently: bool,
  move_destination: String,
}

#[optional_struct]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Behavior {
  create_wrapper_folder: bool,
  extracted_handling: ExtractedHandling,
}

#[optional_struct]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
  directory: Directory,
  behavior: Behavior,
}

impl ::std::default::Default for Config {
  fn default() -> Self {
    Config {
      directory: Directory {
        monitor: "".to_string(),
        output: "".to_string(),
      },
      behavior: Behavior {
        create_wrapper_folder: true,
        extracted_handling: ExtractedHandling { 
          handling_mode: ExtractedHandlingMode::NoAction, 
          delete_permanently: false,
          move_destination: "".to_string(),
        }
      }
    }
  }
}

pub fn load_config() -> Config {
  confy::load("autoextracter-bms", "config").expect("Failed to load configuration")
}

pub fn save_config(config: &Config) {
  confy::store("autoextracter-bms", "config", config).expect("Failed to save configuration")
}

#[tauri::command]
pub fn write_config(config: OptionalConfig, state: State<'_, Mutex<AppState>>) {
  let mut state = state.lock().unwrap();
  let state_config = &mut state.config;
  config.apply_to(state_config);
  save_config(&state_config);
}