const handleDragAndDrop = (event: DragEvent) => {
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = "none";
  }

  event.preventDefault();
};

export function registerVisualDragAndDropPreventionHandlers() {
  document.documentElement.addEventListener("dragenter", handleDragAndDrop);
  document.documentElement.addEventListener("dragover", handleDragAndDrop);
}

export function unregisterVisualDragAndDropPreventionHandlers() {
  document.documentElement.removeEventListener("dragenter", handleDragAndDrop);
  document.documentElement.removeEventListener("dragover", handleDragAndDrop);
}
