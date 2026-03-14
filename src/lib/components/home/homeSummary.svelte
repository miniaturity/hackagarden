<script lang="ts">
  import { userData } from "$lib/store";


</script>

{#if $userData}
  <div class="summary">
    {#if $userData.latest_heartbeat}
      <header>
      <span>{"{"}&nbsp;</span> latest_heartbeat <span>&nbsp;{"}"}</span>
      </header>
      
      <table class="s__data">
        <tbody>
          <tr>
            <td>Name:</td>
            <td>{"\"" + ($userData.latest_heartbeat.project?.toLowerCase() || "...") + "\""}</td>
          </tr>
          
          <tr>
            <td>Language:</td>
            <td>{"\"" + ($userData.latest_heartbeat.language?.toLowerCase() || "...") + "\""}</td>
          </tr>

          <tr>
            <td>Editor:</td>
            <td>{"\"" + ($userData.latest_heartbeat.editor?.toLowerCase() || "...") + "\""}</td>
          </tr>

          <tr>
            <td>Hours:</td>
            
            <td>{"\"" + (($userData.projects.find(p => p.name === $userData.latest_heartbeat?.project)?.total_seconds || 0) / 3600).toFixed(2) + "\""}</td>
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

    --code-bg: #1e1e1e;
    --code-var: #9CDCFE;
    --code-flow: #C586C0;
    --code-string: #CE9178;
    --code-class: #4EC9B0;
    --trans: all 0.4s ease-out;
    --b: 2px solid #1e1e1e;
  }

  .summary:has(header:hover) {
    & tr { 
      & td:nth-child(1) {
        background-color: #1e1e1e;
        color: var(--code-class);
      }

      & td:nth-child(2) {
        background-color: #1e1e1e;
        color: var(--code-string);
      }
    }
  }

  header {
    width: 100%; height: 45px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-bottom: var(--border);

    font-family: "GP_Square";
    font-size: 1.4rem;
    transition: var(--trans);

    & span {
      transition: var(--trans);
    }
    
    &:hover {
      color: var(--code-var);
      background-color: var(--code-bg);

      & span {
        color: var(--code-flow);
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
      width: 25%;
      transition: var(--trans);
      font-family: "GeistMono";
      border-right: var(--border);

      &:hover {
        background-color: #1e1e1e;
        color: var(--code-class);
      }
    }

    & td:nth-child(2) {
      transition: var(--trans);
      font-family: "GeistMono";

      &:hover {
        background-color: #1e1e1e;
        color: var(--code-string);
      }
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