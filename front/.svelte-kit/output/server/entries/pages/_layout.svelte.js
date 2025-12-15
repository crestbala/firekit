import "clsx";
import { a as attr, s as stringify, b as base } from "../../chunks/vendor.js";
function Footer($$payload) {
  $$payload.out += `<div class="flex flex-col w-full justify-center items-center gap-4 mt-24 mb-4 text-slate-500"><div class="flex flex-row gap-1 justify-center items-center"><p>Demo App</p> <p>•</p> <span class="flex flex-row gap-1 justify-center items-center"><p>Made by</p> <a href="https://hugoduprez.com" target="_blank" class="underline">Hugo</a></span></div> <p>Improved and Modified by <a href="https://www.linkedin.com/in/balachandran-manimegalai-9478242a5/" target="_blank" class="underline">Balachandran Manimegalai</a></p></div>`;
}
function Nav($$payload) {
  $$payload.out += `<nav class="flex items-center justify-center gap-4 w-full p-6 text-slate-500"><a data-sveltekit-reload=""${attr("href", `${stringify(base)}/`)}>Home</a> <a${attr("href", `${stringify(base)}/infos`)}>Infos</a></nav>`;
}
function _layout($$payload, $$props) {
  let { children } = $$props;
  $$payload.out += `<div class="min-h-screen flex flex-col">`;
  Nav($$payload);
  $$payload.out += `<!----> <div class="flex flex-1 justify-center items-start">`;
  children($$payload);
  $$payload.out += `<!----></div> `;
  Footer($$payload);
  $$payload.out += `<!----></div>`;
}
export {
  _layout as default
};
