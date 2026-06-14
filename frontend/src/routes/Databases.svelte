<script lang="ts">
  import { onMount } from 'svelte';
  import DataTable from '../components/DataTable.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import SectionHeader from '../components/SectionHeader.svelte';
  import { getDatabaseRows, getDatabaseTables } from '../lib/api';
  import type { DatabaseRows, DatabaseTable } from '../lib/types';

  let tables: DatabaseTable[] = [];
  let selected = '';
  let tableRows: DatabaseRows | null = null;
  let loading = true;
  let error = '';

  onMount(loadTables);

  async function loadTables() {
    try {
      loading = true;
      tables = await getDatabaseTables();
      selected = tables[0]?.name ?? '';
      if (selected) await loadRows(selected);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Database request failed';
    } finally {
      loading = false;
    }
  }

  async function loadRows(table: string) {
    try {
      selected = table;
      tableRows = await getDatabaseRows(table);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Table request failed';
    }
  }
</script>

<div class="db-page">
  <aside class="panel compact connections">
    <SectionHeader eyebrow="SQLite" title="Internal database" description="Read-only viewer." />
    {#if loading}
      <p class="muted">Loading tables...</p>
    {:else}
      {#each tables as table}
      <button class:active={selected === table.name} type="button" on:click={() => loadRows(table.name)}>
        <strong>{table.name}</strong>
        <small>{table.row_count} rows</small>
      </button>
      {:else}
        <EmptyState title="No tables" description="Backend database endpoint is not responding." />
      {/each}
    {/if}
  </aside>

  <section class="panel browser">
    <SectionHeader
      eyebrow="Schema"
      title="Allowed tables"
      description="Only LightPanel internal tables are exposed."
    />
    <div class="schema-tree">
      {#each tables as table}
        <span>{table.name}</span>
      {/each}
    </div>
  </section>

  <section class="panel editor">
    {#if error}<p class="error">{error}</p>{/if}
    <SectionHeader
      eyebrow="Query"
      title="Read-only query editor"
      description="Placeholder only. Arbitrary SQL is intentionally disabled."
    />
    <textarea class="query" readonly>{`SELECT * FROM ${selected || 'table'} LIMIT 50;`}</textarea>
  </section>

  <section class="panel results">
    <SectionHeader
      eyebrow="Rows"
      title={tableRows?.table ?? 'Select table'}
      description="First 50 rows from allowlisted internal tables."
    />
    {#if tableRows}
      <DataTable columns={tableRows.columns} rows={tableRows.rows} />
    {:else}
      <EmptyState title="No table selected" description="Choose a table from the left sidebar." />
    {/if}
  </section>
</div>

<style>
  .db-page {
    display: grid;
    gap: var(--space-4);
    grid-template-columns: 250px 220px minmax(0, 1fr);
    grid-template-rows: auto minmax(360px, 1fr);
  }

  .results {
    grid-column: 2 / 4;
  }

  .connections,
  .schema-tree {
    display: grid;
    gap: var(--space-2);
  }

  .connections {
    grid-row: 1 / 3;
  }

  button,
  .schema-tree span {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    color: var(--text);
    padding: 10px;
    text-align: left;
  }

  button.active {
    border-color: rgb(110 231 183 / 45%);
  }

  small {
    color: var(--text-muted);
    display: block;
    margin-top: 4px;
  }

  .query {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    color: var(--text-muted);
    font-family: var(--font-mono);
    min-height: 118px;
    padding: var(--space-3);
    resize: vertical;
    width: 100%;
  }

  @media (max-width: 1100px) {
    .db-page {
      grid-template-columns: 1fr;
    }

    .results {
      grid-column: auto;
    }

    .connections {
      grid-row: auto;
    }
  }
</style>
