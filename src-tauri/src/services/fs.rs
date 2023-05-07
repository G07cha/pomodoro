use std::{fs, io};

use tauri::AppHandle;

use crate::state::Settings;

const SETTINGS_FILENAME: &str = "settings.json";

pub fn load_settings(handle: AppHandle) -> Result<Settings, io::Error> {
  let app_data_dir = handle
    .path_resolver()
    .app_data_dir()
    .expect("failed to resolve app data dir");
  let settings_path = app_data_dir.join(SETTINGS_FILENAME);
  let settings_data = fs::read_to_string(settings_path)?;

  Ok(serde_json::from_str::<Settings>(&settings_data).expect("Invalid settings format received"))
}

pub fn save_settings(handle: AppHandle, new_settings: &Settings) -> io::Result<()> {
  let app_data_dir = handle
    .path_resolver()
    .app_data_dir()
    .expect("failed to resolve app data dir");
  let settings_path = app_data_dir.join(SETTINGS_FILENAME);

  fs::create_dir_all(app_data_dir)?;
  fs::write(
    settings_path,
    serde_json::to_string::<Settings>(new_settings).unwrap(),
  )
}
