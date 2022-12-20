<script lang="ts">
    import type { RangeInclusive } from "src/lib/rust_types";
    import RangeSlider from "svelte-range-slider-pips";

    export let range: RangeInclusive

    export let min: number
    export let max: number
</script>

<RangeSlider values={[range.start, range.end]}
    on:change={(e) => {
        range.start = e.detail.values[0]
        range.end = e.detail.values[1]
    }}
    range pushy pips float hoverable all="label" max={max} min={min} 
/>