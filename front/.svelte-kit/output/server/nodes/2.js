

export const index = 2;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_page.svelte.js')).default;
export const imports = ["_app/immutable/nodes/2.B5jRHV-6.js","_app/immutable/chunks/CILNjB47.js","_app/immutable/chunks/C45B6edQ.js"];
export const stylesheets = ["_app/immutable/assets/_page.Cu-pCCvH.css","_app/immutable/assets/2.DLzuhIBy.css"];
export const fonts = [];
