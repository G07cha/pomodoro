#[cfg(target_os = "windows")]
use window_vibrancy::apply_blur;
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use tauri::Window;

pub fn decorate_window(window: &Window) {
  #[cfg(target_os = "macos")]
  apply_vibrancy(
    window,
    NSVisualEffectMaterial::Popover,
    Some(window_vibrancy::NSVisualEffectState::Active),
    Some(8.0),
  )
  .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

  #[cfg(target_os = "windows")]
  apply_blur(window, Some((18, 18, 18, 125)))
    .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
}
