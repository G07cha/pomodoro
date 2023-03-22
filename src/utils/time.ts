/**
 * @param timeMs elapsed time in milliseconds
 * @returns time string in format `MM:SS`
 */
export const formatTime = (timeMs: number): string => {
  const minutes = Math.floor(timeMs / 1000 / 60);
  const seconds = (timeMs / 1000) % 60;

  return `${minutes.toString().padStart(2, '0')}:${seconds
    .toString()
    .padStart(2, '0')}`;
};
