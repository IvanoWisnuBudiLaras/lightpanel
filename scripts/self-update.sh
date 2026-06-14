#!/usr/bin/env bash
set -euo pipefail

ARTIFACT="${1:-}"
INSTALL_DIR="${LIGHTPANEL_INSTALL_DIR:-/opt/lightpanel}"
SERVICE_NAME="${LIGHTPANEL_SERVICE_NAME:-lightpanel.service}"
SYSTEMD_UNIT_PATH="${SYSTEMD_UNIT_PATH:-/etc/systemd/system/$SERVICE_NAME}"
BACKUP_ROOT="$INSTALL_DIR/backups"
CONFIG_FILE="$INSTALL_DIR/config/lightpanel.toml"

fail() {
  echo "self-update error: $*" >&2
  exit 1
}

safe_rm_dir() {
  local path="$1"
  case "$path" in
    "$INSTALL_DIR"/*) rm -rf "$path" ;;
    *) fail "refusing to remove unsafe path: $path" ;;
  esac
}

validate_input() {
  [ -n "$ARTIFACT" ] || fail "artifact path argument is required"
  [ -f "$ARTIFACT" ] || fail "artifact not found: $ARTIFACT"
  command -v tar >/dev/null 2>&1 || fail "tar is required"
  command -v systemctl >/dev/null 2>&1 || fail "systemctl is required"
}

create_backup() {
  local backup_dir="$BACKUP_ROOT/$(date -u +%Y%m%dT%H%M%SZ)"
  mkdir -p "$backup_dir"

  [ -f "$INSTALL_DIR/lightpanel" ] && cp "$INSTALL_DIR/lightpanel" "$backup_dir/lightpanel"
  [ -d "$INSTALL_DIR/frontend" ] && cp -a "$INSTALL_DIR/frontend" "$backup_dir/frontend"
  [ -d "$INSTALL_DIR/migrations" ] && cp -a "$INSTALL_DIR/migrations" "$backup_dir/migrations"
  [ -f "$INSTALL_DIR/VERSION" ] && cp "$INSTALL_DIR/VERSION" "$backup_dir/VERSION"
  [ -f "$SYSTEMD_UNIT_PATH" ] && cp "$SYSTEMD_UNIT_PATH" "$backup_dir/lightpanel.service"

  echo "$backup_dir"
}

extract_artifact() {
  local tmp_dir="$1"
  tar -xzf "$ARTIFACT" -C "$tmp_dir"
  [ -f "$tmp_dir/lightpanel" ] || fail "artifact missing lightpanel binary"
  [ -d "$tmp_dir/frontend/dist" ] || fail "artifact missing frontend/dist"
}

install_files() {
  local tmp_dir="$1"
  mkdir -p "$INSTALL_DIR/config" "$INSTALL_DIR/data"

  install -m 0755 "$tmp_dir/lightpanel" "$INSTALL_DIR/lightpanel.new"
  mv "$INSTALL_DIR/lightpanel.new" "$INSTALL_DIR/lightpanel"

  safe_rm_dir "$INSTALL_DIR/frontend.new"
  cp -a "$tmp_dir/frontend" "$INSTALL_DIR/frontend.new"
  safe_rm_dir "$INSTALL_DIR/frontend"
  mv "$INSTALL_DIR/frontend.new" "$INSTALL_DIR/frontend"

  safe_rm_dir "$INSTALL_DIR/migrations.new"
  cp -a "$tmp_dir/migrations" "$INSTALL_DIR/migrations.new"
  safe_rm_dir "$INSTALL_DIR/migrations"
  mv "$INSTALL_DIR/migrations.new" "$INSTALL_DIR/migrations"

  cp "$tmp_dir/config/lightpanel.example.toml" "$INSTALL_DIR/config/lightpanel.example.toml"
  [ -f "$CONFIG_FILE" ] || cp "$tmp_dir/config/lightpanel.example.toml" "$CONFIG_FILE"
  [ -f "$tmp_dir/VERSION" ] && cp "$tmp_dir/VERSION" "$INSTALL_DIR/VERSION"
}

install_service_if_changed() {
  local tmp_dir="$1"
  local service_file="$tmp_dir/deploy/systemd/lightpanel.service"
  [ -f "$service_file" ] || return 0

  if [ ! -f "$SYSTEMD_UNIT_PATH" ] || ! cmp -s "$service_file" "$SYSTEMD_UNIT_PATH"; then
    install -m 0644 "$service_file" "$SYSTEMD_UNIT_PATH"
    systemctl daemon-reload
  fi
}

restart_service() {
  systemctl restart "$SERVICE_NAME"
  if ! systemctl is-active --quiet "$SERVICE_NAME"; then
    systemctl status "$SERVICE_NAME" --no-pager || true
    return 1
  fi
}

main() {
  validate_input
  mkdir -p "$INSTALL_DIR" "$BACKUP_ROOT"
  local backup_dir tmp_dir
  backup_dir="$(create_backup)"
  tmp_dir="$(mktemp -d)"
  trap 'rm -rf "$tmp_dir"' EXIT

  echo "backup created: $backup_dir"
  extract_artifact "$tmp_dir"
  install_files "$tmp_dir"
  install_service_if_changed "$tmp_dir"

  if restart_service; then
    echo "LightPanel update completed"
  else
    echo "restart failed. Roll back with: sudo bash scripts/rollback.sh --latest" >&2
    echo "backup kept at: $backup_dir" >&2
    exit 1
  fi
}

main

