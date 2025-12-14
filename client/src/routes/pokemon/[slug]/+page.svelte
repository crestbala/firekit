<script lang="ts">
    // Svelte
    import { onMount } from "svelte";
    import { page } from "$app/state";
    import { slide, fade, fly, scale } from "svelte/transition";
    import { cubicOut, elasticOut } from "svelte/easing";

    // Utils
    import { capitalize } from "$lib/utils/capitalise";

    // Actions
    import { clickOutside } from "$lib/actions/clickOutside";

    // Props
    type Props = {
        pokemonName: string;
    };
    let { pokemonName }: Props = $props();

    // Variables
    const slug = page.params.slug;
    const apiUrl = (path: string) =>
        `${import.meta.env.VITE_API_URL || ""}${path}`;
    let searchPokemonPromise: Promise<any> | null = $state(null);
    let pokemonViaApi: Promise<any> | null = $state(null);

    onMount(() => {
        searchPokemonPromise = searchPokemon(slug || pokemonName);
    });

    async function searchPokemon(name: string) {
        const query = name.toLowerCase();
        const url = apiUrl(
            `/api/search?id_or_name=${encodeURIComponent(query)}`,
        );
        const response = await fetch(url);
        if (!response.ok)
            throw new Error(`Search failed: ${response.statusText}`);
        const pokemon = await response.json();
        return pokemon;
    }

    function formatMeasurements(
        value: number,
        type: "weight" | "height",
    ): string {
        return type === "weight"
            ? `${(value / 10).toFixed(1)} kg`
            : `${value * 10} cm`;
    }
</script>

<!-- Main container -->
<div
    class="fixed inset-0 z-50 bg-slate-900/50 backdrop-blur-sm"
    in:scale={{ duration: 200 }}
