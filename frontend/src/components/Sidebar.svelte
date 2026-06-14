<script lang="ts">
  export let activePath = '/';

  const items = [
    ['/', 'Dashboard'],
    ['/apps', 'Apps'],
    ['/deploy', 'Deploy'],
    ['/logs', 'Logs'],
    ['/databases', 'Databases'],
    ['/security', 'Security'],
    ['/settings', 'Settings'],
  ];

  function active(href: string) {
    return href === '/' ? activePath === '/' : activePath.startsWith(href);
  }
</script>

<aside>
  <div class="brand">
    <span class="mark">LP</span>
    <div>
      <strong>LightPanel</strong>
      <small>native VPS control</small>
    </div>
  </div>
  <nav>
    {#each items as item}
      <a class:active={active(item[0])} href={item[0]}>{item[1]}</a>
    {/each}
  </nav>
  <p class="note">MVP foundation · local-first</p>
</aside>

<style>
  aside {
    background: linear-gradient(180deg, #0c111a, #080b10);
    border-right: 1px solid var(--border);
    color: var(--text);
    height: 100vh;
    left: 0;
    padding: var(--space-5);
    position: fixed;
    top: 0;
    width: var(--sidebar-width);
    z-index: 20;
  }

  .brand {
    align-items: center;
    display: flex;
    gap: var(--space-3);
  }

  .mark {
    align-items: center;
    background: var(--accent);
    border-radius: var(--radius-sm);
    color: #04130d;
    display: inline-flex;
    font-size: 12px;
    font-weight: 800;
    height: 34px;
    justify-content: center;
    width: 34px;
  }

  small,
  .note {
    color: var(--text-faint);
    font-size: 12px;
  }

  nav {
    display: grid;
    gap: var(--space-1);
    margin-top: var(--space-6);
  }

  a {
    color: var(--text-muted);
    border-radius: 6px;
    font-size: 14px;
    padding: 10px 11px;
    text-decoration: none;
  }

  a.active {
    background: var(--bg-hover);
    color: var(--text);
  }

  .note {
    bottom: var(--space-5);
    position: absolute;
  }

  @media (max-width: 760px) {
    aside {
      height: auto;
      padding: 14px 16px;
      position: static;
      width: auto;
    }

    nav {
      grid-template-columns: repeat(4, minmax(0, 1fr));
      margin-top: 14px;
    }

    a {
      font-size: 13px;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .note { display: none; }
  }
</style>
