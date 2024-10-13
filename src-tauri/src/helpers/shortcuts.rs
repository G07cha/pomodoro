use anyhow::Result;
use tauri::{AppHandle, Emitter, Manager, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::{SettingsState, TimerState, MAIN_WINDOW_LABEL};

pub fn setup_shortcuts<R: Runtime>(app_handle: &AppHandle<R>) {
  let settings_state = app_handle.state::<SettingsState>();
  let settings_state = settings_state.read().unwrap();

  if let Some(toggle_timer_shortcut) = settings_state.toggle_timer_shortcut.clone() {
    if let Err(error) = register_toggle_shortcut(app_handle, &toggle_timer_shortcut) {
      eprintln!(
        "Failed to register shortcut {} with error {}",
        toggle_timer_shortcut, error
      );
    }
  }
}

pub fn register_toggle_shortcut<R: Runtime>(
  app_handle: &AppHandle<R>,
  shortcut: &str,
) -> Result<()> {
  if app_handle.global_shortcut().is_registered(shortcut) {
    return Ok(());
  }

  app_handle
    .global_shortcut()
    .on_shortcut(shortcut, |app_handle, _, event| {
      if event.state == ShortcutState::Pressed {
        let timer = app_handle.state::<TimerState>();
        timer.toggle().expect("Failed to toggle timer");
        app_handle
          .get_webview_window(MAIN_WINDOW_LABEL)
          .expect("Failed to find main window")
          .emit("timer-running-change", *timer.is_running())
          .unwrap();
      }
    })?;

  Ok(())
}

pub fn unregister_toggle_shortcut<R: Runtime>(
  app_handle: &AppHandle<R>,
  shortcut: &str,
) -> Result<()> {
  if app_handle.global_shortcut().is_registered(shortcut) {
    app_handle.global_shortcut().unregister(shortcut)?
  }

  Ok(())
}

// Currently broken after upgrading to Tauri 2.0
//
// #[cfg(test)]
// mod tests {
//   use std::sync::RwLock;

//   use crate::state::Settings;

//   use super::*;
//   use tauri::Manager;

//   const ACCELERATOR: &str = "Ctrl + C";

//   fn test_app() -> tauri::Result<tauri::App<impl tauri::Runtime>> {
//     tauri::test::mock_builder()
//       .plugin(tauri_plugin_global_shortcut::Builder::default().build())
//       .build(tauri::generate_context!())
//   }

//   #[test]
//   fn sets_up_shortcuts_from_state() -> Result<()> {
//     let app = test_app()?;
//     let app_handle = app.app_handle();

//     app_handle.manage::<SettingsState>(RwLock::new(Settings {
//       toggle_timer_shortcut: Some(ACCELERATOR.to_owned()),
//       ..Default::default()
//     }));

//     setup_shortcuts(app_handle);

//     assert!(app_handle.global_shortcut().is_registered(ACCELERATOR));

//     Ok(())
//   }

//   #[test]
//   fn sets_up_without_shortcuts() -> Result<()> {
//     let app = test_app()?;
//     let app_handle = app.app_handle();

//     app_handle.manage::<SettingsState>(RwLock::new(Settings::default()));

//     setup_shortcuts(app_handle);

//     Ok(())
//   }

//   #[test]
//   fn registers_toggle_shortcut() -> Result<()> {
//     let app = test_app()?;
//     let app_handle = app.app_handle();

//     register_toggle_shortcut(app_handle, ACCELERATOR)?;

//     assert!(app_handle.global_shortcut().is_registered(ACCELERATOR));

//     Ok(())
//   }

//   #[test]
//   fn noop_if_shortcut_registered() -> Result<()> {
//     let app = test_app()?;
//     let app_handle = app.app_handle();

//     register_toggle_shortcut(app_handle, ACCELERATOR)?;
//     register_toggle_shortcut(app_handle, ACCELERATOR)?;

//     assert!(app_handle.global_shortcut().is_registered(ACCELERATOR));

//     Ok(())
//   }

//   #[test]
//   fn unregisters_shortcut() -> Result<()> {
//     let app = test_app()?;
//     let app_handle = app.app_handle();

//     app_handle.global_shortcut().register(ACCELERATOR)?;

//     unregister_toggle_shortcut(app_handle, ACCELERATOR)?;

//     assert!(!app_handle.global_shortcut().is_registered(ACCELERATOR));

//     Ok(())
//   }

//   #[test]
//   fn noop_if_shortcut_unregistered() -> Result<()> {
//     let app = test_app()?;
//     let app_handle = app.app_handle();

//     app_handle.global_shortcut().register(ACCELERATOR)?;

//     unregister_toggle_shortcut(app_handle, ACCELERATOR)?;
//     unregister_toggle_shortcut(app_handle, ACCELERATOR)?;

//     assert!(!app_handle.global_shortcut().is_registered(ACCELERATOR));

//     Ok(())
//   }
// }
