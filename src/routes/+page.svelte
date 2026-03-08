<script lang="ts">

  let slack_id = $state("Not logged in.");
  
</script>

<main class="container">
  <div class="auth">
    <div class="id">
      Your ID: {slack_id}
    </div>
    <button>
      login with hack club
    </button>
  </div>
</main>

<style>


:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}


</style>
