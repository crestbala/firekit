

export const index = 3;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/infos/_page.svelte.js')).default;
export const imports = ["_app/immutable/nodes/3.CqkJlaig.js","_app/immutable/chunks/CILNjB47.js"];
export const stylesheets = [];
export const fonts = [];
