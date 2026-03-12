<script lang="ts">
    import HomeSummary from "./homeSummary.svelte";
    import GardenButton from "./main/gardenButton.svelte";
    import GreenhouseButton from "./main/greenhouseButton.svelte";




</script>

<main>
  <div class="m__left">
    <div class="ml__qbuttons">
      <div class="gh">
        <GreenhouseButton />
      </div>

      <div class="gh">
        <GardenButton />
      </div>
    </div>
  </div>

  <div class="m__right">
    <HomeSummary />
  </div>
</main>

<style lang="scss">
  main {
    display: flex;
    flex-direction: row;
  
    width: 100%; height: 100%;
  }
  
  .m__left {
    width: 50%; height: 100%;
    border-right: var(--border);

    display: flex;
    flex-direction: column;
  }

  .m__right {
    width: 50%;
    height: 100%;
  }

  .ml__qbuttons {
    display: flex;
    flex-direction: column;
    height: 125px;
    gap: var(--margin);
    border-bottom: var(--border);
  }

  .gh {
    height: 50px;
    width: calc(100% - calc(2 * var(--margin)));
  }
</style>