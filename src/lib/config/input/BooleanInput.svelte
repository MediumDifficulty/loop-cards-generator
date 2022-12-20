<script lang="ts">
    import { Switch } from "@rgossiaux/svelte-headlessui";

    export let enabled: boolean
</script>

<Switch 
    checked={enabled} 
    class={enabled ? "rounded-full inline-flex items-center h-6 w-11 bg-tertiary shadow-md" : "rounded-full inline-flex items-center h-6 w-11 bg-primary shadow-md"}
    on:change={(e) => enabled = e.detail}
>
    <span class="inline-block w-4 h-4 bg-white rounded-full duration-300 shadow-md" class:translate-x-[1.5rem]={enabled} class:translate-x-[0.25rem]={!enabled}></span>
</Switch>
