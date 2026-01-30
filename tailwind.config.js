/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: '#007ACC',
        'primary-hover': '#005a9e',
        success: '#5a7c5c',
        'success-hover': '#4a6c4c',
        error: '#8b5a5a',
        'error-hover': '#7b4a4a',
        warning: '#FF9800',
        'warning-hover': '#e68900',
      },
      fontFamily: {
        sans: ['Microsoft YaHei', 'sans-serif'],
      },
    },
  },
  plugins: [],
  darkMode: 'class',
}
