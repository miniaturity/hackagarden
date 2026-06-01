<script lang="ts">
    import type { Item } from "$lib/plants";

    let {
        item
    }: {
        item: Item;
    } = $props();

</script>

<div class="store-item">
    <div class="si-logo">
        <img src={item.texture} alt=""/>
    </div>

    
</div>

<style lang="scss">

</style>