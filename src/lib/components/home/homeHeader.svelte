<script lang="ts">
    import Time from "../time.svelte";
    import HomeBalance from "./homeBalance.svelte";

    import { currencyState } from '$lib/store';
    import { userData } from "$lib/store";
    import HomeBubbleText from "./homeBubbleText.svelte";

    let sortedLanguages = $derived(
      $currencyState
        ? Object.entries($currencyState.language_hours)
            .sort(([, a], [, b]) => b - a)
            .map(([name, hours]) => ({ name, hours }))
        : []
    );
</script>

<div class="th__header">
  <section class="thh__upper">
    <div class="th__time">
      <Time />
    </div>
    <div class="th__stats">
      <div class="ths__balance">
          <HomeBalance balance={$currencyState?.balance} />
      </div>
    </div>
  </section>

  <section class="thh__lower">
    <HomeBubbleText username={$userData?.username}/>
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
    align-items: center;
    justify-content: center;
    width: 100%; height: 50%;

  }

  .th__time {
    width: 25%;
    border-right: var(--border);
  }

  .th__stats {
    flex-grow: 1;
    height: 100%;
    display: flex;
    align-items: center;
  }

  .ths__balance {
    width: 100px;
    height: 35px;
    border: 2px solid var(--purple);
    margin-left: 5px;
  }
</style>