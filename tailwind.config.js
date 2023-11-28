/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        background: "#172425",
        darkGrey: "#2d3131",
        lightGrey: "#4d5250",
        orange: "#fd8d18",
      },
    },
  },
  plugins: [],
};
