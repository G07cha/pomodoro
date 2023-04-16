use serde::Serialize;
use ts_rs::TS;

use crate::state::{AppState, TimerMode};

#[derive(Serialize, TS)]
#[ts(export)]
pub struct GetSettingsResponse {
  cycles: u32,
  work_duration: u32,
  relax_duration: u32,
  long_relax_duration: u32,
  mode: TimerMode,
}

#[tauri::command]
pub fn get_settings(state: tauri::State<'_, AppState>) -> GetSettingsResponse {
  let settings = state.settings.lock().unwrap();

  GetSettingsResponse {
    cycles: settings.cycles,
    work_duration: settings.work_duration.as_secs() as u32,
    relax_duration: settings.relax_duration.as_secs() as u32,
    long_relax_duration: settings.long_relax_duration.as_secs() as u32,
    mode: settings.mode,
  }
}
