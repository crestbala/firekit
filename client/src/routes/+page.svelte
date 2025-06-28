<script lang="ts">
	// Svelte
	import { onMount } from "svelte";
	import { page } from "$app/state";
	import { pushState } from "$app/navigation";
	import { slide } from "svelte/transition";

	// Utils
	import { capitalize } from "$lib/utils/capitalise";

	// Pages
	import PokemonPage from "./pokemon/[slug]/+page.svelte";

	// Actions
	import { clickOutside } from "$lib/actions/clickOutside";

	// Variables
	const apiUrl = (path: string) => `${import.meta.env.VITE_API_URL || ""}${path}`;
	let searchQuery = $state("");
	let searchPokemonPromise: Promise<any> | null = $state(null);
	let getRandomPokemonsPromise: Promise<any> | null = $state(null);

	onMount(() => {
		getRandomPokemonsPromise = getRandomPokemons(12);
	});

	async function searchPokemon(query: string) {
		const url = apiUrl(`/api/search?id_or_name=${encodeURIComponent(query.toLowerCase())}`);
		const response = await fetch(url);
		if (!response.ok) throw new Error(`Search failed: ${response.statusText}`);
		const pokemon = await response.json();
		return pokemon;
	}

	async function getRandomPokemons(n: number) {
		const url = apiUrl(`/api/random?n=${n}`);
		const response = await fetch(url);
		const pokemons = await response.json();
		return pokemons;
	}

	async function handleSearch(event: SubmitEvent) {
		event.preventDefault();
		if (!searchQuery.trim()) return;
		searchPokemonPromise = searchPokemon(searchQuery);
	}
</script>

<svelte:window
	onkeydown={(e) => {
		if (e.key === "Escape" && page.state.pokemon) history.back();
	}}
/>

<!-- Modal -->
{#if page.state.pokemon}
	<div class="fixed inset-0 z-50 bg-slate-900/50">
		<div
			in:slide={{ duration: 200, axis: "x" }}
			use:clickOutside={() => history.back()}
			class="fixed flex flex-col h-full top-0 right-0 bg-white w-[600px] shadow-lg overflow-y-auto"
		>
			<PokemonPage pokemonName={page.state.pokemon} />
		</div>
	</div>
{/if}

<!-- React folks, you CAN have multiple components in a single file -->
{#snippet PokemonCard(pokemon: any)}
	<div class="group flex flex-col h-full w-full max-w-40">
		<a
			onclick={async (e) => {
				if (
					innerWidth < 640 || // Just navigate on mobile
					e.shiftKey || // or the link is opened in a new window
					e.metaKey ||
					e.ctrlKey // or a new tab (mac: metaKey, win/linux: ctrlKey)
				)
					return;

				// prevent navigation for shallow routing
				e.preventDefault();

				const { href } = e.currentTarget;
				pushState(href, { pokemon: pokemon.name });
			}}
			href="/pokemon/{pokemon.name}"
			class="rounded-xl flex z-10 flex-col justify-center items-center p-4 sm:hover:ring-2 sm:group-hover:shadow-lg sm:group-hover:ring-offset-2 sm:group-hover:ring-offset-gray-50 sm:group-hover:ring-gray-300 sm:group-hover:-translate-y-2 sm:group-hover:-translate-x-2 transition-all duration-200"
			style={`background-color: rgb(from ${pokemon.color === "white" ? "#6b7280" : pokemon.color} r g b / 0.1);`}
		>
			<img
				src={pokemon.image_url}
				alt={capitalize(pokemon.name)}
				height="128"
				width="128"
				style="image-rendering: pixelated"
			/>
			<p class="max-w-full truncate">{capitalize(pokemon.name)}</p>
		</a>
	</div>
{/snippet}

{#snippet LoadingCard()}
	<div class="bg-slate-100 w-full max-w-40 rounded-xl flex flex-col justify-center items-center p-4">
		<div class="w-full max-w-32 aspect-square rounded-lg bg-slate-200 animate-pulse"></div>
		<div class="h-3 w-full rounded-lg bg-slate-200 animate-pulse mt-2"></div>
	</div>
{/snippet}

<!-- Main container -->
<div class="flex flex-col justify-center items-center px-4 max-w-2xl mx-auto w-full">
	<!-- Title -->
	<h1 class="text-3xl font-semibold mt-10">
		Welcome to your <span class="font-serif font-normal italic text-red-600">Software</span>
	</h1>

	<!-- Search form -->
	<form onsubmit={handleSearch} class="flex justify-center items-center flex-row gap-2 w-full mt-10 text-base">
		<input
			type="text"
			name="q"
			placeholder="Pikachu"
			bind:value={searchQuery}
			class="border border-slate-200 p-2 px-4 w-full focus:outline-pink-600 rounded-full"
		/>
		<button type="submit" class="bg-slate-900 text-white py-2 px-8 font-semibold rounded-full">Search</button>
	</form>

	<!-- Search results -->
	<div class="mt-6 pb-6 flex w-full justify-center items-center flex-col">
		{#if searchPokemonPromise}
			<div class="max-w-xl w-full flex justify-center items-center">
				{#await searchPokemonPromise}
					{@render LoadingCard()}
				{:then pokemon}
					{@render PokemonCard(pokemon)}
				{:catch error}
					<p class="w-full text-center text-red-600">Error: {error.message}</p>
				{/await}
			</div>
		{:else if getRandomPokemonsPromise}
			{#await getRandomPokemonsPromise}
				<div class="grid grid-cols-2 sm:grid-cols-4 gap-3 w-full">
					{#each Array(12) as _}
						{@render LoadingCard()}
					{/each}
				</div>
			{:then pokemons}
				<div class="grid grid-cols-2 sm:grid-cols-4 gap-3 w-full">
					{#each pokemons as pokemon}
						{@render PokemonCard(pokemon)}
					{/each}
				</div>
			{:catch error}
				<p class="w-full text-center text-red-600">Error: {error.message}</p>
			{/await}
		{/if}
	</div>
</div>
