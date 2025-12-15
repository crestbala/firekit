import "clsx";
import { p as page, c as pop, d as push } from "../../../../chunks/vendor.js";
function _page($$payload, $$props) {
  push();
  let { pokemonName } = $$props;
  page.params.slug;
  $$payload.out += `<div class="fixed inset-0 z-50 bg-slate-900/50 backdrop-blur-sm svelte-1gym04"><div class="fixed flex flex-col h-full top-0 right-0 bg-white w-full m:w-[600px] shadow-2xl overflow-y-auto svelte-1gym04"><div class="flex flex-col w-full justify-start items-center h-full svelte-1gym04">`;
  {
    $$payload.out += "<!--[!-->";
  }
  $$payload.out += `<!--]--></div></div></div>`;
  pop();
}
export {
  _page as default
};
