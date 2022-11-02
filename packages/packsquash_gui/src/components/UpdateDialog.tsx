import {
  Dialog,
  DialogDescription,
  DialogOverlay,
  DialogPanel,
  DialogTitle
} from "solid-headless";
import FluidParagraph from "./FluidParagraph";
import FluidSubtitleHeader from "./FluidSubtitleHeader";
import PrimaryActionButton from "./PrimaryActionButton";
import { createSignal, getOwner, runWithOwner, Show } from "solid-js";
import { useI18n } from "../contexts/i18n";
import { installUpdate, UpdateManifest } from "@tauri-apps/api/updater";
import { toast } from "solid-toast";
import { SpinnerGap } from "phosphor-solid";
import { relaunch } from "@tauri-apps/api/process";

export default (props: { updateManifest?: UpdateManifest }) => {
  const [l10n] = useI18n();

  const [showDialog, setShowDialog] = createSignal(true);
  const [hideDialogPanel, setHideDialogPanel] = createSignal(false);

  const componentOwner = getOwner();
  if (!componentOwner) {
    throw new Error("No component owner for OptimizationProgress");
  }

  const updatePublicationDate = new Date(props.updateManifest?.date ?? NaN);

  return (
    <Dialog
      class="fixed inset-0 z-10 flex flex-col justify-center"
      isOpen={showDialog()}
    >
      <DialogOverlay class="absolute inset-0 -z-10 bg-zinc-800 bg-opacity-50" />
      <DialogPanel
        class="mx-auto w-full max-w-lg p-2 sm:p-4"
        classList={{ hidden: hideDialogPanel() }}
      >
        <div class="rounded-md bg-zinc-200 p-2 drop-shadow-lg sm:p-4">
          <DialogTitle class="mb-2 sm:mb-4">
            <FluidSubtitleHeader>
              {l10n("update-dialog-title")}
            </FluidSubtitleHeader>
          </DialogTitle>

          <DialogDescription>
            <FluidParagraph>{l10n("update-dialog-caption")}</FluidParagraph>

            <Show when={props.updateManifest} keyed={false}>
              <FluidParagraph class="mt-2">
                {l10n("update-dialog-update-version", {
                  version: props.updateManifest?.version ?? "-"
                })}
              </FluidParagraph>
              <Show
                when={!isNaN(updatePublicationDate.getTime())}
                keyed={false}
              >
                <FluidParagraph>
                  {l10n("update-dialog-update-publication-date", {
                    date: updatePublicationDate
                  })}
                </FluidParagraph>
              </Show>
              <FluidParagraph>
                {l10n("update-dialog-update-version-notes")}
                <span lang="en" dir="ltr">{` ${
                  props.updateManifest?.body ?? "-"
                }`}</span>
              </FluidParagraph>
            </Show>

            <div class="mt-2 flex justify-end gap-2 sm:gap-3">
              <PrimaryActionButton
                onClick={async () => {
                  setHideDialogPanel(true);

                  runWithOwner(componentOwner, () => {
                    toast(
                      l10n("update-dialog-update-in-progress-notification"),
                      {
                        icon: <SpinnerGap class="animate-spin" />,
                        duration: Number.POSITIVE_INFINITY,
                        id: "update-toast"
                      }
                    );
                  });

                  try {
                    await installUpdate();
                    await relaunch();
                  } catch (error) {
                    runWithOwner(componentOwner, () => {
                      toast.dismiss("update-toast");
                      toast.error(
                        l10n("update-dialog-update-error", {
                          errorDescription: String(error)
                        }),
                        {
                          duration: 8000
                        }
                      );

                      setShowDialog(false);
                    });
                  }
                }}
              >
                {l10n("update-dialog-update-accept-action")}
              </PrimaryActionButton>

              <PrimaryActionButton onClick={() => setShowDialog(false)}>
                {l10n("update-dialog-update-reject-action")}
              </PrimaryActionButton>
            </div>
          </DialogDescription>
        </div>
      </DialogPanel>
    </Dialog>
  );
};
