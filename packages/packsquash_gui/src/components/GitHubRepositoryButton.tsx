import { open } from "@tauri-apps/api/shell";
import { GithubLogo } from "phosphor-solid";
import { useI18n } from "../contexts/i18n";
import { tippy as solid_tippy } from "solid-tippy";

// Hack to make the TypeScript compiler happy with the use:tippy directive
// eslint-disable-next-line @typescript-eslint/no-unused-vars
const tippy = solid_tippy;

export default () => {
  const [, l10n] = useI18n();

  const openGitHubRepoTooltipText = l10n("open-project-github-repo");

  return (
    <button
      class="h-12 w-12 rounded-full bg-zinc-900 drop-shadow-md hover:scale-110"
      onClick={() =>
        open("https://packsquash.page.link/Main-project-repository")
      }
      use:tippy={{
        hidden: true,
        props: {
          content: openGitHubRepoTooltipText,
          placement: "bottom",
          animation: "scale-subtle"
        }
      }}
    >
      <GithubLogo class="p-2 text-zinc-50" size="100%" />
    </button>
  );
};
