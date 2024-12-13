import type { Duration } from './duration';

/**
 * @param timeSecs elapsed time in seconds
 * @returns time string in format `MM:SS`
 */
export const formatTime = (duration: Duration): string => {
  const minutes = duration.mins;
  const seconds = duration.secs % 60;

  return `${minutes.toString().padStart(2, '0')}:${seconds
    .toString()
    .padStart(2, '0')}`;
};
