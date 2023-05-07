/**
 * @param timeSecs elapsed time in seconds
 * @returns time string in format `MM:SS`
 */
export const formatTime = (timeSecs: number): string => {
  const minutes = secsToMins(timeSecs);
  const seconds = timeSecs % 60;

  return `${minutes.toString().padStart(2, '0')}:${seconds
    .toString()
    .padStart(2, '0')}`;
};

export const minsToSecs = (valueMins: number) => valueMins * 60;

export const secsToMins = (valueSecs: number) => Math.floor(valueSecs / 60);
