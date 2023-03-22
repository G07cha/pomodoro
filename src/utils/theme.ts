export const enum Theme {
  Dark = 'dark',
  Light = 'light',
}

export const setTheme = (theme: Theme) => {
  const html = document.querySelector('html');

  if (!html) {
    throw new Error('"html" element not found');
  }

  html.dataset.theme = theme;
};

export const followSystemTheme = () => {
  const matchMediaPrefDark = window.matchMedia('(prefers-color-scheme: dark)');
  const handleSystemThemeChange = (event: MediaQueryListEvent) => {
    const isDark = event.matches;
    setTheme(isDark ? Theme.Dark : Theme.Light);
  };

  handleSystemThemeChange(
    new MediaQueryListEvent('init', { matches: matchMediaPrefDark.matches })
  );

  matchMediaPrefDark.addEventListener('change', handleSystemThemeChange);

  return () =>
    matchMediaPrefDark.removeEventListener('change', handleSystemThemeChange);
};
