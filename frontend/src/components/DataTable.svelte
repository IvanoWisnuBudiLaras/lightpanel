<script lang="ts">
  export let columns: string[] = [];
  export let rows: Record<string, string>[] = [];
</script>

<div class="table-wrap">
  <table>
    <thead>
      <tr>
        <th class="row-index">#</th>
        {#each columns as column}
          <th>{column}</th>
        {/each}
      </tr>
    </thead>
    <tbody>
      {#each rows as row, index}
        <tr>
          <td class="row-index">{index + 1}</td>
          {#each columns as column}
            <td>{row[column] ?? ''}</td>
          {/each}
        </tr>
      {:else}
        <tr>
          <td colspan={columns.length + 1}>No rows</td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .table-wrap {
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    max-height: 560px;
    overflow-x: auto;
    overflow-y: auto;
  }

  table {
    border-collapse: collapse;
    min-width: 640px;
    width: 100%;
  }

  th,
  td {
    border-bottom: 1px solid var(--border);
    border-right: 1px solid var(--border);
    padding: 11px 12px;
    text-align: left;
    vertical-align: top;
  }

  th {
    background: var(--bg-panel-soft);
    color: var(--text-muted);
    font-size: 12px;
    position: sticky;
    top: 0;
    text-transform: uppercase;
    z-index: 1;
  }

  td {
    color: var(--text);
    font-family: var(--font-mono);
    font-size: 12px;
    max-width: 320px;
    white-space: nowrap;
  }

  .row-index {
    background: var(--bg-elevated);
    color: var(--text-faint);
    left: 0;
    position: sticky;
    width: 48px;
    z-index: 2;
  }
</style>
