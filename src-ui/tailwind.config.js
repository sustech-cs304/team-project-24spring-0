/** @type {import('tailwindcss').Config} */
const {nextui} = require("@nextui-org/react");

module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
    "./node_modules/@nextui-org/theme/dist/**/*.{js,ts,jsx,tsx}",
  ],
  // plugins: [
  //   require("daisyui"),
  // ],
  // daisyui: {
  //   themes: ["light", "dark", "cupcake", "acid", "lofi", 'nord', 'synthwave', 'retro', 'cyberpunk', 'valentine', 'halloween', 'garden', 'forest', 'aqua', 'lofi', 'pastel', 'dracula'],
  // },
  themes: {
    extend: {},
  },
  darkMode: "class",
  plugins: [nextui()],
};
