import { message } from '@tauri-apps/plugin-dialog';

import * as theme from '../../utils/theme';
import * as time from '../../utils/time';
import {
  disableAnimationsWhenInactive,
  disableContextMenu,
} from '../../utils/dom';

import { TimerUIController, UIState } from './timer.view';
import { TimerService } from './timer.service';

theme.followSystemTheme();
disableAnimationsWhenInactive();
disableContextMenu();

const timerService = new TimerService();
const timerUI = new TimerUIController();
let lastTickTime = '';

timerUI.setUIState(UIState.Reset);

timerService.onStart(() => {
  timerUI.setMode(timerService.mode);
  timerUI.setCycle(timerService.cycle % 5);
});

timerService.onTick((duration) => {
  lastTickTime = time.formatTime(duration);
  timerUI.setText(lastTickTime);
  timerUI.setProgress(
    ((duration.secs / timerService.duration.secs) * 100 - 100) * -1,
  );
});

timerService.onEnd(() => {
  timerUI.setUIState(UIState.Reset);
  message('Timer is done', { kind: 'info' }).then(() =>
    timerService.nextCycle(),
  );
});

timerService.onPause(() => {
  timerUI.setUIState(UIState.Paused);
});

timerService.onResume(() => {
  timerUI.setUIState(UIState.Running);
  timerUI.setMode(timerService.mode);
  timerUI.setCycle(timerService.cycle % 5);
  timerUI.setText(lastTickTime);
});

timerUI.onPlayClick(() => timerService.toggle());
timerUI.onRestartClick(() => timerService.reset());

window.addEventListener('click', () => {
  if (timerService.isRunning) {
    timerService.toggle();
  }
});
