<script lang="ts">
	// Svelte
	import { onMount } from "svelte";
	import { page } from "$app/state";

	// Utils
	import { capitalize } from "$lib/utils/capitalise";

	// Props
	type Props = {
		pokemonName: string;
	};
	let { pokemonName }: Props = $props();

	// Variables
	const slug = page.params.slug;
	const apiUrl = (path: string) => `${import.meta.env.VITE_API_URL || ""}${path}`;
	let searchPokemonPromise: Promise<any> | null = $state(null);
	let pokemonViaApi: Promise<any> | null = $state(null);

	onMount(() => {
		searchPokemonPromise = searchPokemon(slug || pokemonName);
	});

	async function searchPokemon(name: string) {
		const query = name.toLowerCase();
		const url = apiUrl(`/api/search?id_or_name=${encodeURIComponent(query)}`);
		const response = await fetch(url);
		if (!response.ok) throw new Error(`Search failed: ${response.statusText}`);
		const pokemon = await response.json();
		return pokemon;
	}

	function formatMeasurements(value: number, type: "weight" | "height"): string {
		return type === "weight" ? `${(value / 10).toFixed(1)} kg` : `${value * 10} cm`;
	}
</script>

<!-- Main container -->
<div class="flex flex-col w-full justify-start items-center h-full">
	{#if searchPokemonPromise}
		{#await searchPokemonPromise}
			<div class="h-screen w-full flex flex-col justify-center items-center">
				<img src="/icons/loading.svg" alt="Loading..." class="animate-spin" height="24" width="24" />
			</div>
		{:then pokemon}
			<div class="flex flex-col justify-start items-center w-full h-full">
				<!-- Header -->
				<div
					class="relative flex flex-col justify-center items-center py-12 w-full"
					style={`background-color: rgb(from ${pokemon.color === "white" ? "#6b7280" : pokemon.color} r g b / 0.1);`}
				>
					<img src={pokemon.image_url} alt={pokemon.name} height="256" width="256" style="image-rendering: pixelated" />
					<div class="flex flex-col gap-3 justify-center items-center">
						<p class="text-3xl font-bold">{capitalize(pokemon.name)}</p>
						<div class="flex flex-row gap-2">
							{#each pokemon.types as type}
								<p class="py-1 px-2 rounded-md text-xs bg-slate-500 text-slate-100">
									{capitalize(type.type.name)}
								</p>
							{/each}
						</div>
					</div>
					<p class="absolute top-6 right-6 text-slate-300 text-xl">#{pokemon.id}</p>
				</div>
				<!-- Informations -->
				<div class="flex flex-col justify-start items-center w-full">
					<div class="flex flex-col justify-start items-center h-full w-full max-w-2xl p-4 sm:p-6 gap-3">
						<p class="text-2xl font-serif italic w-full text-left">Informations</p>
						<div class="flex flex-col justify-center items-center w-full gap-6">
							<div class="flex flex-row justify-start items-center w-full gap-1 text-slate-500">
								<p>{formatMeasurements(pokemon.weight, "weight")}</p>
								<p>•</p>
								<p>{formatMeasurements(pokemon.height, "height")}</p>
							</div>
							<div class="w-full flex flex-col justify-center items-start gap-2">
								<p class="font-bold">Abilities</p>
								<div class="flex flex-wrap w-full justify-start items-center gap-2">
									{#each pokemon.abilities as ability, i}
										<p class="py-1 px-3 bg-slate-200 rounded-full">
											{capitalize(ability.ability.name)}
										</p>
									{/each}
								</div>
							</div>
						</div>
					</div>
				</div>
				<!-- Usage -->
				<div class="flex flex-col justify-start items-center w-full">
					<div class="flex flex-col justify-start items-center max-w-2xl w-full p-4 sm:p-6 h-full pt-0 gap-3">
						<p class="text-2xl font-serif italic w-full text-left">Usage</p>
						<div class="flex flex-col justify-start items-center w-full">
							<p class="w-full text-left break-words overflow-hidden">
								Your <a href="/infos" class="underline">Software</a> expose a public API to access to this Pokémon programmatically
								in other apps.
							</p>
							<div class="flex flex-row justify-start items-stretch w-full gap-2 mt-6">
								<p class="p-3 px-4 border rounded-full border-slate-200 bg-white font-mono w-full text-left break-all">
									{`GET /api/search?id_or_name=${pokemon.id}`}
								</p>
								<button
									onclick={() => (pokemonViaApi = searchPokemon(pokemon.id.toString()))}
									class="bg-slate-900 text-white py-2 px-8 font-semibold text-nowrap rounded-full"
								>
									Try it!
								</button>
							</div>
							{#if pokemonViaApi}
								<div class="flex flex-col justify-start items-center w-full mt-4">
									{#await pokemonViaApi}
										<img src="/icons/loading.svg" alt="Loading..." class="animate-spin" height="24" width="24" />
									{:then pokemonViaApi}
										<pre
											class="p-3 rounded-xl bg-slate-900 text-slate-100 font-mono w-full text-left break-all overflow-x-auto">{JSON.stringify(
												pokemonViaApi,
												null,
												2
											)}</pre>
									{:catch error}
										<p>Error: {error.message}</p>
									{/await}
								</div>
							{/if}
						</div>
					</div>
				</div>
			</div>
		{:catch error}
			<p class="w-full text-center text-red-600">Error: {error.message}</p>
		{/await}
	{/if}
</div>
