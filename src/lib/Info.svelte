<script lang="ts">
  import { appStore } from "./state";
  import type { Artist } from "./state/artist";

  const getArtistLink = (artist: Artist) => {
    const url = artist.external_urls["spotify"];
    if (!url) return;
    return `spotify:artist:${artist.id}`;
  };

  $: albumLink = $appStore.curr?.album?.id
    ? `spotify:album:${$appStore.curr?.album?.id}`
    : undefined;
</script>

<div class="info">
  <a href={albumLink}>
    <h1>{$appStore.curr?.name}</h1>
  </a>
  <h3>
    {#if !$appStore.curr || $appStore.curr.artists.length == 0}
      <div class="skeleton-inline" data-placeholder="first last" />
    {:else}
      {#each $appStore.curr.artists as artist, index}
        <a href={getArtistLink(artist)}>
          <span>
            {artist.name}{index < $appStore.curr.artists.length - 1 ? "," : ""}
          </span>
        </a>
      {/each}
    {/if}
  </h3>
</div>

<style>
  .info {
    margin-top: 0.5em;
    display: flex;
    flex-direction: column;
  }

  .info h1,
  .info h3 {
    width: 100%;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
