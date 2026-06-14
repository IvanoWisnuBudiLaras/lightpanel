# LightPanel

LightPanel is a super lightweight native VPS control panel foundation.

Current status: MVP foundation in progress. Required tools may not be installed
yet.

## Goals

- Rust backend with axum.
- Svelte SPA frontend.
- SQLite internal database.
- Native deployment without Docker.
- systemd for process management.
- Nginx for reverse proxy.
- acme.sh integration later.
- journalctl, deploy.log, and SQLite audit logs for observability.

## Non-goals

- Full cPanel clone.
- Browser terminal.
- Plugin system.
- Docker requirement.
- Arbitrary shell execution.
- PostgreSQL/MySQL dependency for LightPanel internal data.

## Repository Layout

- `backend/`: Rust backend.
- `frontend/`: Svelte SPA.
- `migrations/`: SQLite migrations.
- `config/`: example panel config.
- `deploy/`: systemd and Nginx examples.
- `scripts/`: local, packaging, deploy, update, and rollback scripts.
- `docs/`: architecture and deployment documentation.

## GitHub Publishing

Create a GitHub repository, add it as `origin`, and push the `main` branch:

```bash
git remote add origin git@github.com:YOUR_USER/lightpanel.git
git branch -M main
git push -u origin main
```

See [docs/github-publish.md](docs/github-publish.md) for repository and secret
setup.

## Auto-update Deployment

LightPanel is intended to deploy from a prebuilt artifact:

1. GitHub Actions builds frontend and backend.
2. GitHub Actions creates `dist/lightpanel-linux-x64.tar.gz`.
3. GitHub Actions uploads the tarball to the VPS.
4. The VPS runs `scripts/self-update.sh`.
5. The update script backs up the old install, installs the new artifact, and
   restarts `lightpanel.service`.

The server must not run `cargo build` or `npm install`.

See [docs/auto-update.md](docs/auto-update.md).

## Required GitHub Secrets

- `SERVER_HOST`
- `SERVER_USER`
- `SERVER_PORT`
- `SERVER_SSH_KEY`
- `SERVER_DEPLOY_PATH`
- `LIGHTPANEL_SERVICE_NAME`

Never commit SSH private keys, server passwords, tokens, private IP addresses, or
production secrets.

## Server Preparation

Example Ubuntu/Debian preparation:

```bash
sudo useradd --system --home /opt/lightpanel --shell /usr/sbin/nologin lightpanel
sudo mkdir -p /opt/lightpanel/{config,data,backups}
sudo mkdir -p /opt/lightpanel-upload
sudo chown -R lightpanel:lightpanel /opt/lightpanel
sudo chown "$USER":"$USER" /opt/lightpanel-upload
```

For the first MVP automation, `SERVER_USER` must be able to run
`scripts/self-update.sh` non-interactively. That means using `root` temporarily
or a carefully prepared deployment user with permission to write `/opt/lightpanel`,
update the systemd unit, and restart `lightpanel.service`.

Install the example systemd unit only after reviewing it:

```bash
sudo cp deploy/systemd/lightpanel.service /etc/systemd/system/lightpanel.service
sudo systemctl daemon-reload
```

Run environment checks on the server:

```bash
bash scripts/check-deploy-env.sh
```

## Rollback

The update script stores backups under `/opt/lightpanel/backups`. List backups:

```bash
sudo bash scripts/rollback.sh --list
```

Restore the latest backup:

```bash
sudo bash scripts/rollback.sh --latest
```
