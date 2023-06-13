import { TimerMode } from '../src-tauri/bindings/TimerMode';

import * as dom from './utils/dom';

export enum TimerIcon {
  Play = 'play',
}

export class TimerUIController {
  timerTextElement: HTMLElement;
  timerProgressElement: HTMLElement;
  timerCyclesElement: HTMLElement;

  constructor() {
    this.timerTextElement = dom.getElementByIdOrThrow('timer-text');
    this.timerProgressElement = dom.getElementByIdOrThrow('timer-progress');
    this.timerCyclesElement = dom.getElementByIdOrThrow('timer-cycles');
  }

  setMode(mode: TimerMode) {
    this.timerProgressElement.dataset.mode = mode;
  }

  /**
   * Sets current cycle
   * @param cycle a number between 0 and 4
   */
  setCycle(cycle: number) {
    this.timerCyclesElement.dataset.count = cycle.toString();
  }

  /**
   * Sets timer progress
   * @param progress a number between 0 and 100
   */
  setProgress(progress: number) {
    this.timerProgressElement.style.setProperty(
      '--current-progress',
      progress.toString()
    );
  }

  setText(text: string) {
    this.timerTextElement.innerHTML = text;
  }

  showIcon(icon: TimerIcon) {
    const iconElement = dom.getElementByIdOrThrow(`${icon}-icon`);
    iconElement.style.visibility = 'visible';
  }

  hideIcon(icon: TimerIcon) {
    const iconElement = dom.getElementByIdOrThrow(`${icon}-icon`);
    iconElement.style.visibility = 'hidden';
  }
}
