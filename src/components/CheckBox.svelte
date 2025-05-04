<script lang="ts">
  import { onMount } from "svelte";
  import { v4 as uuidv4 } from "uuid";

  export let value: boolean = false;
  export let onUpdate: (value: boolean) => void = (v) => {};
  
  const uid = uuidv4();
</script>

<div class="container">
  <input id="checkbox-{uid}" type="checkbox" bind:checked={value} onchange={() => {onUpdate(value)}}/>
  <label for="checkbox-{uid}">
    <p>
      <slot/>
    </p>
  </label>
</div>

<style>
  .container {
    position: relative;
    display: flex;
    column-gap: 10px;
  }

  p {
    line-height: 1.2lh;
  }
</style>