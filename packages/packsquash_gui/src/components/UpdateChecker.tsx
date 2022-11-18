import { createEffect, createResource, Show } from "solid-js";
import { checkUpdate } from "@tauri-apps/api/updater";
import UpdateDialog from "./UpdateDialog";
import osFamily from "../util/osFamily";

export default () => {
  const [updateResult] = createResource(checkUpdate);

  if (import.meta.env.DEV) {
    createEffect(() => {
      switch (updateResult.state) {
        case "ready":
          console.info(
            "Update check completed successfully. Result:",
            updateResult.latest
          );
          break;
        case "errored":
          console.error("Update check error:", updateResult.error);
          break;
      }
    });
  }

  // Don't check for updates on Linux platforms, because it's reinventing the wheel for no
  // good reason: Linux has proper package managers with frontends that allow users to update
  // installed software from different sources at their convenience, in a centralized manner,
  // and users can be assumed to be somewhat educated about proper software management
  // practices. There even is an AppImage daemon for updating AppImages!
  // The above does not apply for development builds, because it's good to test the updater
  // behavior no matter the platform
  return import.meta.env.DEV || osFamily != "Linux" ? (
    <Show
      when={updateResult.state == "ready" && updateResult.latest.shouldUpdate}
      keyed={false}
    >
      <UpdateDialog updateManifest={updateResult.latest?.manifest} />
    </Show>
  ) : (
    <></>
  );
};
