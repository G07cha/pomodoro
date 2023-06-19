import * as dom from '../utils/dom';
import { ShortcutBuilder } from '../utils/shortcut-builder';

interface FormValues {
  autostart: boolean;
  toggleTimerShortcut?: string;
  longRelaxDuration: number;
  relaxDuration: number;
  workDuration: number;
}

export class SettingsUIController {
  autostartInput: HTMLInputElement;
  toggleShortcutInput: HTMLInputElement;
  longRelaxDurationInput: HTMLInputElement;
  relaxDurationInput: HTMLInputElement;
  workDurationInput: HTMLInputElement;
  settingsForm: HTMLFormElement;

  constructor() {
    this.autostartInput = dom.getElementByIdOrThrow(
      'autostart'
    ) as HTMLInputElement;
    this.toggleShortcutInput = dom.getElementByIdOrThrow(
      'toggleShortcut'
    ) as HTMLInputElement;
    this.longRelaxDurationInput = dom.getElementByIdOrThrow(
      'long-relax-duration'
    ) as HTMLInputElement;
    this.relaxDurationInput = dom.getElementByIdOrThrow(
      'relax-duration'
    ) as HTMLInputElement;
    this.workDurationInput = dom.getElementByIdOrThrow(
      'work-duration'
    ) as HTMLInputElement;
    this.settingsForm = dom.getElementByIdOrThrow(
      'settings-form'
    ) as HTMLFormElement;

    const shortcutBuilder = new ShortcutBuilder();

    this.toggleShortcutInput.addEventListener('keydown', (event) => {
      event.preventDefault();

      if (event.code === 'Backspace') {
        this.toggleShortcutInput.value = '';
        shortcutBuilder.clear();
        return;
      }

      if (event.code === 'Escape' || event.code === 'Tab') {
        this.toggleShortcutInput.blur();
        return;
      }

      shortcutBuilder.addKey(event.code);

      this.toggleShortcutInput.value = shortcutBuilder.toString();
    });

    this.toggleShortcutInput.addEventListener('keyup', () =>
      shortcutBuilder.clear()
    );
  }

  setFormValues(values: FormValues): void {
    this.autostartInput.checked = values.autostart;
    this.toggleShortcutInput.value = values.toggleTimerShortcut ?? '';
    this.longRelaxDurationInput.value = values.longRelaxDuration.toString();
    this.relaxDurationInput.value = values.relaxDuration.toString();
    this.workDurationInput.value = values.workDuration.toString();
  }

  getFormValues(): FormValues {
    const formData = new FormData(this.settingsForm);
    const formValues: FormValues = {
      autostart: formData.get('autostart') === 'on',
      toggleTimerShortcut: formData.get('toggleShortcut')?.toString(),
      longRelaxDuration: parseInt(
        formData.get('long-relax-duration')?.toString() ?? '0',
        10
      ),
      relaxDuration: parseInt(
        formData.get('relax-duration')?.toString() ?? '0',
        10
      ),
      workDuration: parseInt(
        formData.get('work-duration')?.toString() ?? '0',
        10
      ),
    };

    return formValues;
  }
}
