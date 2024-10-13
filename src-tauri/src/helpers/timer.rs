use std::{sync::mpsc::SyncSender, time::Duration};

use tauri::{AppHandle, Emitter, EventTarget, Manager, Runtime};

use crate::{ui::tray::TRAY_ID, PomodoroState, TimerStatePayload, MAIN_WINDOW_LABEL};

pub fn create_timer_listener<R: Runtime>(
  app_handle: &AppHandle<R>,
  timer_end_sender: SyncSender<()>,
) -> impl Fn(Duration) {
  let app_handle = app_handle.clone();
  let main_window = app_handle.get_webview_window(MAIN_WINDOW_LABEL).unwrap();

  move |remaining: Duration| {
    let remaining_secs = remaining.as_secs();

    #[cfg(target_os = "macos")]
    app_handle
      .tray_by_id(TRAY_ID)
      .unwrap()
      .set_title(Some(&format!(
        "{minutes:0>2}:{seconds:0>2}",
        minutes = remaining_secs / 60,
        seconds = remaining_secs % 60
      )))
      .expect("Can't update tray title");

    app_handle
      .emit_to(
        EventTarget::webview_window(MAIN_WINDOW_LABEL),
        "timer-tick",
        &remaining_secs,
      )
      .unwrap();

    if remaining.is_zero() {
      let pomodoro_state = app_handle.state::<PomodoroState>();
      let pomodoro_state = pomodoro_state.lock().unwrap();

      main_window
        .emit::<TimerStatePayload>(
          "timer-state",
          TimerStatePayload {
            mode: pomodoro_state.mode,
            cycle: pomodoro_state.cycles,
            is_ended: true,
            duration_secs: remaining.as_secs() as u32,
          },
        )
        .unwrap();
      timer_end_sender.send(()).unwrap();
    }
  }
}
