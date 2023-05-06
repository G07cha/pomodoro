#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use tauri::Window;

pub fn decorate_window(window: &Window) {
  #[cfg(target_os = "macos")]
  apply_vibrancy(
    window,
    NSVisualEffectMaterial::Popover,
    Some(window_vibrancy::NSVisualEffectState::FollowsWindowActiveState),
    Some(8.0),
  )
  .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

  #[cfg(target_os = "windows")]
  {
    use tauri::{Theme, WindowEvent};
    use window_vibrancy::apply_acrylic;

    fn apply_windows_theme(theme: &Theme, window: &Window) {
      match theme {
        Theme::Light => apply_acrylic(window, Some((255, 255, 255, 125)))
          .expect("Unsupported platform! 'apply_acrylic' is only supported on Windows"),
        Theme::Dark => apply_acrylic(window, Some((0, 0, 0, 50)))
          .expect("Unsupported platform! 'apply_acrylic' is only supported on Windows"),
        _ => todo!(),
      }
    }

    apply_windows_theme(&window.theme().unwrap(), window);

    window.on_window_event({
      let window = window.clone();
      move |event| {
        if let WindowEvent::ThemeChanged(theme) = event {
          apply_windows_theme(theme, &window)
        }
      }
    });
  }
}
