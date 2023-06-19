use tauri::{AppHandle, Manager};

use crate::{
  state::TimerMode, PomodoroState, SettingsState, TimerState, TimerStatePayload, MAIN_WINDOW_LABEL,
};

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

      let new_duration = match pomodoro_state.mode {
        TimerMode::Relax => {
          pomodoro_state.mode = TimerMode::Work;
          pomodoro_state.cycles += 1;

          settings.read().unwrap().work_duration
        }
        TimerMode::Work => {
          pomodoro_state.mode = TimerMode::Relax;

          if pomodoro_state.cycles % 4 == 0 && pomodoro_state.cycles > 0 {
            settings.read().unwrap().long_relax_duration
          } else {
            settings.read().unwrap().relax_duration
          }
        }
      };

      let timer = app_handle.state::<TimerState>();
      timer.reset(new_duration).unwrap();
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
