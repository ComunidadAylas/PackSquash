import { Title } from "@solidjs/meta";
import { useI18n } from "../contexts/i18n";
import ViewContainer from "../components/ViewContainer";
import { CircleWavyWarning } from "phosphor-solid";
import FluidTitleHeader from "../components/FluidTitleHeader";
import FluidSubtitleHeader from "../components/FluidSubtitleHeader";
import { lastOptimizationFailure } from "./OptimizationProgress";
import PrimaryActionNavigateButton from "../components/PrimaryActionNavigateButton";
import requestFocusOnMount from "../util/requestFocusOnMount";
import CenteredButtonGrid from "../components/CenteredButtonGrid";
import PrimaryActionButton from "../components/PrimaryActionButton";
import { open } from "@tauri-apps/api/shell";

const previousViewRoute = "/optimizationProgress";

export default () => {
  const [l10n] = useI18n();

  requestFocusOnMount();

  return (
    <>
      <Title>PackSquash · {l10n("optimization-failure-screen-title")}</Title>

      <ViewContainer>
        <div class="container mx-auto flex min-h-0 grow flex-col justify-center gap-3 break-words text-center drop-shadow-md">
          <CircleWavyWarning
            class="max-h-48 min-h-0 grow basis-0 drop-shadow-md"
            size="100%"
          />

          <FluidTitleHeader>
            {l10n("optimization-failure-screen-header")}
          </FluidTitleHeader>

          <FluidSubtitleHeader>
            {l10n("optimization-failure-screen-caption")}
          </FluidSubtitleHeader>

          <samp
            class="select-text overflow-auto whitespace-pre-line break-words text-[4vmin] sm:text-base"
            lang="en"
            dir="ltr"
          >
            {lastOptimizationFailure ?? "unknown"}
          </samp>

          <CenteredButtonGrid columns={2}>
            <PrimaryActionNavigateButton
              class="col-span-full mx-auto sm:col-auto"
              route="/"
            >
              {l10n("optimization-failure-screen-start-over-action")}
            </PrimaryActionNavigateButton>

            <PrimaryActionNavigateButton
              class="col-span-full mx-auto sm:col-auto"
              route={previousViewRoute}
            >
              {l10n("optimization-failure-screen-try-again-action")}
            </PrimaryActionNavigateButton>

            <PrimaryActionButton
              class="col-span-full mx-auto"
              onClick={() =>
                open(
                  "https://packsquash.page.link/Troubleshooting-pack-processing-errors"
                )
              }
            >
              {l10n("optimization-failure-screen-open-online-help-action")}
            </PrimaryActionButton>
          </CenteredButtonGrid>
        </div>
      </ViewContainer>
    </>
  );
};
