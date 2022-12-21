<script lang="ts">
    import type { ModuleStates } from "../backend_types";
    import ModuleToggle from "./ModuleToggle.svelte";

    export let states: ModuleStates

</script>


<div class="m-3 space-y-2">
    <ModuleToggle bind:state={states.addition} label="Addition" />
    <ModuleToggle bind:state={states.subtraction} label="Subtraction" />
    <ModuleToggle bind:state={states.multiplication} label="Multiplication" />
    <ModuleToggle bind:state={states.division} label="Division" />
    <ModuleToggle bind:state={states.double} label="Double" />
    <ModuleToggle bind:state={states.half_of} label="Half of" />
</div>