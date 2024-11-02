/** @type {import('tailwindcss').Config} */
module.exports = {
	content: [
		"./index.html",
		"./src/**/*.{html,vue,jsx,js}", // Укажите пути к вашим компонентам
	],
	theme: {
		extend: {},
	},
	plugins: [require('tailwindcss-primeui')],
}

