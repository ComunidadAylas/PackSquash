import { Title } from "@solidjs/meta";
import { invoke } from "@tauri-apps/api";
import ViewContainer from "../components/ViewContainer";
import PackSquashLogo from "../components/PackSquashLogo";
import { useI18n } from "../contexts/i18n";
import BackButton from "../components/BackButton";
import { CaretDoubleDown } from "phosphor-solid";
import { onCleanup, onMount } from "solid-js";
import { renderDependencyListItems } from "../util/dependencyList";

const BUILD_VARIABLES = [
  "app_build_version",
  "app_build_date",
  "app_build_target_triple",
  "app_build_profile"
] as const;
type BuildVariable = (typeof BUILD_VARIABLES)[number];

const PLACEHOLDER_BUILD_VARIABLE_VALUE = "MissingIpc.";
const BUILD_DATA: Record<BuildVariable, string> = {
  app_build_date: PLACEHOLDER_BUILD_VARIABLE_VALUE,
  app_build_profile: PLACEHOLDER_BUILD_VARIABLE_VALUE,
  app_build_target_triple: PLACEHOLDER_BUILD_VARIABLE_VALUE,
  app_build_version: PLACEHOLDER_BUILD_VARIABLE_VALUE
};

for (const buildDataVar of BUILD_VARIABLES) {
  try {
    BUILD_DATA[buildDataVar] = await invoke<string>(buildDataVar);
  } catch (error) {
    console.error(`Could not get ${buildDataVar}:`, error);
  }
}

const creditsBlurRadius = 2;
const creditsBlurSpreadRadius = 3; // Should be greater than or equal to creditsBlurRadius

export default () => {
  const [l10n] = useI18n();

  const licenseText = l10n("about-screen-app-license-text");

  // TODO write credits
  // The <ul> must be positioned for the container blur (box shadow) to display as desired
  const credits = (
    <ul
      class="relative -z-10 mx-auto overflow-hidden text-center break-words"
      lang="en"
      dir="ltr"
      style={{
        width: `calc(100% - ${creditsBlurSpreadRadius * 2}rem)`,
        animation: "120s linear infinite credits-roll"
      }}
    >
      <li><h1 class="font-bold text-lg mt-8 mb-8">Third-party libraries</h1></li>
      {renderDependencyListItems()}
    </ul>
  );

  const creditsContainer = (
    <div
      class="relative overflow-hidden"
      style={`box-shadow: inset 0 0 ${creditsBlurRadius}rem ${creditsBlurSpreadRadius}rem var(--color-squashbrown-900)`}
    >
      {credits}
    </div>
  );

  const scrollToCredits = (ev: Event) =>
    (credits as Element).scrollIntoView({
      behavior: ev.type == "resize" ? "auto" : "smooth"
    });

  const creditsFocusIntersectionThreshold = Number.MIN_VALUE;
  const creditsFocusObserver = new IntersectionObserver(
    ([entry]) => {
      if (entry.intersectionRatio > creditsFocusIntersectionThreshold) {
        addEventListener("resize", scrollToCredits);
        (credits as HTMLElement).getAnimations()[0].currentTime = 0;
      } else {
        removeEventListener("resize", scrollToCredits);
      }
    },
    { threshold: creditsFocusIntersectionThreshold }
  );

  onMount(() => {
    creditsFocusObserver.observe(creditsContainer as Element);
  });

  onCleanup(() => {
    creditsFocusObserver.disconnect();
    removeEventListener("resize", scrollToCredits);
  });

  return (
    <>
      <Title>PackSquash · {l10n("about-screen-title")}</Title>

      <ViewContainer>
        <aside class="fixed z-[9998]">
          <BackButton route="/" class="bg-zinc-200" />
        </aside>

        <div class="container mx-auto flex min-h-0 grow flex-col">
          <div class="flex min-h-0 grow flex-col justify-center">
            <PackSquashLogo />
          </div>

          <div class="mt-2 flex flex-col justify-center gap-2 break-words text-center drop-shadow-md">
            <h1
              class="select-text text-lg font-bold sm:text-xl md:text-3xl lg:text-4xl"
              lang="en"
              dir="ltr"
            >
              PackSquash {BUILD_DATA["app_build_version"]}
            </h1>
            <p class="mt-2 font-bold sm:mt-4">
              {l10n("about-screen-app-build-date")}
              <br />
              <samp class="select-text font-normal" lang="en" dir="ltr">
                {BUILD_DATA["app_build_date"]}
              </samp>
            </p>
            <p class="font-bold">
              {l10n("about-screen-app-build-profile")}
              <br />
              <samp class="select-text font-normal" lang="en" dir="ltr">
                {BUILD_DATA["app_build_profile"]}
              </samp>
            </p>
            <p class="font-bold">
              {l10n("about-screen-app-build-target")}
              <br />
              <samp class="select-text font-normal" lang="en" dir="ltr">
                {BUILD_DATA["app_build_target_triple"]}
              </samp>
            </p>
            <p class="font-bold">
              {l10n("about-screen-app-user-agent")}
              <br />
              <samp class="select-text font-normal" lang="en" dir="ltr">
                {navigator.userAgent}
              </samp>
            </p>
            <i class="mx-auto mt-2 max-w-prose whitespace-pre-line sm:mt-4">
              {licenseText()}
            </i>

            <button
              class="mx-auto mt-2 text-lg font-bold transition hover:scale-110 sm:mt-4"
              onClick={scrollToCredits}
            >
              <CaretDoubleDown class="mr-2 inline" />
              {l10n("about-screen-thanks")}
              <CaretDoubleDown class="ml-2 inline" />
            </button>
          </div>

          {/* Move child element outside flexbox. Positioned so w-full in child works */}
          <div class="relative">
            {/* Remove children from the flow, so the parent is zero height */}
            <div class="absolute flex h-screen w-full flex-col justify-center">
              {creditsContainer}
            </div>
          </div>
        </div>
      </ViewContainer>
    </>
  );
};
