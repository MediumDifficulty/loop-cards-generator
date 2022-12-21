<script lang="ts">
    import type { SumConfig } from "../../backend_types";
    import RangeInput from "./RangeInput.svelte";


    export let config: SumConfig

</script>

<div>
    <div>Answer Range</div>
    <RangeInput min={0} max={100} bind:range={config.answer_range} />
    <div>Distance Range</div>
    <RangeInput min={0} max={100} bind:range={config.distance_range} />
</div>
