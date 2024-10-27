use anyhow::Result;

use tauri::{
  webview::PageLoadEvent, AppHandle, Runtime, WebviewUrl, WebviewWindow, WebviewWindowBuilder,
  WindowEvent,
};

#[allow(dead_code)]
fn decorate_window<R: Runtime>(window: &WebviewWindow<R>) {
  #[cfg(target_os = "macos")]
  {
    use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

    apply_vibrancy(
      window,
      NSVisualEffectMaterial::Popover,
      Some(window_vibrancy::NSVisualEffectState::FollowsWindowActiveState),
      Some(8.0),
    )
    .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
  }

  #[cfg(target_os = "windows")]
  {
    use tauri::{Theme, WindowEvent};
    use window_vibrancy::apply_acrylic;

    fn apply_windows_theme<R: Runtime>(theme: &Theme, window: &WebviewWindow<R>) {
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

pub fn setup_main_window<R: Runtime>(app_handle: &AppHandle<R>) -> Result<WebviewWindow<R>> {
  let main_window = WebviewWindowBuilder::new(
    app_handle,
    crate::MAIN_WINDOW_LABEL,
    WebviewUrl::App("/pages/main/main.html".into()),
  )
  .visible(false)
  .resizable(false)
  .inner_size(170., 170.)
  .focused(true)
  .skip_taskbar(true)
  .decorations(false)
  .always_on_top(true)
  .visible_on_all_workspaces(true)
  .transparent(true)
  .build()?;

  #[cfg(target_os = "macos")]
  {
    use tauri_nspanel::{
      cocoa::appkit::{NSMainMenuWindowLevel, NSWindowCollectionBehavior},
      WebviewWindowExt,
    };

    let panel = main_window.to_panel().unwrap();

    panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel);
    panel.set_collection_behaviour(
      NSWindowCollectionBehavior::NSWindowCollectionBehaviorTransient
        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorMoveToActiveSpace
        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary
        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorIgnoresCycle,
    );

    #[allow(non_upper_case_globals)]
    const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;
    panel.set_level(NSMainMenuWindowLevel + 1);
  }

  #[cfg(not(test))]
  decorate_window(&main_window);

  main_window.on_window_event({
    let main_window = main_window.clone();
    move |event| {
      if matches!(event, WindowEvent::Focused(false)) {
        main_window.hide().unwrap();
      }
    }
  });

  Ok(main_window)
}

pub fn setup_settings_window<R: Runtime>(app_handle: &AppHandle<R>) -> Result<WebviewWindow<R>> {
  let settings_window = WebviewWindowBuilder::new(
    app_handle,
    crate::SETTINGS_WINDOW_LABEL,
    WebviewUrl::App("/pages/settings/settings.html".into()),
  )
  .title("Pomodoro settings")
  .visible(false)
  .resizable(false)
  .inner_size(350., 325.)
  .focused(true)
  .skip_taskbar(true)
  .on_page_load(|window, payload| {
    if payload.event() == PageLoadEvent::Finished {
      window
        .show()
        .expect("Failed to show settings window on load")
    }
  })
  .build()?;

  Ok(settings_window)
}

pub fn setup_about_window<R: Runtime>(app_handle: &AppHandle<R>) -> Result<WebviewWindow<R>> {
  let mut about_window = WebviewWindowBuilder::new(
    app_handle,
    crate::ABOUT_WINDOW_LABEL,
    WebviewUrl::App("/pages/about/about.html".into()),
  )
  .title("")
  .resizable(false)
  .inner_size(350., 285.)
  .focused(true)
  .on_page_load(|window, payload| {
    if payload.event() == PageLoadEvent::Finished {
      window.show().expect("Failed to show about window on load")
    }
  })
  .skip_taskbar(true);

  #[cfg(target_os = "macos")]
  {
    use tauri::TitleBarStyle;
    about_window = about_window.title_bar_style(TitleBarStyle::Overlay);
  }

  let about_window = about_window.build()?;

  Ok(about_window)
}
