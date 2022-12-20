<script lang="ts">
    import type { ModuleStates } from "../backend_types";
    import BooleanInput from "./input/BooleanInput.svelte";
    import ModuleToggle from "./ModuleToggle.svelte";

    export let states: ModuleStates

</script>


<div class="m-3 space-y-2">
    <ModuleToggle bind:state={states.addition} label="Addition" />
    <ModuleToggle bind:state={states.subtraction} label="Subtraction" />
</div>