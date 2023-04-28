use crate::{state::TimerMode, PomodoroState, SettingsState, TimerState, TimerStatePayload};

#[tauri::command]
pub fn toggle_timer(window: tauri::Window, timer: tauri::State<'_, TimerState>) {
  timer.toggle();

  window
    .emit("timer-running-change", timer.is_running())
    .unwrap()
}

#[tauri::command]
pub fn get_timer_state(
  settings: tauri::State<'_, SettingsState>,
  pomodoro_state: tauri::State<'_, PomodoroState>,
) -> TimerStatePayload {
  let settings = settings.read().unwrap();
  let pomodoro_state = pomodoro_state.lock().unwrap();

  let duration = match pomodoro_state.mode {
    TimerMode::Work => settings.work_duration,
    TimerMode::Relax => {
      if pomodoro_state.cycles % 4 == 0 && pomodoro_state.cycles > 0 {
        settings.long_relax_duration
      } else {
        settings.relax_duration
      }
    }
  };

  TimerStatePayload {
    mode: pomodoro_state.mode,
    cycle: pomodoro_state.cycles,
    is_ended: false,
    duration: duration.as_secs() as u32,
  }
}
