use tauri::{command, State, Window};

use crate::{PomodoroState, SettingsState, TimerState, TimerStatePayload};

#[command]
pub fn toggle_timer(window: Window, timer: State<'_, TimerState>) {
  timer.toggle();

  window
    .emit("timer-running-change", timer.is_running())
    .unwrap()
}

#[command]
pub fn get_timer_state(
  settings: State<'_, SettingsState>,
  pomodoro_state: State<'_, PomodoroState>,
) -> TimerStatePayload {
  let settings = settings.read().unwrap();
  let pomodoro_state = pomodoro_state.lock().unwrap();

  TimerStatePayload {
    mode: pomodoro_state.mode,
    cycle: pomodoro_state.cycles,
    is_ended: false,
    duration_secs: pomodoro_state.duration(&settings).as_secs() as u32,
  }
}
