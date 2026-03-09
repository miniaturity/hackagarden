<script lang="ts">

</script>

<div class="loading">
  <div class="l__content">
    loading...
  </div>
</div>

<style lang="scss">
  @property --rotation {
    syntax: "<angle>";
    inherits: false;
    initial-value: 0deg;
  }

  :global(body) {
    margin: 0; padding: 0;
  }

  @keyframes rotate {
    to {
      --rotation: 360deg;
    }
  }
  @mixin gradient-bg($opacity) {
    &:before{
      content: "";
      display: block;
      background: conic-gradient(from var(--rotation), purple, blue, green, yellow, orange, red);
      filter: blur(20px);
      opacity: $opacity;
      position: absolute;
      inset: 4px;
      z-index: -2;
      animation: rotate 5s linear infinite;
      transition: opacity 0.3s ease-in-out;
    }
  }
  .loading {
    position: absolute;
    top: 0; left: 0;
    width: 100%; height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .l__content {
    position: relative;
    padding: var(--margin);
    background: var(--bg-col-l);
    border: var(--border);
    font-size: 1.5rem;
    font-family: "GeistMono";

    @include gradient-bg(1);
  }
</style>