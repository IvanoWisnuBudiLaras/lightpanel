# Publishing LightPanel to GitHub

## Create the Repository

Create an empty GitHub repository named `LightPanel` or `lightpanel`. Do not add
a generated README if this local repository already has one.

## Push the Repository

From the project root:

```bash
git remote add origin git@github.com:YOUR_USER/lightpanel.git
git branch -M main
git push -u origin main
```

The deployment workflow uses the `main` branch and can also be started manually
with `workflow_dispatch`.

## Required Secrets

Add these repository secrets in GitHub:

- `SERVER_HOST`: VPS hostname or IP.
- `SERVER_USER`: SSH deployment user.
- `SERVER_PORT`: SSH port, usually `22`.
- `SERVER_SSH_KEY`: private key used only for deployment.
- `SERVER_DEPLOY_PATH`: upload directory, for example `/opt/lightpanel-upload`.
- `LIGHTPANEL_SERVICE_NAME`: usually `lightpanel.service`.

Never commit the private key, token values, server password, or private server
details to the repository.

`SERVER_USER` must run the update script without an interactive password prompt.
For the MVP this can be `root`, or a deployment user with narrowly configured
permissions to update `/opt/lightpanel` and restart `lightpanel.service`.

## SSH Key Setup

Create a dedicated deploy key on your machine:

```bash
ssh-keygen -t ed25519 -C "lightpanel-deploy" -f ./lightpanel_deploy
```

Install the public key on the server deployment user:

```bash
ssh-copy-id -i ./lightpanel_deploy.pub user@server
```

Store the private key content in `SERVER_SSH_KEY`. Remove local key files if
they are no longer needed.

## Manual Deploy

Open GitHub Actions, choose `Build and Deploy LightPanel`, then run workflow.
Read each step log. Secret values should be masked and must not be printed.

The workflow uses official GitHub actions for checkout and Node.js setup. Rust is
installed with `rustup` on the runner to avoid extra deployment actions.
