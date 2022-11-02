import { appWindow } from "@tauri-apps/api/window";

export default () => {
  let previousTitle: string | undefined;

  new MutationObserver(() => {
    const currentTitle = document.title;

    // Comparing title strings is cheaper than inspecting the actual mutations
    if (currentTitle !== previousTitle) {
      appWindow.setTitle(currentTitle).catch();
      previousTitle = currentTitle;
    }
  }).observe(
    // The SolidJS MetaProvider title change works by recreating the <title> element.
    // Thus, listening for changes to a <title> element won't work. Instead, we have
    // to listen for changes in <head> children
    document.head,
    { subtree: true, childList: true }
  );
};
