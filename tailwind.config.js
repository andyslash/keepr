module.exports = {
  content: [
    // include all rust, html and css files in the src directory
    "./src/**/*.{rs,html,css}",
    // include all html files in the output (dist) directory
    "./dist/**/*.html",
  ],

  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
};
