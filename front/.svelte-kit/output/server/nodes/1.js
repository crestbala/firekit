

export const index = 1;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/fallbacks/error.svelte.js')).default;
export const imports = ["_app/immutable/nodes/1.CgV6Wo62.js","_app/immutable/chunks/DNLHdk0b.js"];
export const stylesheets = [];
export const fonts = [];
