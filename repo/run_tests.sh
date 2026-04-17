#!/usr/bin/env bash
set -euo pipefail

echo "=== FleetReserve Operations Suite - Test Runner ==="
echo ""

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BACKEND_DIR="$SCRIPT_DIR/backend"

if ! command -v docker >/dev/null 2>&1; then
  echo "error: docker is required for this test runner." >&2
  echo "  Install Docker and re-run this script." >&2
  exit 1
fi

echo "--- Running all tests in Docker (rust:bookworm) ---"
docker run --rm \
  -v "$SCRIPT_DIR:/app" \
  -w /app/backend \
  rust:bookworm \
  bash -c 'set -euo pipefail; export CARGO_INCREMENTAL=0; echo "--- Backend Unit + Lib Tests ---"; CARGO_TARGET_DIR=/tmp/fleetreserve-target-backend cargo test --lib -- --nocapture; echo ""; echo "--- Backend Integration Tests ---"; CARGO_TARGET_DIR=/tmp/fleetreserve-target-backend cargo test --test integration_tests -- --nocapture; echo ""; echo "--- Unit Tests (backend/tests/unit) ---"; CARGO_TARGET_DIR=/tmp/fleetreserve-target-backend cargo test --test unit_tests_runner -- --nocapture; echo ""; echo "--- API Tests (backend/tests/api — HTTP routes via axum-test) ---"; CARGO_TARGET_DIR=/tmp/fleetreserve-target-backend cargo test --test api_tests_runner -- --nocapture; echo ""; echo "--- Frontend (Leptos) unit + integration tests ---"; cd /app/frontend && CARGO_TARGET_DIR=/tmp/fleetreserve-target-frontend cargo test --lib --tests -- --nocapture' 2>&1

cd "$SCRIPT_DIR"
echo ""
echo "=== All available tests complete ==="
