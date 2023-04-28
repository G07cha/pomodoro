import { expect, test, describe } from 'vitest';

import { formatTime, minsToSecs, secsToMins } from './time';

describe('formatTime', () => {
  test.each([
    [0, '00:00'],
    [2, '00:02'],
    [60, '01:00'],
    [60 * 20 + 5, '20:05'],
  ])('formats the time correctly', (input, output) => {
    expect(formatTime(input)).toBe(output);
  });
});

describe('minsToSecs', () => {
  test('converts minutes to seconds correctly', () => {
    expect(minsToSecs(3)).toBe(180);
  });
});

describe('secsToMins', () => {
  test('converts seconds to minutes omitting decimal part', () => {
    expect(secsToMins(150)).toBe(2);
  });
});
