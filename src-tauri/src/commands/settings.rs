use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle, Manager, State};
use ts_rs::TS;

use crate::{
  helpers::{
    fs::save_settings,
    shortcuts::{register_toggle_shortcut, unregister_toggle_shortcut},
  },
  state::{Settings, TimerMode},
  PomodoroState, SettingsState, TimerState, TimerStatePayload, MAIN_WINDOW_LABEL,
};

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SettingsPayload {
  pub work_duration_secs: u32,
  pub relax_duration_secs: u32,
  pub long_relax_duration_secs: u32,
  pub toggle_timer_shortcut: Option<String>,
}

impl From<Settings> for SettingsPayload {
  fn from(settings: Settings) -> Self {
    Self {
      long_relax_duration_secs: settings.long_relax_duration.as_secs() as u32,
      relax_duration_secs: settings.relax_duration.as_secs() as u32,
      work_duration_secs: settings.work_duration.as_secs() as u32,
      toggle_timer_shortcut: settings.toggle_timer_shortcut,
    }
  }
}

#[command]
pub fn get_settings(settings: State<'_, SettingsState>) -> Result<SettingsPayload, String> {
  let settings = settings
    .read()
    .map_err(|e| format!("Failed to read settings, {}", e))?;

  Ok(settings.to_owned().into())
}

#[command]
pub fn set_settings(
  new_settings: SettingsPayload,
  settings: State<'_, SettingsState>,
  timer: State<'_, TimerState>,
  pomodoro_state: State<'_, PomodoroState>,
  app_handle: AppHandle,
) -> Result<(), String> {
  let mut settings = settings.write().unwrap();
  let old_settings = settings.clone();

  *settings = new_settings.into();

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
    .expect("Cannot send event to main window");

  save_settings(&app_handle, &settings)
    .map_err(|e| format!("Failed to write settings with error {}", e))?;

  if let Some(toggle_timer_shortcut) = &settings.toggle_timer_shortcut {
    register_toggle_shortcut(&app_handle, toggle_timer_shortcut)
      .map_err(|e| format!("Failed to register shortcut with error {}", e))?;

    if let Some(old_toggle_timer_shortcut) = &old_settings.toggle_timer_shortcut {
      unregister_toggle_shortcut(&app_handle, old_toggle_timer_shortcut)
        .map_err(|e| format!("Failed to unregister old shortcut with error {}", e))?;
    }
  } else if let Some(old_toggle_timer_shortcut) = &old_settings.toggle_timer_shortcut {
    unregister_toggle_shortcut(&app_handle, old_toggle_timer_shortcut)
      .map_err(|e| format!("Failed to unregister old shortcut with error {}", e))?;
  }

  Ok(())
}
