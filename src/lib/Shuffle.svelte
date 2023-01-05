<script lang="ts">
  import { Shuffle } from "phosphor-svelte";
  import { appStore } from "./state";
  import ThiccButton from "./ThiccButton.svelte";
</script>

<div class={`shuffle ${$appStore.shuffle ? "shuffle-on" : "shuffle-off"}`}>
  <ThiccButton onClick={appStore.toggleShuffle}>
    <Shuffle size={20} weight="regular" />
  </ThiccButton>
</div>

<style>
  .shuffle {
    color: var(--icon-color);
    display: flex;
  }
  .shuffle:hover {
    color: var(--icon-color-hover);
  }
  .shuffle.shuffle-on {
    color: var(--spotify-green);
  }
  .shuffle.shuffle-on:hover {
    color: var(--spotify-green-hover);
  }
</style>
