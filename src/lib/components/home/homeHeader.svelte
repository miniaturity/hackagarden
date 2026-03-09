<script lang="ts">
    import Time from "../time.svelte";


</script>

<div class="th__header">
  <section class="thh__upper">
    <div class="th__time">
      <Time />
    </div>
  </section>

  <section class="thh__lower">

  </section>
</div>

<style lang="scss">
  .th__header {
    width: 100%;
    height: 100%;

    display: flex;
    flex-direction: column;
    align-items: center;
    flex-shrink: 0;
  }

  .thh__upper {
    display: flex;
    flex-direction: row;

    width: 100%; height: 50%;
    border-bottom: var(--border);
  }

  .thh__lower {
    display: flex;
    flex-direction: row;
    
    width: 100%; height: 50%;

  }

  .th__time {
    width: 25%;
    border-right: var(--border);
  }
</style>