import { Title } from "@solidjs/meta";
import ViewContainer from "../components/ViewContainer";
import { open } from "@tauri-apps/api/dialog";
import { toast } from "solid-toast";
import { useI18n } from "../contexts/i18n";
import {
  createSignal,
  getOwner,
  onCleanup,
  onMount,
  runWithOwner
} from "solid-js";
import { appWindow } from "@tauri-apps/api/window";
import { UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";
import PrimaryActionButton from "../components/PrimaryActionButton";
import BackButton from "../components/BackButton";
import ProgressBreadcrumb from "../components/ProgressBreadcrumb";
import FluidTitleHeader from "../components/FluidTitleHeader";
import FluidSubtitleHeader from "../components/FluidSubtitleHeader";
import { useNavigate } from "@solidjs/router";
import {
  registerVisualDragAndDropPreventionHandlers,
  unregisterVisualDragAndDropPreventionHandlers
} from "../util/dragAndDrop";

let selectedPackPath: string | undefined;
export { selectedPackPath };

const previousViewRoute = "/";
const nextViewRoute = "/configuration";

export default () => {
  const [l10n, l10nAccessor] = useI18n();
  const navigate = useNavigate();

  const [packDropEffect, setPackDropEffect] = createSignal<"link" | "none">();

  const componentOwner = getOwner();
  if (!componentOwner) {
    throw Error("No component owner for PackSelection view");
  }

  const setDropEffect = (event: DragEvent) => {
    // On Linux WebKit at least, this dataTransfer never contains files, so we
    // cannot work out the drop effect here. Moreover, the Tauri API functions
    // neither can override the cursor shown during drag and drop. So we have to
    // wait until we know the appropriate drop effect from Tauri events. Luckily,
    // dragover fires on mouse movement, which in practice means that this quirk
    // is not noticeable
    const dropEffect = packDropEffect();
    if (dropEffect && event.dataTransfer) {
      event.dataTransfer.dropEffect = dropEffect;
    }

    event.preventDefault();
  };

  let unlistenFileDropEvents: UnlistenFn;
  let droppedPackDirectory: string | undefined;
  onMount(async () => {
    unregisterVisualDragAndDropPreventionHandlers();

    document.documentElement.addEventListener("dragenter", setDropEffect);
    document.documentElement.addEventListener("dragover", setDropEffect);

    unlistenFileDropEvents = await appWindow.onFileDropEvent(async (event) => {
      const eventPayloadType = event.payload.type;

      if (eventPayloadType == "hover") {
        const pathCount = event.payload.paths.length;

        if (pathCount == 0 || pathCount > 1) {
          runWithOwner(componentOwner, () =>
            toast.error(l10n("pack-selection-screen-bad-drag-item-count-error"))
          );

          setPackDropEffect("none");
        } else {
          const candidatePackDirectory = event.payload.paths[0];

          if (
            await invoke("is_plausible_pack_directory", {
              path: candidatePackDirectory
            })
          ) {
            droppedPackDirectory = candidatePackDirectory;

            setPackDropEffect("link");
          } else {
            runWithOwner(componentOwner, () =>
              toast.error(
                l10n("pack-selection-screen-bad-pack-directory-error")
              )
            );

            setPackDropEffect("none");
          }
        }
      } else if (eventPayloadType == "drop" && droppedPackDirectory) {
        selectedPackPath = droppedPackDirectory;

        navigate(nextViewRoute, { replace: true });
      } else {
        droppedPackDirectory = undefined;
        setPackDropEffect(undefined);
      }
    });
  });

  onCleanup(() => {
    document.documentElement.removeEventListener("dragenter", setDropEffect);
    document.documentElement.removeEventListener("dragover", setDropEffect);

    registerVisualDragAndDropPreventionHandlers();

    unlistenFileDropEvents();
  });

  return (
    <>
      <Title>PackSquash · {l10n("pack-selection-screen-title")}</Title>

      <ViewContainer>
        <aside class="fixed z-[9998] sm:static sm:z-auto sm:flex sm:items-center">
          <BackButton
            route={previousViewRoute}
            class="bg-zinc-200 sm:bg-transparent"
          />
          <ProgressBreadcrumb
            steps={[
              l10nAccessor("pack-selection-screen-title"),
              l10nAccessor("configuration-screen-title"),
              l10nAccessor("optimization-screen-title")
            ]}
            class="hidden grow drop-shadow-md sm:block"
            currentStepIndex={0}
          />
        </aside>

        <div
          class="flex grow flex-col justify-center gap-2 rounded-2xl border-4 border-dashed border-packblue-900 p-2 text-center transition"
          classList={{
            "bg-packblue-900 bg-opacity-10": packDropEffect() == "link"
          }}
        >
          <FluidTitleHeader>
            {l10n("pack-selection-screen-caption")}
          </FluidTitleHeader>
          <FluidSubtitleHeader class="mx-auto max-w-prose whitespace-pre-line">
            {l10n("pack-selection-screen-caption-text")}
          </FluidSubtitleHeader>

          <PrimaryActionButton
            class="mx-auto mt-2"
            onClick={async () => {
              const packPath = await open({
                directory: true,
                multiple: false
              });

              // null will be returned if the user didn't select a directory
              if (typeof packPath != "string") {
                return;
              }

              if (
                await invoke("is_plausible_pack_directory", {
                  path: packPath
                })
              ) {
                selectedPackPath = packPath;

                navigate(nextViewRoute, { replace: true });
              } else {
                runWithOwner(componentOwner, () =>
                  toast.error(
                    l10n("pack-selection-screen-bad-pack-directory-error")
                  )
                );
              }
            }}
          >
            {l10n("pack-selector-dialog-button")}
          </PrimaryActionButton>
        </div>
      </ViewContainer>
    </>
  );
};
