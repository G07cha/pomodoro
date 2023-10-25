use anyhow::{Context, Result};
use rodio::OutputStreamHandle;
use rodio::{Decoder, OutputStream, Source};
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct SoundPlayer {
  // Can't drop it because it will stop the audio, see https://github.com/RustAudio/rodio/issues/330
  #[allow(dead_code)]
  stream: OutputStream,
  stream_handle: OutputStreamHandle,
}

impl SoundPlayer {
  pub fn new() -> Result<Self> {
    let (stream, stream_handle) = OutputStream::try_default()?;

    Ok(Self {
      stream_handle,
      stream,
    })
  }

  pub fn play<P: AsRef<Path> + Display>(&self, path: P) -> Result<()> {
    let file =
      BufReader::new(File::open(&path).with_context(|| format!("Failed to open '{}' file", path))?);
    let source = Decoder::new(file)?;
    self.stream_handle.play_raw(source.convert_samples())?;

    Ok(())
  }
}
