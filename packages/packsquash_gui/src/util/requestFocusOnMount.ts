import { TauriEvent, UnlistenFn } from "@tauri-apps/api/event";
import { onCleanup, onMount } from "solid-js";
import { appWindow, UserAttentionType } from "@tauri-apps/api/window";

export default () => {
  // Do nothing if we already have focus
  if (document.hasFocus()) {
    return;
  }

  const undoUserAttentionRequest = () => appWindow.requestUserAttention(null);

  let focusEventUnlistenFn: UnlistenFn;
  onMount(async () => {
    focusEventUnlistenFn = await appWindow.once(
      TauriEvent.WINDOW_FOCUS,
      undoUserAttentionRequest
    );

    appWindow.requestUserAttention(UserAttentionType.Informational).catch();
  });

  onCleanup(async () => {
    undoUserAttentionRequest().catch();
    focusEventUnlistenFn();
  });
};
