<script lang="ts">
  import { onMount } from 'svelte';
  import ActionButton from '../components/ActionButton.svelte';
  import AppStatusBadge from '../components/AppStatusBadge.svelte';
  import RuntimeBadge from '../components/RuntimeBadge.svelte';
  import SectionHeader from '../components/SectionHeader.svelte';
  import StatusCard from '../components/StatusCard.svelte';
  import { getApps } from '../lib/api';
  import type { AppRecord } from '../lib/types';

  export let id = '';

  let app: AppRecord | undefined;
  let error = '';

  onMount(async () => {
    try {
      app = (await getApps()).find((item) => item.id === id);
    } catch (err) {
      error = err instanceof Error ? err.message : 'App request failed';
    }
  });
</script>

<div class="page">
  {#if error}
    <p class="error">{error}</p>
  {:else if app}
    <section class="panel heading">
      <div>
        <SectionHeader
          eyebrow="Application"
          title={app.name}
          description={app.primary_domain ?? 'No domain assigned'}
        />
        <RuntimeBadge name={app.runtime} status="installed" detail="configured" />
      </div>
      <div class="actions">
        <AppStatusBadge status={app.status} />
        <ActionButton href="/deploy" label="Deploy" />
      </div>
    </section>

    <div class="grid">
      <StatusCard label="Runtime" value={app.runtime} />
      <StatusCard label="Internal port" value={String(app.internal_port)} />
      <StatusCard label="Created" value={app.created_at} />
      <StatusCard label="Updated" value={app.updated_at} />
    </div>
  {:else}
    <p class="muted">Loading app...</p>
  {/if}
</div>

<style>
  .heading {
    align-items: center;
    display: flex;
    justify-content: space-between;
  }

  .actions {
    align-items: center;
    display: flex;
    gap: var(--space-2);
  }
</style>
