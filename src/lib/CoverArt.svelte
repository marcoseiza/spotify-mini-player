<script lang="ts">
  import { appStore } from "./state";

  $: imageSrc = $appStore.curr?.imageUrl;
</script>

<div class="cover-art">
  <a href={$appStore.curr?.contextUri}>
    <img src={imageSrc} alt="" />
  </a>
</div>

<style>
  .cover-art {
    flex-shrink: 0;
    width: 100%;
    aspect-ratio: 1/1;
    background-color: rgba(120, 120, 120, 0.2);
    border-radius: 5px;
    overflow: hidden;
  }

  .cover-art img {
    width: 100%;
  }
</style>
