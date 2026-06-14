<script lang="ts">
  export let eyebrow = '';
  export let title = '';
  export let description = '';
</script>

<div class="section-header">
  {#if eyebrow}
    <span class="kicker">{eyebrow}</span>
  {/if}
  <div>
    <h2>{title}</h2>
    {#if description}
      <p>{description}</p>
    {/if}
  </div>
</div>

<style>
  .section-header {
    display: grid;
    gap: var(--space-2);
    margin-bottom: var(--space-4);
  }

  h2,
  p {
    margin: 0;
  }

  h2 {
    font-size: 17px;
  }

  p {
    color: var(--text-muted);
    font-size: 13px;
  }
</style>
