<script lang="ts">
  import { onMount } from 'svelte';
  import LogViewer from '../components/LogViewer.svelte';
  import SectionHeader from '../components/SectionHeader.svelte';
  import { getAppDeployLog, getApps, getErrorLogs, getPanelDeployLogs } from '../lib/api';
  import { logLines } from '../lib/mock-data';
  import type { AppRecord, LogSnapshot } from '../lib/types';

  let tab = 'errors';
  let filter = '';
  let apps: AppRecord[] = [];
  let selectedId = '';
  let snapshot: LogSnapshot | null = null;
  let loading = true;
  let error = '';

  $: rawLines = tab === 'activity' ? logLines : snapshot?.lines ?? [];
  $: lines = rawLines.filter((line) => line.toLowerCase().includes(filter.toLowerCase()));
  $: source = tab === 'activity' ? 'SQLite audit_logs placeholder' : snapshot?.source ?? 'not loaded';

  onMount(async () => {
    try {
      apps = await getApps();
      selectedId = apps[0]?.id ?? '';
      await loadLogs();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Logs request failed';
    } finally {
      loading = false;
    }
  });

  async function switchTab(next: string) {
    tab = next;
    await loadLogs();
  }

  async function loadLogs() {
    if (tab === 'activity') {
      snapshot = null;
      return;
    }

    error = '';
    loading = true;

    try {
      if (tab === 'errors') snapshot = await getErrorLogs();
      if (tab === 'deploy') snapshot = await getPanelDeployLogs();
      if (tab === 'app' && selectedId) snapshot = await getAppDeployLog(selectedId);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Log fetch failed';
    } finally {
      loading = false;
    }
  }
</script>

<div class="page">
  <section class="panel">
    <SectionHeader
      eyebrow="Logs"
      title="Infrastructure console"
      description="Bounded read-only log snapshots. No browser terminal."
    />

    {#if error}<p class="error">{error}</p>{/if}
    {#if snapshot?.warning}<p class="warning">{snapshot.warning}</p>{/if}

    <div class="tabs">
      {#each ['errors', 'deploy', 'app', 'activity'] as item}
        <button class:active={tab === item} type="button" on:click={() => switchTab(item)}>
          {item}
        </button>
      {/each}
    </div>

    <div class="filter">
      <input class="field" bind:value={filter} placeholder="Filter log lines" />
      {#if tab === 'app'}
        <select class="field" bind:value={selectedId} on:change={loadLogs}>
          {#each apps as app}<option value={app.id}>{app.name}</option>{/each}
        </select>
      {/if}
      <span>{loading ? 'loading' : source}</span>
    </div>

    {#if lines.length}
      <LogViewer {lines} />
    {:else}
      <LogViewer lines={[loading ? 'loading log snapshot...' : 'no log lines available']} />
    {/if}
  </section>
</div>

<style>
  .tabs,
  .filter {
    align-items: center;
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
    margin-bottom: var(--space-3);
  }

  button {
    background: var(--bg-panel-soft);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    color: var(--text-muted);
    min-height: 34px;
    padding: 0 12px;
    text-transform: capitalize;
  }

  button.active {
    border-color: rgb(110 231 183 / 35%);
    color: var(--accent);
  }

  input { min-width: 260px; }
  select { min-width: 180px; }

  span,
  .warning {
    color: var(--text-muted);
    font-size: 13px;
  }

  .warning { color: var(--amber); }
</style>
