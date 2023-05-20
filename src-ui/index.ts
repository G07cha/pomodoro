import { message } from '@tauri-apps/api/dialog';

import { TimerStatePayload } from '~bindings/TimerStatePayload';

import { invoke, listen } from './utils/tauri-events';
import * as theme from './utils/theme';
import * as time from './utils/time';
import { TimerIcon, TimerUIController } from './views/timer';

theme.followSystemTheme();
document.addEventListener('contextmenu', (event) => event.preventDefault());

let {
  mode,
  cycle,
  duration: durationSecs,
} = await invoke<TimerStatePayload>('get_timer_state');
const timerUI = new TimerUIController();
let lastTickTime = '';

timerUI.setText('');
timerUI.showIcon(TimerIcon.Play);

listen<number>('timer-tick', ({ payload: timeSecs }) => {
  lastTickTime = time.formatTime(timeSecs);
  timerUI.setText(time.formatTime(timeSecs));
  timerUI.setProgress(((timeSecs / durationSecs) * 100 - 100) * -1);
});

listen<TimerStatePayload>('timer-state', ({ payload }) => {
  mode = payload.mode;
  cycle = payload.cycle;
  durationSecs = payload.duration;

  if (payload.is_ended) {
    timerUI.showIcon(TimerIcon.Play);
    timerUI.setText('');
    timerUI.setCycle(0);
    message('Timer is done', { type: 'info' }).then(() =>
      invoke('toggle_timer')
    );
  } else {
    timerUI.setMode(mode);
    timerUI.setCycle(cycle % 5);
  }
});

listen<boolean>('timer-running-change', ({ payload: isRunning }) => {
  if (isRunning) {
    timerUI.hideIcon(TimerIcon.Play);
    timerUI.setMode(mode);
    timerUI.setCycle(cycle % 5);
    timerUI.setText(lastTickTime);
  } else {
    timerUI.showIcon(TimerIcon.Play);
    timerUI.setText('');
    timerUI.setCycle(0);
  }
});

window.addEventListener('click', () => invoke('toggle_timer'));

// Disable animations when window is hidden to avoid jumping timer progress
const noTransitionStylesheet = new CSSStyleSheet({ disabled: true });
noTransitionStylesheet.replaceSync('* { transition: none !important }');
document.adoptedStyleSheets = [noTransitionStylesheet];
document.addEventListener('visibilitychange', () => {
  noTransitionStylesheet.disabled = document.visibilityState === 'visible';
});
