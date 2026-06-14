<script lang="ts">
  import Layout from './components/Layout.svelte';
  import AppDetail from './routes/AppDetail.svelte';
  import Apps from './routes/Apps.svelte';
  import Dashboard from './routes/Dashboard.svelte';
  import Databases from './routes/Databases.svelte';
  import Deploy from './routes/Deploy.svelte';
  import Logs from './routes/Logs.svelte';
  import Security from './routes/Security.svelte';
  import Settings from './routes/Settings.svelte';

  let path = window.location.pathname;

  const titles: Record<string, string> = {
    '/': 'Dashboard',
    '/apps': 'Apps',
    '/deploy': 'Deploy',
    '/logs': 'Logs',
    '/databases': 'Databases',
    '/security': 'Security',
    '/settings': 'Settings',
  };

  function syncPath() {
    path = window.location.pathname;
  }

  function navigate(event: MouseEvent) {
    if (event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;

    const link = (event.target as HTMLElement).closest('a');
    const href = link?.getAttribute('href');

    if (!href?.startsWith('/')) return;

    event.preventDefault();
    history.pushState(null, '', href);
    syncPath();
  }

  $: title = titles[path] ?? (path.startsWith('/apps/') ? 'App Detail' : 'Dashboard');
</script>

<svelte:window on:click={navigate} on:popstate={syncPath} />

<Layout {path} {title}>
  {#key path}
    {#if path === '/'}
      <Dashboard />
    {:else if path === '/apps'}
      <Apps />
    {:else if path.startsWith('/apps/')}
      <AppDetail id={path.split('/')[2]} />
    {:else if path === '/deploy'}
      <Deploy />
    {:else if path === '/logs'}
      <Logs />
    {:else if path === '/databases'}
      <Databases />
    {:else if path === '/security'}
      <Security />
    {:else if path === '/settings'}
      <Settings />
    {:else}
      <Dashboard />
    {/if}
  {/key}
</Layout>
