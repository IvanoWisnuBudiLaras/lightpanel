<script lang="ts">
  export let label = '';
  export let value = '';
  export let detail = '';
  export let tone = 'neutral';
</script>

<article class={tone}>
  <span>{label}</span>
  <strong>{value}</strong>
  <small>{detail}</small>
</article>

<style>
  article {
    background: var(--bg-panel);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    display: grid;
    gap: var(--space-2);
    padding: var(--space-4);
  }

  span,
  small {
    color: var(--text-muted);
    font-size: 12px;
  }

  strong {
    font-size: 22px;
  }

  .good {
    border-color: rgb(34 197 94 / 38%);
  }
</style>
