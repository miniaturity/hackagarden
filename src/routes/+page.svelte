<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let username = $state<string | null>(null);
  let stats = $state<any>(null);
  let loading = $state(false);

  async function login() {
    loading = true;
    try {
      await invoke('start_auth');
      const info = await invoke<Record<string, any>>('get_user_info');
      console.log('user info:', info);  // check the browser console
      username = info.username ?? info.login ?? info.name ?? info.display_name;
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