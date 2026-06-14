export const resourceMetrics = [
  { label: 'CPU load', value: '0.21', detail: '1 min average', tone: 'good' },
  { label: 'Memory', value: '42%', detail: 'placeholder usage', tone: 'neutral' },
  { label: 'Disk', value: '18%', detail: '/ volume placeholder', tone: 'neutral' },
  { label: 'Uptime', value: '4d 09h', detail: 'placeholder until API exists', tone: 'good' },
];

export const activeServices = [
  { name: 'lightpanel-backend', status: 'planned', detail: 'systemd preview only' },
  { name: 'nginx', status: 'not connected', detail: 'config generator pending' },
  { name: 'ufw', status: 'read-only planned', detail: 'do not lock out SSH' },
];

export const recentActivity = [
  'SQLite metadata foundation added',
  'systemd unit preview generated safely',
  'static and node adapters staged',
  'frontend shell upgraded to dark-first UI',
];

export const deploymentHealth = [
  { label: 'Static adapter', value: 'ready', detail: 'local source directory' },
  { label: 'Node adapter', value: 'foundation', detail: 'npm/pnpm allowlist' },
  { label: 'Nginx', value: 'pending', detail: 'preview generator next' },
];

export const securityItems = [
  { label: 'Firewall', value: 'placeholder', detail: 'read-only status planned' },
  { label: 'SSL', value: 'pending', detail: 'acme.sh integration later' },
  { label: 'Sessions', value: 'not implemented', detail: 'secure cookies later' },
];

export const quickActions = [
  { label: 'Review apps', href: '/apps' },
  { label: 'Deploy source', href: '/deploy' },
  { label: 'Inspect logs', href: '/logs' },
  { label: 'Security posture', href: '/security' },
];

export const logLines = [
  '15:44:02 lightpanel deploy[static] created release directory',
  '15:44:02 lightpanel deploy[static] copied static source files',
  '15:44:03 lightpanel deploy[node] detected package manager: npm',
  '15:44:03 lightpanel audit delete_app placeholder requires confirmation later',
  '15:44:04 lightpanel nginx preview generator not implemented yet',
];

export const databaseConnections = [
  { name: 'local-panel.sqlite', type: 'SQLite', status: 'planned' },
  { name: 'mysql-production', type: 'MySQL', status: 'placeholder' },
  { name: 'postgres-analytics', type: 'PostgreSQL', status: 'placeholder' },
];

export const settingsPaths = [
  ['App data directory', '/opt/lightpanel/data/apps'],
  ['Nginx sites available', '/etc/nginx/sites-available'],
  ['acme.sh path', '/root/.acme.sh/acme.sh'],
  ['systemd unit path', '/etc/systemd/system'],
  ['Node runtime', '/usr/bin/node'],
  ['Python runtime', '/usr/bin/python3'],
];
