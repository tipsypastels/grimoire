/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "selector",
  content: ["./templates/**/*.html"],
  theme: {
    extend: {
      colors: {
        swimsuit: "#2f2bad",
        spraypaint: "#ad2bad",
        sweetness: "#e42692",
        sexercise: "#f71568",
        solidgold: "#f9da4b",
      },
    },
  },
  plugins: [require("@tailwindcss/typography"), require("@tailwindcss/forms")],
};