<header>
  <button class="logs">
    ログ
  </button>
  <button class="settings">
    設定
  </button>
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

header > button {
  background-color: #aaa0;
  height: 100%;
  padding: 0 10px;
  transition: background-color 0.1s;
}

header > button:hover {
  background-color: #aaa3;
}

</style>