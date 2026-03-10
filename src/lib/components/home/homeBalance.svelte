<script lang="ts">

  let {
    balance
  }: {
    balance?: number
  } = $props();

</script>

<div class="balance">
  <div class="b__img">
  </div>
  {#if balance}
  <div>{balance}</div>
  {:else}
  <div>...</div>
  {/if}
</div>

<style lang="scss">
  .balance {
    width: 100%; height: 100%;
    color: var(--sat-purple);
    background-color: var(--l-purple);

    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-around;
    font-family: "GP_Square";
    font-size: 1.5rem;
  }

  .b__img {
    aspect-ratio: 3 / 4;
    width: 14px;
    display: flex;
    align-items: center;
    image-rendering: pixelated;
    background-repeat: no-repeat;
    background-position: 50% 50%;
    background-size: contain;
    background-image: url("$lib/assets/images/hour-1.png");

    &:hover {
      background-image: url("$lib/assets/images/hour.gif");
    }
  }
</style>