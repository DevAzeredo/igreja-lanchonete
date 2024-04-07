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
    extend: {
      colors: {
        'button-100': '#f0f0f0', 'button-200': '#e0e0e0',
        'button-text-700': '#4A5568',
      },
    },
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