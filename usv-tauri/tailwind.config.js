/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.vue"],
  theme: {
    extend: {
      colors: {
        "foreground" : "#191b22"
      }
    },
  },
  plugins: [require("daisyui")],
}

