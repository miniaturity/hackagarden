<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let username = $state<string | null>(null);
  let stats = $state<any>(null);
  let loading = $state(false);

  async function login() {
    loading = true;
    try {
      await invoke('start_auth');          // blocks until user finishes login
      const info = await invoke<{ username: string }>('get_user_info');
      username = info.username;
      stats = await invoke('get_stats');
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }
</script>

{#if loading}
  <p>Waiting for login...</p>
{:else if username}
  <p>Logged in as {username}</p>
  <p>Streak: {stats?.streak?.streak_days ?? 0} days</p>
  <p>This week: {((stats?.weekly_hours?.total_seconds ?? 0) / 3600).toFixed(1)}h</p>
  <button onclick={() => invoke('logout').then(() => { username = null; })}>Logout</button>
{:else}
  <button onclick={login}>Login with Hack Club</button>
{/if}