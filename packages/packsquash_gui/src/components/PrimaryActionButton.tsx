import { ComponentProps, splitProps } from "solid-js";

export default (props: ComponentProps<"button">) => {
  const [componentProps, buttonProps] = splitProps(props, [
    "class",
    "children"
  ]);

  return (
    <button
      class={
        "rounded-lg bg-packblue-900 p-2 text-[4vmin] font-bold text-zinc-50 drop-shadow-md transition hover:scale-105 hover:bg-packblue-100 sm:p-3 sm:text-lg lg:text-xl" +
        (componentProps.class ? ` ${componentProps.class}` : "")
      }
      {...buttonProps}
    >
      {componentProps.children}
    </button>
  );
};
