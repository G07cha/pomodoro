import { TimerMode } from '../../../src-tauri/bindings/TimerMode';
import * as dom from '../../utils/dom';
import { assertType } from '../../utils/type';

export enum TimerIcon {
  Play = 'play',
  Restart = 'restart',
}

export enum UIState {
  Running,
  Paused,
  Reset,
}

export class TimerUIController {
  private _appElement: HTMLElement;
  private _timeTextElement: HTMLElement;
  private _progressElement: HTMLElement;
  private _cyclesElement: HTMLElement;
  private _playIconElement: HTMLElement;
  private _restartIconElement: HTMLElement;

  constructor() {
    this._appElement = dom.getElementByIdOrThrow('app');
    this._timeTextElement = dom.getElementByIdOrThrow('timer-text');
    this._progressElement = dom.getElementByIdOrThrow('timer-progress');
    this._cyclesElement = dom.getElementByIdOrThrow('timer-cycles');
    this._playIconElement = dom.getElementByIdOrThrow('play-icon');
    this._restartIconElement = dom.getElementByIdOrThrow('restart-icon');
  }

  setMode(mode: TimerMode) {
    this._progressElement.dataset.mode = mode;
  }

  /**
   * Sets current cycle
   * @param cycle a number between 0 and 4
   */
  setCycle(cycle: number) {
    this._cyclesElement.dataset.count = cycle.toString();
  }

  /**
   * Sets timer progress
   * @param progress a number between 0 and 100
   */
  setProgress(progress: number) {
    this._progressElement.style.setProperty(
      '--current-progress',
      progress.toString(),
    );
  }

  setText(text: string) {
    this._timeTextElement.innerHTML = text;
  }

  showIcon(icon: TimerIcon) {
    const iconElement = dom.getElementByIdOrThrow(`${icon}-icon`);
    iconElement.style.visibility = 'visible';
  }

  hideIcon(icon: TimerIcon) {
    const iconElement = dom.getElementByIdOrThrow(`${icon}-icon`);
    iconElement.style.visibility = 'hidden';
  }

  onPlayClick(callback: () => void): void {
    this._playIconElement.addEventListener('click', callback, false);
  }

  onRestartClick(callback: () => void) {
    this._restartIconElement.addEventListener('click', callback, false);
  }

  setUIState(state: UIState) {
    switch (state) {
      case UIState.Running:
        this._appElement.classList.remove('paused');
        this.hideIcon(TimerIcon.Play);
        this.hideIcon(TimerIcon.Restart);
        break;
      case UIState.Paused:
        this._appElement.classList.add('paused');
        this.showIcon(TimerIcon.Play);
        this.showIcon(TimerIcon.Restart);
        this.setText('');
        this.setCycle(0);
        break;
      case UIState.Reset:
        this._appElement.classList.add('paused');
        this.showIcon(TimerIcon.Play);
        this.setText('');
        this.setCycle(0);
        break;
      default:
        assertType<never>(state);
    }
  }
}
