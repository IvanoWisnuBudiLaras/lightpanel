<script lang="ts">
  export let label = '';
  export let value = 'Unknown';
  export let detail = '';
</script>

<article>
  <span>{label}</span>
  <strong>{value}</strong>
  {#if detail}
    <small>{detail}</small>
  {/if}
</article>

<style>
  article {
    background: var(--bg-panel);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    display: grid;
    gap: 8px;
    min-height: 86px;
    padding: var(--space-4);
  }

  span,
  small {
    color: var(--text-muted);
    font-size: 13px;
  }

  strong {
    font-size: 18px;
  }
</style>
