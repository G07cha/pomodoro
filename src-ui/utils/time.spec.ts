import { expect, test, describe } from 'vitest';

import { formatTime } from './time';
import { Duration } from './duration';

describe('formatTime', () => {
  test.each([
    [0, '00:00'],
    [2, '00:02'],
    [60, '01:00'],
    [60 * 20 + 5, '20:05'],
  ])('formats the time correctly', (input, output) => {
    expect(formatTime(Duration.fromSecs(input))).toBe(output);
  });
});
