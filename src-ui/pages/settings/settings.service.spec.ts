import { afterEach, describe, test, expect, beforeEach } from 'vitest';

import { SettingsPayload } from '~bindings/SettingsPayload';

import { Duration } from '../../utils/duration';
import { setupIPCMock, clearMocks, mockCommand } from '../../utils/test-ipc';

import { SettingsService } from './settings.service';

describe('Settings service', () => {
  beforeEach(() => {
    setupIPCMock();
  });

  afterEach(() => {
    clearMocks();
  });

  test('retrieves and parses settings from backend', async () => {
    const response: SettingsPayload = {
      long_relax_duration_secs: 15,
      relax_duration_secs: 10,
      work_duration_secs: 100,
      toggle_timer_shortcut: 'Ctrl + C',
    };
    mockCommand('get_settings', response);
    const settingsService = new SettingsService();

    const settings = await settingsService.getSettings();

    expect(settings.longRelaxDuration.secs).toBe(
      response.long_relax_duration_secs
    );
    expect(settings.relaxDuration.secs).toBe(response.relax_duration_secs);
    expect(settings.workDuration.secs).toBe(response.work_duration_secs);
    expect(settings.toggleTimerShortcut).toBe(response.toggle_timer_shortcut);
  });

  test('serializes settings before sending them to backend', async () => {
    const setSettingsMock = mockCommand('set_settings', {});
    const settingsService = new SettingsService();

    settingsService.setSettings({
      autostart: false,
      toggleTimerShortcut: 'Ctrl + C',
      longRelaxDuration: Duration.fromSecs(20),
      relaxDuration: Duration.fromSecs(15),
      workDuration: Duration.fromSecs(50),
    });

    expect(setSettingsMock).resolves.toEqual({
      newSettings: {
        toggle_timer_shortcut: 'Ctrl + C',
        long_relax_duration_secs: 20,
        relax_duration_secs: 15,
        work_duration_secs: 50,
      },
    });
  });
});
