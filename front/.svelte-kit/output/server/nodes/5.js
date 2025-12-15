import * as universal from '../entries/pages/pokemon/_slug_/_page.ts.js';

export const index = 5;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/pokemon/_slug_/_page.svelte.js')).default;
export { universal };
export const universal_id = "src/routes/pokemon/[slug]/+page.ts";
export const imports = ["_app/immutable/nodes/5.DXFcqjVv.js","_app/immutable/chunks/0C8E2mPE.js","_app/immutable/chunks/C1W_HtSc.js"];
export const stylesheets = ["_app/immutable/assets/_page.Cu-pCCvH.css"];
export const fonts = [];
