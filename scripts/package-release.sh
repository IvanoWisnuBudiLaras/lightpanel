#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$ROOT_DIR/dist"
STAGE_DIR="$DIST_DIR/lightpanel-release"
ARTIFACT="$DIST_DIR/lightpanel-linux-x64.tar.gz"

need_tool() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required tool: $1" >&2
    exit 1
  fi
}

write_version_file() {
  local commit branch built_at
  commit="$(git -C "$ROOT_DIR" rev-parse HEAD 2>/dev/null || echo unknown)"
  branch="${GITHUB_REF_NAME:-$(git -C "$ROOT_DIR" branch --show-current 2>/dev/null || echo unknown)}"
  built_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  {
    echo "commit=$commit"
    echo "branch=$branch"
    echo "built_at=$built_at"
  } > "$STAGE_DIR/VERSION"
}

build_frontend() {
  pushd "$ROOT_DIR/frontend" >/dev/null
  if [ -f package-lock.json ]; then
    npm ci
  else
    npm install
  fi
  npm run build
  popd >/dev/null
}

build_backend() {
  pushd "$ROOT_DIR/backend" >/dev/null
  cargo build --release
  popd >/dev/null
}

copy_backend_binary() {
  local binary=""
  if [ -f "$ROOT_DIR/backend/target/release/lightpanel" ]; then
    binary="$ROOT_DIR/backend/target/release/lightpanel"
  elif [ -f "$ROOT_DIR/backend/target/release/lightpanel-backend" ]; then
    binary="$ROOT_DIR/backend/target/release/lightpanel-backend"
  else
    echo "release binary not found" >&2
    exit 1
  fi

  install -m 0755 "$binary" "$STAGE_DIR/lightpanel"
}

package_release() {
  rm -rf "$STAGE_DIR" "$ARTIFACT"
  mkdir -p "$STAGE_DIR/frontend" "$STAGE_DIR/config" "$STAGE_DIR/scripts" "$STAGE_DIR/deploy/systemd"

  copy_backend_binary
  cp -a "$ROOT_DIR/frontend/dist" "$STAGE_DIR/frontend/dist"
  cp "$ROOT_DIR/config/lightpanel.example.toml" "$STAGE_DIR/config/lightpanel.example.toml"
  cp -a "$ROOT_DIR/migrations" "$STAGE_DIR/migrations"
  cp "$ROOT_DIR/scripts/self-update.sh" "$STAGE_DIR/scripts/self-update.sh"
  cp "$ROOT_DIR/deploy/systemd/lightpanel.service" "$STAGE_DIR/deploy/systemd/lightpanel.service"
  cp "$ROOT_DIR/README.md" "$STAGE_DIR/README.md"
  write_version_file

  tar -C "$STAGE_DIR" -czf "$ARTIFACT" .
  echo "created $ARTIFACT"
}

need_tool git
need_tool tar
need_tool date
need_tool npm
need_tool cargo

build_frontend
build_backend
package_release

