<script lang="ts">
  import { onMount } from 'svelte';
  import AppStatusBadge from '../components/AppStatusBadge.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import SectionHeader from '../components/SectionHeader.svelte';
  import StatusCard from '../components/StatusCard.svelte';
  import { getApps, getNginxPreview, getSslPreview } from '../lib/api';
  import type { AppRecord, NginxPreview, SslPreview } from '../lib/types';

  let apps: AppRecord[] = [];
  let selectedId = '';
  let nginx: NginxPreview | null = null;
  let ssl: SslPreview | null = null;
  let loading = true;
  let previewing = false;
  let error = '';

  $: selectedApp = apps.find((app) => app.id === selectedId);

  onMount(async () => {
    try {
      apps = await getApps();
      selectedId = apps[0]?.id ?? '';
      if (selectedId) await loadPreview();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Domain request failed';
    } finally {
      loading = false;
    }
  });

  async function loadPreview() {
    if (!selectedId) return;
    error = '';
    previewing = true;
    nginx = null;
    ssl = null;

    try {
      [nginx, ssl] = await Promise.all([
        getNginxPreview(selectedId),
        getSslPreview(selectedId),
      ]);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Preview failed';
    } finally {
      previewing = false;
    }
  }
</script>

<div class="page">
  {#if error}<p class="error">{error}</p>{/if}

  <section class="panel">
    <div class="toolbar">
      <SectionHeader
        eyebrow="Domains"
        title="Domain and SSL manager"
        description="Preview-only Nginx and acme.sh planning. No host files are changed."
      />
      <button class="primary" type="button" on:click={loadPreview} disabled={previewing || !selectedId}>
        {previewing ? 'Loading preview' : 'Refresh preview'}
      </button>
    </div>

    {#if loading}
      <p class="muted">Loading apps...</p>
    {:else if apps.length}
      <div class="selector">
        <select class="field" bind:value={selectedId} on:change={loadPreview}>
          {#each apps as app}
            <option value={app.id}>{app.name} - {app.primary_domain ?? 'no domain'}</option>
          {/each}
        </select>
        {#if selectedApp}<AppStatusBadge status={selectedApp.status} />{/if}
      </div>
    {:else}
      <EmptyState title="No apps" description="Create an app and assign a primary domain first." />
    {/if}
  </section>

  <div class="metric-grid">
    <StatusCard label="Primary domain" value={selectedApp?.primary_domain ?? 'unassigned'} detail="Required for Nginx and SSL preview" />
    <StatusCard label="Internal port" value={selectedApp ? String(selectedApp.internal_port) : 'n/a'} detail="Nginx proxy target" />
    <StatusCard label="SSL mode" value={ssl?.mode ?? 'preview_only'} detail="acme.sh is not executed yet" />
  </div>

  <div class="two-column">
    <section class="panel">
      <SectionHeader
        eyebrow="Nginx"
        title="Generated site config"
        description={nginx ? `${nginx.available_path}` : 'Select an app with a domain.'}
      />
      {#if nginx}
        <pre>{nginx.config}</pre>
      {:else}
        <EmptyState title="No Nginx preview" description="The selected app needs a primary domain." />
      {/if}
    </section>

    <section class="panel">
      <SectionHeader
        eyebrow="SSL"
        title="Certificate plan"
        description={ssl ? ssl.certificate_dir : 'acme.sh preview is not ready.'}
      />
      {#if ssl}
        <div class="stack">
          {#each ssl.commands as command}
            <code>{command.program} {command.args.join(' ')}</code>
          {/each}
        </div>
      {:else}
        <EmptyState title="No SSL preview" description="Domain and current release path are required." />
      {/if}
    </section>
  </div>
</div>

<style>
  .selector {
    align-items: center;
    display: flex;
    gap: var(--space-3);
    margin-top: var(--space-4);
  }

  select { min-width: 320px; }

  button {
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    min-height: 36px;
    padding: 0 14px;
  }

  .primary {
    background: var(--accent);
    color: #03120c;
    font-weight: 700;
  }

  pre,
  code {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    color: var(--text);
    display: block;
    overflow-x: auto;
    padding: var(--space-3);
    white-space: pre-wrap;
  }
</style>
