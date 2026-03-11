<script lang="ts">
  import { goto } from "$app/navigation";

  function onClick() {
    goto("/garden");
  }
</script>

<button class="gh-button" onclick={onClick}>
  <div>
    <span>To Garden</span> <strong>→</strong>
  </div>
</button>

<style lang="scss">
  @property --rotation {
    syntax: "<angle>";
    inherits: false;
    initial-value: 0deg;
  }

  @keyframes rotate {
    to {
      --rotation: 360deg;
    }
  }

  @mixin gradient-bg($opacity) {
    position: relative;
    &:before{
      content: "";
      display: block;
      background: conic-gradient(from var(--rotation), #f390fc, #f799ff, #f8a7ff, #f9bcff);
      filter: blur(20px);
      opacity: $opacity;
      position: absolute;
      inset: 4px;
      
      animation: rotate 5s linear infinite;
      transition: opacity 0.3s ease-in-out;
    }
  }
  .gh-button {
    border: 2px solid var(--pink);
    width: 100%; height: 100%;

    font-family: "Geist";
    font-size: 1.4rem;
    margin: var(--margin);
    
    display: flex;
    align-items: center;
    justify-content: center;

    background-color: var(--l-pink);
    color: var(--sat-pink);
    cursor: pointer;
    transition: all 0.1s ease-in-out;
    position: relative;

    & div {
      position: relative;
      display: flex;
      flex-direction: row;
      width: 100%;
    
      & strong {
        transition: scale 0.3s ease-in-out;
        position: absolute;
        right: 0;
      }
    }

    &:hover { 
      @include gradient-bg(1);
    }
    
    &:hover div strong {
      scale: 1.2;
    }
  }
</style>