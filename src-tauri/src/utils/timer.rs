use std::{
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  thread::{self, JoinHandle},
  time::{Duration, Instant},
};

pub enum TimerEvent {
  Tick { remaining: Duration },
  TimeEnd {},
}

type TimerEventHandler = dyn Fn(TimerEvent) + Send + Sync + 'static;

pub struct Timer {
  duration: Duration,
  update_frequency: Duration,
  elapsed: Duration,
  last_start: Option<Instant>,
  is_running: Arc<AtomicBool>,
  event_handler: Option<Arc<TimerEventHandler>>,
  timer_thread: Option<JoinHandle<()>>,
}

pub struct TimerSettings {
  pub duration: Duration,
  pub update_frequency: Duration,
}

impl Timer {
  /// Creates a new [`Timer`].
  pub fn new(settings: TimerSettings) -> Self {
    Timer {
      duration: settings.duration,
      update_frequency: settings.update_frequency,
      elapsed: Duration::from_secs(0),
      last_start: None,
      is_running: Arc::new(AtomicBool::new(false)),
      event_handler: None,
      timer_thread: None,
    }
  }

  /// Pauses the [`Timer`]
  pub fn pause(&mut self) -> &mut Self {
    match self.last_start.take() {
      Some(x) => self.elapsed += x.elapsed(),
      None => debug_assert!(false, "Tried to pause a non-running Timer"),
    }

    self.is_running.store(false, Ordering::SeqCst);

    // Joining in event handler causes deadlock, do we even need to join it?
    // TODO: Yes we do! Or have a single thread running for the whole lifetime
    // self
    //   .timer_thread
    //   .take()
    //   .unwrap()
    //   .join()
    //   .expect("Failed to join timer thread");

    self
  }

  /// Starts the [`Timer`]
  pub fn start(&mut self) {
    let current_start = Instant::now();
    let elapsed = self.elapsed();
    let event_handler = self.event_handler.clone();
    let is_running = self.is_running.clone();
    let duration = self.duration;
    let update_frequency = self.update_frequency;
    self.is_running.store(true, Ordering::SeqCst);
    self.last_start.replace(current_start);

    // if let Some(ref event_handler) = event_handler {
    //   event_handler(TimerEvent::StateChange { is_running: true })
    // }

    self.timer_thread.replace(thread::spawn(move || {
      let mut total_elapsed = elapsed + current_start.elapsed();

      while is_running.load(Ordering::SeqCst) && total_elapsed < duration {
        if let Some(ref event_handler) = event_handler {
          event_handler(TimerEvent::Tick {
            remaining: duration - total_elapsed,
          });
        }

        thread::sleep(update_frequency);
        total_elapsed = elapsed + current_start.elapsed();
      }

      if let Some(event_handler) = event_handler {
        if total_elapsed >= duration {
          event_handler(TimerEvent::TimeEnd {});
        }
      }
    }));
  }

  /// Toggles running state of the [`Timer`]
  pub fn toggle(&mut self) {
    if self.is_running.load(Ordering::SeqCst) {
      self.pause();
    } else {
      self.start();
    }
  }

  /// Stops the [`Timer`] and resets elapsed time to provided or initial duration
  pub fn reset(&mut self, duration: Option<Duration>) -> &mut Self {
    self.pause();
    if let Some(x) = duration {
      self.duration = x
    }
    self.elapsed = Duration::from_secs(0);
    self
  }

  /// Returns the elapsed time of this [`Timer`].
  pub fn elapsed(&self) -> Duration {
    let mut total_elapsed = self.elapsed;
    if let Some(last_start) = self.last_start {
      total_elapsed += last_start.elapsed();
    }

    if total_elapsed > self.duration {
      self.duration
    } else {
      total_elapsed
    }
  }

  pub fn remaining(&self) -> Duration {
    self.duration - self.elapsed()
  }

  /// Returns the running status of this [`Timer`].
  pub fn is_running(&self) -> bool {
    if self.elapsed() < self.duration {
      self.is_running.load(Ordering::SeqCst)
    } else {
      false
    }
  }

  pub fn duration(&self) -> Duration {
    self.duration
  }

  pub fn on_event<F: Fn(TimerEvent) + Send + Sync + 'static>(&mut self, f: F) -> &mut Self {
    self.event_handler.replace(Arc::new(f));
    self
  }
}

#[cfg(test)]
mod tests {
  use std::thread::sleep;

  use super::*;

  #[test]
  fn it_initializes_with_stopped_timer() {
    let timer = Timer::new(TimerSettings {
      duration: Duration::from_secs(10),
      update_frequency: Duration::from_secs(1),
    });

    assert!(!timer.is_running());
    assert_eq!(timer.elapsed().as_millis(), 0);
    assert_eq!(timer.remaining().as_secs(), 10);
  }

  #[test]
  fn it_starts_the_timer() {
    let mut timer = Timer::new(TimerSettings {
      duration: Duration::from_secs(10),
      update_frequency: Duration::from_secs(1),
    });

    timer.start();

    assert!(timer.is_running());
  }

  #[test]
  fn it_pauses_the_timer() {
    let mut timer = Timer::new(TimerSettings {
      duration: Duration::from_secs(10),
      update_frequency: Duration::from_millis(100),
    });

    timer.start();
    timer.pause();

    assert!(!timer.is_running());
    assert_eq!(timer.elapsed().as_secs(), Duration::from_secs(0).as_secs());
  }

  #[test]
  fn it_resets_the_timer() {
    let old_duration = Duration::from_secs(10);
    let new_duration = old_duration + Duration::from_secs(10);
    let mut timer = Timer::new(TimerSettings {
      duration: old_duration,
      update_frequency: Duration::from_millis(200),
    });

    timer.start();
    timer.reset(Some(new_duration));

    assert!(!timer.is_running());
    assert_eq!(timer.duration(), new_duration);
    assert_eq!(timer.elapsed(), Duration::from_secs(0));
  }

  #[test]
  fn it_stops_the_timer_on_end() {
    let duration = Duration::from_millis(10);
    let mut timer = Timer::new(TimerSettings {
      duration,
      update_frequency: Duration::from_millis(10),
    });

    timer.start();
    sleep(Duration::from_millis(20));

    assert!(!timer.is_running());
    assert_eq!(timer.elapsed(), duration);
  }
}
