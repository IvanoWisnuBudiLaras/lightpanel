<script lang="ts">
  export let name = 'runtime';
  export let status = 'not_installed';
  export let version: string | null = null;
  export let detail = '';
</script>

<span class={status}>
  <strong>{name}</strong>
  {detail || status.replace('_', ' ')}
  {#if version}
    <small>{version}</small>
  {/if}
</span>

<style>
  span {
    align-items: center;
    background: var(--bg-panel-soft);
    border: 1px solid var(--border);
    border-radius: 999px;
    display: inline-flex;
    gap: 8px;
    padding: 7px 10px;
  }

  .installed {
    border-color: rgb(34 197 94 / 45%);
    color: var(--green);
  }

  .error {
    border-color: rgb(251 191 36 / 45%);
    color: var(--amber);
  }

  small {
    color: var(--text-muted);
  }
</style>
