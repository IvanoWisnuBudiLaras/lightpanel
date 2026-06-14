<script lang="ts">
  import type { SystemdPreview } from '../lib/types';
  import SectionHeader from './SectionHeader.svelte';

  export let preview: SystemdPreview | null = null;
</script>

<section class="panel">
  <SectionHeader
    eyebrow="Systemd"
    title="Service preview"
    description={preview ? preview.service_name : 'Select an app to preview service config.'}
  />

  {#if preview}
    <div class="meta">
      <span>Start script</span>
      <code>{preview.start_script_path}</code>
    </div>
    <pre>{preview.unit_file}</pre>
  {:else}
    <p class="muted">No systemd preview loaded.</p>
  {/if}
</section>

<style>
  .meta {
    display: grid;
    gap: var(--space-1);
    margin-bottom: var(--space-3);
  }

  span {
    color: var(--text-muted);
    font-size: 12px;
  }

  pre,
  code {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    color: var(--text);
    overflow-x: auto;
    padding: var(--space-3);
  }

  pre {
    max-height: 360px;
    white-space: pre-wrap;
  }
</style>
