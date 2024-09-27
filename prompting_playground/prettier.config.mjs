/** @type {import('prettier').Config & import('prettier-plugin-tailwindcss').options} */
const config = {
  printWidth: 120,
  trailingComma: "es5",
  plugins: ["prettier-plugin-tailwindcss"],
};

export default config;
