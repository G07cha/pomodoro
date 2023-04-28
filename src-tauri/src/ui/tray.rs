use tauri::{
  App, CustomMenuItem, Manager, PhysicalPosition, Position, SystemTray, SystemTrayEvent,
  SystemTrayMenu, WindowEvent,
};

pub fn setup_tray(app: &mut App, window_label: &str) {
  let settings_menu_item = CustomMenuItem::new("settings", "Settings");
  let settings_menu_item_id = settings_menu_item.id_str.clone();
  let quit_menu_item = CustomMenuItem::new("quit", "Quit");
  let quit_menu_item_id = quit_menu_item.id_str.clone();
  let system_tray = SystemTray::new()
    .with_menu(
      SystemTrayMenu::new()
        .add_item(settings_menu_item)
        .add_item(quit_menu_item),
    )
    .with_menu_on_left_click(false);
  let window = app.get_window(window_label).unwrap();

  window.on_window_event({
    let window = window.clone();
    move |event| {
      if matches!(event, WindowEvent::Focused(false)) {
        window.hide().unwrap();
      }
    }
  });

  system_tray
    .on_event({
      let handle = app.handle();
      move |event| match event {
        SystemTrayEvent::LeftClick { position, size, .. } => {
          if window.is_visible().unwrap() {
            window.hide().unwrap();
          } else {
            let tray_size = size.width as i32;
            let window_size = window.outer_size().unwrap();
            let window_width = window_size.width as i32;

            let tray_icon_x = position.x as i32;

            let window_position = Position::Physical(PhysicalPosition {
              x: (tray_icon_x + (tray_size / 2)) - (window_width / 2),
              y: 0,
            });

            window.set_position(window_position).unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
          }
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
          if id == quit_menu_item_id {
            handle.exit(0);
          } else if id == settings_menu_item_id {
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
                  .build()
                  .unwrap();
                });
              });
            }
          }
        }
        _ => {}
      }
    })
    .build(app)
    .unwrap();
}
