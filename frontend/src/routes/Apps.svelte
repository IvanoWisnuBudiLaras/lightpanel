<script lang="ts">
  import { onMount } from 'svelte';
  import ActionButton from '../components/ActionButton.svelte';
  import AppStatusBadge from '../components/AppStatusBadge.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import RuntimeBadge from '../components/RuntimeBadge.svelte';
  import SectionHeader from '../components/SectionHeader.svelte';
  import { createApp, deleteApp, getApps } from '../lib/api';
  import type { AppPayload, AppRecord } from '../lib/types';

  const runtimes = ['static', 'node', 'php', 'python', 'rust', 'go', 'java', 'dotnet'];

  let apps: AppRecord[] = [];
  let error = '';
  let message = '';
  let saving = false;
  let form: AppPayload = {
    name: '',
    runtime: 'static',
    internal_port: 3001,
    primary_domain: null,
  };

  onMount(loadApps);

  async function loadApps() {
    try {
      apps = await getApps();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Apps request failed';
    }
  }

  async function submitApp() {
    error = '';
    message = '';
    saving = true;

    try {
      await createApp({
        ...form,
        primary_domain: form.primary_domain || null,
        internal_port: Number(form.internal_port),
      });
      message = 'App metadata created.';
      form = { name: '', runtime: 'static', internal_port: 3001, primary_domain: null };
      await loadApps();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Create app failed';
    } finally {
      saving = false;
    }
  }

  async function removeApp(id: string) {
    error = '';
    message = '';

    try {
      await deleteApp(id);
      message = 'App soft deleted.';
      await loadApps();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Delete app failed';
    }
  }
</script>

<div class="page">
  {#if error}<p class="error">{error}</p>{/if}
  {#if message}<p class="success">{message}</p>{/if}

  <section class="panel">
    <SectionHeader
      eyebrow="Write"
      title="Create app metadata"
      description="This only writes internal SQLite metadata. It does not deploy yet."
    />
    <form class="app-form" on:submit|preventDefault={submitApp}>
      <input class="field" bind:value={form.name} placeholder="app-name" required />
      <select class="field" bind:value={form.runtime}>
        {#each runtimes as runtime}
          <option value={runtime}>{runtime}</option>
        {/each}
      </select>
      <input class="field" type="number" min="1" max="65535" bind:value={form.internal_port} />
      <input class="field" bind:value={form.primary_domain} placeholder="domain optional" />
      <button class="primary" type="submit" disabled={saving}>
        {saving ? 'Creating...' : 'Create app'}
      </button>
    </form>
  </section>

  <section class="panel">
    <div class="toolbar">
      <SectionHeader
        eyebrow="Applications"
        title="App inventory"
        description="Soft delete keeps records out of the active list."
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
                <td class="actions">
                  <ActionButton href={`/apps/${app.id}`} label="View" />
                  <ActionButton href="/domains" label="Domain" />
                  <button type="button" on:click={() => removeApp(app.id)}>Delete</button>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {:else}
      <EmptyState title="No apps registered" description="Create app metadata above before deployment." />
    {/if}
  </section>
</div>

<style>
  .app-form {
    display: grid;
    gap: var(--space-3);
    grid-template-columns: 1.1fr .8fr .6fr 1fr auto;
  }

  table {
    border-collapse: collapse;
    min-width: 980px;
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

  button {
    background: var(--bg-panel-soft);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    color: var(--text);
    min-height: 34px;
    padding: 0 12px;
  }

  .primary {
    background: var(--accent);
    color: #03120c;
    font-weight: 700;
  }

  .actions {
    display: flex;
    gap: var(--space-2);
  }

  .success {
    color: var(--green);
  }
</style>
