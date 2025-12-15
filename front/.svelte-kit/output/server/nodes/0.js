import * as universal from '../entries/pages/_layout.ts.js';

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export { universal };
export const universal_id = "src/routes/+layout.ts";
export const imports = ["_app/immutable/nodes/0.C-4f_k6U.js","_app/immutable/chunks/CILNjB47.js"];
export const stylesheets = ["_app/immutable/assets/0.Bc30h-th.css"];
export const fonts = [];
