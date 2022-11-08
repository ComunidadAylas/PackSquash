import { ParentProps } from "solid-js/types/render/component";

export default (props: ParentProps<{ columns: number }>) => {
  return (
    <div
      class={`grid justify-center gap-3`}
      style={{
        "grid-template-columns": `repeat(${props.columns},auto)`
      }}
    >
      {props.children}
    </div>
  );
};
