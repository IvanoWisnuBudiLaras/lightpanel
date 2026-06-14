export type AppStatus =
  | 'unknown'
  | 'created'
  | 'deployed'
  | 'running'
  | 'stopped'
  | 'failed';

export interface Health {
  status: string;
  service: string;
  version: string;
}

export interface ServerIdentity {
  hostname: string;
  os_name: string;
  os_version: string;
  kernel: string;
  architecture: string;
}

export interface RuntimeInfo {
  name: string;
  command: string;
  status: 'installed' | 'not_installed' | 'error';
  version: string | null;
}

export interface ResourceUsage {
  uptime_seconds: number;
  uptime_label: string;
  load: {
    one: string;
    five: string;
    fifteen: string;
  };
  memory: {
    total_mb: number;
    used_mb: number;
    available_mb: number;
    used_percent: number;
  };
  disk: {
    filesystem: string;
    mount: string;
    total_gb: number;
    used_gb: number;
    available_gb: number;
    used_percent: number;
  };
}

export interface AppRecord {
  id: string;
  name: string;
  runtime: string;
  internal_port: number;
  status: AppStatus;
  primary_domain: string | null;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

export interface AppPayload {
  name: string;
  runtime: string;
  internal_port: number;
  primary_domain: string | null;
}

export interface DeployRequest {
  source_path: string;
}

export interface DeployResult {
  deployment_id: string;
  status: string;
  release_id: string;
  release_path: string;
  current_path: string;
  deploy_log_path: string;
  package_manager?: string;
}

export interface SystemdPreview {
  service_name: string;
  unit_file: string;
  start_script_path: string;
  start_script: string;
}

export interface DatabaseTable {
  name: string;
  row_count: number;
}

export interface DatabaseRows {
  table: string;
  columns: string[];
  rows: Record<string, string>[];
}

export interface LogSnapshot {
  source: string;
  lines: string[];
  warning: string | null;
}

export interface CommandPreview {
  program: string;
  args: string[];
}

export interface NginxPreview {
  app_name: string;
  domain: string;
  internal_port: number;
  available_path: string;
  enabled_path: string;
  config: string;
}

export interface SslPreview {
  app_name: string;
  domain: string;
  mode: string;
  webroot: string;
  certificate_dir: string;
  commands: CommandPreview[];
}
