use anyhow::Result;
use tauri::{
  menu::{MenuBuilder, MenuEvent, MenuItemBuilder},
  tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
  App, AppHandle, Manager, Runtime,
};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

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

#[cfg(target_os = "macos")]
fn position_menubar_panel<R: Runtime>(app_handle: &tauri::AppHandle<R>, padding_top: f64) {
  use tauri_nspanel::{
    cocoa::{
      base::id,
      foundation::{NSPoint, NSRect},
    },
    objc::{class, msg_send, runtime::NO, sel, sel_impl},
  };

  let window = app_handle.get_webview_window(MAIN_WINDOW_LABEL).unwrap();
  let monitor = monitor::get_monitor_with_cursor().unwrap();

  let scale_factor = monitor.scale_factor();

  let visible_area = monitor.visible_area();

  let monitor_pos = visible_area.position().to_logical::<f64>(scale_factor);

  let monitor_size = visible_area.size().to_logical::<f64>(scale_factor);

  let mouse_location: NSPoint = unsafe { msg_send![class!(NSEvent), mouseLocation] };

  let handle: id = window.ns_window().unwrap() as _;

  let mut win_frame: NSRect = unsafe { msg_send![handle, frame] };

  win_frame.origin.y = (monitor_pos.y + monitor_size.height) - win_frame.size.height;

  win_frame.origin.y -= padding_top;

  win_frame.origin.x = {
    let top_right = mouse_location.x + (win_frame.size.width / 2.0);

    let is_offscreen = top_right > monitor_pos.x + monitor_size.width;

    if !is_offscreen {
      mouse_location.x - (win_frame.size.width / 2.0)
    } else {
      let diff = top_right - (monitor_pos.x + monitor_size.width);

      mouse_location.x - (win_frame.size.width / 2.0) - diff
    }
  };

  let _: () = unsafe { msg_send![handle, setFrame: win_frame display: NO] };
}

fn handle_tray_icon_event<R: Runtime>(tray: &TrayIcon<R>, event: TrayIconEvent) {
  let app = tray.app_handle();
  tauri_plugin_positioner::on_tray_event(app, &event);

  if let TrayIconEvent::Click {
    button: MouseButton::Left,
    button_state: MouseButtonState::Up,
    ..
  } = event
  {
    #[cfg(target_os = "macos")]
    {
      use tauri_nspanel::ManagerExt;

      let main_panel = app.get_webview_panel(MAIN_WINDOW_LABEL).unwrap();
      if main_panel.is_visible() {
        main_panel.order_out(None);
      } else {
        position_menubar_panel(app, 0.0);
        main_panel.show();
      }
    }

    #[cfg(not(target_os = "macos"))]
    {
      use tauri_plugin_positioner::WindowExt;
      let main_window = app.get_webview_window(MAIN_WINDOW_LABEL).unwrap();
      if main_window.is_visible().unwrap() {
        main_window.hide().unwrap();
      } else {
        main_window
          .as_ref()
          .window()
          .move_window(tauri_plugin_positioner::Position::TrayCenter)
          .unwrap();
        main_window.show().unwrap();
      }
    }
  }
}

pub fn setup_tray<R: Runtime>(app: &mut App<R>) -> Result<()> {
  let menu = MenuBuilder::new(app)
    .items(&[
      &MenuItemBuilder::new("Settings")
        .id(SETTINGS_MENU_ITEM_ID)
        .build(app)?,
      &MenuItemBuilder::new("Check for updates")
        .id(CHECK_UPDATES_MENU_ITEM_ID)
        .build(app)?,
      &MenuItemBuilder::new("About")
        .id(ABOUT_MENU_ITEM_ID)
        .build(app)?,
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
