export default (enabled: boolean) => {
  const classList = document.documentElement.classList;

  if (enabled) {
    classList.replace("cursor-default", "cursor-progress");
  } else {
    classList.replace("cursor-progress", "cursor-default");
  }
};
