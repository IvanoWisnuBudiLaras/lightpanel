<script lang="ts">
  import Sidebar from './Sidebar.svelte';
  import Topbar from './Topbar.svelte';

  export let path = '/';
  export let title = 'Dashboard';
</script>

<div class="shell">
  <Sidebar activePath={path} />
  <div class="workspace">
    <Topbar {title} />
    <main>
      <slot />
    </main>
  </div>
</div>

<style>
  .shell {
    min-height: 100vh;
  }

  .workspace {
    background: var(--bg);
    min-height: 100vh;
    min-width: 0;
  }

  main {
    margin-left: var(--sidebar-width);
    min-height: calc(100vh - var(--topbar-height));
    padding: var(--space-6);
  }

  @media (max-width: 760px) {
    main {
      margin-left: 0;
      padding: var(--space-4);
    }
  }
</style>
