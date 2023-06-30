use tauri::{
  api::dialog, App, AppHandle, CustomMenuItem, Manager, PhysicalPosition, Position, Runtime,
  SystemTray, SystemTrayEvent, SystemTrayMenu, WindowEvent,
};

use crate::MAIN_WINDOW_LABEL;

use super::window::{setup_about_window, setup_settings_window};

const QUIT_MENU_ITEM_ID: &str = "quit";
const SETTINGS_MENU_ITEM_ID: &str = "settings";
const CHECK_UPDATES_MENU_ITEM_ID: &str = "check_updates";
const ABOUT_MENU_ITEM_ID: &str = "about";

fn create_window_event_handler<R: Runtime>(app_handle: AppHandle<R>) -> impl Fn(SystemTrayEvent) {
  let main_window = app_handle.get_window(MAIN_WINDOW_LABEL).unwrap();

  move |event| match event {
    SystemTrayEvent::LeftClick { position, size, .. } => {
      if main_window.is_visible().unwrap() {
        main_window.hide().unwrap();
      } else {
        let tray_size = size.width as i32;
        let window_size = main_window.outer_size().unwrap();
        let window_width = window_size.width as i32;
        let window_height = window_size.height as i32;

        let tray_icon_x = position.x as i32;
        let tray_y = position.y as i32;

        let window_position = Position::Physical(PhysicalPosition {
          x: (tray_icon_x + (tray_size / 2)) - (window_width / 2),
          y: tray_y - window_height,
        });

        main_window.set_position(window_position).unwrap();
        main_window.show().unwrap();
        main_window.set_focus().unwrap();
      }
    }
    SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
      QUIT_MENU_ITEM_ID => app_handle.exit(0),
      SETTINGS_MENU_ITEM_ID => {
        if let Some(settings_window) = app_handle.get_window(crate::SETTINGS_WINDOW_LABEL) {
          settings_window.show().unwrap();
        } else {
          std::thread::scope(|s| {
            s.spawn(|| {
              setup_settings_window(&app_handle).unwrap();
            });
          });
        }
      }
      ABOUT_MENU_ITEM_ID => {
        if let Some(about_window) = app_handle.get_window(crate::ABOUT_WINDOW_LABEL) {
          about_window.show().unwrap();
        } else {
          std::thread::scope(|s| {
            s.spawn(|| {
              setup_about_window(&app_handle).unwrap();
            });
          });
        }
      }
      CHECK_UPDATES_MENU_ITEM_ID => {
        let handle = app_handle.clone();
        tauri::async_runtime::spawn(async move {
          let another_handle = handle.clone();
          match tauri::updater::builder(handle).check().await {
            Ok(update) => {
              if update.is_update_available() {
                update.download_and_install().await.unwrap();
              } else {
                dialog::message(
                  another_handle.get_window(MAIN_WINDOW_LABEL).as_ref(),
                  "The app is up-to-date",
                  "You are already using the latest version of Pomodoro!",
                );
              }
            }
            Err(error) => {
              dialog::message(
                another_handle.get_window(MAIN_WINDOW_LABEL).as_ref(),
                "Failed to retrieve update",
                error.to_string(),
              );
            }
          }
        });
      }
      id => eprintln!("Unsupported menu item clicked {:?}", id),
    },
    _ => {}
  }
}

pub fn setup_tray<R: Runtime>(app: &mut App<R>) {
  let settings_menu_item = CustomMenuItem::new(SETTINGS_MENU_ITEM_ID, "Settings");
  let check_updates_menu_item =
    CustomMenuItem::new(CHECK_UPDATES_MENU_ITEM_ID, "Check for updates");
  let about_menu_item = CustomMenuItem::new(ABOUT_MENU_ITEM_ID, "About");
  let quit_menu_item = CustomMenuItem::new(QUIT_MENU_ITEM_ID, "Quit");
  let system_tray = SystemTray::new().with_menu(
    SystemTrayMenu::new()
      .add_item(settings_menu_item)
      .add_item(check_updates_menu_item)
      .add_item(about_menu_item)
      .add_native_item(tauri::SystemTrayMenuItem::Separator)
      .add_item(quit_menu_item),
  );

  #[cfg(target_os = "macos")]
  let system_tray = system_tray.with_menu_on_left_click(false);

  let main_window = app.get_window(MAIN_WINDOW_LABEL).unwrap();

  main_window.on_window_event({
    let main_window = main_window.clone();
    move |event| {
      if matches!(event, WindowEvent::Focused(false)) {
        main_window.hide().unwrap();
      }
    }
  });

  system_tray
    .on_event(create_window_event_handler(app.handle()))
    .build(app)
    .unwrap();
}
