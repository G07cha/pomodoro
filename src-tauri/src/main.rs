#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::mpsc;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

use anyhow::Result;
use commands::settings::*;
use commands::timer::*;
use helpers::fs::load_settings;
use helpers::shortcuts::setup_shortcuts;
use helpers::sound::SoundPlayer;
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

fn create_audio_notification_thread<R: tauri::Runtime>(
  app_handle: &tauri::AppHandle<R>,
  timer_end_receiver: mpsc::Receiver<()>,
) -> Result<()> {
  let bell_audio_path = app_handle
    .path()
    .resolve("audio/bell.mp3", tauri::path::BaseDirectory::Resource)?
    .to_string_lossy()
    .into_owned();

  thread::spawn({
    let app_handle = app_handle.clone();
    move || match SoundPlayer::new() {
      Ok(sound_player) => {
        for _ in timer_end_receiver {
          let settings_state = app_handle.state::<SettingsState>();
          let settings_state = settings_state.read().unwrap();
          if settings_state.should_play_sound == Some(true) {
            sound_player.play(&bell_audio_path).unwrap();
          }
        }
      }
      Err(error) => eprintln!("Unable to initialize sound player due to error {:?}", error),
    }
  });

  Ok(())
}

fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
  builder
    .manage::<SettingsState>(RwLock::new(Settings::default()))
    .manage::<PomodoroState>(Mutex::new(Pomodoro {
      cycles: 0,
      mode: state::TimerMode::Work,
    }))
    .setup(|app| {
      let app_handle = app.handle();
      let main_window = setup_main_window(app_handle).unwrap();

      #[cfg(debug_assertions)]
      main_window
        .get_webview_window(MAIN_WINDOW_LABEL)
        .expect("failed to get webview window")
        .open_devtools();

      #[cfg(target_os = "macos")]
      app_handle
        .set_activation_policy(tauri::ActivationPolicy::Accessory)
        .unwrap();

      let (timer_end_sender, timer_end_receiver) = mpsc::sync_channel(1);

      let timer = Timer::new(
        Duration::from_millis(100),
        create_timer_listener(app_handle, timer_end_sender),
      );

      create_audio_notification_thread(app_handle, timer_end_receiver).unwrap();

      app_handle.manage::<TimerState>(Arc::new(timer));

      {
        let mut work_duration = app.state::<SettingsState>().read().unwrap().work_duration;
        match load_settings(app_handle) {
          Ok(settings) => {
            work_duration = settings.work_duration;
            *app.state::<SettingsState>().write().unwrap() = settings;
          }
          Err(error) => {
            eprintln!("Failed to load settings with error {:?}", error);
          }
        }
        app.state::<TimerState>().reset(work_duration).unwrap();
      }

      setup_tray(app).unwrap();

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      toggle_timer,
      reset_timer,
      next_timer_cycle,
      get_timer_state,
      get_settings,
      set_settings
    ])
    .build(tauri::generate_context!())
    .expect("Error while building tauri application")
}

fn main() {
  let builder = tauri::Builder::default()
    .plugin(tauri_plugin_autostart::init(
      MacosLauncher::LaunchAgent,
      None,
    ))
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_global_shortcut::Builder::default().build())
    .plugin(tauri_plugin_positioner::init())
    .plugin(tauri_plugin_updater::Builder::new().build());

  #[cfg(target_os = "macos")]
  let builder = builder.plugin(tauri_nspanel::init());

  let app = create_app(builder);

  app.run(move |app_handle, e| {
    if matches!(e, RunEvent::Ready) {
      setup_shortcuts(app_handle);
    }
  });
}

#[cfg(test)]
mod tests {
  use super::*;
  use tauri::{Manager, WebviewWindowBuilder};

  #[test]
  fn it_creates_main_window() -> Result<()> {
    let app = create_app(tauri::test::mock_builder());
    WebviewWindowBuilder::new(&app, MAIN_WINDOW_LABEL, Default::default()).build()?;

    assert!(app.get_webview_window(MAIN_WINDOW_LABEL).is_some());

    Ok(())
  }
}
