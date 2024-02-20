/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    fontFamily: {
      'sans': ['Fira Mono', 'ui-monospace', 'SFMono-Regular']
    },
    extend: {},
  },
  plugins: [],
}

