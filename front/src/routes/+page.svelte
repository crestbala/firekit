<script lang="ts">
    // Svelte
    import { onMount } from "svelte";
    import { page } from "$app/state";
    import { pushState } from "$app/navigation";
    import { slide, scale, fly } from "svelte/transition";
    import { cubicOut, elasticOut } from "svelte/easing";

    // Utils
    import { capitalize } from "$lib/utils/capitalise";

    // Pages
    import PokemonPage from "./pokemon/[slug]/+page.svelte";

    // Actions
    import { clickOutside } from "$lib/actions/clickOutside";

    // Variables
    const apiUrl = (path: string) =>
        `${import.meta.env.VITE_API_URL || ""}${path}`;
    let searchQuery = $state("");
    let searchPokemonPromise: Promise<any> | null = $state(null);
    let getRandomPokemonsPromise: Promise<any> | null = $state(null);

    onMount(() => {
        getRandomPokemonsPromise = getRandomPokemons(12);
    });

    async function searchPokemon(query: string) {
        const url = apiUrl(
            `/api/search?id_or_name=${encodeURIComponent(query.toLowerCase())}`,
        );
        const response = await fetch(url);
        if (!response.ok)
            throw new Error(`Search failed: ${response.statusText}`);
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
    <PokemonPage pokemonName={page.state.pokemon} />
{/if}

<!-- React folks, you CAN have multiple components in a single file -->
{#snippet PokemonCard(pokemon: any, index: number = 0)}
    <div
        class="group flex flex-col h-full w-full max-w-40"
        in:fly={{ y: 20, duration: 400, delay: index * 50, easing: cubicOut }}
    >
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
            class="pokemon-card rounded-xl flex z-10 flex-col justify-center items-center p-4 relative overflow-hidden sm:hover:ring-2 sm:group-hover:shadow-2xl sm:group-hover:ring-offset-2 sm:group-hover:ring-offset-gray-50 sm:group-hover:ring-gray-300 sm:group-hover:-translate-y-2 sm:group-hover:-translate-x-2 sm:group-hover:rotate-1 sm:group-hover:scale-105 transition-all duration-300 ease-out"
            style={`background-color: rgb(from ${pokemon.color === "white" ? "#6b7280" : pokemon.color} r g b / 0.1);`}
        >
            <!-- Shine effect on hover -->
            <div
                class="shine-effect absolute inset-0 opacity-0 sm:group-hover:opacity-100 transition-opacity duration-300"
            ></div>

            <img
                src={pokemon.image_url}
                alt={capitalize(pokemon.name)}
                height="160"
                width="160"
                style="image-rendering: pixelated"
                class="relative z-10 sm:group-hover:scale-110 sm:group-hover:rotate-3 transition-transform duration-300 ease-out drop-shadow-lg"
            />
            <p
                class="max-w-full truncate relative z-10 font-medium sm:group-hover:font-semibold transition-all duration-200"
            >
                {capitalize(pokemon.name)}
            </p>
        </a>
    </div>
{/snippet}

{#snippet LoadingCard()}
    <div
        class="bg-slate-100 w-full max-w-40 rounded-xl flex flex-col justify-center items-center p-4"
    >
        <div
            class="w-full max-w-32 aspect-square rounded-lg bg-slate-200 animate-pulse"
        ></div>
        <div
            class="h-3 w-full rounded-lg bg-slate-200 animate-pulse mt-2"
        ></div>
    </div>
{/snippet}

<!-- Main container -->
<div
    class="flex flex-col justify-center items-center px-4 max-w-2xl mx-auto w-full"
>
    <!-- Title -->
    <h1
        class="text-3xl font-semibold mt-10"
        in:fly={{ y: -20, duration: 600, easing: elasticOut }}
    >
        Welcome to your <span
            class="font-normal italic text-red-600 inline-block hover:scale-110 hover:rotate-2 transition-transform duration-200"
            >Software</span
        >
    </h1>

    <!-- Search form -->
    <form
        onsubmit={handleSearch}
        class="flex justify-center items-center flex-row gap-2 w-full mt-10 text-base"
        in:fly={{ y: -20, duration: 600, delay: 100, easing: cubicOut }}
    >
        <input
            type="text"
            name="q"
            placeholder="Pikachu"
            bind:value={searchQuery}
            class="border border-slate-200 p-2 px-4 w-full focus:outline-pink-600 focus:ring-2 focus:ring-pink-200 rounded-full transition-all duration-200 focus:scale-[1.02]"
        />
        <button
            type="submit"
            class="bg-slate-900 text-white py-2 px-8 font-semibold rounded-full hover:bg-slate-800 hover:scale-105 active:scale-95 transition-all duration-200 hover:shadow-lg"
        >
            Search
        </button>
    </form>

    <!-- Search results -->
    <div class="mt-6 pb-6 flex w-full justify-center items-center flex-col">
        {#if searchPokemonPromise}
            <div class="max-w-xl w-full flex justify-center items-center">
                {#await searchPokemonPromise}
                    {@render LoadingCard()}
                {:then pokemon}
                    {@render PokemonCard(pokemon, 0)}
                {:catch error}
                    <p
                        class="w-full text-center text-red-600"
                        in:scale={{ duration: 300 }}
                    >
                        Error: {error.message}
                    </p>
                {/await}
            </div>
        {:else if getRandomPokemonsPromise}
            {#await getRandomPokemonsPromise}
                <div
                    class="grid grid-cols-2 sm:grid-cols-4 gap-3 w-full place-items-center"
                >
                    {#each Array(12) as _}
                        {@render LoadingCard()}
                    {/each}
                </div>
            {:then pokemons}
                <div
                    class="grid grid-cols-2 sm:grid-cols-4 gap-3 w-full place-items-center"
                >
                    {#each pokemons as pokemon, i}
                        {@render PokemonCard(pokemon, i)}
                    {/each}
                </div>
            {:catch error}
                <p
                    class="w-full text-center text-red-600"
                    in:scale={{ duration: 300 }}
                >
                    Error: {error.message}
                </p>
            {/await}
        {/if}
    </div>
</div>

<style>
    .pokemon-card {
        background-image:
            radial-gradient(
                circle at 20% 50%,
                rgba(255, 255, 255, 0.2) 0%,
                transparent 50%
            ),
            radial-gradient(
                circle at 80% 80%,
                rgba(255, 255, 255, 0.1) 0%,
                transparent 50%
            );
    }

    .shine-effect {
        background: linear-gradient(
            135deg,
            transparent 0%,
            transparent 40%,
            rgba(255, 255, 255, 0.6) 50%,
            transparent 60%,
            transparent 100%
        );
        transform: translateX(-100%);
        animation: shine 0.6s ease-in-out;
    }

    @media (min-width: 640px) {
        .group:hover .shine-effect {
            animation: shine 0.6s ease-in-out;
        }
    }

    @keyframes shine {
        0% {
            transform: translateX(-100%) skewX(-15deg);
        }
        100% {
            transform: translateX(200%) skewX(-15deg);
        }
    }

    /* Add subtle floating animation for cards */
    @media (min-width: 640px) {
        .pokemon-card {
            animation: float 6s ease-in-out infinite;
        }

        .pokemon-card:nth-child(2n) {
            animation-delay: 1s;
        }

        .pokemon-card:nth-child(3n) {
            animation-delay: 2s;
        }
    }

    @keyframes float {
        0%,
        100% {
            transform: translateY(0px);
        }
        50% {
            transform: translateY(-4px);
        }
    }
</style>
