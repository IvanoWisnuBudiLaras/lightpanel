# LightPanel Auto-update Model

## Flow

1. GitHub Actions checks out the repository.
2. The runner installs Rust stable and Node.js LTS.
3. `scripts/package-release.sh` builds frontend and backend.
4. The runner creates `dist/lightpanel-linux-x64.tar.gz`.
5. The tarball and `scripts/self-update.sh` are uploaded to the VPS.
6. The VPS runs the controlled update script.
7. The script backs up the current install, installs the artifact, and restarts
   `lightpanel.service`.

## Why the Server Does Not Build

The VPS should be small and predictable. It should not need Rust, Cargo, Node.js,
or npm to update LightPanel. It receives only a release tarball.

## Artifact Contents

The release tarball contains:

- `lightpanel` binary.
- `frontend/dist` static assets.
- `config/lightpanel.example.toml`.
- `migrations`.
- `scripts/self-update.sh`.
- `deploy/systemd/lightpanel.service`.
- `VERSION`.
- `README.md`.

## Backup and Rollback

`scripts/self-update.sh` stores backups under `/opt/lightpanel/backups`.
Production data in `/opt/lightpanel/data` is not deleted. Production config at
`/opt/lightpanel/config/lightpanel.toml` is preserved.

List backups:

```bash
sudo bash scripts/rollback.sh --list
```

Restore latest backup:

```bash
sudo bash scripts/rollback.sh --latest
```

## Security Notes

- Never commit private SSH keys.
- Never print secrets in workflow logs.
- The workflow uploads a tarball and calls only `self-update.sh`.
- The update script does not run `cargo build` or `npm install`.
- The update script must be reviewed before production use.
- The SSH user must not require an interactive sudo password during update.

## Common Failure Cases

- Missing GitHub secret.
- Upload directory does not exist on the server.
- SSH key has no access to the server.
- `lightpanel` system user does not exist.
- `systemctl restart lightpanel.service` fails.
- The service file points to a missing binary.
