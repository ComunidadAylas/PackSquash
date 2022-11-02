import { ComponentProps, splitProps } from "solid-js";
import PrimaryActionButton from "./PrimaryActionButton";
import { A } from "@solidjs/router";

export default (
  props: ComponentProps<"button"> & {
    route: string;
  }
) => {
  const [componentProps, buttonProps] = splitProps(props, [
    "children",
    "route",
    "onClick"
  ]);

  const routerLink = (
    <A href={componentProps.route} tabIndex={-1} role="presentation">
      {componentProps.children}
    </A>
  );

  return (
    <PrimaryActionButton
      onClick={() => (routerLink as HTMLElement).click()}
      {...buttonProps}
    >
      {routerLink}
    </PrimaryActionButton>
  );
};
