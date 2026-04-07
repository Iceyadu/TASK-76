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
}

if command -v cargo >/dev/null 2>&1; then
  run_backend_tests
elif command -v docker >/dev/null 2>&1; then
  echo "(cargo not found; using Docker image rust:bookworm)" >&2
  # Use bash -c (not -lc): login shells on this image drop /usr/local/cargo/bin from PATH.
  docker run --rm \
    -v "$BACKEND_DIR:/app" \
    -w /app \
    rust:bookworm \
    bash -c 'set -euo pipefail; cargo test --lib -- --nocapture; echo ""; echo "--- Backend Integration Tests ---"; cargo test --test integration_tests -- --nocapture' 2>&1
else
  echo "error: neither 'cargo' nor 'docker' is available." >&2
  echo "  Install Rust: https://rustup.rs" >&2
  echo "  Or install Docker and re-run this script." >&2
  exit 1
fi

cd "$SCRIPT_DIR"
echo ""
echo "--- Unit Tests (standalone) ---"
echo "See unit_tests/ directory for standalone test files."
echo "These tests reference backend crate internals and are run via the backend test harness."
echo ""

echo "--- API Tests (standalone) ---"
echo "See API_tests/ directory for API integration test files."
echo "These tests require a running backend instance or are run via the backend integration tests."
echo ""

echo "=== All available tests complete ==="