>
    <div
        in:slide={{ duration: 300, axis: "x" }}
        use:clickOutside={() => history.back()}
        class="fixed flex flex-col h-full top-0 right-0 bg-white w-[600px] shadow-2xl overflow-y-auto"
    >
        <div class="flex flex-col w-full justify-start items-center h-full">
            {#if searchPokemonPromise}
                {#await searchPokemonPromise}
                    <div
                        class="h-screen w-full flex flex-col justify-center items-center"
                    >
                        <img
                            src="/icons/loading.svg"
                            alt="Loading..."
                            class="animate-spin"
                            height="24"
                            width="24"
                        />
                    </div>
                {:then pokemon}
                    <div
                        class="flex flex-col justify-start items-center w-full h-full"
                        in:fade={{ duration: 300 }}
                    >
                        <!-- Header -->
                        <div
                            class="relative flex flex-col justify-center items-center py-12 w-full overflow-hidden header-gradient"
                            style={`background-color: rgb(from ${pokemon.color === "white" ? "#6b7280" : pokemon.color} r g b / 0.1);`}
                            in:fly={{ y: -50, duration: 600, easing: cubicOut }}
                        >
                            <!-- Animated background circles -->
                            <div class="floating-circles">
                                <div class="circle circle-1"></div>
                                <div class="circle circle-2"></div>
                                <div class="circle circle-3"></div>
                            </div>

                            <!-- Pokemon Image with float animation -->
                            <div
                                class="pokemon-image-container"
                                in:scale={{
                                    duration: 600,
                                    delay: 200,
                                    easing: elasticOut,
                                }}
                            >
                                <img
                                    src={pokemon.image_url}
                                    alt={pokemon.name}
                                    height="360"
                                    width="360"
                                    style="image-rendering: pixelated"
                                    class="pokemon-image"
                                />
                            </div>

                            <div
                                class="flex flex-col gap-3 justify-center items-center relative z-10"
                                in:fly={{ y: 20, duration: 500, delay: 300 }}
                            >
                                <p class="text-3xl font-bold pokemon-name">
                                    {capitalize(pokemon.name)}
                                </p>
                                <div class="flex flex-row gap-2">
                                    {#each pokemon.types as type, i}
                                        <p
                                            class="py-1 px-2 rounded-md text-xs bg-slate-500 text-slate-100 type-badge"
                                            in:scale={{
                                                duration: 300,
                                                delay: 400 + i * 100,
                                                easing: cubicOut,
                                            }}
                                        >
                                            {capitalize(type.type.name)}
                                        </p>
                                    {/each}
                                </div>
                            </div>
                            <p
                                class="absolute top-6 right-6 text-slate-300 text-xl pokemon-id"
                                in:fade={{ duration: 400, delay: 500 }}
                            >
                                #{pokemon.id}
                            </p>
                        </div>

                        <!-- Informations -->
                        <div
                            class="flex flex-col justify-start items-center w-full"
                            in:fly={{ y: 30, duration: 500, delay: 400 }}
                        >
                            <div
                                class="flex flex-col justify-start items-center h-full w-full max-w-2xl p-4 sm:p-6 gap-3"
                            >
                                <p
                                    class="text-2xl font-serif italic w-full text-left section-title"
                                >
                                    Informations
                                </p>
                                <div
                                    class="flex flex-col justify-center items-center w-full gap-6"
                                >
                                    <div
                                        class="flex flex-row justify-start items-center w-full gap-1 text-slate-500 measurement-text"
                                    >
                                        <p>
                                            {formatMeasurements(
                                                pokemon.weight,
                                                "weight",
                                            )}
                                        </p>
                                        <p class="pulse-dot">•</p>
                                        <p>
                                            {formatMeasurements(
                                                pokemon.height,
                                                "height",
                                            )}
                                        </p>
                                    </div>
                                    <div
                                        class="w-full flex flex-col justify-center items-start gap-2"
                                    >
                                        <p class="font-bold">Abilities</p>
                                        <div
                                            class="flex flex-wrap w-full justify-start items-center gap-2"
                                        >
                                            {#each pokemon.abilities as ability, i}
                                                <p
                                                    class="py-1 px-3 bg-slate-200 rounded-full ability-badge"
                                                    in:scale={{
                                                        duration: 300,
                                                        delay: 500 + i * 80,
                                                        easing: cubicOut,
                                                    }}
                                                >
                                                    {capitalize(
                                                        ability.ability.name,
                                                    )}
                                                </p>
                                            {/each}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <!-- Usage -->
                        <div
                            class="flex flex-col justify-start items-center w-full"
                            in:fly={{ y: 30, duration: 500, delay: 500 }}
                        >
                            <div
                                class="flex flex-col justify-start items-center max-w-2xl w-full p-4 sm:p-6 h-full pt-0 gap-3"
                            >
                                <p
                                    class="text-2xl font-serif italic w-full text-left section-title"
                                >
                                    Usage
                                </p>
                                <div
                                    class="flex flex-col justify-start items-center w-full"
                                >
                                    <p
                                        class="w-full text-left break-words overflow-hidden"
                                    >
                                        Your <a
                                            href="/infos"
                                            class="underline hover:text-pink-600 transition-colors duration-200"
                                            >Software</a
                                        > expose a public API to access to this Pokémon
                                        programmatically in other apps.
                                    </p>
                                    <div
                                        class="flex flex-row justify-start items-stretch w-full gap-2 mt-6"
                                    >
                                        <p
                                            class="p-3 px-4 border rounded-full border-slate-200 bg-white font-mono w-full text-left break-all api-endpoint"
                                        >
                                            {`GET /api/search?id_or_name=${pokemon.id}`}
                                        </p>
                                        <button
                                            onclick={() =>
                                                (pokemonViaApi = searchPokemon(
                                                    pokemon.id.toString(),
                                                ))}
                                            class="bg-slate-900 text-white py-2 px-8 font-semibold text-nowrap rounded-full try-button"
                                        >
                                            Try it!
                                        </button>
                                    </div>
                                    {#if pokemonViaApi}
                                        <div
                                            class="flex flex-col justify-start items-center w-full mt-4"
                                            in:scale={{ duration: 300 }}
                                        >
                                            {#await pokemonViaApi}
                                                <img
                                                    src="/icons/loading.svg"
                                                    alt="Loading..."
                                                    class="animate-spin"
                                                    height="24"
                                                    width="24"
                                                />
                                            {:then pokemonViaApi}
                                                <pre
                                                    class="p-3 rounded-xl bg-slate-900 text-slate-100 font-mono w-full text-left break-all overflow-x-auto code-block"
                                                    in:fly={{
                                                        y: 20,
                                                        duration: 400,
                                                    }}>{JSON.stringify(
                                                        pokemonViaApi,
                                                        null,
                                                        2,
                                                    )}</pre>
                                            {:catch error}
                                                <p
                                                    class="text-red-600"
                                                    in:scale={{ duration: 300 }}
                                                >
                                                    Error: {error.message}
                                                </p>
                                            {/await}
                                        </div>
                                    {/if}
                                </div>
                            </div>
                        </div>
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
</div>

<style>
    /* Header gradient animation */
    .header-gradient {
        position: relative;
        background-image:
            radial-gradient(
                circle at 20% 30%,
                rgba(255, 255, 255, 0.15) 0%,
                transparent 50%
            ),
            radial-gradient(
                circle at 80% 70%,
                rgba(255, 255, 255, 0.1) 0%,
                transparent 50%
            );
        animation: gradientShift 8s ease-in-out infinite;
    }

    @keyframes gradientShift {
        0%,
        100% {
            background-position: 0% 50%;
        }
        50% {
            background-position: 100% 50%;
        }
    }

    /* Floating circles in background */
    .floating-circles {
        position: absolute;
        inset: 0;
        overflow: hidden;
        z-index: 0;
    }

    .circle {
        position: absolute;
        border-radius: 50%;
        background: radial-gradient(
            circle,
            rgba(255, 255, 255, 0.2) 0%,
            transparent 70%
        );
        animation: float-circle 20s ease-in-out infinite;
    }

    .circle-1 {
        width: 200px;
        height: 200px;
        top: -50px;
        left: -50px;
        animation-delay: 0s;
    }

    .circle-2 {
        width: 150px;
        height: 150px;
        bottom: -30px;
        right: -30px;
        animation-delay: 7s;
    }

    .circle-3 {
        width: 100px;
        height: 100px;
        top: 50%;
        right: 10%;
        animation-delay: 14s;
    }

    @keyframes float-circle {
        0%,
        100% {
            transform: translate(0, 0) scale(1);
            opacity: 0.3;
        }
        33% {
            transform: translate(30px, -30px) scale(1.1);
            opacity: 0.5;
        }
        66% {
            transform: translate(-20px, 20px) scale(0.9);
            opacity: 0.4;
        }
    }

    /* Pokemon image float animation */
    .pokemon-image-container {
        position: relative;
        z-index: 5;
    }

    .pokemon-image {
        animation: float-pokemon 4s ease-in-out infinite;
        filter: drop-shadow(0 10px 20px rgba(0, 0, 0, 0.15));
    }

    @keyframes float-pokemon {
        0%,
        100% {
            transform: translateY(0px) rotate(0deg);
        }
        50% {
            transform: translateY(-15px) rotate(2deg);
        }
    }

    /* Pokemon name pulse */
    .pokemon-name {
        animation: subtle-pulse 3s ease-in-out infinite;
    }

    @keyframes subtle-pulse {
        0%,
        100% {
            transform: scale(1);
        }
        50% {
            transform: scale(1.02);
        }
    }

    /* Type badges hover effect */
    .type-badge {
        transition: all 0.3s ease;
        animation: badge-float 3s ease-in-out infinite;
    }

    .type-badge:nth-child(2n) {
        animation-delay: 1.5s;
    }

    .type-badge:hover {
        transform: translateY(-3px) scale(1.05);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    }

    @keyframes badge-float {
        0%,
        100% {
            transform: translateY(0px);
        }
        50% {
            transform: translateY(-2px);
        }
    }

    /* Pokemon ID fade */
    .pokemon-id {
        animation: fade-pulse 4s ease-in-out infinite;
    }

    @keyframes fade-pulse {
        0%,
        100% {
            opacity: 0.3;
        }
        50% {
            opacity: 0.5;
        }
    }

    /* Section titles */
    .section-title {
        position: relative;
        display: inline-block;
    }

    .section-title::after {
        content: "";
        position: absolute;
        bottom: -4px;
        left: 0;
        width: 100%;
        height: 2px;
        background: linear-gradient(90deg, #e11d48 0%, transparent 100%);
        animation: underline-grow 2s ease-in-out infinite;
    }

    @keyframes underline-grow {
        0%,
        100% {
            width: 60%;
        }
        50% {
            width: 100%;
        }
    }

    /* Measurement text */
    .measurement-text {
        animation: slide-in 3s ease-in-out infinite;
    }

    @keyframes slide-in {
        0%,
        100% {
            transform: translateX(0);
        }
        50% {
            transform: translateX(3px);
        }
    }

    /* Pulse dot */
    .pulse-dot {
        animation: pulse-scale 1.5s ease-in-out infinite;
    }

    @keyframes pulse-scale {
        0%,
        100% {
            transform: scale(1);
            opacity: 0.5;
        }
        50% {
            transform: scale(1.3);
            opacity: 1;
        }
    }

    /* Ability badges */
    .ability-badge {
        transition: all 0.3s ease;
        animation: gentle-bounce 4s ease-in-out infinite;
    }

    .ability-badge:nth-child(odd) {
        animation-delay: 0.5s;
    }

    .ability-badge:nth-child(even) {
        animation-delay: 1s;
    }

    .ability-badge:hover {
        background-color: #cbd5e1;
        transform: translateY(-2px) scale(1.05);
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    }

    @keyframes gentle-bounce {
        0%,
        100% {
            transform: translateY(0);
        }
        50% {
            transform: translateY(-3px);
        }
    }

    /* API endpoint shimmer */
    .api-endpoint {
        position: relative;
        overflow: hidden;
    }

    .api-endpoint::before {
        content: "";
        position: absolute;
        top: 0;
        left: -100%;
        width: 100%;
        height: 100%;
        background: linear-gradient(
            90deg,
            transparent,
            rgba(255, 255, 255, 0.4),
            transparent
        );
        animation: shimmer 3s ease-in-out infinite;
    }

    @keyframes shimmer {
        0% {
            left: -100%;
        }
        50%,
        100% {
            left: 100%;
        }
    }

    /* Try button */
    .try-button {
        transition: all 0.3s ease;
        animation: button-pulse 2s ease-in-out infinite;
    }

    .try-button:hover {
        background-color: #475569;
        transform: scale(1.05);
        box-shadow: 0 8px 16px rgba(0, 0, 0, 0.2);
    }

    .try-button:active {
        transform: scale(0.98);
    }

    @keyframes button-pulse {
        0%,
        100% {
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }
        50% {
            box-shadow: 0 6px 12px rgba(0, 0, 0, 0.15);
        }
    }

    /* Code block fade in */
    .code-block {
        animation: code-appear 0.5s ease-out;
    }

    @keyframes code-appear {
        from {
            opacity: 0;
            transform: translateY(10px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }
</style>
