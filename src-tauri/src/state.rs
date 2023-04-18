use std::{
  sync::{Arc, Mutex},
  time::Duration,
};

use ticking_timer::Timer;
use ts_rs::TS;

#[derive(Clone, Copy, serde::Serialize, Debug, TS)]
#[ts(export)]
pub enum TimerMode {
  Work,
  Relax,
}

#[derive(Clone, Copy, serde::Serialize, Debug)]
pub struct SettingsState {
  pub cycles: u32,
  pub work_duration: Duration,
  pub relax_duration: Duration,
  pub long_relax_duration: Duration,
  pub mode: TimerMode,
}

pub struct AppState {
  pub timer: Arc<Timer>,
  pub settings: Mutex<SettingsState>,
}
