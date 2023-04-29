#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

use commands::settings::{get_settings, set_settings};
use commands::timer::{get_timer_state, toggle_timer};
use serde::Serialize;
use services::fs::load_settings;
use state::{Pomodoro, Settings};
use tauri::{App, Manager};
use tauri_plugin_autostart::MacosLauncher;
use ticking_timer::Timer;
use ts_rs::TS;
use ui::tray::setup_tray;
mod commands;
mod services;
mod state;
mod ui;

use crate::state::TimerMode;
use crate::ui::window::decorate_window;

#[derive(Clone, Serialize, TS)]
#[ts(export)]
pub struct TimerStatePayload {
  mode: TimerMode,
  cycle: u32,
  is_ended: bool,
  duration: u32,
}

pub const MAIN_WINDOW_LABEL: &str = "main";
pub const SETTINGS_WINDOW_LABEL: &str = "settings";

pub type TimerState = Arc<Timer>;
pub type SettingsState = RwLock<Settings>;
pub type PomodoroState = Mutex<Pomodoro>;

fn create_timer_listener(app: &mut App) -> impl Fn() {
  let handle = app.handle();
  let window = handle
    .get_window(MAIN_WINDOW_LABEL)
    .expect("Unable to retrieve main window");
  let update_receiver = handle.state::<TimerState>().update_receiver.clone();

  move || loop {
    let remaining = update_receiver
      .recv()
      .expect("Failed to receive timer tick");
    let remaining_secs = remaining.as_secs();

    #[cfg(not(target_os = "windows"))]
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
      let settings = handle.state::<SettingsState>();
      let pomodoro_state = handle.state::<PomodoroState>();
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

      let timer = handle.state::<TimerState>();
      timer.reset(new_duration).unwrap();
      window
        .emit::<TimerStatePayload>(
          "timer-state",
          TimerStatePayload {
            mode: pomodoro_state.mode,
            cycle: pomodoro_state.cycles,
            is_ended: true,
            duration: new_duration.as_secs() as u32,
          },
        )
        .unwrap();
    }
  }
}

fn main() {
  tauri::Builder::default()
    .manage::<TimerState>(Arc::new(Timer::new(Duration::from_millis(100))))
    .manage::<SettingsState>(RwLock::new(Settings {
      work_duration: Duration::from_secs(60 * 25),
      relax_duration: Duration::from_secs(60 * 5),
      long_relax_duration: Duration::from_secs(60 * 15),
    }))
    .manage::<PomodoroState>(Mutex::new(Pomodoro {
      cycles: 0,
      mode: state::TimerMode::Work,
    }))
    .setup(|app| {
      {
        match load_settings(app.handle()) {
          Ok(settings) => *app.state::<SettingsState>().write().unwrap() = settings,
          Err(error) => {
            eprintln!("Failed to load settings from FS with error {:?}", error);
          }
        }
      }

      let window = app.get_window(MAIN_WINDOW_LABEL).unwrap();
      {
        let duration = app.state::<SettingsState>().read().unwrap().work_duration;
        app.state::<TimerState>().reset(duration).unwrap();
      }

      decorate_window(&window);
      setup_tray(app);

      #[cfg(debug_assertions)]
      window.open_devtools();

      thread::Builder::new()
        .name("Timer listener".into())
        .spawn(create_timer_listener(app))?;

      Ok(())
    })
    .plugin(tauri_plugin_autostart::init(
      MacosLauncher::LaunchAgent,
      None,
    ))
    .invoke_handler(tauri::generate_handler![
      toggle_timer,
      get_timer_state,
      get_settings,
      set_settings
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
