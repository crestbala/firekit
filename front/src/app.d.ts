// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		interface PageState {
			pokemon: string;
		}
		// interface Platform {}
	}

	interface ImportMetaEnv {
		VITE_API_URL: string;
	}
}

export {};
