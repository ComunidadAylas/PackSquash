import { ParentProps } from "solid-js/types/render/component";

export default (props: ParentProps<{ class?: string }>) => {
  return (
    <p
      class={
        (props.class ? `${props.class} ` : "") +
        "text-[max(2.25vmin,8pt)] text-zinc-800 drop-shadow-md sm:text-sm"
      }
    >
      {props.children}
    </p>
  );
};
