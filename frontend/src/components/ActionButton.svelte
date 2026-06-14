<script lang="ts">
  export let href = '';
  export let label = '';
  export let variant = 'default';
</script>

{#if href}
  <a class={variant} {href}>{label}</a>
{:else}
  <button class={variant} type="button">{label}</button>
{/if}

<style>
  a,
  button {
    align-items: center;
    background: var(--bg-panel-soft);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    color: var(--text);
    display: inline-flex;
    font-size: 13px;
    min-height: 34px;
    padding: 0 12px;
    text-decoration: none;
  }

  .primary {
    background: var(--accent);
    border-color: var(--accent);
    color: #03120c;
    font-weight: 700;
  }
</style>
