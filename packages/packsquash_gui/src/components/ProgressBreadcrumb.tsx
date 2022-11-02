import { Accessor, For } from "solid-js";

export default (props: {
  steps: Iterable<string | Accessor<string>>;
  currentStepIndex?: number;
  class?: string;
}) => {
  return (
    <nav
      class={(props.class ? `${props.class} ` : "") + "drop-shadow-md"}
      aria-label="Breadcrumb"
    >
      <ol class="progress-breadcrumb flex justify-center text-center">
        <For each={Array.from(props.steps)}>
          {(step, i) => (
            <li
              class={
                (i() <= (props.currentStepIndex as number)
                  ? i() == props.currentStepIndex
                    ? "active last-active "
                    : "active "
                  : "") + "relative w-0 shrink grow basis-0"
              }
            >
              {typeof step == "string" ? step : step()}
            </li>
          )}
        </For>
      </ol>
    </nav>
  );
};
