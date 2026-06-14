<script lang="ts">
  import { onMount } from 'svelte';
  import ActionButton from '../components/ActionButton.svelte';
  import AppStatusBadge from '../components/AppStatusBadge.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import SectionHeader from '../components/SectionHeader.svelte';
  import StatusCard from '../components/StatusCard.svelte';
  import { getApps } from '../lib/api';
  import { deploymentHealth } from '../lib/mock-data';
  import type { AppRecord } from '../lib/types';

  let apps: AppRecord[] = [];
  let error = '';

  onMount(async () => {
    try {
      apps = await getApps();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Deploy request failed';
    }
  });
</script>

<div class="page">
  {#if error}
    <p class="error">{error}</p>
  {/if}

  <section class="panel">
    <SectionHeader
      eyebrow="Deployment"
      title="Adapter readiness"
      description="Local source deployment only. No Git, Nginx, or systemd execution here."
    />
    <div class="metric-grid">
      {#each deploymentHealth as item}
        <StatusCard label={item.label} value={item.value} detail={item.detail} />
      {/each}
    </div>
  </section>

  <section class="panel">
    <div class="toolbar">
      <SectionHeader
        eyebrow="Targets"
        title="Deployable apps"
        description="Actions are placeholders until forms are wired."
      />
      <ActionButton label="Static source" variant="primary" />
    </div>

    {#if apps.length}
      <div class="deploy-list">
        {#each apps as app}
          <div class="deploy-row">
            <span>
              <strong>{app.name}</strong>
              <small>{app.runtime} · /opt/lightpanel/data/apps/{app.name}</small>
            </span>
            <div class="actions">
              <AppStatusBadge status={app.status} />
              <ActionButton href={`/apps/${app.id}`} label="Inspect" />
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <EmptyState title="No deployment targets" description="Create app metadata before using adapters." />
    {/if}
  </section>
</div>

<style>
  .deploy-list {
    display: grid;
    gap: 10px;
  }

  .deploy-row {
    align-items: center;
    border-bottom: 1px solid var(--border);
    display: flex;
    justify-content: space-between;
    padding: 12px 0;
  }

  .actions,
  span {
    align-items: center;
    display: flex;
    gap: 4px;
  }

  span {
    align-items: flex-start;
    display: grid;
  }

  small {
    color: var(--text-muted);
  }
</style>
