<script lang="ts">
  import LogViewer from '../components/LogViewer.svelte';
  import SectionHeader from '../components/SectionHeader.svelte';
  import { logLines } from '../lib/mock-data';

  let tab = 'deploy';
  let filter = '';

  $: lines = logLines.filter((line) => line.toLowerCase().includes(filter.toLowerCase()));
</script>

<div class="page">
  <section class="panel">
    <SectionHeader
      eyebrow="Logs"
      title="Infrastructure console"
      description="Placeholder console. Streaming is only planned for open log pages."
    />

    <div class="tabs">
      {#each ['app', 'deploy', 'activity'] as item}
        <button class:active={tab === item} type="button" on:click={() => (tab = item)}>
          {item}
        </button>
      {/each}
    </div>

    <div class="filter">
      <input class="field" bind:value={filter} placeholder="Filter log lines" />
      <span>{tab}.log · placeholder</span>
    </div>

    <LogViewer {lines} />
  </section>
</div>

<style>
  .tabs,
  .filter {
    align-items: center;
    display: flex;
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
    color: var(--accent);
    border-color: rgb(110 231 183 / 35%);
  }

  input {
    min-width: 260px;
  }

  span {
    color: var(--text-muted);
    font-size: 13px;
  }
</style>
