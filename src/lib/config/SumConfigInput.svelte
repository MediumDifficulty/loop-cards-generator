<script lang="ts">
    import type { SumConfig } from "../backend_types";
    import type { RangeInclusive } from "../rust_types";
    import NumberInput from "./input/NumberInput.svelte";
    import RangeInput from "./input/RangeInput.svelte";


    export let config: SumConfig

</script>

<div>
    <div>Answer Range</div>
    <RangeInput min={0} max={100} bind:range={config.answer_range} />
    <div>Max Distance</div>
    <NumberInput min={0} max={100} bind:value={config.max_distance} />
</div>
