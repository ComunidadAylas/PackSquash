import { open } from "@tauri-apps/api/shell";
import { DiscordLogo } from "phosphor-solid";
import { useI18n } from "../contexts/i18n";
import { tippy as solid_tippy } from "solid-tippy";

// Hack to make the TypeScript compiler happy with the use:tippy directive
// eslint-disable-next-line @typescript-eslint/no-unused-vars
const tippy = solid_tippy;

export default () => {
  const [, l10n] = useI18n();

  const openDiscordTooltipText = l10n("open-project-discord");

  return (
    <button
      class="h-12 w-12 rounded-full bg-[#5865f2] drop-shadow-md hover:scale-110"
      onClick={() =>
        open("https://packsquash.page.link/Discord-server-invite-link")
      }
      use:tippy={{
        hidden: true,
        props: {
          content: openDiscordTooltipText,
          placement: "bottom",
          animation: "scale-subtle"
        }
      }}
    >
      <DiscordLogo class="w-full p-2 text-zinc-50" size="auto" />
    </button>
  );
};
