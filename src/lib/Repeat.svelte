<script lang="ts">
  import RepeatContext from "./icons/RepeatContext.svelte";
  import RepeatTrack from "./icons/RepeatTrack.svelte";
  import { appStore } from "./state";
  import ThiccButton from "./ThiccButton.svelte";

  $: repeatState = $appStore.repeatState;
</script>

<div class={`repeat ${repeatState}`}>
  <ThiccButton onClick={appStore.cycleRepeatState}>
    {#if repeatState == "track"}
      <RepeatTrack size={20} />
    {:else}
      <RepeatContext size={20} />
    {/if}
  </ThiccButton>
</div>

<style>
  .repeat {
    color: var(--icon-color);
    display: flex;
  }
  .repeat:hover {
    color: var(--icon-color-hover);
  }
  .repeat.context,
  .repeat.track {
    color: var(--spotify-green);
  }
  .repeat.context:hover,
  .repeat.track:hover {
    color: var(--spotify-green-hover);
  }
</style>
