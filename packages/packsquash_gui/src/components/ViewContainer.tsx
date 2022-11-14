import { ComponentProps, splitProps } from "solid-js";

export default (props: ComponentProps<"div">) => {
  const [, divProps] = splitProps(props, ["class"]);

  return (
    <div
      class="absolute inset-0 flex flex-col gap-2 overflow-auto p-2 sm:p-4"
      {...divProps}
    >
      {divProps.children}
    </div>
  );
};
