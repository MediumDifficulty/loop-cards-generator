<script lang="ts">
    import Config from "$lib/Config.svelte";
    import { scale } from "svelte/transition";
    import { quintOut } from "svelte/easing";
    import * as backend from "$lib/backend"
    import { tick } from "svelte";
    import ErrorDialog from "$lib/ErrorDialog.svelte";

    let cards: [string, string][] = []

    let config = {
        card_count: 10,
        card_offset: 1,
        max_attempts: 100,
        states: {
            addition: true,
            subtraction: true
        },
        addition: {
            answer_range: {
                start: 0,
                end: 100
            },
            max_distance: 10
        },
        subtraction: {
            answer_range: {
                start: 0,
                end: 100
            },
            max_distance: 10
        }
    }
    
    let seed: bigint = 0n

    async function generate() {
        cards = []
        await tick()

        try {
            cards = backend.generate(config, seed)
        } catch (err) {
            errorMsg = err as string
            errorDialog = true
        }
    }

    let errorDialog = false
    let errorMsg = ""
</script>


<div class="bg-slate-800 min-h-screen">
    <div class="flex items-center">
        <Config bind:config={config} />
        <button on:click={generate} class="ml-2 bg-primary hover:bg-tertiary duration-300 p-2 rounded-full text-white">Generate</button>
    </div>

    <ErrorDialog bind:isOpen={errorDialog} message={errorMsg} />

    <div class="grid grid-cols-5">
        {#each cards as card, i}
            <div in:scale="{{ duration: 500, easing: quintOut, delay: i * 50 }}" class="rounded-lg shadow-sm hover:shadow-lg duration-200 p-5 bg-primary m-5 grid grid-cols-2">
                <span class="text-center border-r-2 text-white text-lg">{card[0]}</span>
                <span class="text-center text-white text-lg">{card[1]}</span>
            </div>
        {/each}
    </div>
</div>

