<script lang="ts">
  type OnClick = (
    event: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement }
  ) => any;

  export let onClick: OnClick = () => {};
</script>

<button class={"thicc-button"} on:click={(e) => onClick(e)}>
  <slot />
</button>

<style>
  .thicc-button {
    padding: 0.3em 0.6em;
    background-color: rgba(200, 200, 200, 0.3);
    border-radius: 0.5em;
    display: flex;
    align-items: center;
  }
</style>
