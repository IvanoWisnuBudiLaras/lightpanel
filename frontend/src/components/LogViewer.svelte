<script lang="ts">
  export let lines: string[] = [];
</script>

<pre>{lines.length ? lines.join('\n') : 'No logs loaded.'}</pre>

<style>
  pre {
    background: #05070a;
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    color: #d8f3dc;
    font-size: 13px;
    line-height: 1.55;
    margin: 0;
    min-height: 260px;
    overflow: auto;
    padding: var(--space-4);
  }
</style>
