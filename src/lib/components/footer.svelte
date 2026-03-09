<script lang="ts">
  import { onMount } from "svelte";

  const url = "https://api.github.com/repos/miniaturity/hackagarden/commits?per_page=1";

  type Commit = {
    sha: string;
    commit: {
      message: string;
      author: {
        name: string;
        date: string;
      };
    };
    html_url: string;
  };

  let latestCommit = $state<Commit | null>(null);

  async function getCommits() {
    const res = await fetch(url);
    const data: Commit[] = await res.json();
    latestCommit = data[0];
  }

  onMount(getCommits);
</script>

{#if latestCommit}
<footer>
  <div>Commit 
    <a href={latestCommit.html_url} target="_blank" rel="noopener noreferrer">{latestCommit.sha.slice(0, 6)}</a>
  @ {latestCommit.commit.author.date}
  </div>
  
</footer>
{/if}

<style lang="scss">
  footer {
    user-select: none;
    position: absolute;
    left: 0; bottom: 0;
    margin: var(--margin);

    color: color-mix(in srgb, var(--bg-col), #000 30%);
    font-family: "GP_Grid";
    font-size: 0.8rem;

    & a {
      color: inherit;
      font-family: inherit;

      &:hover {
        color: color-mix(in srgb, var(--bg-col), #000 65%)
      }
    }
  }

  
</style>