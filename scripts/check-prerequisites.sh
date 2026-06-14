#!/usr/bin/env bash
set -u

tools=(
  rustc
  cargo
  node
  npm
  sqlite3
  nginx
  systemctl
  journalctl
  ufw
)

echo "Checking LightPanel prerequisites..."

for tool in "${tools[@]}"; do
  if command -v "$tool" >/dev/null 2>&1; then
    echo "ok: $tool"
  else
    echo "missing: $tool"
  fi
done

echo "This script only checks tools. It does not install or modify anything."

