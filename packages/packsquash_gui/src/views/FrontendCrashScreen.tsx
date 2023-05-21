import { Title } from "@solidjs/meta";
import { useI18n } from "../contexts/i18n";
import ViewContainer from "../components/ViewContainer";
import PrimaryActionButton from "../components/PrimaryActionButton";
import { CircleWavyWarning } from "phosphor-solid";

export default (props: { throwable: unknown; reset: () => void }) => {
  const [l10n] = useI18n();

  let throwableDescription;
  if (props.throwable instanceof Error) {
    // Production builds are bundled and minified, so stack traces are less useful there
    throwableDescription = import.meta.env.PROD
      ? props.throwable.message
      : `${props.throwable.message}\n\n${
          props.throwable.stack || "Stack trace unavailable"
        }`;
  } else {
    throwableDescription = String(props.throwable);
  }

  // The browser devtools allow for comfortable stack trace visualization and navigation
  // when logging instances of Error, so don't suppress them
  console.error(props.throwable);

  return (
    <>
      <Title>PackSquash · {l10n("crash-screen-title")}</Title>

      <ViewContainer>
        <div class="container mx-auto flex min-h-0 grow flex-col justify-center gap-3 break-words text-center">
          <CircleWavyWarning class="max-h-48 min-h-0" size="auto" />

          <h1 class="text-[7vmin] text-lg font-bold drop-shadow-md sm:text-xl md:text-3xl lg:text-4xl">
            {l10n("crash-screen-header")}
          </h1>

          <p class="whitespace-pre-line text-[4vmin] drop-shadow-md sm:text-base">
            {l10n("crash-screen-error-details-text")}
          </p>
          <samp
            class="select-text overflow-auto whitespace-pre-line text-[4vmin] sm:text-base"
            lang="en"
            dir="ltr"
          >
            {throwableDescription}
          </samp>

          <PrimaryActionButton class="mx-auto mt-2" onClick={props.reset}>
            {l10n("crash-screen-continue-action")}
          </PrimaryActionButton>
        </div>
      </ViewContainer>
    </>
  );
};
