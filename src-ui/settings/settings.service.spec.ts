import { afterEach, describe, test, expect, vi, beforeEach } from 'vitest';

import { TimerMode } from '~bindings/TimerMode';
import { TimerStatePayload } from '~bindings/TimerStatePayload';
import { GetSettingsResponse } from '~bindings/GetSettingsResponse';

import { Duration } from '../utils/duration';
import {
  setupIPCMock,
  clearMocks,
  emitEvent,
  mockCommand,
} from '../utils/test-ipc';

import { SettingsService } from './settings.service';

describe('Settings service', () => {
  beforeEach(() => {
    setupIPCMock();
  });

  afterEach(() => {
    clearMocks();
  });

  test('retrieves and parses settings from backend', async () => {
    const response: GetSettingsResponse = {
      long_relax_duration_secs: 15,
      relax_duration_secs: 10,
      work_duration_secs: 100,
    };
    mockCommand('get_settings', response);
    const settingsService = new SettingsService();

    const settings = await settingsService.getSettings();

    expect(settings.longRelaxDuration.secs).toBe(
      response.long_relax_duration_secs
    );
    expect(settings.relaxDuration.secs).toBe(response.relax_duration_secs);
    expect(settings.workDuration.secs).toBe(response.work_duration_secs);
  });

  test('serializes settings before sending them to backend', async () => {
    const setSettingsMock = mockCommand('set_settings', {});
    const settingsService = new SettingsService();

    settingsService.setSettings({
      autostart: false,
      longRelaxDuration: Duration.fromSecs(20),
      relaxDuration: Duration.fromSecs(15),
      workDuration: Duration.fromSecs(50),
    });

    expect(setSettingsMock).resolves.toEqual({
      newSettings: {
        long_relax_duration_secs: 20,
        relax_duration_secs: 15,
        work_duration_secs: 50,
      },
    });
  });
});
