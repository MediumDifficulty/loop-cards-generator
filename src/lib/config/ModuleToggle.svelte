<script lang="ts">
    import BooleanInput from "./input/BooleanInput.svelte";

    export let state: boolean
    export let label: string
</script>


<div class="inline-flex items-center justify-between w-full">
    <span>{label}</span>
    <BooleanInput bind:enabled={state} />
</div>