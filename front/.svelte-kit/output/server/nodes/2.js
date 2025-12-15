

export const index = 2;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_page.svelte.js')).default;
export const imports = ["_app/immutable/nodes/2.BWnPdnvK.js","_app/immutable/chunks/C1W_HtSc.js","_app/immutable/chunks/0C8E2mPE.js"];
export const stylesheets = ["_app/immutable/assets/_page.Cu-pCCvH.css","_app/immutable/assets/2.DLzuhIBy.css"];
export const fonts = [];
