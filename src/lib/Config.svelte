<script lang="ts">
    import type { GeneratorConfig } from "./backend_types";
    import { Popover, PopoverButton, PopoverPanel } from "@rgossiaux/svelte-headlessui";
    import { scale } from "svelte/transition";
    import { quintOut } from "svelte/easing";
    import NumberInput from "./config/input/NumberInput.svelte";
    import ConfigBlock from "./config/ConfigBlock.svelte";
    import ModuleStates from "./config/ModuleStates.svelte";
    import SumConfigInput from "./config/input/SumConfigInput.svelte";
    import InlineSVG from "svelte-inline-svg"
    import MultiplicationConfigInput from "./config/input/TimesTablesConfigInput.svelte";
    import TimesTablesConfigInput from "./config/input/TimesTablesConfigInput.svelte";
    import RangeConfigInput from "./config/input/RangeConfigInput.svelte";

    export let config: GeneratorConfig
</script>

<Popover class="inline">
    <PopoverButton>
        <InlineSVG src={`${process.env.NODE_ENV === "production" ? "loop-cards-generator/" : ""}more_horiz.svg`} class="fill-primary m-1 hover:fill-tertiary duration-300" />
    </PopoverButton>

    <PopoverPanel class="inline">
        <div class="absolute w-3/4 left-[12.5%] rounded-lg shadow-lg bg-white" style="--range-handle-focus: #367FC9" transition:scale={{ duration: 500, easing: quintOut }}>
            <div class="grid grid-cols-3">
                <ConfigBlock label="Card Count">
                    <NumberInput bind:value={config.card_count} max={100} />
                </ConfigBlock>
                <ConfigBlock label="Attempts">
                    <NumberInput bind:value={config.max_attempts} max={10000} step={1000} />
                </ConfigBlock>
                <ConfigBlock label="Addition">
                    <SumConfigInput bind:config={config.addition} />
                </ConfigBlock>
                <div class="row-span-2">
                    <ConfigBlock label="Modules">
                        <ModuleStates bind:states={config.states} />
                    </ConfigBlock>
                </div>
                <ConfigBlock label="Subtraction">
                    <SumConfigInput bind:config={config.subtraction} />
                </ConfigBlock>
                <ConfigBlock label="Multiplication">
                    <TimesTablesConfigInput bind:config={config.multiplication} />
                </ConfigBlock>
                <ConfigBlock label="Division">
                    <TimesTablesConfigInput bind:config={config.division} />
                </ConfigBlock>
                <ConfigBlock label="Double">
                    <RangeConfigInput bind:config={config.double} />
                </ConfigBlock>
                <ConfigBlock label="Half of">
                    <RangeConfigInput bind:config={config.half_of} />
                </ConfigBlock>
            </div>
        </div>
    </PopoverPanel>
</Popover>

