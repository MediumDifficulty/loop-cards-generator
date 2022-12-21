<script lang="ts">
    import type { TimesTablesConfig } from "../../backend_types";
    import RangeInput from "./RangeInput.svelte";


    export let config: TimesTablesConfig

</script>

<div>
    <div>Times Tables</div>
    <RangeInput min={0} max={100} bind:range={config.times_tables} />
</div>
