<script lang="ts">
  import { userData } from "$lib/store";


</script>

{#if $userData}
  <div class="summary">
    {#if $userData.latest_heartbeat}
      <header>
        [ latest heartbeat ]
      </header>
      <div class="s__name">
        {$userData.latest_heartbeat.project}
      </div>
      <table class="s__data">
        <thead>
          <tr>
            <th>name</th>
            <th>data</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>lang</td>
            <td>{$userData.latest_heartbeat.language}</td>
          </tr>

          <tr>
            <td>editor</td>
            <td>{$userData.latest_heartbeat.editor}</td>
          </tr>

          <tr>
            <td>hours</td>
            
            <td>{(($userData.projects.find(p => p.name === $userData.latest_heartbeat?.project)?.total_seconds || 0) / 3600).toFixed(2)}</td>
          </tr>
        </tbody>
      </table>
      
    {/if}
  </div>
{:else}
  <div class="summary-loading">

  </div>
{/if}

<style lang="scss">

</style>