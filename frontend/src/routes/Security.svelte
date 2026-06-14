<script lang="ts">
  import SectionHeader from '../components/SectionHeader.svelte';
  import StatusCard from '../components/StatusCard.svelte';
  import { securityItems } from '../lib/mock-data';
</script>

<div class="page">
  <section class="panel warning">
    <SectionHeader
      eyebrow="Safety"
      title="Do not lock out SSH"
      description="Firewall actions will stay read-only until explicit safeguards exist."
    />
  </section>

  <div class="metric-grid">
    {#each securityItems as item}
      <StatusCard label={item.label} value={item.value} detail={item.detail} />
    {/each}
    <StatusCard label="Blocked IPs" value="placeholder" detail="No rule editing yet" />
    <StatusCard label="Panel access" value="planned" detail="Sessions and CSRF later" />
  </div>
</div>

<style>
  .warning {
    border-color: rgb(251 191 36 / 40%);
  }
</style>
