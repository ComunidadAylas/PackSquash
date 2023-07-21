import type { Options, Services } from "@wdio/types";
import { spawn, spawnSync, ChildProcess } from "child_process";
import { mkdirSync } from "fs";

const isWindowsHost = process.platform == "win32";

const targettingWindows = process.env.CARGO_BUILD_TARGET
  ? process.env.CARGO_BUILD_TARGET.includes("-windows-")
  : isWindowsHost;

const executableExtension = targettingWindows ? ".exe" : "";

export const config: Options.Testrunner = {
  // On production builds Tauri uses a custom URL scheme
  baseUrl: targettingWindows ? "https://tauri.localhost" : "tauri://localhost",

  // Tauri WebDriver intermediate end listens on the first
  // IPv4 loopback address only, but the default value for
  // this hostname, localhost, may resolve to an IPv6 address
  // on Windows
  hostname: "127.0.0.1",

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
        application: `../../target/${
          process.env.CARGO_BUILD_TARGET || "."
        }/release/packsquash_gui${executableExtension}`
      }
    }
  ],

  reporters: [
    [
      "spec",
      { realtimeReporting: true, addConsoleLogs: true, showPreface: false }
    ]
  ],

  framework: "cucumber",
  cucumberOpts: {
    require: ["test/features/steps/definitions.ts"]
  },

  // Allow relative imports without .js extension in WDIO test code.
  // See: https://stackoverflow.com/questions/62619058/appending-js-extension-on-relative-import-statements-during-typescript-compilat
  execArgv: ["--experimental-specifier-resolution=node"],

  // Save screenshots of failing Cucumber steps for debugging
  afterStep: async (step, scenario, result) => {
    if (!result.passed) {
      const sanitizedScenarioName = sanitizeMaybeReservedFilenameCharacters(
        scenario.name
      );
      const sanitizedStepName = sanitizeMaybeReservedFilenameCharacters(
        step.text
      );

      await mkdirSync(`reports/${sanitizedScenarioName}/failures`, {
        recursive: true
      });
      await browser.saveScreenshot(
        `reports/${sanitizedScenarioName}/failures/${sanitizedStepName}.png`
      );
    }
  },

  // WebdriverIO assumes that the Webdriver remote or intermediate end is running when
  // running tests. In usual web applications, services (see https://webdriver.io/docs/customservices/),
  // which package reusable WDIO hooks, take care of launching the browser-specific driver
  // process, which handles capabilities that target such a browser. However, Tauri doesn't
  // ship with WDIO services to do that, so bring our own to launch the Tauri proxy driver
  // as documented at the following pages:
  // https://tauri.app/v1/guides/testing/webdriver/example/webdriverio/#config
  // https://webdriver.io/docs/customservices
  services: [
    [
      new (class implements Services.ServiceInstance {
        private tauriDriver?: ChildProcess;

        onPrepare() {
          if (process.env.PACKSQUASH_GUI_WDIO_SKIP_PREPARE) {
            return;
          }
          spawnSync(`npm${isWindowsHost ? ".cmd" : ""}`, ["run", "build:staging"], {
            stdio: "inherit"
          });
          spawnSync("cargo", ["build", "--release"], { stdio: "inherit" });
        }

        beforeSession() {
          this.tauriDriver = spawn(
            "tauri-driver",
            process.env.NATIVE_WEBDRIVER_BINARY
              ? ["--native-driver", process.env.NATIVE_WEBDRIVER_BINARY]
              : [],
            {
              stdio: ["ignore", "ignore", "inherit"]
            }
          );
        }

        afterSession() {
          this.tauriDriver?.kill();
        }
      })(),
      {}
    ] as never // Cast necessary due to WDIO's bad TypeScript definitions
  ]
};

function sanitizeMaybeReservedFilenameCharacters(filePath: string) {
  return filePath.replace(/[^a-zA-Z0-9 +-_]/g, "-");
}
