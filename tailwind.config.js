/** @type {import('tailwindcss').Config} */

/**
 * Tailwind CSS配置文件
 * 定义Steam风格主题变量、颜色系统、动画
 */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}"
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        // Steam深色主题
        steam: {
          bg: {
            primary: '#171a21',
            secondary: '#1b2838',
            tertiary: '#2a475e',
            quaternary: '#101822'
          },
          text: {
            primary: '#c7d5e0',
            secondary: '#8f98a0',
            muted: '#5c6b7a'
          },
          accent: {
            blue: '#1b9fff',
            green: '#66c0f4',
            hover: '#2a475e'
          },
          border: {
            DEFAULT: '#3d5a72',
            light: '#4a6b87'
          }
        },
        // Steam浅色主题
        'steam-light': {
          bg: {
            primary: '#f0f2f5',
            secondary: '#e8ebf0',
            tertiary: '#ffffff',
            quaternary: '#dce1e8'
          },
          text: {
            primary: '#1b2838',
            secondary: '#4a5568',
            muted: '#718096'
          },
          accent: {
            blue: '#1b9fff',
            green: '#66c0f4',
            hover: '#dce1e8'
          },
          border: {
            DEFAULT: '#cbd5e0',
            light: '#e2e8f0'
          }
        }
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Roboto', 'sans-serif']
      },
      fontSize: {
        'xs': ['12px', { lineHeight: '1.5' }],
        'sm': ['14px', { lineHeight: '1.5' }],
        'base': ['14px', { lineHeight: '1.5' }],
        'lg': ['16px', { lineHeight: '1.5' }],
        'xl': ['18px', { lineHeight: '1.4' }],
        '2xl': ['24px', { lineHeight: '1.3' }]
      },
      spacing: {
        '18': '4.5rem',
        '22': '5.5rem'
      },
      borderRadius: {
        'sm': '4px',
        'DEFAULT': '8px',
        'lg': '12px'
      },
      boxShadow: {
        'steam': '0 4px 12px rgba(0, 0, 0, 0.15)',
        'steam-lg': '0 8px 24px rgba(0, 0, 0, 0.2)'
      },
      transitionDuration: {
        'steam': '150ms',
        'steam-slow': '300ms'
      },
      transitionTimingFunction: {
        'steam': 'ease-out'
      },
      animation: {
        'fade-in': 'fadeIn 0.3s ease-out',
        'slide-down': 'slideDown 0.2s ease-out',
        'scale-in': 'scaleIn 0.2s ease-out'
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' }
        },
        slideDown: {
          '0%': { opacity: '0', transform: 'translateY(-8px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' }
        },
        scaleIn: {
          '0%': { opacity: '0', transform: 'scale(0.95)' },
          '100%': { opacity: '1', transform: 'scale(1)' }
        }
      }
    }
  },
  plugins: []
}
