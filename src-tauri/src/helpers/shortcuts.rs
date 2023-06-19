use anyhow::Result;
use tauri::{AppHandle, GlobalShortcutManager, Manager, Runtime};

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
  if !app_handle
    .global_shortcut_manager()
    .is_registered(shortcut)?
  {
    app_handle.global_shortcut_manager().register(shortcut, {
      let app_handle = app_handle.clone();
      move || {
        let timer = app_handle.state::<TimerState>();
        timer.toggle();
        app_handle
          .get_window(MAIN_WINDOW_LABEL)
          .expect("Failed to find main window")
          .emit("timer-running-change", timer.is_running())
          .unwrap();
      }
    })?
  }

  Ok(())
}

pub fn unregister_toggle_shortcut<R: Runtime>(
  app_handle: &AppHandle<R>,
  shortcut: &str,
) -> Result<()> {
  if app_handle
    .global_shortcut_manager()
    .is_registered(shortcut)?
  {
    app_handle.global_shortcut_manager().unregister(shortcut)?
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use std::sync::RwLock;

  use crate::state::Settings;

  use super::*;
  use tauri::test;
  use tauri::Manager;

  const ACCELERATOR: &str = "Ctrl + C";

  #[test]
  fn sets_up_shortcuts_from_state() -> Result<()> {
    let app_handle = test::mock_app().app_handle();

    app_handle.manage::<SettingsState>(RwLock::new(Settings {
      toggle_timer_shortcut: Some(ACCELERATOR.to_owned()),
      ..Default::default()
    }));

    setup_shortcuts(&app_handle);

    assert!(app_handle
      .global_shortcut_manager()
      .is_registered(ACCELERATOR)?);

    Ok(())
  }

  #[test]
  fn sets_up_without_shortcuts() -> Result<()> {
    let app_handle = test::mock_app().app_handle();

    app_handle.manage::<SettingsState>(RwLock::new(Settings::default()));

    setup_shortcuts(&app_handle);

    Ok(())
  }

  #[test]
  fn registers_toggle_shortcut() -> Result<()> {
    let app_handle = test::mock_app().app_handle();

    register_toggle_shortcut(&app_handle, ACCELERATOR)?;

    assert!(app_handle
      .global_shortcut_manager()
      .is_registered(ACCELERATOR)?);

    Ok(())
  }

  #[test]
  fn noop_if_shortcut_registered() -> Result<()> {
    let app_handle = test::mock_app().app_handle();

    register_toggle_shortcut(&app_handle, ACCELERATOR)?;
    register_toggle_shortcut(&app_handle, ACCELERATOR)?;

    assert!(app_handle
      .global_shortcut_manager()
      .is_registered(ACCELERATOR)?);

    Ok(())
  }

  #[test]
  fn unregisters_shortcut() -> Result<()> {
    let app_handle = test::mock_app().app_handle();

    app_handle
      .global_shortcut_manager()
      .register(ACCELERATOR, || {})?;

    unregister_toggle_shortcut(&app_handle, ACCELERATOR)?;

    assert!(!app_handle
      .global_shortcut_manager()
      .is_registered(ACCELERATOR)?);

    Ok(())
  }

  #[test]
  fn noop_if_shortcut_unregistered() -> Result<()> {
    let app_handle = test::mock_app().app_handle();

    app_handle
      .global_shortcut_manager()
      .register(ACCELERATOR, || {})?;

    unregister_toggle_shortcut(&app_handle, ACCELERATOR)?;
    unregister_toggle_shortcut(&app_handle, ACCELERATOR)?;

    assert!(!app_handle
      .global_shortcut_manager()
      .is_registered(ACCELERATOR)?);

    Ok(())
  }
}
