<script lang="ts" generics="T">
  import { onMount } from "svelte";

  export let options: [T, string][];
  export let value: string = "";
  export let onUpdate: (value: string) => void = (v) => {};
</script>

<div class="container">
  <select bind:value onchange={() => {onUpdate(value)}}>
    {#each options as [optKey, optValue]}
      <option value="{optKey}">{optValue}</option>
    {/each}
  </select>
</div>

<style>
  .container {
    position: relative;
    display: flex;
    column-gap: 10px;
    padding: 5px 5px;
    outline-offset: -3px;
    outline: solid 2px #ddd;
    width: fit-content;
  }

  select {
    background-color: #ddd;
    padding: 5px 30px 5px 10px;
    font-size: 0.7rem;
    box-shadow: 0 0 3px #0003 inset;
    border: solid 1px #888;
  }

  select:focus-within {
    border: solid 1px #000;
  }

  .container::after {
    pointer-events: none;
    content: "";
    z-index: 10;
    width: 12px;
    height: 8px;
    clip-path: polygon(
      50% 0,
      0 100%,
      100% 100%
    );
    background-color: #0006;
    transform: rotateZ(180deg);
    position: absolute;
    right: 14px;
    top: 14px;
  }

  .container:focus-within::after {
    background-color: #000a;
  }
</style>