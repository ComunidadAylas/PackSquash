import { open } from "@tauri-apps/api/shell";
import KofiLogo from "./KofiLogo";
import { useI18n } from "../contexts/i18n";
import { tippy as solid_tippy } from "solid-tippy";

// Hack to make the TypeScript compiler happy with the use:tippy directive
// eslint-disable-next-line @typescript-eslint/no-unused-vars
const tippy = solid_tippy;

export default () => {
  const [, l10n] = useI18n();

  const openKofiTooltipText = l10n("open-project-kofi");

  return (
    <button
      class="h-12 w-12 rounded-full drop-shadow-md hover:scale-110"
      onClick={() => open("https://packsquash.page.link/Ko-fi")}
      use:tippy={{
        hidden: true,
        props: {
          content: openKofiTooltipText,
          placement: "bottom",
          animation: "scale-subtle"
        }
      }}
    >
      <KofiLogo />
    </button>
  );
};
