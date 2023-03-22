import {
  listen as tauriListen,
  EventName,
  EventCallback,
  UnlistenFn,
} from '@tauri-apps/api/event';
import { invoke as tauriInvoke, InvokeArgs } from '@tauri-apps/api/tauri';
import { createLogger } from './logging';

const log = createLogger('EVENTS');

export const listen = <TPayload>(
  eventName: EventName,
  handler: EventCallback<TPayload>
): Promise<UnlistenFn> =>
  tauriListen<TPayload>(eventName, (event) => {
    log('Received event', eventName, 'with payload', event.payload);
    handler(event);
  });

export const invoke = <TResponse>(
  cmd: string,
  args?: InvokeArgs
): Promise<TResponse> => {
  log('Invoked command', cmd, ...(args ? ['with args', args] : []));
  return tauriInvoke<TResponse>(cmd, args);
};
