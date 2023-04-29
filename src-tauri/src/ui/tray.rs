use tauri::{
  App, CustomMenuItem, Manager, PhysicalPosition, Position, SystemTray, SystemTrayEvent,
  SystemTrayMenu, WindowEvent,
};

use crate::MAIN_WINDOW_LABEL;

const QUIT_MENU_ITEM_ID: &str = "quit";
const SETTINGS_MENU_ITEM_ID: &str = "settings";

fn create_window_event_handler(app: &mut App) -> impl Fn(SystemTrayEvent) {
  let handle = app.handle();
  let main_window = handle.get_window(MAIN_WINDOW_LABEL).unwrap();

  move |event| match event {
    SystemTrayEvent::LeftClick { position, size, .. } => {
      if main_window.is_visible().unwrap() {
        main_window.hide().unwrap();
      } else {
        let tray_size = size.width as i32;
        let window_size = main_window.outer_size().unwrap();
        let window_width = window_size.width as i32;

        let tray_icon_x = position.x as i32;

        let window_position = Position::Physical(PhysicalPosition {
          x: (tray_icon_x + (tray_size / 2)) - (window_width / 2),
          y: 0,
        });

        main_window.set_position(window_position).unwrap();
        main_window.show().unwrap();
        main_window.set_focus().unwrap();
      }
    }
    SystemTrayEvent::MenuItemClick { id, .. } => {
      if id == QUIT_MENU_ITEM_ID {
        handle.exit(0);
      } else if id == SETTINGS_MENU_ITEM_ID {
        if let Some(settings_window) = handle.get_window(crate::SETTINGS_WINDOW_LABEL) {
          settings_window.show().unwrap();
        } else {
          std::thread::scope(|s| {
            s.spawn(|| {
              tauri::WindowBuilder::new(
                &handle,
                crate::SETTINGS_WINDOW_LABEL,
                tauri::WindowUrl::App("settings.html".into()),
              )
              .title("Pomodoro settings")
              .visible(true)
              .resizable(false)
              .inner_size(350.0, 230.0)
              .focused(true)
              .skip_taskbar(true)
              .build()
              .unwrap();
            });
          });
        }
      }
    }
    _ => {}
  }
}

pub fn setup_tray(app: &mut App) {
  let settings_menu_item = CustomMenuItem::new(SETTINGS_MENU_ITEM_ID, "Settings");
  let quit_menu_item = CustomMenuItem::new(QUIT_MENU_ITEM_ID, "Quit");
  let system_tray = SystemTray::new().with_menu(
    SystemTrayMenu::new()
      .add_item(settings_menu_item)
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
    .on_event(create_window_event_handler(app))
    .build(app)
    .unwrap();
}
