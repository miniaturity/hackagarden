<script lang="ts">
    import { onMount } from "svelte";


function formatTwoDigits(num: number): string {
    return num < 10 ? `0${num}` : num.toString();
}

function getTime(): string {
    const now = new Date();
    const hours = now.getHours();
    const minutes = now.getMinutes();
    const seconds = now.getSeconds();

    const formattedHours = formatTwoDigits(hours);
    const formattedMinutes = formatTwoDigits(minutes);
    const formattedSeconds = formatTwoDigits(seconds);

    return `${formattedHours}:${formattedMinutes}:${formattedSeconds}`;
}

  let time = $state(getTime());

  onMount(() => {
    const interval = setInterval(() => {
        time = getTime();
    }, 1000);
    
    return () => clearInterval(interval);
  })

</script>

<div class="time">
  <div>{time}</div>
</div>

<style lang="scss">
  .time {
    width: 100%; height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;

    font-family: "Geist";
  }
</style>