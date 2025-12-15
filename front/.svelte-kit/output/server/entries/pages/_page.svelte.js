import { p as page, a as attr, c as pop, d as push } from "../../chunks/vendor.js";
import _page$1 from "./pokemon/_slug_/_page.svelte.js";
function _page($$payload, $$props) {
  push();
  let searchQuery = "";
  if (page.state.pokemon) {
    $$payload.out += "<!--[-->";
    _page$1($$payload, { pokemonName: page.state.pokemon });
  } else {
    $$payload.out += "<!--[!-->";
  }
  $$payload.out += `<!--]-->   <div class="flex flex-col justify-center items-center px-4 max-w-2xl mx-auto w-full svelte-v998dy"><h1 class="text-3xl font-semibold mt-10 svelte-v998dy">Welcome to your <span class="font-normal italic text-red-600 inline-block hover:scale-110 hover:rotate-2 transition-transform duration-200 svelte-v998dy">Software</span></h1> <form class="flex justify-center items-center flex-row gap-2 w-full mt-10 text-base svelte-v998dy"><input type="text" name="q" placeholder="Pikachu"${attr("value", searchQuery)} class="border border-slate-200 p-2 px-4 w-full focus:outline-pink-600 focus:ring-2 focus:ring-pink-200 rounded-full transition-all duration-200 focus:scale-[1.02] svelte-v998dy"/> <button type="submit" class="bg-slate-900 text-white py-2 px-8 font-semibold rounded-full hover:bg-slate-800 hover:scale-105 active:scale-95 transition-all duration-200 hover:shadow-lg svelte-v998dy">Search</button></form> <div class="mt-6 pb-6 flex w-full justify-center items-center flex-col svelte-v998dy">`;
  {
    $$payload.out += "<!--[!-->";
  }
  $$payload.out += `<!--]--></div></div>`;
  pop();
}
export {
  _page as default
};
