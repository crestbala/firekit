<script lang="ts">
	// Styles
	import "../app.css";

	// Components
	import Footer from "$lib/components/Footer.svelte";

	// Props
	type Props = {
		children: Snippet<[]>;
	};
	let { children }: Props = $props();

	// Components
	import Nav from "$lib/components/Nav.svelte";
	import type { Snippet } from "svelte";
</script>

<div class="min-h-screen flex flex-col">
	<Nav />
	<div class="flex flex-1 justify-center items-start">
		{@render children()}
	</div>
	<Footer />
</div>
