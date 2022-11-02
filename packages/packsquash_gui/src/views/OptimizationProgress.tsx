import { Title } from "@solidjs/meta";
import ViewContainer from "../components/ViewContainer";
import { useI18n } from "../contexts/i18n";
import ProgressBreadcrumb from "../components/ProgressBreadcrumb";
import { ArchiveBox, SpinnerGap, Warning } from "phosphor-solid";
import FluidTitleHeader from "../components/FluidTitleHeader";
import {
  batch,
  createSignal,
  For,
  getOwner,
  JSX,
  onCleanup,
  onMount,
  runWithOwner,
  Show
} from "solid-js";
import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
import { UnlistenFn } from "@tauri-apps/api/event";
import { selectedPackPath } from "./PackSelection";
import FluidSubtitleHeader from "../components/FluidSubtitleHeader";
import { useNavigate } from "@solidjs/router";
import { EllidedCircularBuffer } from "../util/ellidedCircularBuffer";
import setWaitingCursorEnabled from "../util/setWaitingCursorEnabled";
import { packSquashOptions } from "./Configuration";

let lastOptimizationFailure: string | undefined;
export { lastOptimizationFailure };

const successRoute = "/optimizationSuccess";
const failureRoute = "/optimizationFailure";

const warningsRingBufferSize = 250;
const warningsRingBuffer = new EllidedCircularBuffer<
  { assetPath: string; warnings: string[] },
  string
>(
  warningsRingBufferSize,
  `More than ${warningsRingBufferSize} warnings were emitted. Only the last ${warningsRingBufferSize} warnings are shown.`
);
export { warningsRingBuffer };

