// Silence IntelliJ false-positive warning for unused export
// noinspection JSUnusedGlobalSymbols

import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
import { createHtmlPlugin } from "vite-plugin-html";

export default defineConfig({
  plugins: [
    solidPlugin(),
    createHtmlPlugin({
      minify: true
    })
  ],
  server: {
    port: 3000
  },
  build: {
    target: "esnext",
    rollupOptions: {
      output: {
        // Shorten file names to save some space on the final executable
        chunkFileNames: "[hash].js",
        assetFileNames: "[hash].[ext]"
      }
    }
  },
  esbuild: {
    // On production builds, strip out console logging and debugger instructions
    drop: ["debugger"],
    pure: [
      "console.log",
      "console.error",
      "console.warn",
      "console.info",
      "console.debug",
      "console.trace",
      "console.assert"
    ]
  },
  assetsInclude: ["**/*.xm", "**/*.ftl"]
});
