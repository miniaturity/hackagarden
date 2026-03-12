<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import { userData } from '$lib/store';
  import { onMount } from 'svelte';
  import "$lib/styles/global.css";
  import HomeHeader from '$lib/components/home/homeHeader.svelte';
    import HomeLoading from '$lib/components/home/homeLoading.svelte';
    import HomeMain from '$lib/components/home/homeMain.svelte';

  function formatTime(seconds: number): string {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    if (hours > 0) return `${hours}h ${minutes}m`;
    return `${minutes}m`;
  }

  async function logout() {
    await invoke('logout');
    userData.set(null);
    goto('/login');
  }

  let loaded = $state<boolean>(false);

  $effect(() => {
    if (confirmingLogout) {
      setTimeout(() => {
        confirmingLogout = false;
      }, 2000);
    }
  });


  let confirmingLogout = $state<boolean>(false);
  let animateWelcome = $state<boolean>(false);

  async function logoutConfirm() {
    if (!confirmingLogout) {
      confirmingLogout = true;
    } else {
      logout();
    }
  }

  $effect(() => {
    if (loaded) {
      setTimeout(() => animateWelcome = true, 2000);
    }
  });

  $effect(() => {
    if (!loaded && $userData) {
      loaded = true;
    }
  })
</script>

{#if loaded && $userData}
  <div class="page">
    <div class="container">

      <section class="top">
        <button class="t__pfp" onclick={logoutConfirm}>
          <img alt="" src={`https://github.com/${$userData.username}.png`}/>
          <div class="tp__logout">
            {#if !confirmingLogout}
            <svg stroke="#e96a6a" fill="none" stroke-width="2" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round" height="33px" width="33px" xmlns="http://www.w3.org/2000/svg"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path><polyline points="16 17 21 12 16 7"></polyline><line x1="21" y1="12" x2="9" y2="12"></line></svg>
            {:else}
            <svg stroke="red" fill="red" stroke-width="0" viewBox="0 0 512 512" height="33px" width="33px" xmlns="http://www.w3.org/2000/svg"><path d="M256 90c44.3 0 86 17.3 117.4 48.6C404.7 170 422 211.7 422 256s-17.3 86-48.6 117.4C342 404.7 300.3 422 256 422s-86-17.3-117.4-48.6C107.3 342 90 300.3 90 256s17.3-86 48.6-117.4C170 107.3 211.7 90 256 90m0-42C141.1 48 48 141.1 48 256s93.1 208 208 208 208-93.1 208-208S370.9 48 256 48z"></path><path d="M360 330.9L330.9 360 256 285.1 181.1 360 152 330.9l74.9-74.9-74.9-74.9 29.1-29.1 74.9 74.9 74.9-74.9 29.1 29.1-74.9 74.9z"></path></svg>
            {/if}
          </div>
        </button>
        <header class="t__header">
          <div class={`th__content`}>

            <div class={`th__welcome ${animateWelcome && "slide-left"}`}>
              <div class="thw__content"><span>Welcome, {$userData.username}!</span></div>
            </div>

            <HomeHeader />
          </div>
        </header>
      </section>
      <section class="bottom">
        

        <div class="b__main">
          <HomeMain />
        </div>
      </section>

    </div>
  </div>

{:else}
  <HomeLoading />
{/if}

<style lang="scss">

  :global(body) {
    margin: 0; padding: 0;
  }

  

  @mixin pixel-font() {
    text-rendering: optimizeLegibility;
    font-synthesis-weight: none;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    image-rendering: pixelated; 
    
  }

  @property --rotation {
    syntax: "<angle>";
    inherits: false;
    initial-value: 0deg;
  }

  @keyframes rotate {
    to {
      --rotation: 360deg;
    }
  }

  @mixin gradient-bg($opacity) {
    position: relative;
    &:before{
      content: "";
      display: block;
      background: conic-gradient(from var(--rotation), purple, blue, green, yellow, orange, red);
      filter: blur(20px);
      opacity: $opacity;
      position: absolute;
      inset: 4px;
      z-index: -2;
      animation: rotate 5s linear infinite;
      transition: opacity 0.3s ease-in-out;
    }
  }

  .page {
    width: 100vw; height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    overflow: hidden;
  }

  .container {
    width: 700px; height: 500px;
    display: flex;
    flex-direction: column;

    gap: calc(var(--margin) * 2);
  }

  .top {
    display: flex;
    flex-direction: row;
    height: 100px; width: 100%;
    gap: calc(var(--margin) * 2);
  }

  .bottom {
    display: flex;
    flex-direction: row;
    width: 100%;
    flex-grow: 1;
    gap: calc(var(--margin) * 2);
  }

  .t__pfp {
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    height: 104px; width: 104px; // note: had to account for border
    border: var(--border);
    background-color: var(--bg-col-l);
    margin: 0; padding: 0; 

    & img {
      width: 100%; height: 100%; 
      object-fit: cover;
    }

    @include gradient-bg(1);

    &:hover {
      .tp__logout {
        opacity: 1;
      }
    }
  }

  .t__header {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    height: 100px;
    flex-grow: 1;
    border: var(--border);
    background-color: var(--bg-col-l);
    font-size: 1.8rem;
    font-family: "GP_Square";
    user-select: none;
    overflow: hidden;
    flex-wrap: nowrap;

    @include pixel-font();
  }

  .th__content {
    width: 100%;
    height: 100%;
    flex-shrink: 0;

    display: flex;
    flex-direction: row;
    flex-wrap: nowrap;
    overflow: hidden;
    position: relative;
  }

  .th__welcome {
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    
    flex-shrink: 0;

    overflow: hidden;
    white-space: nowrap;
    clip-path: inset(0 0 0 0);
    z-index: 3;
    background-color: var(--bg-col-l);
  }

  .thw__content {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }


  .slide-left {
    animation: slide-left 1s ease-in-out forwards;
  }

  @keyframes slide-left {
    to {
      clip-path: inset(0 100% 0 0);
      display: none;
    }
  }


  .tp__logout {
    display: flex;
    align-items: center;
    justify-content: center;
    position: absolute;
    left: 20%;
    top: 20%;
    width: 60%; height: 60%;
    border-radius: 100%;
    background-color: #ffffffbb;
    opacity: 0;
    transition: opacity 0.2s ease-in;
    filter: drop-shadow(5px 5px 10px #000);
    cursor: pointer;
  }

  .b__nav {
    width: 100px;
    height: 100%;
    border: var(--border);
    background-color: var(--bg-col-l);
  }

  .b__main {
    height: 100%;
    flex-grow: 1;
    border: var(--border);
    background-color: var(--bg-col-l);
  }
</style>