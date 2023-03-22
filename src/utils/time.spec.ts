import { expect, test } from 'vitest';
import { formatTime } from './time';

test.each([
  [0, '00:00'],
  [2000, '00:02'],
  [1000 * 60, '01:00'],
  [1000 * 60 * 20 + 5000, '20:05'],
])('formats the time correctly', (input, output) => {
  expect(formatTime(input)).toBe(output);
});
