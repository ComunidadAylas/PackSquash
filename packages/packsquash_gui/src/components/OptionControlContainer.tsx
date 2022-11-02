import { useI18n } from "../contexts/i18n";
import { JSXElement } from "solid-js";

export default (props: {
  optionName: string;
  children: (labelId: string) => JSXElement;
  disableChildrenClickEvents?: boolean;
}) => {
  const [l10n] = useI18n();

  const labelId = `${props.optionName}-label`;
  return (
    <details>
      <summary class="grid cursor-help grid-cols-[min-content_1fr_fit-content(50%)] items-center gap-x-4 break-words">
        <h1 id={labelId} class="min-w-0 text-center font-bold sm:text-left">
          {l10n(`packsquash-option-${props.optionName}-title`)}
        </h1>
        <div
          class="col-span-full mx-auto flex cursor-default items-center sm:col-auto"
          onClick={(event) =>
            props.disableChildrenClickEvents ? event.preventDefault() : void 0
          }
        >
          {props.children(labelId)}
        </div>
      </summary>
      <p class="mt-2 max-w-prose whitespace-pre-line text-sm">
        {l10n(`packsquash-option-${props.optionName}-description`)}
      </p>
    </details>
  );
};
