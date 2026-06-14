<script lang="ts">
  export let title = 'Nothing here yet';
  export let description = 'This area is waiting for backend integration.';
</script>

<div class="empty">
  <strong>{title}</strong>
  <p>{description}</p>
</div>

<style>
  .empty {
    border: 1px dashed var(--border-strong);
    border-radius: var(--radius-md);
    display: grid;
    gap: var(--space-2);
    padding: var(--space-5);
  }

  p {
    color: var(--text-muted);
    margin: 0;
  }
</style>
