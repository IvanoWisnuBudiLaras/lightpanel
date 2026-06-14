# Deployment Model

LightPanel deploys apps natively on the VPS.

Planned flow:

1. Create app metadata.
2. Create app directory.
3. Generate release directory.
4. Generate `start.sh`.
5. Generate systemd unit.
6. Start or restart the systemd service.
7. Generate Nginx site config.
8. Test and reload Nginx.

Docker is intentionally not required.

