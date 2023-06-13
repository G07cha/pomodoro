import * as autostart from 'tauri-plugin-autostart-api';

import { GetSettingsResponse } from '~bindings/GetSettingsResponse';
import { SetSettingsPayload } from '~bindings/SetSettingsPayload';

import { Duration } from '../utils/duration';
import { invoke } from '../utils/tauri-events';

export interface Settings {
  workDuration: Duration;
  relaxDuration: Duration;
  longRelaxDuration: Duration;
  autostart: boolean;
}

export class SettingsService {
  async getSettings(): Promise<Settings> {
    const settings = await invoke<GetSettingsResponse>('get_settings');

    return {
      workDuration: Duration.fromSecs(settings.work_duration_secs),
      relaxDuration: Duration.fromSecs(settings.relax_duration_secs),
      longRelaxDuration: Duration.fromSecs(settings.long_relax_duration_secs),
      autostart: await autostart.isEnabled(),
    };
  }

  async setSettings(newSettings: Settings) {
    const settings: SetSettingsPayload = {
      work_duration_secs: newSettings.workDuration.secs,
      relax_duration_secs: newSettings.relaxDuration.secs,
      long_relax_duration_secs: newSettings.longRelaxDuration.secs,
    };

    await invoke('set_settings', { newSettings: settings });

    if (newSettings.autostart !== (await autostart.isEnabled())) {
      if (newSettings.autostart) {
        await autostart.enable();
      } else {
        await autostart.disable();
      }
    }
  }
}
