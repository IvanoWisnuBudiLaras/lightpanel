<script lang="ts">
  import { onMount } from 'svelte';
  import ActionButton from '../components/ActionButton.svelte';
  import AppStatusBadge from '../components/AppStatusBadge.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import RuntimeBadge from '../components/RuntimeBadge.svelte';
  import SectionHeader from '../components/SectionHeader.svelte';
  import { getApps } from '../lib/api';
  import type { AppRecord } from '../lib/types';

  let apps: AppRecord[] = [];
  let error = '';

  onMount(async () => {
    try {
      apps = await getApps();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Apps request failed';
    }
  });
</script>

<div class="page">
  {#if error}
    <p class="error">{error}</p>
  {/if}

  <section class="panel">
    <div class="toolbar">
      <SectionHeader
        eyebrow="Applications"
        title="App inventory"
        description="Metadata only. Deployment actions remain explicit."
      />
      <ActionButton href="/deploy" label="Open deploy" variant="primary" />
    </div>

    {#if apps.length}
      <div class="table-shell">
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>Runtime</th>
              <th>Status</th>
              <th>Domain</th>
              <th>Port</th>
              <th>Last deploy</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each apps as app}
              <tr>
                <td><a href={`/apps/${app.id}`}>{app.name}</a></td>
                <td><RuntimeBadge name={app.runtime} status="installed" detail="configured" /></td>
                <td><AppStatusBadge status={app.status} /></td>
                <td>{app.primary_domain ?? 'unassigned'}</td>
                <td>{app.internal_port}</td>
                <td>{app.status === 'deployed' ? 'succeeded' : 'none'}</td>
                <td><ActionButton href={`/apps/${app.id}`} label="View" /></td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {:else}
      <EmptyState title="No apps registered" description="Create app metadata from the API before deployment." />
    {/if}
  </section>
</div>

<style>
  table {
    border-collapse: collapse;
    min-width: 920px;
    width: 100%;
  }

  th,
  td {
    border-bottom: 1px solid var(--border);
    padding: 12px;
    text-align: left;
  }

  th {
    color: var(--text-muted);
    font-size: 12px;
    text-transform: uppercase;
  }

  a {
    text-decoration: none;
  }
</style>
