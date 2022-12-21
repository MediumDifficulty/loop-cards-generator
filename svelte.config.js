import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess'

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: preprocess({
		replace: [
			[/process\.env\.NODE_ENV/g, JSON.stringify(process.env.NODE_ENV)]
		],
		postcss: true
	}),

	kit: {
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: null,
			precompress: false,
			strict: true
		}),
		paths: {
			base: process.env.NODE_ENV === "production" ? "/loop-cards-generator" : ""
		},
	},
};

export default config;
