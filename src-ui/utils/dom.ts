export const getElementByIdOrThrow = (id: string): HTMLElement => {
  const element = document.getElementById(id);

  if (!element) {
    throw new Error(`Element with selector "#${id}" not found`);
  }

  return element;
};
