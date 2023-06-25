use tauri::{AppHandle, Manager};

use crate::{
  state::{Pomodoro, TimerMode},
  PomodoroState, SettingsState, TimerState, TimerStatePayload, MAIN_WINDOW_LABEL,
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

pub fn setup_timer_listener(app_handle: &AppHandle) -> impl Fn() {
  let window = app_handle
    .get_window(MAIN_WINDOW_LABEL)
    .expect("Unable to retrieve main window");
  let update_receiver = app_handle.state::<TimerState>().update_receiver.clone();
  let app_handle = app_handle.clone();

  move || loop {
    let remaining = update_receiver
      .recv()
      .expect("Failed to receive timer tick");
    let remaining_secs = remaining.as_secs();

    #[cfg(target_os = "macos")]
    app_handle
      .tray_handle()
      .set_title(
        format!(
          "{minutes:0>2}:{seconds:0>2}",
          minutes = remaining_secs / 60,
          seconds = remaining_secs % 60
        )
        .as_str(),
      )
      .expect("Can't update tray title");

    app_handle.emit_all("timer-tick", &remaining_secs).unwrap();

    if remaining.is_zero() {
      let settings = app_handle.state::<SettingsState>();
      let pomodoro_state = app_handle.state::<PomodoroState>();
      let mut pomodoro_state = pomodoro_state.lock().unwrap();

      *pomodoro_state = update_pomodoro_state(&pomodoro_state);
      let new_duration = pomodoro_state.duration(&settings.read().unwrap());
      app_handle
        .state::<TimerState>()
        .reset(new_duration)
        .unwrap();

      window
        .emit::<TimerStatePayload>(
          "timer-state",
          TimerStatePayload {
            mode: pomodoro_state.mode,
            cycle: pomodoro_state.cycles,
            is_ended: true,
            duration_secs: new_duration.as_secs() as u32,
          },
        )
        .unwrap();
    }
  }
}
