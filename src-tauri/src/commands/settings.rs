use std::time::Duration;

use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle, Manager, State};
use ts_rs::TS;

use crate::{
  helpers::{
    fs::save_settings,
    shortcuts::{register_toggle_shortcut, unregister_toggle_shortcut},
  },
  state::TimerMode,
  PomodoroState, SettingsState, TimerState, TimerStatePayload, MAIN_WINDOW_LABEL,
};

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SettingsPayload {
  work_duration_secs: u32,
  relax_duration_secs: u32,
  long_relax_duration_secs: u32,
  toggle_timer_shortcut: Option<String>,
}

#[command]
pub fn get_settings(settings: State<'_, SettingsState>) -> SettingsPayload {
  let settings = settings.read().unwrap();

  SettingsPayload {
    work_duration_secs: settings.work_duration.as_secs() as u32,
    relax_duration_secs: settings.relax_duration.as_secs() as u32,
    long_relax_duration_secs: settings.long_relax_duration.as_secs() as u32,
    toggle_timer_shortcut: settings.toggle_timer_shortcut.clone(),
  }
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

  settings.work_duration = Duration::from_secs(new_settings.work_duration_secs.into());
  settings.relax_duration = Duration::from_secs(new_settings.relax_duration_secs.into());
  settings.long_relax_duration = Duration::from_secs(new_settings.long_relax_duration_secs.into());
  settings.toggle_timer_shortcut = new_settings.toggle_timer_shortcut.clone();

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

  if let Some(toggle_timer_shortcut) = new_settings.toggle_timer_shortcut {
    register_toggle_shortcut(&app_handle, &toggle_timer_shortcut)
      .map_err(|e| format!("Failed to register shortcut with error {}", e))?;

    if let Some(old_toggle_timer_shortcut) = old_settings.toggle_timer_shortcut {
      unregister_toggle_shortcut(&app_handle, &old_toggle_timer_shortcut)
        .map_err(|e| format!("Failed to unregister old shortcut with error {}", e))?;
    }
  } else if let Some(old_toggle_timer_shortcut) = old_settings.toggle_timer_shortcut {
    unregister_toggle_shortcut(&app_handle, &old_toggle_timer_shortcut)
      .map_err(|e| format!("Failed to unregister old shortcut with error {}", e))?;
  }

  Ok(())
}
