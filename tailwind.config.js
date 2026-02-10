/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        // 有机生物亲和设计色板
        primary: {
          DEFAULT: '#228B22', // 森林绿
          50: '#E8F5E8',
          100: '#C8E6C8',
          200: '#A5D6A5',
          300: '#81C784',
          400: '#66BB6A',
          500: '#228B22',
          600: '#1E7B1E',
          700: '#1A6B1A',
          800: '#165B16',
          900: '#124B12',
        },
        secondary: {
          DEFAULT: '#6B7B3C', // 橄榄绿
          50: '#F4F5EE',
          100: '#E5E8D8',
          200: '#CDD3B5',
          300: '#B5BE92',
          400: '#9DA96F',
          500: '#6B7B3C',
          600: '#5D6B34',
          700: '#4F5B2C',
          800: '#414B24',
          900: '#333B1C',
        },
        accent: {
          DEFAULT: '#CA8A04', // 收获金
          50: '#FEF9E7',
          100: '#FDF0C3',
          200: '#FBE59B',
          300: '#F9DA73',
          400: '#F7CF4B',
          500: '#CA8A04',
          600: '#B27B04',
          700: '#9A6C03',
          800: '#825D03',
          900: '#6A4E02',
        },
        cream: {
          DEFAULT: '#F5F0E1', // 柔和奶油色
          50: '#FDFCF8',
          100: '#FAF8F0',
          200: '#F5F0E1',
          300: '#EDE5CF',
          400: '#E5DABD',
          500: '#DDCFAB',
          600: '#C4B38E',
          700: '#AB9771',
          800: '#927B54',
          900: '#795F37',
        },
        // 暗色模式色板
        dark: {
          bg: '#1A1F16',
          surface: '#252B1F',
          border: '#3A4230',
          text: '#E8E6E1',
          muted: '#9CA38B',
        }
      },
      fontFamily: {
        heading: ['Lora', 'Georgia', 'serif'],
        body: ['Raleway', 'system-ui', 'sans-serif'],
      },
      borderRadius: {
        'organic': '16px',
        'organic-lg': '24px',
      },
      boxShadow: {
        'organic': '0 4px 20px -4px rgba(34, 139, 34, 0.15)',
        'organic-lg': '0 8px 30px -6px rgba(34, 139, 34, 0.2)',
        'organic-dark': '0 4px 20px -4px rgba(0, 0, 0, 0.4)',
      },
    },
  },
  plugins: [],
}