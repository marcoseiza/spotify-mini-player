<script lang="ts">
  import { HeartStraight } from "phosphor-svelte";
  import { appStore } from "./state";
  import ThiccButton from "./ThiccButton.svelte";

  $: saved = $appStore.curr?.saved || false;
</script>

<div class={`heart ${saved ? "saved" : "not-saved"}`}>
  <ThiccButton onClick={appStore.toggleSaved}>
    <HeartStraight size={20} weight={saved ? "fill" : "regular"} />
  </ThiccButton>
</div>

<style>
  .heart {
    color: var(--icon-color);
    display: flex;
  }
  .heart:hover {
    color: var(--icon-color-hover);
  }
  .heart.saved {
    color: var(--spotify-green);
  }
  .heart.saved:hover {
    color: var(--spotify-green-hover);
  }
</style>
