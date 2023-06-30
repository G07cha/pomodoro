#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

use commands::settings::*;
use commands::timer::*;
use helpers::fs::load_settings;
use helpers::shortcuts::setup_shortcuts;
use helpers::timer::create_timer_listener;
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

fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
  builder
    .menu(tauri::Menu::new())
    .manage::<SettingsState>(RwLock::new(Settings::default()))
    .manage::<PomodoroState>(Mutex::new(Pomodoro {
      cycles: 0,
      mode: state::TimerMode::Work,
    }))
    .setup(|app| {
      let app_handle = app.handle();
      let main_window = setup_main_window(&app_handle).unwrap();

      #[cfg(not(test))]
      crate::ui::window::decorate_window(&main_window);

      #[cfg(debug_assertions)]
      main_window.open_devtools();

      let timer = Timer::new(
        Duration::from_millis(100),
        create_timer_listener(&app_handle),
      );

      app_handle.manage::<TimerState>(Arc::new(timer));

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

      setup_tray(app);

      Ok(())
    })
    .plugin(tauri_plugin_autostart::init(
      MacosLauncher::LaunchAgent,
      None,
    ))
    .invoke_handler(tauri::generate_handler![
      toggle_timer,
      reset_timer,
      get_timer_state,
      get_settings,
      set_settings
    ])
    .build(tauri::generate_context!())
    .expect("Error while building tauri application")
}

fn main() {
  let app = create_app(tauri::Builder::default());
  app.run(move |app_handle, e| {
    if matches!(e, RunEvent::Ready) {
      setup_shortcuts(app_handle);
    }
  });
}

#[cfg(test)]
mod tests {
  use super::*;
  use tauri::Manager;

  #[test]
  fn it_creates_main_window() {
    let app = create_app(tauri::test::mock_builder());
    assert!(app.get_window(MAIN_WINDOW_LABEL).is_some());
  }
}
