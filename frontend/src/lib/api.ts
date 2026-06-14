export const apiBaseUrl = '/api';

import type { AppRecord, Health, RuntimeInfo, ServerIdentity } from './types';

interface ApiResponse<T> {
  success: boolean;
  data: T;
}

async function getJson<T>(path: string): Promise<T> {
  const response = await fetch(`${apiBaseUrl}${path}`);

  if (!response.ok) {
    throw new Error(`Request failed: ${response.status}`);
  }

  const body = (await response.json()) as ApiResponse<T>;
  return body.data;
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

export function getApps(): Promise<AppRecord[]> {
  return getJson<AppRecord[]>('/apps');
}
