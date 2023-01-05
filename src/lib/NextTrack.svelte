<script lang="ts">
  import { SkipForward } from "phosphor-svelte";
  import { appStore } from "./state";
</script>

<button on:click={appStore.nextTrack}>
  <SkipForward size={20} weight="fill" />
</button>
