import type { Options } from "@wdio/types";
import { spawn, spawnSync, ChildProcess } from "child_process";

let tauriDriver: ChildProcess;

export const config: Options.Testrunner = {
  // On production builds Tauri uses a custom URL scheme
  baseUrl: "tauri://localhost#",

  // Tests to run
  specs: ["test/features/**/*.feature"],

  // Capabilities to pass to WebDriver remote ends (servers).
  // See: https://w3c.github.io/webdriver/#capabilities
  maxInstances: 1,
  capabilities: [
    {
      browserName: "wry",
      // @ts-expect-error "tauri:options" is a Tauri-specific WebDriver proxy extension capability
      "tauri:options": {
        application: "../../target/release/packsquash_gui"
      }
    }
  ],

  reporters: ["spec"],

  framework: "cucumber",
  cucumberOpts: {
    require: ["test/features/steps/definitions.ts"]
  },

  // Allow relative imports without .js extension in WDIO test code.
  // See: https://stackoverflow.com/questions/62619058/appending-js-extension-on-relative-import-statements-during-typescript-compilat
  execArgv: ["--experimental-specifier-resolution=node"],

  // WebdriverIO assumes that the Webdriver remote or intermediate end is running when
  // running tests. In usual web applications, services (see https://webdriver.io/docs/customservices/),
  // which package reusable WDIO hooks, take care of launching the browser-specific driver
  // process, which handles capabilities that target such a browser. However, Tauri doesn't
  // ship with WDIO services to do that, so bring our own hooks to launch the Tauri proxy
  // driver as documented at
  // https://tauri.app/v1/guides/testing/webdriver/example/webdriverio/#config
  onPrepare: () => {
    if (process.env.PACKSQUASH_GUI_WDIO_SKIP_PREPARE) {
      return;
    }
    spawnSync("npm", ["run", "build:staging"], { stdio: "inherit" });
    spawnSync("cargo", ["build", "--release"], { stdio: "inherit" });
  },
  beforeSession: () =>
    (tauriDriver = spawn("tauri-driver", { stdio: "ignore" })),
  afterSession: () => tauriDriver?.kill()
};