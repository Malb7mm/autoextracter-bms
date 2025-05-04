<script lang="ts">
  import "@/styles/styles.css";
  import { setupViewTransition } from 'sveltekit-view-transition';
  import { page } from '$app/stores';
  import { onMount } from "svelte";
  import { initialize } from "./initialize.svelte";

  const tabs = [
    { path: '/logs', label: 'ログ' },
    { path: '/settings', label: '設定' },
  ];

  setupViewTransition();
  
  onMount(() => {
    initialize();
  });
</script>

<header>
  {#each tabs as {path, label} }
    <a href="{path}" class="{$page.url.pathname === path ? "active" : ""}">
      {label}
    </a>
  {/each}
</header>
<main>
  <slot/>
</main>

<style>

:global(body) {
  background-color: var(--bg-color);
  overflow: hidden;
}

header {
  user-select: none;
  height: 30px;
  padding: 0 15px;

  display: flex;
  align-items: center;
  column-gap: 10px;

  background-color: var(--bg-color);
  border-bottom: solid 1px #aaa;
  box-shadow: 0 0 3px #aaa;
  font-size: 0.8rem;
  color: #222;
}

header > a {
  background-color: #aaa0;
  height: 100%;
  padding: 0 10px;
  transition: background-color 0.1s, color 0.1s, font-weight 0.1s, border-bottom 0.1s;

  display: flex;
  align-items: center;
  border-bottom: solid 2px #9790;
}

header > a.active {
  color: #979;
  font-weight: 700;

  box-sizing: content-box;
  border-bottom: solid 2px #979;
}

header > a:hover {
  background-color: #aaa3;
}

main {
  padding: 30px 50px;
}

</style>