<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import { userData } from '$lib/store';
  import "$lib/styles/global.css";

  import loading_svg from "$lib/assets/images/loading.svg";

  let loading = $state(false);
  let error = $state<string | null>(null);

  async function login() {
    loading = true;
    error = null;
    try {
      await invoke('start_auth');

      const [info, streakData, projectsData, heartbeatData, apiKey] = await Promise.all([
        invoke<Record<string, any>>('get_user_info'),
        invoke<Record<string, any>>('get_streak'),
        invoke<Record<string, any>>('get_projects'),
        invoke<Record<string, any>>('get_latest_heartbeat').catch(() => null),
        invoke<string>('get_api_key').catch(() => null),
      ]);

      userData.set({
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
      });

      goto('/home');
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }
</script>

<div class="page">
  <div class="p__title">{"{"}hack-a-garden{"}"}</div>
  <button onclick={login} disabled={loading} class="p__login">
    {#if !loading}
      [login with hackatime]
    {:else}
      <img src={loading_svg} alt="loading..." />
    {/if}
  </button>

</div>

{#if error}
  <p>{error}</p>
{/if}

<style lang="scss">
  @property --rotation {
    syntax: "<angle>";
    inherits: false;
    initial-value: 0deg;
  }

  :global(body) {
    margin: 0; padding: 0;

    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
  }

  .page {
    width: 100vw; height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--margin);
  }

  .p__title {
    font-family: "GP_Square";
    font-size: 4rem;

    font-synthesis-weight: none;
  }

  .p__login {
    font-family: "GeistMono";
    border: var(--border);
    background-color: #fff;
    cursor: pointer;
    padding: 8px;
    position: relative;
    font-size: 1rem;
    width: 250px; height: 48px;
    transition: width 0.3s ease-in-out;

    &:before {
      content: "";
      display: block;
      background: conic-gradient(from var(--rotation), purple, blue, green, yellow, orange, red);
      filter: blur(20px);
      opacity: 0.5;
      position: absolute;
      inset: 4px;
      z-index: -2;
      animation: rotate 5s linear infinite;
      transition: opacity 0.3s ease-in-out;
    }
    
    &:hover:before {
      opacity: 1;
    }

    &:hover {
      font-weight: bold;
    }

    &:disabled {
      width: 48px; height: 48px;
      margin: 0; padding: 0;
      display: flex;
      align-items: center; justify-content: center;
      cursor: progress;
      & img {
        width: 24px; height: 24px;
      }

      &:before {
        opacity: 1;
        filter: blur(20px);
        inset: 0px;
      }
    }
  }

  @keyframes rotate {
    to {
      --rotation: 360deg;
    }
  }
</style>