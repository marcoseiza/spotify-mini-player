<script lang="ts">
  import { millisecondsToTime } from "./helpers";
  import { appStore } from "./state";

  $: progress = $appStore.progressMs;
  $: progressString = millisecondsToTime(progress);
  $: duration = $appStore.curr?.durationMs || 1;
  $: durationString = millisecondsToTime(duration);
  $: percent = (progress / duration) * 100;
</script>

<div class="progress-bar">
  <div class="completed" style={`--progress:${percent}%`}>
    <div class="knob" />
  </div>
</div>
<div class="progress-info">
  <div class="progress">
    <span>{progressString}</span>
  </div>
  <div class="duration">
    <span>{durationString}</span>
  </div>
</div>

<style>
  .progress-bar {
    position: relative;
    flex-shrink: 0;
    width: 100%;
    height: 2px;
    background-color: rgba(150, 150, 150);
    display: flex;
    border-radius: 1em;
    margin-top: 0.5em;
    margin-bottom: 0.4em;
  }

  .completed {
    position: relative;
    width: var(--progress);
    padding-right: 5px;
    background-color: white;
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

  .progress-info {
    display: flex;
    justify-content: space-between;
  }

  .progress,
  .duration {
    font-size: 0.7em;
    top: 0.5em;
  }

  .duration {
    right: 0;
  }
</style>
