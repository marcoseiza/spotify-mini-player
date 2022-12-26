<script lang="ts">
  import Spotify from "./lib/icons/Spotify.svelte";
  import {
    Play,
    SkipBack,
    SkipForward,
    HeartStraight,
    Shuffle,
  } from "phosphor-svelte";
  import { playback } from "./lib/state/playback";
  import { user } from "./lib/state/user";

  $: imageSrc = $playback?.item.album.images.at(1)?.url;

  $: numberOfArtists = $playback?.item.artists.length;

  $: setup = async () => {
    await playback.init();
    await user.login();
  };
</script>

<main class="container">
  {#await setup()}
    <div>Fetching...</div>
  {:then}
    <div class="logo">
      <Spotify />
    </div>
    <div class="cover-art">
      <img src={imageSrc} alt="" />
    </div>
    <div class="info">
      <h1>{$playback?.item.name}</h1>
      <h3>
        {#each $playback?.item.artists || [] as artist, index}
          <a
            href={artist.external_urls.spotify}
            target="_blank"
            rel="noopener noreferrer"
          >
            <span>
              {artist.name}{index < numberOfArtists - 1 ? "," : ""}
            </span>
          </a>
        {/each}
      </h3>
    </div>
    <div class="progress-bar">
      <div class="completed">
        <div class="knob" />
      </div>
    </div>
    <div class="controls">
      <div class="music-controls">
        <SkipBack size={20} weight="fill" />
        <Play size={20} weight="fill" />
        <SkipForward size={20} weight="fill" />
      </div>
      <div class="play-controls">
        <div class="thicc-button">
          <HeartStraight size={20} weight="regular" />
        </div>
        <div class="thicc-button">
          <Shuffle size={20} weight="regular" />
        </div>
      </div>
    </div>
  {/await}
</main>

<style>
  .container {
    position: relative;
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
  }

  .logo {
    position: absolute;
    top: 8px;
    left: 8px;
    --color: 200, 200, 200;
    color: rgb(var(--color));
    filter: drop-shadow(0px 0px 5px rgba(var(--color), 0.3));
  }

  .cover-art {
    width: 100%;
    aspect-ratio: 1/1;
    background-color: rgba(120, 120, 120, 0.2);
    border-radius: 5px;
    overflow: hidden;
  }

  .cover-art img {
    width: 100%;
  }

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

  .controls {
    flex-grow: 1;
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 0.7em;
  }

  .music-controls {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1.5em;
  }

  :global(.music-controls > *:hover) {
    color: #1db954;
  }

  .play-controls {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5em;
  }

  .thicc-button {
    padding: 0.3em 0.6em;
    background-color: rgba(200, 200, 200, 0.3);
    border-radius: 0.5em;
    display: flex;
    align-items: center;
  }

  .progress-bar {
    flex-shrink: 0;
    width: 100%;
    height: 2px;
    background-color: rgba(150, 150, 150);
    display: flex;
    border-radius: 1em;
    margin: 0.5em 0;
  }

  .completed {
    position: relative;
    background-color: white;
    width: 30%;
    border-radius: 1em;
  }

  .knob {
    position: absolute;
    top: 50%;
    right: 0;
    transform: translateY(-50%);
    width: 5px;
    height: 5px;
    border-radius: 1em;
    background-color: white;
  }
</style>
