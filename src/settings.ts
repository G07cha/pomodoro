import * as autostart from 'tauri-plugin-autostart-api';
import { appWindow } from '@tauri-apps/api/window';

import { GetSettingsResponse } from '@bindings/GetSettingsResponse';
import { SetSettingsPayload } from '@bindings/SetSettingsPayload';

import { invoke } from './utils/tauri-events';
import * as theme from './utils/theme';
import { minsToSecs, secsToMins } from './utils/time';
import { SettingsUIController } from './views/settings';

theme.followSystemTheme();

const settings = await invoke<GetSettingsResponse>('get_settings');
const settingsUI = new SettingsUIController();

settingsUI.setFormValues({
  workDuration: secsToMins(settings.work_duration),
  relaxDuration: secsToMins(settings.relax_duration),
  longRelaxDuration: secsToMins(settings.long_relax_duration),
  autostart: await autostart.isEnabled(),
});

appWindow.onCloseRequested(async () => {
  const formValues = settingsUI.getFormValues();
  const newSettings: SetSettingsPayload = {
    ...settings,
    work_duration: minsToSecs(formValues.workDuration),
    relax_duration: minsToSecs(formValues.relaxDuration),
    long_relax_duration: minsToSecs(formValues.longRelaxDuration),
  };

  if (JSON.stringify(newSettings) !== JSON.stringify(settings)) {
    await invoke('set_settings', { newSettings });
  }

  if (formValues.autostart && (await autostart.isEnabled()) === false) {
    await autostart.enable();
  } else {
    await autostart.disable();
  }
});
