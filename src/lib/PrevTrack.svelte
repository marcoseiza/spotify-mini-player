<script lang="ts">
  import { SkipBack } from "phosphor-svelte";
  import { appStore } from "./state";
</script>

<button on:click={appStore.prevTrack}>
  <SkipBack size={20} weight="fill" />
</button>
