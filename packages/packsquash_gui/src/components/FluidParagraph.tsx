import { ParentProps } from "solid-js/types/render/component";

export default (props: ParentProps<{ class?: string }>) => {
  return (
    <p
      class={
        (props.class ? `${props.class} ` : "") +
        "text-[max(3vmin,10pt)] text-zinc-800 drop-shadow-md sm:text-base"
      }
    >
      {props.children}
    </p>
  );
};
