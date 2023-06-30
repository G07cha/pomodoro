use std::time::Duration;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::commands::settings::SettingsPayload;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub enum TimerMode {
  Work,
  Relax,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Settings {
  pub work_duration: Duration,
  pub relax_duration: Duration,
  pub long_relax_duration: Duration,
  pub toggle_timer_shortcut: Option<String>,
}

impl From<SettingsPayload> for Settings {
  fn from(payload: SettingsPayload) -> Self {
    Self {
      long_relax_duration: Duration::from_secs(payload.long_relax_duration_secs.into()),
      relax_duration: Duration::from_secs(payload.relax_duration_secs.into()),
      work_duration: Duration::from_secs(payload.work_duration_secs.into()),
      toggle_timer_shortcut: payload.toggle_timer_shortcut,
    }
  }
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      work_duration: Duration::from_secs(60 * 25),
      relax_duration: Duration::from_secs(60 * 5),
      long_relax_duration: Duration::from_secs(60 * 15),
      toggle_timer_shortcut: None,
    }
  }
}

pub struct Pomodoro {
  pub cycles: u32,
  pub mode: TimerMode,
}

impl Pomodoro {
  pub fn duration(&self, settings: &Settings) -> Duration {
    match self.mode {
      TimerMode::Work => settings.work_duration,
      TimerMode::Relax => {
        if self.cycles % 4 == 0 && self.cycles > 0 {
          settings.long_relax_duration
        } else {
          settings.relax_duration
        }
      }
    }
  }
}
