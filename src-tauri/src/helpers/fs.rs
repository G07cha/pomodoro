use anyhow::{Context, Result};
use std::fs;

use tauri::{AppHandle, Manager, Runtime};

use crate::state::Settings;

const SETTINGS_FILENAME: &str = "settings.json";

pub fn load_settings<R: Runtime>(handle: &AppHandle<R>) -> Result<Settings> {
  let app_data_dir = handle
    .path()
    .app_data_dir()
    .context("Failed to resolve app data dir")?;
  let settings_path = app_data_dir.join(SETTINGS_FILENAME);
  let settings_data = fs::read_to_string(&settings_path)
    .with_context(|| format!("Failed to read settings from {}", settings_path.display()))?;
  let parsed_settings = serde_json::from_str::<Settings>(&settings_data)
    .with_context(|| format!("Failed to parse settings, received {}", settings_data))?;

  Ok(parsed_settings)
}

pub fn save_settings<R: Runtime>(handle: &AppHandle<R>, new_settings: &Settings) -> Result<()> {
  let app_data_dir = handle
    .path()
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

#[cfg(test)]
mod tests {
  use super::*;
  use anyhow::Result;
  use serial_test::serial;
  use std::time::Duration;
  use tauri::test;
  use tauri::Manager;

  #[serial(fs)]
  #[test]
  fn it_saves_settings() {
    let app = test::mock_app();
    let app_handle = app.app_handle();

    assert!(save_settings(
      &app_handle,
      &Settings {
        work_duration: Duration::from_secs(100),
        relax_duration: Duration::from_secs(50),
        long_relax_duration: Duration::from_secs(70),
        toggle_timer_shortcut: Some("Ctrl + F".to_string()),
        should_play_sound: Some(true)
      }
    )
    .is_ok())
  }

  #[serial(fs)]
  #[test]
  fn it_loads_settings() -> Result<()> {
    let app = test::mock_app();
    let app_handle = app.app_handle();
    let settings = Settings {
      work_duration: Duration::from_secs(100),
      relax_duration: Duration::from_secs(200),
      long_relax_duration: Duration::from_secs(300),
      toggle_timer_shortcut: Some("Ctrl + A".to_string()),
      should_play_sound: Some(true),
    };

    save_settings(&app_handle, &settings)?;

    assert_eq!(load_settings(&app_handle)?, settings);

    Ok(())
  }
}
