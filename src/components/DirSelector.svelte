<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";

  export let value: string = "";
  export let onUpdate: (value: string) => void = (v) => {};
  export let disabled: boolean = false;

  let isInvalid = false;

  const openDialog = () => {
    onUpdate(value);
  }

  onMount(() => {
    // TODO: value初期化
  });
</script>

<div class="container">
  <input type="text" placeholder="{value}" bind:value onchange={() => {onUpdate(value)}} readonly={disabled}/>
  <button onclick={openDialog} disabled={disabled}>
    ...
  </button>
  <div class="error {isInvalid ? "" : "hide"}" transition:fade={{duration: 100}}>
    無効なディレクトリです
  </div>
</div>

<style>
  .container {
    position: relative;
    display: flex;
    padding: 5px 5px;
    outline-offset: -3px;
    outline: solid 2px #ddd;
  }
  
  input[type="text"] {
    background: #eee;
    padding: 5px 5px;
    font-size: 0.7rem;
    box-shadow: 0 0 3px #0003 inset;
    border: solid 1px #888;
    width: 100%;
  }

  input[type="text"]:focus {
    border: solid 1px #000;
  }

  button {
    margin-left: 0px;
    padding: 0 8px;
    border-radius: 3px;
    background-color: #ccc;
    box-shadow: 0 0 3px #0005 inset;
    border: solid 1px #888;
    transition: all 0.1s;
  }

  button:hover {
    background-color: #bbb;
  }

  button:active {
    background-color: #888;
    box-shadow: 0 0 5px #fffa inset;
    color: white;
  }

  .error {
    pointer-events: none;
    position: absolute;
    right: 40px;
    top: 8px;
    height: 50%;
    background-color: #faa6;
    border: solid 1px #f666;
    color: #600;
  }

  .error.hide {
    display: none;
  }
</style>