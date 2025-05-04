<script lang="ts">
  import { fade } from "svelte/transition";
  import { logs } from './logMessages.svelte';
</script>

<div class="container">
  <div class="table-wrapper">
    <table class="table">
      <thead>
        <tr>
          <td>
            レベル
          </td>
          <td>
            メッセージ
          </td>
        </tr>
      </thead>
      <tbody>
        {#if logs.length == 0}
          <tr>
            <td class="gray">
              -
            </td>
            <td class="gray">
              ここにログが出力されます
            </td>
          </tr>
        {:else}
          {#each logs as {id, message, jumpTo, type} (id)}
            <tr transition:fade={{duration: 100}} class="{type}">
              <td>
                {type}
              </td>
              <td class="selectable">
                {#if jumpTo}
                  <a href="{jumpTo}">{message}</a>
                {:else}
                  {message}
                {/if}
              </td>
            </tr>
          {/each}
        {/if}
      </tbody>
    </table>
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;

    height: calc(100vh - 160px);
  }

  .table-wrapper {
    box-shadow: 0 0 3px #0004;
    border-radius: 8px;
    width: 100%;

    overflow-y: scroll;
    max-height: calc(100vh - 40px);
  }

  .table {
    font-size: 0.9rem;
    row-gap: 2px;
  }

  .table td {
    user-select: none;
    padding: 3px 20px;
  }

  .table td:nth-child(1) {
    width: 100px;
    text-align: center;
  }

  .table thead td {
    position: sticky;
    top: 0;
    border-radius: 4px;
    font-size: 0.7rem;
    background-color: var(--bg-color);
  }

  .table td.selectable {
    user-select: text;
  }
  
  .table td.gray {
    color: #888;
  }
</style>