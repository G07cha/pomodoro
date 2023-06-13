use std::time::Duration;

use serde::{Deserialize, Serialize};
use tauri::Manager;
use ts_rs::TS;

use crate::{
  services::fs::save_settings, state::TimerMode, PomodoroState, SettingsState, TimerState,
  TimerStatePayload, MAIN_WINDOW_LABEL,
};

#[derive(Serialize, Clone, TS)]
#[ts(export)]
pub struct GetSettingsResponse {
  work_duration_secs: u32,
  relax_duration_secs: u32,
  long_relax_duration_secs: u32,
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct SetSettingsPayload {
  work_duration_secs: u32,
  relax_duration_secs: u32,
  long_relax_duration_secs: u32,
}

#[tauri::command]
pub fn get_settings(settings: tauri::State<'_, SettingsState>) -> GetSettingsResponse {
  let settings = settings.read().unwrap();

  GetSettingsResponse {
    work_duration_secs: settings.work_duration.as_secs() as u32,
    relax_duration_secs: settings.relax_duration.as_secs() as u32,
    long_relax_duration_secs: settings.long_relax_duration.as_secs() as u32,
  }
}

#[tauri::command]
pub fn set_settings(
  new_settings: SetSettingsPayload,
  settings: tauri::State<'_, SettingsState>,
  timer: tauri::State<'_, TimerState>,
  pomodoro_state: tauri::State<'_, PomodoroState>,
  app_handle: tauri::AppHandle,
) {
  let mut settings = settings.write().unwrap();
  settings.work_duration = Duration::from_secs(new_settings.work_duration_secs.into());
  settings.relax_duration = Duration::from_secs(new_settings.relax_duration_secs.into());
  settings.long_relax_duration = Duration::from_secs(new_settings.long_relax_duration_secs.into());
  timer.reset(settings.work_duration).unwrap();

  app_handle
    .get_window(MAIN_WINDOW_LABEL)
    .expect("Cannot find main window")
    .emit(
      "timer-state",
      TimerStatePayload {
        cycle: pomodoro_state.lock().unwrap().cycles,
        duration_secs: settings.work_duration.as_secs() as u32,
        is_ended: false,
        mode: TimerMode::Work,
      },
    )
    .unwrap();

  if let Err(error) = save_settings(app_handle, &settings) {
    eprintln!("Failed to save settings to FS with error {:?}", error);
  }
}
