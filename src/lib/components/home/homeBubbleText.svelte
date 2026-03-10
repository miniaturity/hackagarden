<script lang="ts">
  import { onMount } from "svelte";

  let {
    username
  }: {
    username?: string;
  } = $props();

  let todText = $state<string>("");
  let flavorText = $state<string>("");
  let constructedText = $derived(todText + " " + flavorText);

  const tod = $derived({
    morning: `Good morning, ${username || "{USER}"}!`,
    afternoon: `Good afternoon, ${username || "{USER}"}!`,
    evening: `Good evening, ${username || "{USER}"}!`
  });

  const flavor = [
    "Today is a good day.",
    "You look nice today!",
    "Take this: 🪲",
    "It's a good day to grow your garden.",
    "Don't forget to water your plants!",
    "🐇",
    "How are you? Fine thank you.",
    "Miniaturity was here.",
    "hi lol",
    "--> https://minipa.ws <--"
  ]

  function getTimeOfDay(): 'morning' | 'afternoon' | 'evening' {
    const currentHour = new Date().getHours(); 
    if (currentHour >= 5 && currentHour < 12) {
      return 'morning';
    } else if (currentHour >= 12 && currentHour < 18) {
      return 'afternoon';
    } else {
      return 'evening';
    }
  }

  onMount(() => {
    todText = tod[getTimeOfDay()];
    flavorText = flavor[Math.floor(Math.random() * flavor.length)];
  });

  function recomputeText() {
    let ri: number;
    do {
      ri = Math.floor(Math.random() * flavor.length);
    } while (ri === flavor.indexOf(flavorText))

    todText = tod[getTimeOfDay()];
    flavorText = flavor[ri];
  }
</script>

<div class="bubble-text" onmouseenter={recomputeText} role="cell" tabindex="0">
  {constructedText}
</div>

<style lang="scss">
  .bubble-text {
    font-size: 1.1rem;
    font-family: "Geist";

    &:hover {
      scale: 1.05;
    }
  }
</style>