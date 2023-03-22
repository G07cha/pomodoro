#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::Mutex;
use std::time::Duration;

use commands::settings::get_settings;
use commands::timer::toggle_timer;
use state::{AppState, SettingsState};
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;
use ui::tray::setup_tray;
mod commands;
mod state;
mod ui;
mod utils;

use crate::state::TimerMode;
use crate::ui::window::decorate_window;
use crate::utils::timer::{Timer, TimerEvent, TimerSettings};

#[derive(Clone, serde::Serialize)]
struct TimerStatePayload {
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
  let timer = Timer::new(TimerSettings {
    duration: settings.work_duration,
    update_frequency: Duration::from_millis(200),
  });

  tauri::Builder::default()
    .manage(AppState {
      timer: Mutex::new(timer),
      settings: Mutex::new(settings),
    })
    .setup(|app| {
      let window = app.get_window(WINDOW_LABEL).unwrap();

      decorate_window(&window);
      setup_tray(app, WINDOW_LABEL);

      #[cfg(debug_assertions)]
      window.open_devtools();

      let handle = app.handle();
      app
        .state::<AppState>()
        .timer
        .lock()
        .unwrap()
        .on_event(move |event| match event {
          TimerEvent::Tick { remaining } => {
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
              .unwrap();

            handle.emit_all("timer-tick", &remaining_secs).unwrap();
          }
          TimerEvent::TimeEnd {} => {
            let state = handle.state::<AppState>();
            let mut settings = state.settings.lock().unwrap();
            let mut timer = state.timer.lock().unwrap();

            if matches!(settings.mode, TimerMode::Work) {
              settings.mode = TimerMode::Relax;
              if settings.cycles % 4 == 0 && settings.cycles > 0 {
                timer.reset(Some(settings.long_relax_duration));
              } else {
                timer.reset(Some(settings.relax_duration));
              }
            } else {
              settings.mode = TimerMode::Work;
              settings.cycles += 1;
              timer.reset(Some(settings.work_duration));
            }

            window
              .emit::<TimerStatePayload>(
                "timer-end",
                TimerStatePayload {
                  mode: settings.mode,
                  cycle: settings.cycles,
                  is_running: timer.is_running(),
                },
              )
              .unwrap();
          }
        });

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
