<script lang="ts">
  import DataTable from '../components/DataTable.svelte';
  import SectionHeader from '../components/SectionHeader.svelte';
  import { settingsPaths } from '../lib/mock-data';

  const rows = settingsPaths.map(([setting, value]) => ({ setting, value }));
</script>

<div class="page">
  <section class="panel">
    <SectionHeader
      eyebrow="Settings"
      title="Panel configuration"
      description="Read-only placeholder for config paths and runtime locations."
    />
    <DataTable columns={['setting', 'value']} {rows} />
  </section>

  <section class="panel">
    <SectionHeader
      eyebrow="Runtime paths"
      title="Native runtime assumptions"
      description="These values will come from config/lightpanel.toml later."
    />
    <p class="muted">No settings are written from the UI yet.</p>
  </section>
</div>
