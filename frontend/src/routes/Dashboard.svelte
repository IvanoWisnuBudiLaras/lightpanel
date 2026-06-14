<script lang="ts">
  import { onMount } from 'svelte';
  import ActionButton from '../components/ActionButton.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import MetricCard from '../components/MetricCard.svelte';
  import RuntimeBadge from '../components/RuntimeBadge.svelte';
  import SectionHeader from '../components/SectionHeader.svelte';
  import StatusCard from '../components/StatusCard.svelte';
  import { getHealth, getIdentity, getRuntimes } from '../lib/api';
  import {
    activeServices,
    deploymentHealth,
    quickActions,
    recentActivity,
    resourceMetrics,
    securityItems,
  } from '../lib/mock-data';
  import type { Health, RuntimeInfo, ServerIdentity } from '../lib/types';

  let health: Health | null = null;
  let identity: ServerIdentity | null = null;
  let runtimes: RuntimeInfo[] = [];
  let error = '';

  onMount(async () => {
    try {
      [health, identity, runtimes] = await Promise.all([
        getHealth(),
        getIdentity(),
        getRuntimes(),
      ]);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Dashboard request failed';
    }
  });
</script>

<div class="page">
  {#if error}
    <p class="error">{error}</p>
  {/if}

  <section class="panel hero">
    <SectionHeader
      eyebrow="Server identity"
      title={identity?.hostname ?? 'Detecting host'}
      description={`${identity?.os_name ?? 'OS unknown'} · ${identity?.kernel ?? 'kernel unknown'}`}
    />
    <div class="metric-grid">
      <StatusCard label="API" value={health?.status ?? 'Loading'} detail={health?.version ?? ''} />
      <StatusCard label="Architecture" value={identity?.architecture ?? 'Loading'} />
      <StatusCard label="OS version" value={identity?.os_version ?? 'Unknown'} />
    </div>
  </section>

  <div class="metric-grid">
    {#each resourceMetrics as metric}
      <MetricCard {...metric} />
    {/each}
  </div>

  <div class="two-column">
    <section class="panel">
      <SectionHeader
        eyebrow="Readiness"
        title="Runtime matrix"
        description="Detected with safe version commands only."
      />
      <div class="runtime-list">
        {#each runtimes as runtime}
          <RuntimeBadge name={runtime.name} status={runtime.status} version={runtime.version} />
        {:else}
          <EmptyState title="No runtime data" description="Backend runtime endpoint is not responding yet." />
        {/each}
      </div>
    </section>

    <section class="panel">
      <SectionHeader eyebrow="Services" title="Active services" description="Read-only placeholders." />
      <div class="stack">
        {#each activeServices as service}
          <div class="split-row">
            <span>
              <strong>{service.name}</strong>
              <small>{service.detail}</small>
            </span>
            <code>{service.status}</code>
          </div>
        {/each}
      </div>
    </section>
  </div>

  <div class="two-column">
    <section class="panel">
      <SectionHeader eyebrow="Deployments" title="Health summary" description="Adapter status only." />
      <div class="metric-grid">
        {#each deploymentHealth as item}
          <StatusCard label={item.label} value={item.value} detail={item.detail} />
        {/each}
      </div>
    </section>

    <section class="panel">
      <SectionHeader eyebrow="Security" title="Safety posture" description="No firewall mutation yet." />
      <div class="stack">
        {#each securityItems as item}
          <StatusCard label={item.label} value={item.value} detail={item.detail} />
        {/each}
      </div>
    </section>
  </div>

  <div class="two-column">
    <section class="panel">
      <SectionHeader eyebrow="Activity" title="Recent activity" description="Placeholder activity stream." />
      <ol class="activity">
        {#each recentActivity as item}
          <li>{item}</li>
        {/each}
      </ol>
    </section>

    <section class="panel">
      <SectionHeader eyebrow="Quick actions" title="Common paths" description="Non-destructive navigation." />
      <div class="actions">
        {#each quickActions as action}
          <ActionButton href={action.href} label={action.label} />
        {/each}
      </div>
    </section>
  </div>
</div>

<style>
  .hero {
    border-color: rgb(110 231 183 / 30%);
  }

  .runtime-list {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
  }

  .split-row span {
    display: grid;
    gap: 3px;
  }

  small,
  code {
    color: var(--text-muted);
  }

  .activity {
    color: var(--text-muted);
    display: grid;
    gap: var(--space-2);
    margin: 0;
    padding-left: var(--space-5);
  }

  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
  }
</style>
