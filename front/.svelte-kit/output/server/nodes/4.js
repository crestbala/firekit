

export const index = 4;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/pokemon/_page.svelte.js')).default;
export const imports = ["_app/immutable/nodes/4.CViFEVj6.js","_app/immutable/chunks/CILNjB47.js"];
export const stylesheets = [];
export const fonts = [];
