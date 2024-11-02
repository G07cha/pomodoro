use anyhow::Result;
use tauri::{AppHandle, Runtime};
use tauri_plugin_updater::UpdaterExt;

pub async fn update<R: Runtime>(app: &AppHandle<R>) -> Result<bool> {
  if let Some(update) = app.updater()?.check().await? {
    let mut downloaded = 0;

    update
      .download_and_install(
        |chunk_length, content_length| {
          downloaded += chunk_length;
          println!("downloaded {downloaded} from {content_length:?}");
        },
        || {
          println!("download finished");
        },
      )
      .await?;

    println!("update installed");
  }

  Ok(false)
}
