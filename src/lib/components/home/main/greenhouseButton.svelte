<script lang="ts">
  import { goto } from "$app/navigation";

  function onClick() {
    goto("/greenhouse");
  }
</script>

<button class="gh-button" onclick={onClick}>
  <div>
    <span>To Greenhouse</span> <strong>→</strong>
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
      background: conic-gradient(from var(--rotation), #84e458, #9cdf7e, #a7f185, #b0ff8c);
      filter: blur(20px);
      opacity: $opacity;
      position: absolute;
      inset: 4px;
      
      animation: rotate 5s linear infinite;
      transition: opacity 0.3s ease-in-out;
    }
  }
  
  .gh-button {
    border: 2px solid var(--green);
    width: 100%; height: 100%;

    font-family: "Geist";
    font-size: 1.4rem;
    margin: var(--margin);
    
    display: flex;
    align-items: center;
    justify-content: center;

    background-color: var(--l-green);
    color: var(--sat-green);
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