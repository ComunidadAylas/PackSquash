import { A } from "@solidjs/router";
import { Title } from "@solidjs/meta";
import { useI18n } from "../contexts/i18n";
import { HandWaving } from "phosphor-solid";
import ViewContainer from "../components/ViewContainer";
import LocaleSelector from "../components/LocaleSelector";
import DiscordButton from "../components/DiscordButton";
import GitHubRepositoryButton from "../components/GitHubRepositoryButton";
import KofiButton from "../components/KofiButton";
import PackSquashLogo from "../components/PackSquashLogo";
import PrimaryActionNavigateButton from "../components/PrimaryActionNavigateButton";
import FluidTitleHeader from "../components/FluidTitleHeader";
import FluidSubtitleHeader from "../components/FluidSubtitleHeader";

export default () => {
  const [l10n] = useI18n();

  return (
    <>
      <Title>PackSquash · {l10n("home-screen-title")}</Title>

      <ViewContainer>
        <aside class="flex justify-between gap-2">
          <LocaleSelector />

          <div class="flex justify-evenly gap-2">
            <DiscordButton />
            <GitHubRepositoryButton />
            <KofiButton />
          </div>
        </aside>

        <div class="container mx-auto flex min-h-0 grow flex-col justify-center gap-3 break-words text-center">
          <PackSquashLogo />

          <FluidTitleHeader>
            {l10n("home-screen-welcome")} <HandWaving class="inline" />
          </FluidTitleHeader>
          <FluidSubtitleHeader>
            {l10n("home-screen-landing-text")}
          </FluidSubtitleHeader>

          <PrimaryActionNavigateButton
            class="mx-auto mt-2"
            route="/packSelection"
          >
            {l10n("home-screen-squash-action")}
          </PrimaryActionNavigateButton>

          <div class="mx-auto text-lg text-zinc-700 drop-shadow-sm transition hover:text-zinc-800">
            <A href="/about">{l10n("home-screen-about-action")}</A>
          </div>
        </div>
      </ViewContainer>
    </>
  );
};
