#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

use commands::settings::{get_settings, set_settings};
use commands::timer::{get_timer_state, toggle_timer};
use helpers::fs::load_settings;
use helpers::shortcuts::setup_shortcuts;
use helpers::timer::setup_timer_listener;
use serde::Serialize;
use state::{Pomodoro, Settings};
use tauri::{Manager, RunEvent};
use tauri_plugin_autostart::MacosLauncher;
use ticking_timer::Timer;
use ts_rs::TS;
use ui::tray::setup_tray;
use ui::window::setup_main_window;
mod commands;
mod helpers;
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
  duration_secs: u32,
}

pub const MAIN_WINDOW_LABEL: &str = "main";
pub const SETTINGS_WINDOW_LABEL: &str = "settings";
pub const ABOUT_WINDOW_LABEL: &str = "about";

pub type TimerState = Arc<Timer>;
pub type SettingsState = RwLock<Settings>;
pub type PomodoroState = Mutex<Pomodoro>;

fn main() {
  let app = tauri::Builder::default()
    .menu(tauri::Menu::new())
    .manage::<TimerState>(Arc::new(Timer::new(Duration::from_millis(100))))
    .manage::<SettingsState>(RwLock::new(Settings {
      work_duration: Duration::from_secs(60 * 25),
      relax_duration: Duration::from_secs(60 * 5),
      long_relax_duration: Duration::from_secs(60 * 15),
      toggle_timer_shortcut: None,
    }))
    .manage::<PomodoroState>(Mutex::new(Pomodoro {
      cycles: 0,
      mode: state::TimerMode::Work,
    }))
    .setup(|app| {
      let app_handle = app.handle();

      {
        match load_settings(&app_handle) {
          Ok(settings) => {
            let work_duration = settings.work_duration;
            *app.state::<SettingsState>().write().unwrap() = settings;
            app.state::<TimerState>().reset(work_duration).unwrap();
          }
          Err(error) => {
            eprintln!("Failed to load settings with error {:?}", error);
          }
        }
      }

      {
        let main_window = setup_main_window(&app_handle).unwrap();
        decorate_window(&main_window);

        #[cfg(debug_assertions)]
        main_window.open_devtools();
      }

      setup_tray(app);

      thread::Builder::new()
        .name("Timer listener".into())
        .spawn(setup_timer_listener(&app_handle))?;

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
    .build(tauri::generate_context!())
    .expect("Error while building tauri application");

  app.run(move |app_handle, e| {
    if matches!(e, RunEvent::Ready) {
      setup_shortcuts(app_handle);
    }
  })
}
