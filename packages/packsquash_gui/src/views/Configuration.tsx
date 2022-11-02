import { Title } from "@solidjs/meta";
import ViewContainer from "../components/ViewContainer";
import { useI18n } from "../contexts/i18n";
import BackButton from "../components/BackButton";
import ProgressBreadcrumb from "../components/ProgressBreadcrumb";
import OptionsEditor from "../components/OptionsEditor";
import { selectedPackPath } from "./PackSelection";
import { createSignal, getOwner, runWithOwner, Show } from "solid-js";
import FluidSubtitleHeader from "../components/FluidSubtitleHeader";
import PrimaryActionButton from "../components/PrimaryActionButton";
import { useNavigate } from "@solidjs/router";
import { open } from "@tauri-apps/api/dialog";
import { readTextFile } from "@tauri-apps/api/fs";
import TOML from "@iarna/toml";
import { toast } from "solid-toast";
import FluidSmallParagraph from "../components/FluidSmallParagraph";

let packSquashOptions: Record<string, unknown> | undefined;
export { packSquashOptions };

const previousViewRoute = "/packSelection";
const nextViewRoute = "/optimizationProgress";

export default () => {
  const [l10n] = useI18n();
  const navigate = useNavigate();
  const [showOptionsEditor, setShowOptionsEditor] =
    createSignal<boolean>(false);

  const componentOwner = getOwner();
  if (!componentOwner) {
    throw new Error("No component owner for OptimizationProgress");
  }

  if (!selectedPackPath) {
    throw new Error("No selected pack path");
  }

  return (
    <>
      <Title>PackSquash · {l10n("configuration-screen-title")}</Title>

      <ViewContainer fitContentHeight={true}>
        <aside class="fixed z-[9998] sm:static sm:z-auto sm:flex sm:items-center">
          <Show
            when={showOptionsEditor()}
            fallback={
              <BackButton
                route={previousViewRoute}
                class="bg-zinc-200 sm:bg-transparent"
              />
            }
            keyed={false}
          >
            <BackButton
              backAction={() => setShowOptionsEditor(false)}
              class="bg-zinc-200 sm:bg-transparent"
            />
          </Show>
          <ProgressBreadcrumb
            steps={[
              l10n("pack-selection-screen-title"),
              l10n("configuration-screen-title"),
              l10n("optimization-screen-title")
            ]}
            class="hidden grow drop-shadow-md sm:block"
            currentStepIndex={1}
          />
        </aside>

        <div class="container mx-auto flex grow flex-col justify-center gap-3">
          <Show
            when={showOptionsEditor()}
            keyed={false}
            fallback={
              <>
                <FluidSubtitleHeader class="text-center">
                  {l10n("configuration-screen-caption")}
                </FluidSubtitleHeader>

                <PrimaryActionButton
                  class="mx-auto"
                  onClick={() => {
                    packSquashOptions = { pack_directory: selectedPackPath };
                    navigate(nextViewRoute, { replace: true });
                  }}
                >
                  {l10n("configuration-screen-default-options-action")}
                </PrimaryActionButton>
                <FluidSmallParagraph class="mx-auto max-w-prose text-center">
                  {l10n(
                    "configuration-screen-default-options-action-description"
                  )}
                </FluidSmallParagraph>

                <PrimaryActionButton
                  class="mx-auto"
                  onClick={() => setShowOptionsEditor(true)}
                >
                  {l10n("configuration-screen-custom-options-action")}
                </PrimaryActionButton>
                <FluidSmallParagraph class="mx-auto max-w-prose text-center">
                  {l10n(
                    "configuration-screen-custom-options-action-description"
                  )}
                </FluidSmallParagraph>

                <PrimaryActionButton
                  class="mx-auto"
                  onClick={async () => {
                    const optionsFilePath = await open({
                      directory: false,
                      multiple: false,
                      filters: [{ name: "TOML", extensions: ["toml"] }]
                    });
                    if (!optionsFilePath || Array.isArray(optionsFilePath)) {
                      return;
                    }

                    try {
                      packSquashOptions = TOML.parse(
                        await readTextFile(optionsFilePath)
                      );

                      // Overwrite any pack directory set by the options
                      packSquashOptions.pack_directory = selectedPackPath;

                      navigate(nextViewRoute, { replace: true });
                    } catch (err) {
                      runWithOwner(componentOwner, () => {
                        toast.error(
                          l10n(
                            "configuration-screen-custom-options-file-parse-error",
                            {
                              errorDescription:
                                err instanceof Error ? err.message : String(err)
                            }
                          )
                        );
                      });
                    }
                  }}
                >
                  {l10n("configuration-screen-custom-options-file-action")}
                </PrimaryActionButton>
                <FluidSmallParagraph class="mx-auto max-w-prose text-center">
                  {l10n(
                    "configuration-screen-custom-options-file-action-description"
                  )}
                </FluidSmallParagraph>
              </>
            }
          >
            <OptionsEditor
              optionsSetter={(optionsObject) =>
                (packSquashOptions = optionsObject)
              }
              nextViewRoute={nextViewRoute}
            />
          </Show>
        </div>
      </ViewContainer>
    </>
  );
};
