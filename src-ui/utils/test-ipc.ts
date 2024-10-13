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

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type MockIPCResponse = Promise<any>;

export const setupIPCMock = () => {
  mockIPC((command, args) => {
    if (
      args &&
      'handler' in args &&
      typeof args.event === 'string' &&
      command === 'plugin:event|listen'
    ) {
      const eventCallbackId = `_${args.handler}` as keyof typeof window;
      const listener: (payload: unknown) => void = window[eventCallbackId];
      ipcListeners[args.event] = [
        ...(ipcListeners[args.event] ?? []),
        listener,
      ];
      return Promise.resolve() as MockIPCResponse;
    }

    const matchingHandler = pendingCommands[command]?.pop();

    if (matchingHandler) {
      setTimeout(() => matchingHandler?.onComplete(args));
      return Promise.resolve(matchingHandler?.response) as MockIPCResponse;
    }

    return Promise.resolve() as MockIPCResponse;
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
