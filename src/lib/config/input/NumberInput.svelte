<script lang="ts">
    import RangeSlider from "svelte-range-slider-pips";

    export let value: number = 50

    export let min: number = 0
    export let max: number = 100

    export let step: number = 20
</script>

<RangeSlider values={[value]} on:change={(e) => value = e.detail.value} pipstep={step} pips float hoverable all="label" max={max} min={min}></RangeSlider>