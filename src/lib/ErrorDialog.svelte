<script lang="ts">
    import { Dialog, DialogOverlay } from "@rgossiaux/svelte-headlessui";
    import { quintOut } from "svelte/easing";
    import { fade, scale } from "svelte/transition";

    export let isOpen: boolean
    export let message: string
</script>

<Dialog open={isOpen} on:close={() => isOpen = false} class="fixed z-10 inset-0">
    <div transition:fade={{ duration: 500 }}>
        <DialogOverlay class="fixed inset-0 bg-black opacity-30" />
    </div>
    <div class="flex justify-center items-center h-screen">
        <div class="z-10 bg-white w-1/5 h-1/5 rounded-lg shadow-lg p-3" transition:scale={{ duration: 500, easing: quintOut }}>
            <div class="relative h-full">
                <div class="text-center border-b-2">An error occured.</div>
                <div>{message}</div>
                <div class="absolute bottom-0">
                    <button on:click={() => isOpen = false} class="bg-primary rounded-lg px-4 py-1 text-white shadow-sm hover:shadow-lg duration-300 hover:bg-tertiary">Okay</button>
                </div>
            </div>
        </div>
    </div>
</Dialog>