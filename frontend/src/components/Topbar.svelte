<script lang="ts">
  export let title = 'Dashboard';
</script>

<header>
  <div>
    <span class="kicker">LightPanel</span>
    <h1>{title}</h1>
  </div>
  <span class="status">local API · preview mode</span>
</header>

<style>
  header {
    align-items: center;
    background: rgb(8 11 16 / 82%);
    backdrop-filter: blur(12px);
    border-bottom: 1px solid var(--border);
    display: flex;
    height: var(--topbar-height);
    justify-content: space-between;
    margin-left: var(--sidebar-width);
    padding: 0 var(--space-6);
    position: sticky;
    top: 0;
    z-index: 10;
  }

  h1 {
    font-size: 18px;
    margin: 0;
  }

  .status {
    color: var(--text-muted);
    font-size: 13px;
  }

  @media (max-width: 760px) {
    header {
      margin-left: 0;
      padding: 0 var(--space-4);
    }
  }
</style>
