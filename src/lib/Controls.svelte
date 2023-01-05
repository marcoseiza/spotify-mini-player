<script lang="ts">
  import NextTrack from "./NextTrack.svelte";
  import PlayPause from "./PlayPause.svelte";
  import PrevTrack from "./PrevTrack.svelte";
  import Repeat from "./Repeat.svelte";
  import Save from "./Save.svelte";
  import Shuffle from "./Shuffle.svelte";
</script>

<div class="controls">
  <div class="music-controls">
    <PrevTrack />
    <PlayPause />
    <NextTrack />
  </div>
  <div class="play-controls">
    <Save />
    <Shuffle />
    <Repeat />
  </div>
</div>

<style>
  .controls {
    flex-grow: 1;
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 0.7em;
    color: var(--icon-color);
  }

  .music-controls {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1.5em;
  }

  :global(.music-controls > *:hover) {
    color: var(--icon-color-hover);
  }

  .play-controls {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5em;
  }
</style>
