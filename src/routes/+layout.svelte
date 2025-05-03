<script>
  import "@/styles/styles.css";
  import { page } from '$app/stores';

  const tabs = [
    { path: '/logs', label: 'ログ' },
    { path: '/settings', label: '設定' },
  ];
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
  background-color: hsl(0, 0%, 97%);
}

header {
  user-select: none;
  height: 30px;
  padding: 0 15px;

  display: flex;
  align-items: center;
  column-gap: 10px;

  background-color: hsl(0, 0%, 97%);
  border-bottom: solid 1px #aaa;
  box-shadow: 0 0 3px #aaa;
  font-size: 0.8rem;
  color: #222;
}

header > a {
  background-color: #aaa0;
  height: 100%;
  padding: 0 10px;
  transition: background-color 0.1s, color 0.1s, font-weight 0.1s;

  display: flex;
  align-items: center;
}

header > a.active {
  color: #949;
  font-weight: 700;
}

header > a:hover {
  background-color: #aaa3;
}

</style>