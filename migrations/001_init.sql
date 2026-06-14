-- LightPanel initial migration.

CREATE TABLE IF NOT EXISTS apps (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  runtime TEXT NOT NULL,
  internal_port INTEGER NOT NULL,
  status TEXT NOT NULL DEFAULT 'unknown',
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT,
  CHECK (runtime IN ('static', 'node', 'php', 'python', 'rust', 'go', 'java', 'dotnet')),
  CHECK (internal_port >= 1 AND internal_port <= 65535)
);

CREATE TABLE IF NOT EXISTS app_domains (
  id TEXT PRIMARY KEY,
  app_id TEXT NOT NULL,
  domain TEXT NOT NULL,
  is_primary INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_app_domains_domain
  ON app_domains(domain);

CREATE TABLE IF NOT EXISTS app_env_vars (
  id TEXT PRIMARY KEY,
  app_id TEXT NOT NULL,
  key TEXT NOT NULL,
  value TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE,
  UNIQUE (app_id, key)
);

CREATE TABLE IF NOT EXISTS deployments (
  id TEXT PRIMARY KEY,
  app_id TEXT NOT NULL,
  status TEXT NOT NULL,
  release_path TEXT,
  log_path TEXT,
  created_at TEXT NOT NULL,
  FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS audit_logs (
  id TEXT PRIMARY KEY,
  actor TEXT NOT NULL,
  action TEXT NOT NULL,
  target TEXT NOT NULL,
  details TEXT,
  created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS settings (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
