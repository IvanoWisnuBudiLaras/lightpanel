<script lang="ts">
  import type { AppStatus } from '../lib/types';

  export let status: AppStatus = 'unknown';
</script>

<span class={status}>{status}</span>

<style>
  span {
    background: var(--bg-panel-soft);
    border: 1px solid var(--border);
    border-radius: 999px;
    display: inline-flex;
    font-size: 12px;
    padding: 5px 9px;
    text-transform: capitalize;
  }

  .deployed,
  .running {
    border-color: rgb(34 197 94 / 45%);
    color: var(--green);
  }

  .failed {
    border-color: rgb(248 113 113 / 45%);
    color: var(--red);
  }
</style>
