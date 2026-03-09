<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import { userData } from '$lib/store';
  import "$lib/styles/global.css";
    import { onMount } from 'svelte';

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

  onMount(() => {
    if (!userData) goto("/login");
  })
</script>

{#if userData}
  <div class="page">
    <div class="container">

      <div class="top">

      </div>
      <div class="bottom">

      </div>

    </div>
  </div>
{/if}

<style lang="scss">
  .page {
    width: 100vw; height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
  }

  .container {
    width: 80vw; height: 80vh;
    border: var(--border)
  }
</style>