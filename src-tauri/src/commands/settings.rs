use crate::state::{AppState, SettingsState};

#[tauri::command]
pub fn get_settings(state: tauri::State<'_, AppState>) -> SettingsState {
  let settings = state.settings.lock().unwrap();

  SettingsState {
    cycles: settings.cycles,
    work_duration: settings.work_duration,
    relax_duration: settings.relax_duration,
    long_relax_duration: settings.long_relax_duration,
    mode: settings.mode,
  }
}
