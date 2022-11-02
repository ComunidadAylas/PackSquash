import { ParentProps } from "solid-js/types/render/component";

export default (props: ParentProps<{ class?: string }>) => {
  return (
    <h2
      class={
        (props.class ? `${props.class} ` : "") +
        "text-[5vmin] font-medium text-zinc-800 drop-shadow-md sm:text-2xl"
      }
    >
      {props.children}
    </h2>
  );
};
