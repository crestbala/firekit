

export const index = 3;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/infos/_page.svelte.js')).default;
export const imports = ["_app/immutable/nodes/3.C05-6yc7.js","_app/immutable/chunks/DNLHdk0b.js"];
export const stylesheets = [];
export const fonts = [];
