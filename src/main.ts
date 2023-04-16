import { GetSettingsResponse } from '@bindings/GetSettingsResponse';
import { TimerEndPayload } from '@bindings/TimerEndPayload';
import { TimerMode } from '@bindings/TimerMode';
import './style.less';
import { invoke, listen } from './utils/tauri-events';
import * as theme from './utils/theme';
import * as time from './utils/time';
import { TimerIcon, TimerUIController } from './views/timer';
import { message } from '@tauri-apps/api/dialog';

theme.followSystemTheme();
const timerUI = new TimerUIController();
let mode: TimerMode = 'Work';
let cycle = 0;
let timers = {
  Work: 0,
  Relax: 0,
  LongRelax: 0,
};

invoke<GetSettingsResponse>('get_settings').then((value) => {
  timers = {
    Work: value.work_duration,
    Relax: value.relax_duration,
    LongRelax: value.long_relax_duration,
  };
  mode = value.mode;
  cycle = value.cycles;
});

listen<number>('timer-tick', ({ payload: timeSecs }) => {
  timerUI.setText(time.formatTime(timeSecs * 1000));
  timerUI.setProgress(((timeSecs / timers.Work) * 100 - 100) * -1);
});

listen<TimerEndPayload>('timer-end', ({ payload }) => {
  mode = payload.mode;
  cycle = payload.cycle;
  timerUI.hideIcon(TimerIcon.Play);
  timerUI.setMode(mode);
  timerUI.setCycle(cycle % 5);

  message('Timer is done', { type: 'info' }).then(() => invoke('toggle_timer'));
});

listen<boolean>('timer-state', ({ payload: isRunning }) => {
  if (isRunning) {
    timerUI.hideIcon(TimerIcon.Play);
    timerUI.setMode(mode);
    timerUI.setCycle(cycle % 5);
  } else {
    timerUI.showIcon(TimerIcon.Play);
    timerUI.setText('');
    timerUI.setCycle(0);
  }
});

window.addEventListener('click', () => invoke('toggle_timer'));
