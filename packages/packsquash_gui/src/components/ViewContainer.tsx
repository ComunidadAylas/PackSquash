import { ComponentProps, splitProps } from "solid-js";

export default (
  props: ComponentProps<"div"> & { fitContentHeight?: boolean }
) => {
  const [componentProps, divProps] = splitProps(props, [
    "class",
    "fitContentHeight"
  ]);

  return (
    <div
      class="absolute inset-0 flex flex-col gap-2 overflow-auto p-2 sm:p-4"
      classList={{ "min-h-fit": componentProps.fitContentHeight }}
      {...divProps}
    >
      {divProps.children}
    </div>
  );
};
