#!/usr/bin/env bash
set -euo pipefail

echo "=== FleetReserve Operations Suite - Test Runner ==="
echo ""

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BACKEND_DIR="$SCRIPT_DIR/backend"

echo "--- Backend Unit + Lib Tests ---"
run_backend_tests() {
  cd "$BACKEND_DIR"
  cargo test --lib -- --nocapture 2>&1
  echo ""
  echo "--- Backend Integration Tests ---"
  cargo test --test integration_tests -- --nocapture 2>&1
  echo ""
  echo "--- Unit Tests (backend/tests/unit) ---"
  cargo test --test unit_tests_runner -- --nocapture 2>&1
  echo ""
  echo "--- API Tests (backend/tests/api — HTTP routes via axum-test) ---"
  cargo test --test api_tests_runner -- --nocapture 2>&1
}

run_frontend_tests() {
  FRONTEND_DIR="$SCRIPT_DIR/frontend"
  cd "$FRONTEND_DIR"
  echo "--- Frontend (Leptos) unit tests ---"
  cargo test --lib -- --nocapture 2>&1
}

if command -v cargo >/dev/null 2>&1; then
  run_backend_tests
  echo ""
  run_frontend_tests
elif command -v docker >/dev/null 2>&1; then
  echo "(cargo not found; using Docker image rust:bookworm)" >&2
  # Use bash -c (not -lc): login shells on this image drop /usr/local/cargo/bin from PATH.
  docker run --rm \
    -v "$SCRIPT_DIR:/app" \
    -w /app/backend \
    rust:bookworm \
    bash -c 'set -euo pipefail; cargo test --lib -- --nocapture; echo ""; echo "--- Backend Integration Tests ---"; cargo test --test integration_tests -- --nocapture; echo ""; echo "--- Unit Tests (backend/tests/unit) ---"; cargo test --test unit_tests_runner -- --nocapture; echo ""; echo "--- API Tests (backend/tests/api — HTTP routes via axum-test) ---"; cargo test --test api_tests_runner -- --nocapture; echo ""; echo "--- Frontend (Leptos) unit tests ---"; cd /app/frontend && cargo test --lib -- --nocapture' 2>&1
else
  echo "error: neither 'cargo' nor 'docker' is available." >&2
  echo "  Install Rust: https://rustup.rs" >&2
  echo "  Or install Docker and re-run this script." >&2
  exit 1
fi

cd "$SCRIPT_DIR"
echo ""
echo "=== All available tests complete ==="
