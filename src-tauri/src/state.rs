use std::time::Duration;

use ts_rs::TS;

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize, Debug, TS)]
#[ts(export)]
pub enum TimerMode {
  Work,
  Relax,
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize, Debug)]
pub struct Settings {
  pub work_duration: Duration,
  pub relax_duration: Duration,
  pub long_relax_duration: Duration,
}

pub struct Pomodoro {
  pub cycles: u32,
  pub mode: TimerMode,
}
