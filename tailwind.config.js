module.exports = {
  mode: "all",
  content: [
    "./src/fonts/inter.css",
    // include all rust, html and css files in the src directory
    "./src/**/*.{rs,html,css}",
    // include all html files in the output (dist) directory
    "./dist/**/*.html",
  ],
  theme: {
    fontFamily: {
      sans: [
        '"Inter"',
        {
          fontFeatureSettings: '"calt", "ss01", "tnum"',
        },
      ],
    },
  },
  plugins: [],
}