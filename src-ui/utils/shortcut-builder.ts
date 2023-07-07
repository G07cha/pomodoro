export class ShortcutBuilder {
  private shortcut = new Set<string>();

  addKey = (key: string) => this.shortcut.add(key);

  clear = () => this.shortcut.clear();

  toString = () =>
    Array.from(this.shortcut.values())
      .map((value) =>
        value
          .replace(/^(Key|Digit)/, '')
          .replace(/^Meta/, 'Super')
          .replace(/(Left|Right)$/, ''),
      )
      .join(' + ');
}
