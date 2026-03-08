<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let username = $state<string | null>(null);
  let stats = $state<any>(null);
  let loading = $state(true);

  onMount(async () => {
    try {
      const auth = await invoke<{ is_authenticated: boolean }>('get_auth_state');
      if (auth.is_authenticated) {
        const info = await invoke<Record<string, any>>('get_user_info');
        username = info.github_username;
        stats = await invoke('get_stats');
      }
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  async function login() {
    loading = true;
    try {
      await invoke('start_auth');
      const info = await invoke<Record<string, any>>('get_user_info');
      username = info.github_username;
      stats = await invoke('get_stats');
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }
</script>

{#if loading}
  <p>Loading...</p>
{:else if username}
  <img src={`https://github.com/${username}.png`} alt="pfp"/>
  <p>Logged in as {username}</p>
  <p>Streak: {stats?.streak?.streak_days ?? 0} days</p>
  <p>This week: {((stats?.weekly_hours?.total_seconds ?? 0) / 3600).toFixed(1)}h</p>
  <button onclick={() => invoke('logout').then(() => { username = null; })}>Logout</button>
{:else}
  <button onclick={login}>Login with Hack Club</button>
{/if}
