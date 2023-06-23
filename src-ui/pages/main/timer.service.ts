import { TimerMode } from '~bindings/TimerMode';
import { TimerStatePayload } from '~bindings/TimerStatePayload';

import { Duration } from '../../utils/duration';
import { listen, invoke } from '../../utils/tauri-events';

export class TimerService {
  private _mode: TimerMode = 'Work';
  private _cycle = 0;
  private _duration: Duration = new Duration(0);

  constructor() {
    invoke<TimerStatePayload>('get_timer_state').then(
      ({ cycle, duration_secs, mode }) => {
        this._mode = mode;
        this._cycle = cycle;
        this._duration = Duration.fromSecs(duration_secs);
      }
    );
  }

  onTick = (listener: (duration: Duration) => void) =>
    listen<number>('timer-tick', ({ payload: timeSecs }) => {
      listener(Duration.fromSecs(timeSecs));
    });

  onEnd = (listener: () => void) =>
    listen<TimerStatePayload>('timer-state', ({ payload }) => {
      this._mode = payload.mode;
      this._cycle = payload.cycle;
      this._duration = Duration.fromSecs(payload.duration_secs);

      if (payload.is_ended) {
        listener();
      }
    });

  onStart = (listener: () => void) =>
    listen<TimerStatePayload>('timer-state', ({ payload }) => {
      this._mode = payload.mode;
      this._cycle = payload.cycle;
      this._duration = Duration.fromSecs(payload.duration_secs);

      if (payload.is_ended === false) {
        listener();
      }
    });

  onPause = (listener: () => void) =>
    listen<boolean>('timer-running-change', ({ payload: isRunning }) => {
      if (isRunning === false) {
        listener();
      }
    });

  onResume = (listener: () => void) =>
    listen<boolean>('timer-running-change', ({ payload: isRunning }) => {
      if (isRunning) {
        listener();
      }
    });

  toggle = () => invoke('toggle_timer');

  get duration() {
    return this._duration;
  }

  get mode() {
    return this._mode;
  }

  get cycle() {
    return this._cycle;
  }
}