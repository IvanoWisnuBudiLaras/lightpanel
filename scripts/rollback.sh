#!/usr/bin/env bash
set -euo pipefail

INSTALL_DIR="${LIGHTPANEL_INSTALL_DIR:-/opt/lightpanel}"
SERVICE_NAME="${LIGHTPANEL_SERVICE_NAME:-lightpanel.service}"
SYSTEMD_UNIT_PATH="${SYSTEMD_UNIT_PATH:-/etc/systemd/system/$SERVICE_NAME}"
BACKUP_ROOT="$INSTALL_DIR/backups"

fail() {
  echo "rollback error: $*" >&2
  exit 1
}

latest_backup() {
  find "$BACKUP_ROOT" -mindepth 1 -maxdepth 1 -type d 2>/dev/null | sort | tail -n 1
}

list_backups() {
  if [ ! -d "$BACKUP_ROOT" ]; then
    echo "no backup directory found: $BACKUP_ROOT"
    return
  fi

  find "$BACKUP_ROOT" -mindepth 1 -maxdepth 1 -type d | sort
}

safe_rm_dir() {
  local path="$1"
  case "$path" in
    "$INSTALL_DIR"/*) rm -rf "$path" ;;
    *) fail "refusing to remove unsafe path: $path" ;;
  esac
}

restore_latest() {
  local backup_dir
  backup_dir="$(latest_backup)"
  [ -n "$backup_dir" ] || fail "no backups available"

  echo "restoring backup: $backup_dir"
  [ -f "$backup_dir/lightpanel" ] && install -m 0755 "$backup_dir/lightpanel" "$INSTALL_DIR/lightpanel"

  if [ -d "$backup_dir/frontend" ]; then
    safe_rm_dir "$INSTALL_DIR/frontend"
    cp -a "$backup_dir/frontend" "$INSTALL_DIR/frontend"
  fi

  if [ -d "$backup_dir/migrations" ]; then
    safe_rm_dir "$INSTALL_DIR/migrations"
    cp -a "$backup_dir/migrations" "$INSTALL_DIR/migrations"
  fi

  [ -f "$backup_dir/VERSION" ] && cp "$backup_dir/VERSION" "$INSTALL_DIR/VERSION"
  if [ -f "$backup_dir/lightpanel.service" ]; then
    install -m 0644 "$backup_dir/lightpanel.service" "$SYSTEMD_UNIT_PATH"
    systemctl daemon-reload
  fi

  systemctl restart "$SERVICE_NAME"
  systemctl is-active --quiet "$SERVICE_NAME"
  echo "rollback completed"
}

case "${1:-}" in
  --latest) restore_latest ;;
  --list|"") list_backups ;;
  *) fail "usage: $0 [--list|--latest]" ;;
esac

