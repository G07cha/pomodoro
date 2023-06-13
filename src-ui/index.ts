import { message } from '@tauri-apps/api/dialog';

import * as theme from './utils/theme';
import * as time from './utils/time';
import { TimerIcon, TimerUIController } from './timer.view';
import { disableAnimationsWhenInactive, disableContextMenu } from './utils/dom';
import { TimerService } from './timer.service';

theme.followSystemTheme();
disableAnimationsWhenInactive();
disableContextMenu();

const timerService = new TimerService();
const timerUI = new TimerUIController();
let lastTickTime = '';

timerUI.setText('');
timerUI.showIcon(TimerIcon.Play);

timerService.onStart(() => {
  timerUI.setMode(timerService.mode);
  timerUI.setCycle(timerService.cycle % 5);
});

timerService.onTick((duration) => {
  lastTickTime = time.formatTime(duration);
  timerUI.setText(lastTickTime);
  timerUI.setProgress(
    ((duration.secs / timerService.duration.secs) * 100 - 100) * -1
  );
});

timerService.onEnd(() => {
  timerUI.showIcon(TimerIcon.Play);
  timerUI.setText('');
  timerUI.setCycle(0);
  message('Timer is done', { type: 'info' }).then(() => timerService.toggle());
});

timerService.onPause(() => {
  timerUI.showIcon(TimerIcon.Play);
  timerUI.setText('');
  timerUI.setCycle(0);
});

timerService.onResume(() => {
  timerUI.hideIcon(TimerIcon.Play);
  timerUI.setMode(timerService.mode);
  timerUI.setCycle(timerService.cycle % 5);
  timerUI.setText(lastTickTime);
});

window.addEventListener('click', () => timerService.toggle());
