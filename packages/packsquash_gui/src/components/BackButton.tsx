import { CaretLeft } from "phosphor-solid";
import { useI18n } from "../contexts/i18n";
import { tippy as solid_tippy } from "solid-tippy";
import { A } from "@solidjs/router";

// Hack to make the TypeScript compiler happy with the use:tippy directive
// eslint-disable-next-line @typescript-eslint/no-unused-vars
const tippy = solid_tippy;

interface BaseComponentProps {
  class?: string;
}

interface RouteBackButton extends BaseComponentProps {
  route: string;
  backAction?: never;
}

interface ActionBackButton extends BaseComponentProps {
  route?: never;
  backAction: () => void;
}

type ComponentProps = RouteBackButton | ActionBackButton;

export default (props: ComponentProps) => {
  const [, l10n] = useI18n();

  const goBackTooltipText = l10n("go-back");

  const backIcon = (
    <CaretLeft
      class={
        "w-full rounded-full p-2 text-zinc-700 hover:text-zinc-800" +
        (props.class !== undefined ? ` ${props.class}` : "")
      }
      size="auto"
    />
  );

  const routerLink = props.route ? (
    <A href={props.route} tabIndex={-1} role="presentation">
      {backIcon}
    </A>
  ) : undefined;

  return (
    <button
      class="h-12 w-12 drop-shadow-md hover:scale-110"
      use:tippy={{
        hidden: true,
        props: {
          content: goBackTooltipText,
          placement: "bottom",
          animation: "scale-subtle"
        }
      }}
      onClick={
        routerLink
          ? () => (routerLink as HTMLElement).click()
          : props.backAction
      }
    >
      {routerLink ?? backIcon}
    </button>
  );
};
