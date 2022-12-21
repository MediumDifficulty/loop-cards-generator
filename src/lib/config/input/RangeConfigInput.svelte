<script lang="ts">
    import type { RangeConfig } from "../../backend_types";
    import RangeInput from "./RangeInput.svelte";


    export let config: RangeConfig

</script>

<div>
    <div>Range</div>
    <RangeInput min={0} max={100} bind:range={config.range} />
</div>
