use std::sync::{Arc, Mutex};

use serde::{Serialize, Deserialize};
use optional_struct::{optional_struct, Applicable};
use tauri::{AppHandle, State};

use crate::{extracter::try_launch, AppState};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ExtractedHandlingMode {
  NoAction,
  Delete,
  Move,
}

#[optional_struct]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Directory {
  pub monitor: String,
  pub output: String,
}

#[optional_struct]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ExtractedHandling {
  pub handling_mode: ExtractedHandlingMode,
  pub delete_permanently: bool,
  pub move_destination: String,
}

#[optional_struct]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Behavior {
  pub create_wrapper_folder: bool,
  pub extracted_handling: ExtractedHandling,
}

#[optional_struct]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
  pub directory: Directory,
  pub behavior: Behavior,
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

#[tauri::command]
pub fn load_config() -> Config {
  confy::load("autoextracter-bms", "config").expect("Failed to load configuration")
}

pub fn save_config(config: &Config) {
  confy::store("autoextracter-bms", "config", config).expect("Failed to save configuration")
}

#[tauri::command]
pub fn write_config(config: OptionalConfig, state: State<'_, Arc<Mutex<AppState>>>, app: AppHandle) {
  {
    let mut state = state.lock().unwrap();
    let state_config = &mut state.config;
    config.apply_to(state_config);
    save_config(&state_config);
  }
  try_launch(state, &app);
}