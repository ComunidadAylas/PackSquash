// eslint-disable-next-line @typescript-eslint/no-var-requires
const colors = require("tailwindcss/colors");

/** @type {import("tailwindcss").Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx,css,md,mdx,html,json,scss}"
  ],
  darkMode: "class",
  theme: {
    colors: {
      transparent: "transparent",
      zinc: colors.zinc,
      squashbrown: {
        900: "rgb(var(--color-squashbrown-900) / <alpha-value>)"
      },
      packblue: {
        100: "#023059",
        900: "#0468bf"
      },
      packteal: {
        100: "#024959",
        600: "#037e99",
        900: "#049dbf"
      },
      "form-control-background": colors.zinc["200"],
      "form-control-placeholder-color": colors.zinc["400"]
    }
  },
  plugins: []
};