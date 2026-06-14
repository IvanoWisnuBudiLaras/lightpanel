#!/usr/bin/env bash
set -euo pipefail

INSTALL_DIR="${LIGHTPANEL_INSTALL_DIR:-/opt/lightpanel}"
SERVICE_NAME="${LIGHTPANEL_SERVICE_NAME:-lightpanel.service}"

tools=(tar systemctl nginx sqlite3 bash)

echo "Checking LightPanel server deploy environment..."

for tool in "${tools[@]}"; do
  if command -v "$tool" >/dev/null 2>&1; then
    echo "ok: $tool"
  else
    echo "missing: $tool"
  fi
done

if [ -d "$INSTALL_DIR" ]; then
  echo "ok: install directory exists: $INSTALL_DIR"
else
  echo "missing: $INSTALL_DIR"
  echo "create it with: sudo mkdir -p $INSTALL_DIR/{config,data,backups}"
  echo "then set ownership for the deployment user as appropriate"
fi

if systemctl list-unit-files "$SERVICE_NAME" >/dev/null 2>&1; then
  echo "service status:"
  systemctl status "$SERVICE_NAME" --no-pager || true
else
  echo "service not installed yet: $SERVICE_NAME"
fi

echo "No packages were installed and no services were changed."

