use enclose::enclose;
use tauri::{
  App, Manager, PhysicalPosition, Position, SystemTray, SystemTrayEvent, SystemTrayMenu,
  WindowEvent,
};

pub fn setup_tray(app: &mut App, window_label: &str) {
  let system_tray = SystemTray::new().with_menu(SystemTrayMenu::new());

  let window = app.get_window(window_label).unwrap();

  window.on_window_event(enclose! { (window) move |event| {
    if matches!(event, WindowEvent::Focused(false)) {
      window.hide().unwrap();
    }
  }});

  system_tray
    .on_event(enclose! { (window) move |event| {
      if let SystemTrayEvent::LeftClick { position, size, .. } = event {
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
    }})
    .build(app)
    .unwrap();
}
