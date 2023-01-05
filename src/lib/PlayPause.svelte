<script lang="ts">
  import { Pause, Play } from "phosphor-svelte";
  import { appStore } from "./state";
</script>

<button on:click={appStore.playPause}>
  {#if $appStore.playing}
    <Pause size={20} weight="fill" />
  {:else}
    <Play size={20} weight="fill" />
  {/if}
</button>
