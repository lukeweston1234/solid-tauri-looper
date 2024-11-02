/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        "app-primary": "var(--primary)",
        "app-background": "var(--background)",
        "app-marker": "var(--time-marker)",
      }
    },
  },
  plugins: [],
}