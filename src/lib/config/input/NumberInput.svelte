<script lang="ts">
    import RangeSlider from "svelte-range-slider-pips";

    export let value: number

    export let min: number
    export let max: number
</script>

<RangeSlider values={[value]} on:change={(e) => value = e.detail.value} pips float hoverable all="label" max={max} min={min}></RangeSlider>