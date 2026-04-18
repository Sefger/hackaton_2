import type {Config} from "tailwindcss"

export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      colors: {
        primary: {
          50: "#eff6ff",
          100: "#dbeafe",
          200: "#bfdbfe",
          300: "#93c5fd",
          400: "#60a5fa",
          500: "#3b82f6",
          600: "#2563eb",
          700: "#1d4ed8",
          800: "#1e40af",
          900: "#1e3a8a",
        },
        secondary: {
          50: "#f5f3ff",
          500: "#8b5cf6",
          600: "#7c3aed",
        },
        brand: {
          50: "#f0f9ff",
          100: "#e0f2fe",
          200: "#bae6fd",
          300: "#7dd3fc",
          400: "#38bdf8",
          500: "#0ea5e9",
          600: "#0284c7",
          700: "#0369a1",
          800: "#075985",
          900: "#0c4a6e",
        },
      },
      animation: {
        float: "float 6s ease-in-out infinite",
        shine: "shine 4s linear infinite",
        shoot: "shoot linear infinite",
      },
      keyframes: {
        float: {
          "0%, 100%": {transform: "translateY(0px)"},
          "50%": {transform: "translateY(-15px)"},
        },
        shine: {
          to: {"background-position": "200% center"},
        },
        shoot: {
          "0%": {
            transform: "rotate(215deg) translateX(0) scale(var(--star-scale))",
            opacity: "0",
          },
          "5%": {
            opacity: "1",
          },
          "100%": {
            transform: "rotate(215deg) translateX(-1500px) scale(var(--star-scale))",
            opacity: "0",
          },
        },
      },
      backgroundImage: {
        "glass-gradient": "rgba(10, 10, 18, 0.7)",
      },
    },
  },
  plugins: [],
} satisfies Config
