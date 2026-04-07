# FleetReserve Operations Suite

Offline vehicle fleet scheduling, e-ticket admissions, and auditable multi-role access for local rental yards and dealership service centers.

## Architecture

- **Frontend**: Leptos 0.6 (Rust -> WebAssembly), served via nginx
- **Backend**: Axum 0.7 (Rust), REST API on port 3001
- **Database**: SQLite (WAL mode) as on-device system of record
- **Deployment**: Docker Compose

```
Browser (Leptos WASM) -> nginx:8080 -> Axum:3001 -> SQLite
```

## Prerequisites

- Rust 1.77+ with `wasm32-unknown-unknown` target
- trunk (`cargo install trunk`)
- Docker and Docker Compose (for containerized deployment)

## Quick Start (Docker)

```bash
docker-compose up --build
```

Access the application at http://localhost:8080

Bootstrap admin account `admin` is seeded as disabled by default.
Activate and reset credentials through your secure bootstrap process before first use.

## Quick Start (Development)

```bash
# Terminal 1: Backend
cd backend
ENCRYPTION_KEY="dev-key-32-bytes-long-padding!!" \
HMAC_SECRET="dev-hmac-secret-32-bytes-long!!" \
cargo run

# Terminal 2: Frontend
cd frontend
trunk serve
```

## Running Tests

```bash
./run_tests.sh

# Or individually:
cd backend && cargo test
```

## Package Structure

```
repo/
  frontend/          Leptos WASM application
  backend/           Axum REST API server
  docker-compose.yml Container orchestration
  run_tests.sh       Test runner
  unit_tests/        Reviewer-facing test references
  API_tests/         API behavior specifications
```

## Security Summary

| Feature | Implementation |
|---------|---------------|
| Password hashing | Argon2id (19 MiB, 2 iterations) |
| Session tokens | HMAC-SHA256 signed, 12-hour idle timeout |
| CSRF protection | Random token per session, required on POST/PUT/DELETE |
| XSS prevention | Leptos auto-escaping, Content-Security-Policy |
| Encryption at rest | AES-256-GCM for VIN, license plate, email |
| Data masking | VIN last 4, plate last 2, username first char only |
| Audit trail | Append-only SHA-256 hash chain with periodic anchors |
| Upload validation | Magic bytes + MIME sniff + size limit + SHA-256 dedup |
| Authorization | Route-level + object-level + store isolation |

## Role Model

| Role | Access Scope |
|------|-------------|
| Customer | Own reservations and tickets |
| Photographer | Assigned jobs only |
| Merchant/Store Staff | Own store fleet, reservations, check-in |
| Platform Operations | Cross-store calendars, exports, audit |
| Administrator | All + user/role management, backup/restore |

## API Overview

All endpoints are under `/api/`.

- `POST /api/auth/login` - Authenticate
- `GET /api/calendar` - Availability calendar (day/week, 15-min increments)
- `POST /api/reservations` - Create reservation with conflict detection
- `POST /api/tickets/:id/redeem` - Redeem ticket (blocks re-entry)
- `POST /api/tickets/:id/undo` - Supervised undo (2-min window, reason required)
- `PUT /api/vehicles/:id/status` - Status transition (permission-gated)
- `POST /api/uploads` - Photo upload (JPEG/PNG, magic byte validation)

## Known Limitations

1. Ticket QR rendering is deterministic SVG and not a standards-compliant QR encoder
2. SQLite provides limited concurrent write throughput compared to PostgreSQL
3. File upload content stripping is simplified (strips EXIF APP segments for JPEG)
4. Session tokens are stored in browser memory (cleared on page refresh)
5. Backup encryption uses the system key; key escrow is the admin's responsibility

## Manual Verification Required

1. True concurrent reservation creation under load
2. QR code camera scanning in browser
3. Docker build and deployment
4. Full backup/restore cycle
5. Browser rendering of calendar and ticket views
6. 12-hour idle session timeout in real-time
7. Nginx reverse proxy configuration in Docker
