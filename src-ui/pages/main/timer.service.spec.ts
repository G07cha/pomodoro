import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';

import type { TimerMode } from '~bindings/TimerMode';
import type { TimerStatePayload } from '~bindings/TimerStatePayload';

import { Duration } from '../../utils/duration';
import {
  clearMocks,
  emitEvent,
  mockCommand,
  setupIPCMock,
} from '../../utils/test-ipc';

import { TimerService } from './timer.service';

describe('Timer service', () => {
  beforeEach(() => {
    setupIPCMock();
  });

  afterEach(() => {
    clearMocks();
  });

  test('sets mode, cycle, and duration based on remote timer state', async () => {
    const expectedMode: TimerMode = 'Relax';
    const expectedCycle = 3;
    const expectedDuration = Duration.fromSecs(100);
    const response: TimerStatePayload = {
      mode: expectedMode,
      cycle: expectedCycle,
      duration_secs: expectedDuration.secs,
      is_ended: false,
    };

    const getTimerStateMock = mockCommand('get_timer_state', response);

    const timerService = new TimerService();

    await getTimerStateMock;

    expect(timerService.mode).toBe(expectedMode);
    expect(timerService.cycle).toBe(expectedCycle);
    expect(timerService.duration).toEqual(expectedDuration);
  });

  test('passes duration to listener on tick', async () => {
    const response: TimerStatePayload = {
      mode: 'Relax',
      cycle: 0,
      duration_secs: 10,
      is_ended: false,
    };
    mockCommand('get_timer_state', response);
    const onTickSpy = vi.fn();

    const timerService = new TimerService();
    timerService.onTick(onTickSpy);

    emitEvent('timer-tick', 10);

    expect(onTickSpy).toBeCalledTimes(1);
    expect(onTickSpy).toBeCalledWith(Duration.fromSecs(10));
  });
});
