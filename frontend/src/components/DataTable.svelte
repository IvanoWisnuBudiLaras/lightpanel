<script lang="ts">
  export let columns: string[] = [];
  export let rows: Record<string, string>[] = [];
</script>

<div class="table-wrap">
  <table>
    <thead>
      <tr>
        {#each columns as column}
          <th>{column}</th>
        {/each}
      </tr>
    </thead>
    <tbody>
      {#each rows as row}
        <tr>
          {#each columns as column}
            <td>{row[column] ?? ''}</td>
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .table-wrap {
    overflow-x: auto;
  }

  table {
    border-collapse: collapse;
    min-width: 640px;
    width: 100%;
  }

  th,
  td {
    border-bottom: 1px solid var(--border);
    padding: 11px 12px;
    text-align: left;
  }

  th {
    color: var(--text-muted);
    font-size: 12px;
    text-transform: uppercase;
  }

  td {
    color: var(--text);
  }
</style>