export default () => {
  const [l10n] = useI18n();
  const navigate = useNavigate();

  const componentOwner = getOwner();
  if (!componentOwner) {
    throw new Error("No component owner for OptimizationProgress");
  }

  if (selectedPackPath === undefined) {
    throw new Error("No selected pack path");
  }

  if (packSquashOptions === undefined) {
    throw new Error("No PackSquash options");
  }

  console.log("Optimizing with options:", packSquashOptions);

  const [packFileCount, setPackFileCount] = createSignal<number>(0);
  const [processedFileCount, setProcessedFileCount] = createSignal<number>(0);
  const [warningCount, setWarningCount] = createSignal<number>(0);

  const [packType, setPackType] = createSignal<string>();
  const [packGameVersionRange, setPackGameVersionRange] =
    createSignal<string>();

  const [statusMessages, setStatusMessages] = createSignal<JSX.Element[]>([]);

  const eventUnlistenFunctions: UnlistenFn[] = [];
  onMount(async () => {
    warningsRingBuffer.empty();

    setWaitingCursorEnabled(true);

    // Resolve any relative paths in the options to the directory where the pack is
    await invoke("set_working_directory", {
      path: await invoke("get_parent_path", { path: selectedPackPath })
    });

    // Set up the logger implementation so that status updates logged by PackSquash
    // are sent to us via Tauri events
    await invoke("init_optimization_progress_logger");

    // The command we all were waiting for :)
    invoke<void>("run_packsquash", {
      options: packSquashOptions
    })
      .then(() => navigate(successRoute, { replace: true }))
      .catch((error) => {
        lastOptimizationFailure = error;
        navigate(failureRoute, { replace: true });
      });

    // Register handlers for Tauri logging events

    eventUnlistenFunctions.push(
      await appWindow.listen<number>("pack_file_count", (event) => {
        setPackFileCount(event.payload);
      })
    );

    eventUnlistenFunctions.push(
      await appWindow.listen<[string, string]>(
        "detected_pack_type",
        (event) => {
          const [type, gameVersionRange] = event.payload;

          batch(() => {
            setPackType(type);
            setPackGameVersionRange(gameVersionRange);
          });
        }
      )
    );

    eventUnlistenFunctions.push(
      await appWindow.listen<[string, string, string[]]>(
        "processed_asset",
        (event) => {
          const [assetPath, strategyIdentifier, warnings] = event.payload;

          if (warnings.length > 0) {
            warningsRingBuffer.push({
              assetPath: assetPath,
              warnings: warnings
            });
          }

          batch(() => {
            runWithOwner(componentOwner, () => {
              setStatusMessages((statusMessages) =>
                [
                  ...statusMessages,
                  <p>
                    {l10n("optimization-screen-asset-status-message", {
                      assetPath: assetPath,
                      strategyIdentifier: strategyIdentifier
                    })}
                  </p>,
                  ...warnings.map((warning) => (
                    <p class="text-squashbrown-900" lang="en" dir="ltr">
                      {warning}
                    </p>
                  ))
                ].slice(-15)
              );

              setProcessedFileCount((count) => count + 1);
              setWarningCount(
                (previousWarningCount) => previousWarningCount + warnings.length
              );
            });
          });
        }
      )
    );
  });

  onCleanup(() => {
    setWaitingCursorEnabled(false);

    for (const eventUnlistenFunction of eventUnlistenFunctions) {
      eventUnlistenFunction();
    }
  });

  const optimizationProgress = () =>
    processedFileCount() / Math.max(packFileCount(), 1); // Avoid div by zero

  return (
    <>
      <Title>PackSquash · {l10n("optimization-screen-title")}</Title>

      <ViewContainer>
        <ProgressBreadcrumb
          steps={[
            l10n("pack-selection-screen-title"),
            l10n("configuration-screen-title"),
            l10n("optimization-screen-title")
          ]}
          class="hidden drop-shadow-md sm:block"
          currentStepIndex={2}
        />

        <div class="container mx-auto flex min-h-0 grow flex-col justify-center gap-4 text-center">
          <SpinnerGap
            size="auto"
            class="max-h-36 min-h-0 animate-spin text-zinc-900 drop-shadow-md"
          />
          <FluidTitleHeader>
            <label for="optimization-progress">
              {l10n("optimization-screen-caption")}
            </label>
          </FluidTitleHeader>

          <progress
            id="optimization-progress"
            class="block h-8 min-h-[1rem] w-full appearance-none drop-shadow-md"
            // Use a non-linear formula for display so that the progress appears faster
            // and smoother for common packs that combine fast to process assets in the
            // beginning with slower to process assets in the end.
            // Related paper: https://chrisharrison.net/projects/progressbars/ProgBarHarrison.pdf
            value={Math.pow(
              optimizationProgress() + (1 - optimizationProgress()) * 0.03,
              2
            )}
          />

          <div>
            <Show
              when={
                packType() !== undefined && packGameVersionRange() !== undefined
              }
              keyed={false}
              fallback={
                <FluidSubtitleHeader>
                  {l10n("optimization-screen-pack-type-placeholder")}
                </FluidSubtitleHeader>
              }
            >
              <FluidSubtitleHeader>
                {l10n("optimization-screen-pack-type-description", {
                  gameVersionRange: packGameVersionRange()
                    ?.replaceAll(">=", "≥")
                    .replaceAll("<=", "≤"),
                  packType: packType()?.replaceAll(" ", "_")
                })}
              </FluidSubtitleHeader>
            </Show>

            <div class="flex items-center justify-center gap-4">
              <p>
                <ArchiveBox class="inline align-text-bottom" />{" "}
                {l10n("optimization-screen-processed-file-count", {
                  processedFileCount: processedFileCount(),
                  packFileCount: packFileCount()
                })}
              </p>
              <p classList={{ "font-bold": warningCount() > 0 }}>
                <Warning class="inline align-text-bottom" />{" "}
                {l10n("optimization-screen-warning-count", {
                  warningCount: warningCount()
                })}
              </p>
            </div>
          </div>

          <Show when={statusMessages().length > 0} keyed={false}>
            <samp class="flex min-h-[2.5em] select-text flex-col justify-end overflow-hidden break-words bg-zinc-700 bg-opacity-80 leading-tight text-zinc-50 drop-shadow-md sm:mt-4 sm:p-2">
              <For each={statusMessages()}>
                {(statusMessage) => statusMessage}
              </For>
            </samp>
          </Show>
        </div>
      </ViewContainer>
    </>
  );
};
