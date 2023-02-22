// eslint-disable-next-line @typescript-eslint/no-var-requires
const colors = require("tailwindcss/colors");

/** @type {import("tailwindcss").Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx,css,md,mdx,html,json,scss}"
  ],
  theme: {
    colors: {
      transparent: "transparent",
      zinc: colors.zinc,
      squashbrown: {
        900: "#f2b035"
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
  plugins: [
    // From https://gist.github.com/Merott/d2a19b32db07565e94f10d13d11a8574
    function({ addBase, theme }) {
      function extractColorVars(colorObj, colorGroup = "") {
        return Object.keys(colorObj).reduce((vars, colorKey) => {
          const value = colorObj[colorKey];

          const newVars =
            typeof value === "string"
              ? { [`--color${colorGroup}-${colorKey}`]: value }
              : extractColorVars(value, `-${colorKey}`);

          return { ...vars, ...newVars };
        }, {});
      }

      addBase({
        ":root": extractColorVars(theme("colors"))
      });
    }
  ]
};
