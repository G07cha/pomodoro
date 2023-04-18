#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use commands::settings::get_settings;
use commands::timer::toggle_timer;
use serde::Serialize;
use state::{AppState, SettingsState};
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;
use ticking_timer::Timer;
use ts_rs::TS;
use ui::tray::setup_tray;
mod commands;
mod state;
mod ui;

use crate::state::TimerMode;
use crate::ui::window::decorate_window;

#[derive(Clone, Serialize, TS)]
#[ts(export)]
struct TimerEndPayload {
  mode: TimerMode,
  cycle: u32,
  is_running: bool,
}

fn main() {
  const WINDOW_LABEL: &str = "main";
  let settings = SettingsState {
    work_duration: Duration::from_secs(60 * 25),
    relax_duration: Duration::from_secs(60 * 5),
    long_relax_duration: Duration::from_secs(60 * 15),
    // For testing purposes
    // work_duration: Duration::from_secs(10),
    // relax_duration: Duration::from_secs(5),
    // long_relax_duration: Duration::from_secs(7),
    cycles: 0,
    mode: state::TimerMode::Work,
  };

  let timer = Timer::new(Duration::from_millis(100));
  let update_receiver = timer.update_receiver.clone();
  timer.reset(settings.work_duration).unwrap();

  tauri::Builder::default()
    .manage(AppState {
      settings: Mutex::new(settings),
      timer: Arc::new(timer),
    })
    .setup(|app| {
      let window = app.get_window(WINDOW_LABEL).unwrap();

      decorate_window(&window);
      setup_tray(app, WINDOW_LABEL);

      #[cfg(debug_assertions)]
      window.open_devtools();

      thread::Builder::new()
        .name("Timer listener".into())
        .spawn({
          let handle = app.handle();
          move || loop {
            let remaining = update_receiver.recv().unwrap();
            let remaining_secs = remaining.as_secs();

            handle
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

            handle.emit_all("timer-tick", &remaining_secs).unwrap();

            if remaining.is_zero() {
              let state = handle.state::<AppState>();
              let mut settings = state.settings.lock().unwrap();
              let timer = &state.timer;

              match settings.mode {
                TimerMode::Relax => {
                  settings.mode = TimerMode::Work;
                  settings.cycles += 1;
                  timer.reset(settings.work_duration).unwrap();
                }
                TimerMode::Work => {
                  settings.mode = TimerMode::Relax;
                  let new_duration = if settings.cycles % 4 == 0 {
                    settings.long_relax_duration
                  } else {
                    settings.relax_duration
                  };
                  timer.reset(new_duration).unwrap();
                }
              }

              window
                .emit::<TimerEndPayload>(
                  "timer-end",
                  TimerEndPayload {
                    mode: settings.mode,
                    cycle: settings.cycles,
                    is_running: state.timer.is_running(),
                  },
                )
                .unwrap();
            }
          }
        })?;

      Ok(())
    })
    .plugin(tauri_plugin_autostart::init(
      MacosLauncher::LaunchAgent,
      None,
    ))
    .invoke_handler(tauri::generate_handler![toggle_timer, get_settings])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
