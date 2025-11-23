/** @type {import('tailwindcss').Config} */
module.exports = {
	content: [
		"./src/**/*.rs",
		"./assets/**/*.html",
	],
	theme: {
		extend: {
			colors: {
				editor: {
					bg: '#1e1e1e',
					panel: '#252526',
					border: '#3e3e42',
					text: '#cccccc',
					accent: '#007acc',
				}
			}
		},
	},
	plugins: [],
}
