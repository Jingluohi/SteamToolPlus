/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'class',
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // 深色主题颜色
        dark: {
          primary: '#16213e',
          secondary: '#1a1a2e',
          surface: '#0f3460',
          text: '#ffffff',
          'text-secondary': '#a0a0a0',
        },
        // 浅色主题颜色
        light: {
          primary: '#f8fafc',
          secondary: '#f1f5f9',
          surface: '#e2e8f0',
          text: '#1e293b',
          'text-secondary': '#64748b',
        },
        // 强调色
        accent: {
          blue: '#3b82f6',
          'blue-hover': '#2563eb',
        }
      },
      borderRadius: {
        'card': '12px',
        'button': '8px',
      },
      spacing: {
        'sidebar': '200px',
        'card-gap': '16px',
      },
      transitionDuration: {
        'hover': '150ms',
        'click': '100ms',
        'page': '200ms',
      },
      transitionTimingFunction: {
        'hover': 'ease-out',
      },
    },
  },
  plugins: [],
}
