use anyhow::{Context, Result};
use std::fs;

use tauri::AppHandle;

use crate::state::Settings;

const SETTINGS_FILENAME: &str = "settings.json";

pub fn load_settings(handle: &AppHandle) -> Result<Settings> {
  let app_data_dir = handle
    .path_resolver()
    .app_data_dir()
    .context("Failed to resolve app data dir")?;
  let settings_path = app_data_dir.join(SETTINGS_FILENAME);
  let settings_data = fs::read_to_string(&settings_path)
    .with_context(|| format!("Failed to read settings from {}", settings_path.display()))?;
  let parsed_settings = serde_json::from_str::<Settings>(&settings_data)
    .with_context(|| format!("Failed to parse settings, received {}", settings_data))?;

  Ok(parsed_settings)
}

pub fn save_settings(handle: &AppHandle, new_settings: &Settings) -> Result<()> {
  let app_data_dir = handle
    .path_resolver()
    .app_data_dir()
    .context("Failed to resolve app data dir")?;
  let settings_path = app_data_dir.join(SETTINGS_FILENAME);
  let file_contents =
    serde_json::to_string::<Settings>(new_settings).context("Failed to serialize settings")?;

  fs::create_dir_all(&app_data_dir)
    .with_context(|| format!("Failed to create directories in {}", app_data_dir.display()))?;
  fs::write(&settings_path, file_contents)
    .with_context(|| format!("Failed to write settings to {}", settings_path.display()))?;

  Ok(())
}
