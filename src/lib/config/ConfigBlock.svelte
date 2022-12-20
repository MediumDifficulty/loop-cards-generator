<script lang="ts">
    export let label: String
</script>

<div class="shadow-sm hover:shadow-lg rounded-lg m-3 duration-200">
    <div class="text-center text-lg border-b-2 w-3/4 mx-auto mb-3">{label}</div>
    <slot />
</div>