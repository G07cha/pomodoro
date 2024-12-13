import * as autostart from '@tauri-apps/plugin-autostart';

import type { SettingsPayload } from '~bindings/SettingsPayload';

import { Duration } from '../../utils/duration';
import { invoke } from '../../utils/tauri-events';

export interface Settings {
  workDuration: Duration;
  relaxDuration: Duration;
  longRelaxDuration: Duration;
  autostart: boolean;
  toggleTimerShortcut?: string;
  shouldPlaySound: boolean;
}

export class SettingsService {
  async getSettings(): Promise<Settings> {
    const settings = await invoke<SettingsPayload>('get_settings');

    return {
      workDuration: Duration.fromSecs(settings.work_duration_secs),
      relaxDuration: Duration.fromSecs(settings.relax_duration_secs),
      longRelaxDuration: Duration.fromSecs(settings.long_relax_duration_secs),
      autostart: await autostart.isEnabled(),
      toggleTimerShortcut: settings.toggle_timer_shortcut ?? '',
      shouldPlaySound: Boolean(settings.should_play_sound),
    };
  }

  async setSettings(newSettings: Settings) {
    const settings: SettingsPayload = {
      work_duration_secs: newSettings.workDuration.secs,
      relax_duration_secs: newSettings.relaxDuration.secs,
      long_relax_duration_secs: newSettings.longRelaxDuration.secs,
      toggle_timer_shortcut: newSettings.toggleTimerShortcut || null,
      should_play_sound: newSettings.shouldPlaySound,
    };

    await invoke('set_settings', { newSettings: settings });

    console.log('setings are set');
    if (newSettings.autostart !== (await autostart.isEnabled())) {
      if (newSettings.autostart) {
        await autostart.enable().catch(console.log.bind(console));
      } else {
        await autostart.disable();
      }
    }
  }
}
