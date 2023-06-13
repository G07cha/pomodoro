import { mockIPC } from '@tauri-apps/api/mocks';

// Re-export for convenience
export { clearMocks } from '@tauri-apps/api/mocks';
const ipcListeners: Record<string, ((payload: unknown) => void)[]> = {};

const pendingCommands: Record<
  string,
  {
    response?: unknown;
    onComplete: (value: unknown) => void;
  }[]
> = {};

export const setupIPCMock = () => {
  mockIPC((command, args) => {
    if (
      'message' in args &&
      typeof args.message === 'object' &&
      args.message !== null &&
      'cmd' in args.message &&
      'handler' in args.message &&
      'event' in args.message &&
      typeof args.message.event === 'string' &&
      args.message?.cmd === 'listen'
    ) {
      const eventCallbackId = `_${args.message.handler}` as keyof typeof window;
      const listener: (payload: unknown) => void = window[eventCallbackId];
      ipcListeners[args.message.event] = [
        ...(ipcListeners[args.message.event] ?? []),
        listener,
      ];
      return;
    }

    const matchingHandler = pendingCommands[command]?.pop();

    if (matchingHandler) {
      setTimeout(() => matchingHandler?.onComplete(args));
      return matchingHandler?.response;
    }

    return;
  });
};

export const emitEvent = (name: string, payload: unknown) => {
  const listeners = ipcListeners[name];

  if (Array.isArray(listeners) === false) {
    console.warn(`No listeners found for "${name}" event`);
    return;
  }

  listeners.forEach((listener) => listener({ payload }));
};

export const mockCommand = (command: string, response?: unknown) =>
  new Promise((resolve) => {
    pendingCommands[command] = [
      ...(pendingCommands[command] ?? []),
      {
        response,
        onComplete: resolve,
      },
    ];
  });
