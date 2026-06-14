# LightPanel Roadmap

## Step 1: Repository Template

- Create folder structure.
- Add documentation.
- Add backend and frontend placeholder files.
- Add migration draft.
- Add safe placeholder scripts.
- Add example systemd and Nginx configs.

## Step 2: Backend Health and Dashboard Identity

- Add axum application setup.
- Add health route.
- Add dashboard identity route.
- Read server identity from safe system APIs.

## Step 3: SQLite Migration Structure and Models

- Add SQLite connection module.
- Apply initial migration.
- Define internal metadata models.

## Step 4: App Metadata CRUD

- Create, list, read, update, and delete app metadata.
- Validate app names and domains.
- Add audit logs for destructive actions.

## Step 5: systemd Unit Generator

- Generate allowlisted unit files.
- Keep command construction isolated.

## Step 6: Static Site Deployment Adapter

- Create app folders.
- Write metadata.
- Generate start scripts for static serving.

## Step 7: Node.js Deployment Adapter

- Add Node.js app adapter.
- Avoid requiring Node.js for the panel runtime.

## Step 8: journalctl Log Reader

- Read bounded app logs from journalctl.
- Stream logs only while the log page is open.

## Step 9: Nginx Config Generator

- Generate app site config.
- Test Nginx config.
- Reload Nginx only after validation.

## Step 10: Basic Svelte UI Pages

- Add dashboard, apps, deployment, logs, database, security, and settings pages.
- Keep components small and dependency-light.

