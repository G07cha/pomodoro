export const getElementByIdOrThrow = (id: string): HTMLElement => {
  const element = document.getElementById(id);

  if (!element) {
    throw new Error(`Element with selector "#${id}" not found`);
  }

  return element;
};

export const disableAnimationsWhenInactive = () => {
  const noTransitionStylesheet = new CSSStyleSheet({ disabled: true });
  noTransitionStylesheet.replaceSync('* { transition: none !important }');
  document.adoptedStyleSheets = [noTransitionStylesheet];
  const eventListener = () => {
    noTransitionStylesheet.disabled = document.visibilityState === 'visible';
  };

  document.addEventListener('visibilitychange', eventListener);

  return () => document.removeEventListener('visibilitychange', eventListener);
};

export const disableContextMenu = () => {
  const eventListener = (event: Event) => event.preventDefault();

  document.addEventListener('contextmenu', eventListener);

  return () => document.removeEventListener('contextmenu', eventListener);
};
