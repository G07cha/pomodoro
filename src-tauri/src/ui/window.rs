use anyhow::Result;
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use tauri::{AppHandle, Window};

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

pub fn setup_settings_window(app_handle: &AppHandle) -> Result<Window> {
  let settings_window = tauri::WindowBuilder::new(
    app_handle,
    crate::SETTINGS_WINDOW_LABEL,
    tauri::WindowUrl::App("settings/settings.html".into()),
  )
  .title("Pomodoro settings")
  .visible(false)
  .resizable(false)
  .inner_size(350., 273.)
  .focused(true)
  .skip_taskbar(true)
  .build()?;

  // Wait for DOM to load to avoid showing empty screen
  settings_window.once("window_loaded", {
    let settings_window = settings_window.clone();
    move |_| {
      settings_window
        .show()
        .expect("Failed to show settings window on load")
    }
  });

  Ok(settings_window)
}
