# FleetReserve Operations Suite - Static Reviewer Guide

## Repo Structure

```
repo/
  backend/               Axum REST API server
    Cargo.toml           Dependencies and build config
    Dockerfile           Multi-stage Rust build
    migrations/          SQL schema and seed data
    src/main.rs          Entry point
    src/app/state.rs     AppState with DB, keys, config
    src/routes/mod.rs    Router with all middleware layers
    src/handlers/        Request handlers per resource
    src/services/        Business logic engines
    src/repositories/    Database access layer
    src/models/mod.rs    Domain types and enums
    src/auth/            Authentication subsystem
    src/security/        Headers, encryption, masking
    src/audit/           Tamper-evident log chain
    src/errors/mod.rs    Error types and HTTP mapping
    tests/               Integration tests
  frontend/              Leptos WASM application
    Cargo.toml           Dependencies
    Dockerfile           Trunk build + nginx
    Trunk.toml           Trunk configuration
    index.html           HTML shell
    nginx.conf           Reverse proxy config
    src/lib.rs           Entry point
    src/app.rs           Root component with router
    src/pages/           Page components
    src/components/      Reusable UI components
    src/state/           Auth and app state
    src/api/             API client and types
    src/security/        CSRF, route guards
  docker-compose.yml     Orchestration
  run_tests.sh           Test runner script
  README.md              Setup and usage
  unit_tests/            Standalone unit test files
  API_tests/             API integration test files
```

## Auth Entry Points

| What | File | Key Function/Struct |
|------|------|-------------------|
| Login handler | `backend/src/handlers/auth.rs` | `login()` |
| Password hashing | `backend/src/auth/password.rs` | `hash_password()`, `verify_password()` |
| Token creation | `backend/src/auth/session.rs` | `create_token()`, `validate_token()` |
| Token extraction | `backend/src/handlers/auth.rs` | `extract_claims_required()` |
| Idle timeout refresh | `backend/src/handlers/auth.rs` | `me()` reissues token with fresh `iat` |
| Recovery code issue | `backend/src/handlers/admin.rs` | `issue_recovery_code()` |
| Password reset | `backend/src/handlers/auth.rs` | `reset_password()` |
| Frontend login | `frontend/src/pages/login.rs` | `LoginPage` component |
| Frontend auth state | `frontend/src/state/auth.rs` | `AuthState` |

## Security Middleware

| Layer | File | Purpose |
|-------|------|---------|
| Security headers | `backend/src/security/headers.rs` | CSP, X-Content-Type-Options, X-Frame-Options |
| CSRF validation | `backend/src/handlers/auth.rs` | `require_csrf_with_state()` validates server-stored tokens |
| CSRF token storage | `backend/src/app/state.rs` | `csrf_tokens: Arc<Mutex<HashMap>>` per-session store |
| Auth middleware (any) | `backend/src/routes/mod.rs` | `require_auth` middleware on authenticated routes |
| Auth middleware (staff) | `backend/src/routes/mod.rs` | `require_staff` middleware on MerchantStaff+ routes |
| Auth middleware (ops) | `backend/src/routes/mod.rs` | `require_ops` middleware on PlatformOps+ routes |
| Auth middleware (admin) | `backend/src/routes/mod.rs` | `require_admin` middleware on Administrator routes |
| Store isolation | `backend/src/handlers/auth.rs` | `enforce_store_isolation()` checks claims.store_id |

## Permission Checks

