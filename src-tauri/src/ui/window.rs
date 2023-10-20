use anyhow::Result;
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;

use tauri::{AppHandle, Runtime, Window, WindowBuilder, WindowUrl};

pub fn decorate_window<R: Runtime>(window: &Window<R>) {
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

    fn apply_windows_theme(theme: &Theme, window: &Window<R>) {
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

pub fn setup_main_window<R: Runtime>(app_handle: &AppHandle<R>) -> Result<Window<R>> {
  let main_window = WindowBuilder::new(
    app_handle,
    crate::MAIN_WINDOW_LABEL,
    WindowUrl::App("/pages/main/main.html".into()),
  )
  .visible(false)
  .resizable(false)
  .inner_size(170., 170.)
  .focused(false)
  .skip_taskbar(true)
  .decorations(false)
  .always_on_top(true)
  .transparent(true)
  .build()?;

  Ok(main_window)
}

pub fn setup_settings_window<R: Runtime>(app_handle: &AppHandle<R>) -> Result<Window<R>> {
  let settings_window = WindowBuilder::new(
    app_handle,
    crate::SETTINGS_WINDOW_LABEL,
    WindowUrl::App("/pages/settings/settings.html".into()),
  )
  .title("Settings")
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

pub fn setup_about_window<R: Runtime>(app_handle: &AppHandle<R>) -> Result<Window<R>> {
  let about_window = WindowBuilder::new(
    app_handle,
    crate::ABOUT_WINDOW_LABEL,
    WindowUrl::App("/pages/about/about.html".into()),
  )
  .title("")
  .resizable(false)
  .inner_size(350., 265.)
  .focused(true)
  .skip_taskbar(true);

  #[cfg(target_os = "macos")]
  let about_window = about_window.title_bar_style(TitleBarStyle::Overlay);

  let about_window = about_window.build()?;

  // Wait for DOM to load to avoid showing empty screen
  about_window.once("window_loaded", {
    let about_window = about_window.clone();
    move |_| {
      about_window
        .show()
        .expect("Failed to show settings window on load")
    }
  });

  Ok(about_window)
}
