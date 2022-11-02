export default () => {
  // Only honor the request on production. The context menu is useful on development builds
  if (import.meta.env.PROD) {
    addEventListener("contextmenu", (e) => {
      const target = e.target;

      if (!(target instanceof Element)) {
        return;
      }

      const computedStyle = getComputedStyle(target);
      // The webkit vendor prefix is necessary for the WebKit-based Linux and macOS webviews
      const canSelectText =
        (computedStyle.userSelect || computedStyle.webkitUserSelect) == "text";

      const textSelection = getSelection();
      const hasSelectedTextRange =
        textSelection && textSelection.type == "Range";

      // Disable context menu for components whose text is not selectable,
      // or if the selection is not a text range
      if (!canSelectText || !hasSelectedTextRange) {
        e.preventDefault();
      }
    });
  }
};
