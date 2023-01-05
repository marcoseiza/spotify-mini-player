<script lang="ts">
  import Spotify from "./lib/icons/Spotify.svelte";
  import { onMount } from "svelte";
  import CoverArt from "./lib/CoverArt.svelte";
  import { appStore } from "./lib/state";
  import Info from "./lib/Info.svelte";
  import Controls from "./lib/Controls.svelte";
  import Progress from "./lib/Progress.svelte";

  onMount(() => {
    (async () => {
      await appStore.init();
    })();
  });
</script>

<main class="container">
  <Spotify class="logo" />
  <CoverArt />
  <Info />
  <Progress />
  <Controls />
</main>

<style>
  .container {
    position: relative;
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
  }

  .container :global(> .logo) {
    position: absolute;
    top: 8px;
    left: 8px;
    --color: 200, 200, 200;
    color: rgb(var(--color));
    filter: drop-shadow(0px 0px 5px rgba(var(--color), 0.3));
  }
</style>
