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
