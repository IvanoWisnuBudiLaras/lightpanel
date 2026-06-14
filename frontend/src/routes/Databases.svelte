<script lang="ts">
  import DataTable from '../components/DataTable.svelte';
  import SectionHeader from '../components/SectionHeader.svelte';
  import { databaseConnections } from '../lib/mock-data';

  const rows = [
    { column: 'id', type: 'TEXT', nullable: 'no' },
    { column: 'name', type: 'TEXT', nullable: 'no' },
    { column: 'runtime', type: 'TEXT', nullable: 'no' },
  ];
</script>

<div class="db-page">
  <aside class="panel compact connections">
    <SectionHeader eyebrow="Connections" title="Database manager" description="Placeholder only." />
    {#each databaseConnections as connection}
      <button type="button">
        <strong>{connection.name}</strong>
        <small>{connection.type} · {connection.status}</small>
      </button>
    {/each}
  </aside>

  <section class="panel browser">
    <SectionHeader eyebrow="Schema" title="Table browser" description="DBeaver-inspired lightweight layout." />
    <div class="schema-tree">
      <span>main</span>
      <span>apps</span>
      <span>deployments</span>
      <span>audit_logs</span>
    </div>
  </section>

  <section class="panel editor">
    <SectionHeader eyebrow="Query" title="Query editor" description="Execution is not implemented." />
    <pre>SELECT id, name, runtime FROM apps LIMIT 20;</pre>
    <DataTable columns={['column', 'type', 'nullable']} {rows} />
  </section>
</div>

<style>
  .db-page {
    display: grid;
    gap: var(--space-4);
    grid-template-columns: 260px 220px minmax(0, 1fr);
  }

  .connections,
  .schema-tree {
    display: grid;
    gap: var(--space-2);
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

  small {
    color: var(--text-muted);
    display: block;
    margin-top: 4px;
  }

  pre {
    background: #05070a;
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    margin: 0 0 var(--space-4);
    padding: var(--space-4);
  }

  @media (max-width: 1100px) {
    .db-page {
      grid-template-columns: 1fr;
    }
  }
</style>
