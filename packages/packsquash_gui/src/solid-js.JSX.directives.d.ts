import { TippyOptions } from "solid-tippy";

// Register custom directives for TypeScript. Ref:
// https://github.com/lxsmnsyc/solid-tippy/issues/1
// https://www.solidjs.com/docs/latest/api#use___
declare module "solid-js" {
  namespace JSX {
    interface Directives {
      tippy: TippyOptions;
    }
  }
}
