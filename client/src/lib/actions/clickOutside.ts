// Types
import type { Action } from "svelte/action";

export const clickOutside: Action<HTMLElement, () => void> = (node, cb) => {
	function handleClick(event: MouseEvent) {
		const target = event.target as HTMLElement;

		if (!node.contains(target)) {
			event.stopPropagation();
			cb();
		}
	}

	document.addEventListener("click", handleClick, true);

	return {
		destroy() {
			document.removeEventListener("click", handleClick, true);
		},
	};
};
