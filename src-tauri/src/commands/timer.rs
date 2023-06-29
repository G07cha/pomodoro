use tauri::{command, State, Window};

use crate::{
  state::{Pomodoro, TimerMode},
  PomodoroState, SettingsState, TimerState, TimerStatePayload,
};

fn update_pomodoro_state(state: &Pomodoro) -> Pomodoro {
  match state.mode {
    TimerMode::Relax => Pomodoro {
      mode: TimerMode::Work,
      cycles: state.cycles + 1,
    },
    TimerMode::Work => Pomodoro {
      mode: TimerMode::Relax,
      cycles: state.cycles,
    },
  }
}

#[command]
pub fn toggle_timer(window: Window, timer: State<'_, TimerState>) -> Result<(), String> {
  timer
    .toggle()
    .map_err(|_| "Failed to toggle timer".to_string())?;

  window
    .emit("timer-running-change", *timer.is_running())
    .map_err(|_| "Failed to communicate running state".to_string())
}

#[command]
pub fn reset_timer(
  window: Window,
  timer: State<'_, TimerState>,
  pomodoro_state: State<'_, PomodoroState>,
  settings: State<'_, SettingsState>,
) -> Result<(), String> {
  let mut pomodoro_state = pomodoro_state.lock().unwrap();

  *pomodoro_state = update_pomodoro_state(&pomodoro_state);
  let new_duration = pomodoro_state.duration(&settings.read().unwrap());
  timer
    .reset(new_duration)
    .map_err(|_| "Failed to reset timer".to_string())?;
  timer
    .resume()
    .map_err(|_| "Failed to resume timer".to_string())?;

  window
    .emit::<TimerStatePayload>(
      "timer-state",
      TimerStatePayload {
        mode: pomodoro_state.mode,
        cycle: pomodoro_state.cycles,
        is_ended: false,
        duration_secs: new_duration.as_secs() as u32,
      },
    )
    .map_err(|_| "Failed to communicate new state".to_string())?;

  window
    .emit("timer-running-change", *timer.is_running())
    .map_err(|_| "Failed to communicate running state".to_string())
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
