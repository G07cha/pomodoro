import { expect, test, describe } from 'vitest';

import { ShortcutBuilder } from './shortcut-builder';

describe('ShortcutBuilder', () => {
  test('deduplicates added keys', () => {
    const builder = new ShortcutBuilder();

    builder.addKey('A');
    builder.addKey('B');
    builder.addKey('B');

    expect(builder.toString()).toEqual('A + B');
  });

  test('clears recorded keys', () => {
    const builder = new ShortcutBuilder();

    builder.addKey('A');
    builder.addKey('B');
    builder.clear();

    expect(builder.toString()).toEqual('');
  });
});
