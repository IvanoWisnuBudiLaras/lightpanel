export const apiBaseUrl = '/api';

import type {
  AppPayload,
  AppRecord,
  LogSnapshot,
  NginxPreview,
  DatabaseRows,
  DatabaseTable,
  DeployRequest,
  DeployResult,
  Health,
  ResourceUsage,
  RuntimeInfo,
  ServerIdentity,
  SslPreview,
  SystemdPreview,
} from './types';

interface ApiResponse<T> {
  success: boolean;
  data: T;
}

async function getJson<T>(path: string): Promise<T> {
  return requestJson<T>(path);
}

async function requestJson<T>(path: string, options: RequestInit = {}): Promise<T> {
  const response = await fetch(`${apiBaseUrl}${path}`, {
    headers: { 'Content-Type': 'application/json', ...(options.headers ?? {}) },
    ...options,
  });

  if (!response.ok) {
    throw new Error(await errorMessage(response));
  }

  const body = (await response.json()) as ApiResponse<T>;
  return body.data;
}

async function errorMessage(response: Response): Promise<string> {
  try {
    const body = await response.json();
    return body?.error?.message ?? `Request failed: ${response.status}`;
  } catch {
    return `Request failed: ${response.status}`;
  }
}

export function getHealth(): Promise<Health> {
  return getJson<Health>('/health');
}

export function getIdentity(): Promise<ServerIdentity> {
  return getJson<ServerIdentity>('/dashboard/identity');
}

export function getRuntimes(): Promise<RuntimeInfo[]> {
  return getJson<RuntimeInfo[]>('/dashboard/runtimes');
}

export function getResources(): Promise<ResourceUsage> {
  return getJson<ResourceUsage>('/dashboard/resources');
}

export function getApps(): Promise<AppRecord[]> {
  return getJson<AppRecord[]>('/apps');
}

export function createApp(payload: AppPayload): Promise<AppRecord> {
  return requestJson<AppRecord>('/apps', {
    method: 'POST',
    body: JSON.stringify(payload),
  });
}

export function updateApp(id: string, payload: AppPayload): Promise<AppRecord> {
  return requestJson<AppRecord>(`/apps/${id}`, {
    method: 'PUT',
    body: JSON.stringify(payload),
  });
}

export function deleteApp(id: string): Promise<AppRecord> {
  return requestJson<AppRecord>(`/apps/${id}`, { method: 'DELETE' });
}

export function deployStatic(id: string, payload: DeployRequest): Promise<DeployResult> {
  return requestJson<DeployResult>(`/apps/${id}/deploy/static`, {
    method: 'POST',
    body: JSON.stringify(payload),
  });
}

export function deployNode(id: string, payload: DeployRequest): Promise<DeployResult> {
  return requestJson<DeployResult>(`/apps/${id}/deploy/node`, {
    method: 'POST',
    body: JSON.stringify(payload),
  });
}

export function getSystemdPreview(id: string): Promise<SystemdPreview> {
  return getJson<SystemdPreview>(`/apps/${id}/systemd/preview`);
}

export function getNginxPreview(id: string): Promise<NginxPreview> {
  return getJson<NginxPreview>(`/apps/${id}/nginx/preview`);
}

export function getSslPreview(id: string): Promise<SslPreview> {
  return getJson<SslPreview>(`/apps/${id}/ssl/preview`);
}

export function getAppDeployLog(id: string): Promise<LogSnapshot> {
  return getJson<LogSnapshot>(`/apps/${id}/logs/deploy`);
}

export function getErrorLogs(): Promise<LogSnapshot> {
  return getJson<LogSnapshot>('/logs/errors');
}

export function getPanelDeployLogs(): Promise<LogSnapshot> {
  return getJson<LogSnapshot>('/logs/deploy');
}

export function getDatabaseTables(): Promise<DatabaseTable[]> {
  return getJson<DatabaseTable[]>('/databases/internal/tables');
}

export function getDatabaseRows(table: string): Promise<DatabaseRows> {
  return getJson<DatabaseRows>(`/databases/internal/tables/${table}/rows`);
}
