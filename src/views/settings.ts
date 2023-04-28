import * as dom from '../utils/dom';

interface FormValues {
  workDuration: number;
  relaxDuration: number;
  longRelaxDuration: number;
  autostart: boolean;
}

export class SettingsUIController {
  workDurationInput: HTMLInputElement;
  relaxDurationInput: HTMLInputElement;
  longRelaxDurationInput: HTMLInputElement;
  autostartInput: HTMLInputElement;
  settingsForm: HTMLFormElement;

  constructor() {
    this.workDurationInput = dom.getElementByIdOrThrow(
      'work-duration'
    ) as HTMLInputElement;
    this.relaxDurationInput = dom.getElementByIdOrThrow(
      'relax-duration'
    ) as HTMLInputElement;
    this.longRelaxDurationInput = dom.getElementByIdOrThrow(
      'long-relax-duration'
    ) as HTMLInputElement;
    this.autostartInput = dom.getElementByIdOrThrow(
      'autostart'
    ) as HTMLInputElement;
    this.settingsForm = dom.getElementByIdOrThrow(
      'settings-form'
    ) as HTMLFormElement;
  }

  setFormValues(values: FormValues): void {
    this.workDurationInput.value = values.workDuration.toString();
    this.relaxDurationInput.value = values.relaxDuration.toString();
    this.longRelaxDurationInput.value = values.longRelaxDuration.toString();
    this.autostartInput.checked = values.autostart;
  }

  getFormValues(): FormValues {
    const formData = new FormData(this.settingsForm);
    const formValues: FormValues = {
      workDuration: parseInt(
        formData.get('work-duration')?.toString() ?? '0',
        10
      ),
      relaxDuration: parseInt(
        formData.get('relax-duration')?.toString() ?? '0',
        10
      ),
      longRelaxDuration: parseInt(
        formData.get('long-relax-duration')?.toString() ?? '0',
        10
      ),
      autostart: formData.get('autostart') === 'on',
    };

    return formValues;
  }
}
