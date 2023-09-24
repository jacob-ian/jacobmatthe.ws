/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["src/**/*.rs"],
  theme: {
    fontFamily: {
      mono: ['JetBrains Mono', 'mono']
    },
    extend: {},
  },
  plugins: [require('@tailwindcss/typography')],
}
