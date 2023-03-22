import './style.less';
import { invoke, listen } from './utils/tauri-events';
import * as theme from './utils/theme';
import * as time from './utils/time';
import { TimerIcon, TimerMode, TimerUIController } from './views/timer';
import { message } from '@tauri-apps/api/dialog';

theme.followSystemTheme();
const timerUI = new TimerUIController();
let mode = TimerMode.Work;
let cycle = 0;
let timers = {
  Work: 0,
  Relax: 0,
  LongRelax: 0,
};

invoke('get_settings').then((value: any) => {
  timers = {
    Work: value.work_duration.secs,
    Relax: value.relax_duration.secs,
    LongRelax: value.long_relax_duration.secs,
  };
  mode = value.mode;
  cycle = value.cycles;
});

listen<number>('timer-tick', ({ payload: timeSecs }) => {
  timerUI.setText(time.formatTime(timeSecs * 1000));
  timerUI.setProgress(((timeSecs / timers.Work) * 100 - 100) * -1);
});

listen<{ is_running: boolean; cycle: number; mode: TimerMode }>(
  'timer-end',
  ({ payload }) => {
    mode = payload.mode;
    cycle = payload.cycle;
    timerUI.hideIcon(TimerIcon.Play);
    timerUI.setMode(mode);
    timerUI.setCycle(cycle % 5);

    message('Timer is done', { type: 'info' }).then(() =>
      invoke('toggle_timer')
    );
  }
);

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
