import { Title } from "@solidjs/meta";
import ViewContainer from "../components/ViewContainer";
import { useI18n } from "../contexts/i18n";
import FluidTitleHeader from "../components/FluidTitleHeader";
import ScreenConfettiExplosion from "../components/ScreenConfettiExplosion";
import { CheckCircle, HeartStraight } from "phosphor-solid";
import { getOwner, onCleanup, runWithOwner, Show } from "solid-js";
import FluidSubtitleHeader from "../components/FluidSubtitleHeader";
import PrimaryActionNavigateButton from "../components/PrimaryActionNavigateButton";
import PrimaryActionButton from "../components/PrimaryActionButton";
import { invoke } from "@tauri-apps/api";
import { Command, open } from "@tauri-apps/api/shell";
import { writeText } from "@tauri-apps/api/clipboard";
import { warningsRingBuffer } from "./OptimizationProgress";
import { toast } from "solid-toast";
import { packSquashOptions } from "./Configuration";
import { getOutputZipPath } from "../util/packSquashOptions";
import requestFocusOnMount from "../util/requestFocusOnMount";
import CenteredButtonGrid from "../components/CenteredButtonGrid";
import osFamily from "../util/osFamily";

const osLineSeparator = osFamily == "Windows_NT" ? "\r\n" : "\n";

let showCleanupToast = true;

export default () => {
  const [l10n] = useI18n();
  const warningCount = warningsRingBuffer.length();

  const componentOwner = getOwner();
  if (!componentOwner) {
    throw new Error("No component owner for OptimizationProgress");
  }

  let warningsClipboardText: string | undefined;

  requestFocusOnMount();

  if (showCleanupToast) {
    onCleanup(async () => {
      runWithOwner(componentOwner, () => {
        toast(l10n("optimization-success-screen-start-over-toast-text"), {
          icon: <HeartStraight color="red" class="animate-pulse" />,
          position: "top-right"
        });
      });

      // Do not nag people that optimize several packs in the same session
      showCleanupToast = false;
    });
  }

  return (
    <>
      <Title>PackSquash · {l10n("optimization-success-screen-title")}</Title>

      <Show when={warningCount == 0} keyed={false}>
        <ScreenConfettiExplosion />
      </Show>

      <ViewContainer>
        <div class="container mx-auto flex min-h-0 grow flex-col justify-center gap-3 break-words text-center">
          <CheckCircle class="max-h-48 min-h-0 drop-shadow-md" size="auto" />

          <FluidTitleHeader>
            {l10n("optimization-success-screen-done-text")}
          </FluidTitleHeader>

          <FluidSubtitleHeader>
            {l10n("optimization-success-screen-caption", {
              warningCount: warningCount
            })}
          </FluidSubtitleHeader>

          <CenteredButtonGrid columns={2}>
            <Show when={warningCount > 0} keyed={false}>
              <PrimaryActionButton
                class="col-span-full mx-auto"
                onClick={async () => {
                  // There's no point in doing the same work twice!
                  if (!warningsClipboardText) {
                    warningsClipboardText = Array.from(warningsRingBuffer)
                      .map((warningRecord) =>
                        typeof warningRecord == "string"
                          ? warningRecord
                          : `${
                              warningRecord.assetPath
                            }: ${warningRecord.warnings.join(
                              `${osLineSeparator}  `
                            )}`
                      )
                      .join(osLineSeparator);
                  }

                  await writeText(warningsClipboardText);

                  runWithOwner(componentOwner, () =>
                    toast.success(
                      l10n("optimization-success-screen-warnings-copied")
                    )
                  );
                }}
              >
                {l10n("optimization-success-screen-copy-warnings-action")}
              </PrimaryActionButton>
            </Show>

            <PrimaryActionButton
              class="col-span-full mx-auto sm:col-auto"
              onClick={openZipInFileBrowser}
            >
              {l10n("optimization-success-screen-open-generated-zip-action")}
            </PrimaryActionButton>

            <PrimaryActionNavigateButton
              class="col-span-full mx-auto sm:col-auto"
              route="/"
            >
              {l10n("optimization-success-screen-start-over-action")}
            </PrimaryActionNavigateButton>
          </CenteredButtonGrid>
        </div>
      </ViewContainer>
    </>
  );
};

async function openZipInFileBrowser() {
  if (!packSquashOptions) {
    console.warn("Can't open ZIP on file browser: missing PackSquash options");
    return;
  }

  const packZipPath = await absolutizePath(getOutputZipPath(packSquashOptions));

  let showCommand: Command | undefined;
  if (osFamily == "Linux") {
    // Try to use D-Bus. Related D-Bus interface specification:
    // https://www.freedesktop.org/wiki/Specifications/file-manager-interface/

    if (packZipPath.includes(",")) {
      // Commas are troublesome, because dbus-send interprets them as delimiters, and as far as we know
      // dbus-send does not interpret any escapes. In theory, as we work with URLs, we could percent-encode
      // the comma, but experimentally, at least Dolphin (KDE) treats percent-encoded characters literally.
      // Therefore, fallback to no command. We're not messing with the dbus C API and/or its Rust bindings
      // today
    } else {
      showCommand = new Command("dbus-send", [
        "--dest=org.freedesktop.FileManager1",
        "--type=method_call",
        "/org/freedesktop/FileManager1",
        "org.freedesktop.FileManager1.ShowItems",
        `array:string:file://${packZipPath}`,
        "string:packsquash-result-zip"
      ]);
    }
  } else if (osFamily == "Windows_NT") {
    // Paths with double quotes are not meant to be
    // (see https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file#naming-conventions), but
    // different filesystem implementations (e.g., ntfs-3g) and applications using the Win32 path namespace may
    // be able to get around that because NTFS technically supports it. In such cases, however, even Windows
    // Explorer is prone to failure, so we're kind of hopeless: let's not over-engineer things here.
    // The documentation is not clear about this, but given that explorer likely uses the SHParseDisplayName
    // Windows API function to parse the path, using absolute paths only seems like a good idea that will save
    // problems. See:
    // https://stackoverflow.com/a/54524363/9366153
    // https://stackoverflow.com/questions/62584777/how-to-use-ifileoperation-with-relative-and-absolute-paths
    showCommand = new Command("explorer", [`/select,"${packZipPath}"`]);
  } else if (osFamily == "Darwin") {
    // Against all odds, macOS could be the least quirky OS here, if this was tested ;)
    showCommand = new Command("open", ["-R", packZipPath]);
  }

  // Try to use the OS-specific command to open the containing folder, if available.
  // If not, or if that command fails, fall back to opening the containing folder, without highlighting the file
  if (showCommand) {
    showCommand.on("error", () => openContainingFolder(packZipPath));
    await showCommand.spawn();
  } else {
    await openContainingFolder(packZipPath);
  }
}

async function absolutizePath(path: string) {
  return invoke<string>("absolutize_path", { path: path });
}

async function openContainingFolder(path: string) {
  const parentPath = await invoke<string>("get_parent_path", { path: path });
  return open(parentPath);
}
