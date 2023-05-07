import { isDev } from './env';

export const log = isDev()
  ? (...args: unknown[]) => console.log(...args)
  : () => undefined;

export const createLogger = (prefix: string) => log.bind(log, prefix);
