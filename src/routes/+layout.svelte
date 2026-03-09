<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { invoke } from '@tauri-apps/api/core';
  import { userData } from '$lib/store';
  import type { UserData } from '$lib/store';
    import Footer from '$lib/components/footer.svelte';

  async function fetchUserData(): Promise<UserData> {
    const [info, streakData, projectsData, heartbeatData, apiKey] = await Promise.all([
      invoke<Record<string, any>>('get_user_info'),
      invoke<Record<string, any>>('get_streak'),
      invoke<Record<string, any>>('get_projects'),
      invoke<Record<string, any>>('get_latest_heartbeat').catch(() => null),
      invoke<string>('get_api_key').catch(() => null),
    ]);

    return {
      username: info.github_username ?? 'Unknown',
      streak_days: streakData.streak_days ?? 0,
      projects: (projectsData.projects ?? []).map((p: any) => ({
        name: p.name,
        total_seconds: p.total_seconds ?? 0,
        languages: p.languages ?? [],
        most_recent_heartbeat: p.most_recent_heartbeat ?? null,
      })),
      latest_heartbeat: heartbeatData ? {
        project: heartbeatData.project ?? null,
        language: heartbeatData.language ?? null,
        editor: heartbeatData.editor ?? null,
        entity: heartbeatData.entity ?? null,
      } : null,
      api_key: apiKey,
    };
  }

  onMount(async () => {
    const auth = await invoke<{ is_authenticated: boolean }>('get_auth_state');
    const isLoginPage = $page.url.pathname === '/login';

    if (auth.is_authenticated) {
      if (!$userData) {
        userData.set(await fetchUserData());
      }
      if (isLoginPage) goto('/home');
    } else {
      if (!isLoginPage) goto('/login');
    }
  });
</script>


<slot />
<Footer />
