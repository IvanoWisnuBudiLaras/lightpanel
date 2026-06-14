<script lang="ts">
  import { onMount } from 'svelte';
  import ActionButton from '../components/ActionButton.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import MetricCard from '../components/MetricCard.svelte';
  import RuntimeBadge from '../components/RuntimeBadge.svelte';
  import SectionHeader from '../components/SectionHeader.svelte';
  import StatusCard from '../components/StatusCard.svelte';
  import { getApps, getHealth, getIdentity, getResources, getRuntimes } from '../lib/api';
  import { quickActions, recentActivity, securityItems } from '../lib/mock-data';
  import type { AppRecord, Health, ResourceUsage, RuntimeInfo, ServerIdentity } from '../lib/types';

  let health: Health | null = null;
  let identity: ServerIdentity | null = null;
  let resources: ResourceUsage | null = null;
  let runtimes: RuntimeInfo[] = [];
  let apps: AppRecord[] = [];
  let loading = true;
  let error = '';

  $: deployedApps = apps.filter((app) => app.status === 'deployed').length;
  $: appServices = apps.slice(0, 5);
  $: resourceCards = resources ? [
    { label: 'Load avg', value: resources.load.one, detail: '1 min system load', tone: 'neutral' },
    {
      label: 'Memory',
      value: `${resources.memory.used_percent}%`,
      detail: `${resources.memory.used_mb} / ${resources.memory.total_mb} MB`,
      tone: resources.memory.used_percent > 85 ? 'warn' : 'good',
    },
    {
      label: 'Disk',
      value: `${resources.disk.used_percent}%`,
      detail: `${resources.disk.used_gb} / ${resources.disk.total_gb} GB on ${resources.disk.mount}`,
      tone: resources.disk.used_percent > 85 ? 'warn' : 'good',
    },
    { label: 'Uptime', value: resources.uptime_label, detail: `${resources.uptime_seconds}s` },
  ] : [];

  onMount(async () => {
    try {
      [health, identity, resources, runtimes, apps] = await Promise.all([
        getHealth(),
        getIdentity(),
        getResources(),
        getRuntimes(),
        getApps(),
      ]);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Dashboard request failed';
    } finally {
      loading = false;
    }
  });
</script>

<div class="page">
  {#if error}<p class="error">{error}</p>{/if}

  <section class="panel hero">
    <SectionHeader
      eyebrow="Server identity"
      title={identity?.hostname ?? 'Detecting host'}
      description={`${identity?.os_name ?? 'OS unknown'} - ${identity?.kernel ?? 'kernel unknown'}`}
    />
    <div class="metric-grid">
      <StatusCard label="API" value={loading ? 'Loading' : health?.status ?? 'Unknown'} detail={health?.version ?? ''} />
      <StatusCard label="Architecture" value={identity?.architecture ?? 'Loading'} />
      <StatusCard label="Apps" value={String(apps.length)} detail={`${deployedApps} deployed`} />
    </div>
  </section>

  {#if resourceCards.length}
    <div class="metric-grid">
      {#each resourceCards as metric}
        <MetricCard {...metric} />
      {/each}
    </div>
  {:else}
    <section class="panel compact skeleton-panel">
      <span class="skeleton"></span>
      <span class="skeleton short"></span>
    </section>
  {/if}

  <div class="two-column">
    <section class="panel">
      <SectionHeader eyebrow="Readiness" title="Runtime matrix" description="Safe version checks only." />
      <div class="runtime-list">
        {#each runtimes as runtime}
          <RuntimeBadge name={runtime.name} status={runtime.status} version={runtime.version} />
        {:else}
          <EmptyState title="No runtime data" description="Runtime endpoint is not responding yet." />
        {/each}
      </div>
    </section>

    <section class="panel">
      <SectionHeader eyebrow="Services" title="App services" description="Registered apps from SQLite." />
      <div class="stack">
        {#each appServices as app}
          <div class="split-row">
            <span><strong>{app.name}</strong><small>{app.runtime} on :{app.internal_port}</small></span>
            <code>{app.status}</code>
          </div>
        {:else}
          <EmptyState title="No app services" description="Create app metadata first." />
        {/each}
      </div>
    </section>
  </div>

  <div class="two-column">
    <section class="panel">
      <SectionHeader eyebrow="Deployments" title="Health summary" description="MVP adapter coverage." />
      <div class="metric-grid">
        <StatusCard label="Static adapter" value="ready" detail="local source directory" />
        <StatusCard label="Node adapter" value="ready" detail="npm/pnpm/yarn lock detection" />
        <StatusCard label="Nginx + SSL" value="preview" detail="no host mutation yet" />
      </div>
    </section>

    <section class="panel">
      <SectionHeader eyebrow="Security" title="Safety posture" description="Firewall remains read-only." />
      <div class="stack">
        {#each securityItems as item}
          <StatusCard label={item.label} value={item.value} detail={item.detail} />
        {/each}
      </div>
    </section>
  </div>

  <div class="two-column">
    <section class="panel">
      <SectionHeader eyebrow="Activity" title="Recent activity" description="Placeholder until audit stream is wired." />
      <ol class="activity">
        {#each recentActivity as item}<li>{item}</li>{/each}
      </ol>
    </section>

    <section class="panel">
      <SectionHeader eyebrow="Actions" title="Quick actions" description="Open high-frequency workflows." />
      <div class="actions">
        {#each quickActions as action}<ActionButton href={action.href} label={action.label} />{/each}
      </div>
    </section>
  </div>
</div>

<style>
  .hero { border-color: rgb(110 231 183 / 30%); }
  .runtime-list { display: flex; flex-wrap: wrap; gap: 10px; }
  .split-row span { display: grid; gap: 3px; }
  small, code { color: var(--text-muted); }
  .activity { color: var(--text-muted); display: grid; gap: var(--space-2); margin: 0; padding-left: var(--space-5); }
  .actions { display: flex; flex-wrap: wrap; gap: var(--space-2); }
  .skeleton-panel { display: grid; gap: var(--space-3); }
  .skeleton {
    animation: pulse 1.2s ease-in-out infinite;
    background: linear-gradient(90deg, var(--bg-panel-soft), var(--bg-hover), var(--bg-panel-soft));
    border-radius: var(--radius-sm);
    display: block;
    height: 18px;
  }
  .skeleton.short { width: 40%; }
</style>
