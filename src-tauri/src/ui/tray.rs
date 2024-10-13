use anyhow::Result;
use tauri::{
  menu::{MenuBuilder, MenuEvent, MenuItemBuilder},
  tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
  App, AppHandle, Manager, Runtime,
};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tauri_plugin_positioner::WindowExt;

use crate::{helpers::updater::update, MAIN_WINDOW_LABEL};

use super::window::{setup_about_window, setup_settings_window};

pub const TRAY_ID: &str = "main";

const SETTINGS_MENU_ITEM_ID: &str = "settings";
const CHECK_UPDATES_MENU_ITEM_ID: &str = "check_updates";
const ABOUT_MENU_ITEM_ID: &str = "about";

fn handle_menu_event<R: Runtime>(app_handle: &AppHandle<R>, event: MenuEvent) {
  match event.id.as_ref() {
    SETTINGS_MENU_ITEM_ID => {
      if let Some(settings_window) = app_handle.get_webview_window(crate::SETTINGS_WINDOW_LABEL) {
        settings_window.show().unwrap();
      } else {
        setup_settings_window(app_handle).unwrap();
      }
    }
    ABOUT_MENU_ITEM_ID => {
      if let Some(about_window) = app_handle.get_webview_window(crate::ABOUT_WINDOW_LABEL) {
        about_window.show().unwrap();
      } else {
        std::thread::scope(|s| {
          s.spawn(|| {
            setup_about_window(app_handle).unwrap();
          });
        });
      }
    }
    CHECK_UPDATES_MENU_ITEM_ID => {
      let handle = app_handle.clone();
      tauri::async_runtime::spawn(async move {
        match update(&handle).await {
          Ok(updated) => {
            if updated {
              handle.restart()
            } else {
              handle
                .dialog()
                .message("You are already using the latest version of Pomodoro!")
                .kind(MessageDialogKind::Info)
                .title("The app is up-to-date");
            }
          }
          Err(_) => {
            handle
              .dialog()
              .message("You are already using the latest version of Pomodoro!")
              .kind(MessageDialogKind::Error)
              .title("Failed to retrieve update");
          }
        }
      });
    }
    id => eprintln!("Unsupported menu item clicked {:?}", id),
  }
}

fn handle_tray_icon_event<R: Runtime>(tray: &TrayIcon<R>, event: TrayIconEvent) {
  let app = tray.app_handle();
  let main_window = app.get_webview_window(MAIN_WINDOW_LABEL).unwrap();
  tauri_plugin_positioner::on_tray_event(app, &event);

  if let TrayIconEvent::Click {
    button: MouseButton::Left,
    button_state: MouseButtonState::Up,
    ..
  } = event
  {
    if main_window.is_visible().unwrap() {
      main_window.hide().unwrap();
    } else {
      main_window
        .move_window(tauri_plugin_positioner::Position::TrayCenter)
        .unwrap();
      main_window.show().unwrap();
      main_window.set_focus().unwrap();
    }
  }
}

pub fn setup_tray<R: Runtime>(app: &mut App<R>) -> Result<()> {
  let settings_menu_item = MenuItemBuilder::new("Settings")
    .id(SETTINGS_MENU_ITEM_ID)
    .build(app)?;
  let check_updates_menu_item = MenuItemBuilder::new("Check for updates")
    .id(CHECK_UPDATES_MENU_ITEM_ID)
    .build(app)?;
  let about_menu_item = MenuItemBuilder::new("About")
    .id(ABOUT_MENU_ITEM_ID)
    .build(app)?;

  let menu = MenuBuilder::new(app)
    .items(&[
      &settings_menu_item,
      &check_updates_menu_item,
      &about_menu_item,
    ])
    .separator()
    .quit()
    .build()?;

  let tray = TrayIconBuilder::with_id(TRAY_ID)
    .menu(&menu)
    .icon(app.default_window_icon().unwrap().clone());

  #[cfg(target_os = "macos")]
  let tray = tray.menu_on_left_click(false).icon_as_template(true);

  tray
    .on_menu_event(handle_menu_event)
    .on_tray_icon_event(handle_tray_icon_event)
    .build(app)?;

  Ok(())
}
