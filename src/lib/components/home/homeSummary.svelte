<script lang="ts">
  import { userData } from "$lib/store";


</script>

{#if $userData}
  <div class="summary">
    {#if $userData.latest_heartbeat}
      <header>
      <span>{"{"}&nbsp;</span> latest heartbeat <span>&nbsp;{"}"}</span>
      </header>
      
      <table class="s__data">
        <tbody>
          <tr>
            <td>Name</td>
            <td>{$userData.latest_heartbeat.project?.toLowerCase() || "..."}</td>
          </tr>
          
          <tr>
            <td>Language</td>
            <td>{$userData.latest_heartbeat.language?.toLowerCase() || "..."}</td>
          </tr>

          <tr>
            <td>Editor</td>
            <td>{$userData.latest_heartbeat.editor?.toLowerCase() || "..."}</td>
          </tr>

          <tr>
            <td>Hours</td>
            
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
  .summary {
    width: 100%;

    display: flex;
    flex-direction: column;
    align-items: center;

    font-family: "Geist";
  }

  header {
    width: 100%; height: 45px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-bottom: var(--border);

    font-family: "GP_Square";
    font-size: 1.4rem;
    transition: all 0.4s ease-out;

    & span {
      transition: all 0.4s ease-out;
    }
    
    &:hover {
      color: #22f82d;
      background-color: #000;

      & span {
        color: red;
      }
    }
  }


  table {
    width: 100%;
    border-collapse: collapse;
    table-layout: fixed;
  }

  tr {
    width: 100%;
    
    & td:nth-child(1) {
      border-right: var(--border);
    }

    & td:nth-child(2) {
      font-family: "GeistMono";
    }
  }

  td {
    width: 50%;
    height: 25px;
    padding: var(--margin);
    border-bottom: var(--border);

    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
  }

  
  
</style>