| Check Type | Location | How to Verify |
|------------|----------|---------------|
| Route-level role check | `backend/src/routes/mod.rs` | `require_auth`, `require_staff`, `require_ops`, `require_admin` middleware on route groups |
| Handler-level role check | Each handler file | Look for `require_role()` calls at handler start |
| CSRF on state-changing | All POST/PUT handlers | Look for `require_csrf_with_state()` calls |
| Object-level store check | `backend/src/handlers/reservations.rs`, `vehicles.rs`, `bays.rs`, `tickets.rs`, `uploads.rs` | Look for `enforce_store_isolation()` calls |
| Reservation asset ownership | `backend/src/handlers/reservations.rs` | Verifies asset belongs to claimed store_id before engine call |
| Ticket store isolation | `backend/src/handlers/tickets.rs` | Resolves ticket's store via reservation, checks `enforce_store_isolation` |
| Photographer isolation | `backend/src/handlers/assignments.rs` | Look for `photographer_user_id = user.id` filter |
| Admin-only guards | `backend/src/handlers/admin.rs`, `backup.rs` | Look for `require_admin` middleware + `require_role(Administrator)` |

## Object-Level Access Checks

- **Store isolation**: Search for `store_id` parameter in repository queries. MerchantStaff handlers pass `user.store_id` as mandatory filter. PlatformOps/Admin handlers pass `None` to get all stores.
- **Customer isolation**: Search for `user_id` filter in reservation and ticket queries for Customer role.
- **Photographer isolation**: `backend/src/repositories/assignments.rs` joins on `photographer_user_id`.
- **Vehicle status transitions**: `backend/src/handlers/vehicles.rs` -> `update_status()` validates transition + role.

## Core Business Modules

| Module | File | What to Check |
|--------|------|--------------|
| Reservation engine | `backend/src/services/reservation_engine.rs` | Transaction, version check, retry loop, conflict reasons, alternative computation |
| Ticket engine | `backend/src/services/ticket_engine.rs` | Generation format, redemption block, 2-min undo window, reason requirement |
| Upload validation | `backend/src/services/uploads.rs` | Magic byte check, MIME sniff, size limit, fingerprint dedup |
| Audit chain | `backend/src/audit/chain.rs` | Hash chain computation, append-only enforcement |
| Hash anchors | `backend/src/audit/anchors.rs` | Periodic anchor creation, cumulative hash |
| Encryption | `backend/src/security/encryption.rs` | AES-256-GCM encrypt/decrypt |
| Masking | `backend/src/security/masking.rs` | VIN last-4, plate last-2, username first-char |
| Backup | `backend/src/backup/mod.rs` | Encrypt, write, audit log |

## Test Entry Points

| Test Suite | Location | How to Run |
|-----------|----------|-----------|
| Backend unit tests | `backend/src/` (inline modules) | `cd backend && cargo test --lib` |
| Backend integration | `backend/tests/integration_tests.rs` | `cd backend && cargo test --test integration_tests` |
| Standalone unit tests | `unit_tests/*.rs` | See individual files |
| API tests | `API_tests/*.rs` | See individual files |
| All tests | `run_tests.sh` | `./run_tests.sh` |

## Known Manual Verification Boundaries

The following cannot be fully verified through static review or unit tests and require runtime verification:

1. **True concurrency under load**: SQLite WAL mode + Axum multi-threaded server under concurrent reservation requests. Unit tests simulate version conflicts but do not exercise real concurrent HTTP requests.

2. **QR code scanning**: Camera-based scanning in the check-in UI requires a real device/browser.

3. **Docker deployment**: `docker-compose up` must be tested on a machine with Docker installed.

4. **Backup file integrity**: Full encrypted backup -> restore cycle requires filesystem access and a running system.

5. **Browser rendering**: Leptos WASM rendering, especially calendar grid layout and QR code display, requires a real or headless browser.

6. **12-hour session idle timeout**: Token validation logic is unit-tested with artificial timestamps, but real idle behavior requires a long-running session.

7. **EXIF stripping**: Upload content stripping for JPEG is implemented but verifying that all potential executable payloads are removed requires specialized test files.

8. **Nginx proxy configuration**: The reverse proxy from frontend to backend must be tested in the Docker environment.

9. **File permission security**: The encryption key table and upload directory permissions depend on the deployment environment.

10. **Network isolation**: The system assumes facility-network-only access; actual network segmentation is an infrastructure concern outside the application.
