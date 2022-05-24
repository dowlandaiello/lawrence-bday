module.exports = {
	purge: {
		mode: "all",
		content: [
			"./src/**/*.rs",
			"./index.html",
			"./src/**/*.html",
			"./src/**/*.css",
		],
	},
	theme: {
		extend: {
			transitionTimingFunction: {
				"in-expo": "cubic-bezier(0.005, 0.985, 0.035, 0.985)"
			},
			backgroundImage: {
				"trident-pattern": "url('/img/trident.png')",
			}
		},
	},
	variants: {},
	plugins: [],
};
