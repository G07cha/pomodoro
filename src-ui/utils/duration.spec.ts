import { describe, expect, test } from 'vitest';

import { Duration } from './duration';

describe('Duration', () => {
  test('initializes with correct time', () => {
    expect(new Duration(1000).millis).toBe(1000);
  });

  test('converts to minutes omitting decimal part', () => {
    expect(new Duration(150000).mins).toBe(2);
  });

  test('converts from minutes', () => {
    expect(Duration.fromMins(3).millis).toBe(180000);
  });

  test('converts from seconds', () => {
    expect(Duration.fromSecs(3).millis).toBe(3000);
  });
});
