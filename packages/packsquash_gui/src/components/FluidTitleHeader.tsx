import { ParentProps } from "solid-js/types/render/component";

export default (props: ParentProps<{ class?: string }>) => {
  return (
    <h1
      class={
        (props.class ? `${props.class} ` : "") +
        "text-[7vmin] font-bold text-zinc-900 drop-shadow-md sm:text-4xl"
      }
    >
      {props.children}
    </h1>
  );
};
