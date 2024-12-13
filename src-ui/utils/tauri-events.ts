import { type InvokeArgs, invoke as tauriInvoke } from '@tauri-apps/api/core';
import {
  type EventCallback,
  type EventName,
  type UnlistenFn,
  listen as tauriListen,
} from '@tauri-apps/api/event';

import { createLogger } from './logging';

const log = createLogger('EVENTS');

export const listen = <TPayload>(
  eventName: EventName,
  handler: EventCallback<TPayload>,
): Promise<UnlistenFn> =>
  tauriListen<TPayload>(eventName, (event) => {
    log('Received event', eventName, 'with payload', event.payload);
    handler(event);
  });

export const invoke = async <TResponse>(
  cmd: string,
  args?: InvokeArgs,
): Promise<TResponse> => {
  log('Invoked command', cmd, ...(args ? ['with args', args] : []));
  const response = await tauriInvoke<TResponse>(cmd, args);
  log(
    'Received response for command',
    cmd,
    ...(args ? ['with args', args] : []),
    response,
  );
  return response;
};
