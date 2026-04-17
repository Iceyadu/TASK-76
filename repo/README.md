Project Type: fullstack

# FleetReserve Operations Suite

Offline fleet scheduling + e-ticket check-in with role-aware operations, auditability, and backup/restore.

## Quick Start (Required Command)

```bash
docker-compose up
```

Use `docker-compose up --build` for a fresh rebuild.

## Access

- Frontend UI: [http://localhost:8081](http://localhost:8081)
- Backend API (direct): [http://localhost:3001](http://localhost:3001)

## Auth and Demo Credentials

Authentication is required.

| Role | Username | Password |
|---|---|---|
| Administrator | `admin` | `FleetReserveHttpTest#2026` |
| PlatformOps | `ops1` | `FleetReserveRoleTest#2026` |
| MerchantStaff | `merchant1` | `FleetReserveRoleTest#2026` |
| Photographer | `photo1` | `FleetReserveRoleTest#2026` |
| Customer | `customer1` | `FleetReserveRoleTest#2026` |

### Bootstrap / reset process

If admin credentials were changed in your environment, use API/UI bootstrap only:

1. Start stack (`docker-compose up`).
2. Sign in with current admin credentials if available.
3. If sign-in fails, use the recovery-code reset flow:
   - issue code via `POST /api/admin/recovery-codes` from an existing admin session
   - reset via `POST /api/auth/reset-password`
4. Confirm new credentials by logging in at `http://localhost:8081/login`.

## Startup Verification (Concrete)

1. `docker-compose ps` must show `backend` and `frontend` as `Up`.
2. `curl -s http://localhost:3001/api/auth/login -H "content-type: application/json" -d '{"username":"admin","password":"FleetReserveHttpTest#2026"}'`
   - expected: HTTP `200` with `token` and `csrf_token` fields.
3. Open `http://localhost:8081/login` and sign in using the same admin credentials.

## Test Execution

```bash
./run_tests.sh
```

`run_tests.sh` is Docker-only and runs backend + frontend suites.

### Successful output indicators

- Script ends with `=== All available tests complete ===`
- No `FAILED` test blocks in output
- Docker run exits with code `0`

### Failure interpretation

- `Cannot connect to the Docker daemon`: Docker service not running
- Rust compile/test failure output: test or code regression; inspect preceding `error:` blocks
- Non-zero exit from script: at least one suite failed

## Backend API Inventory

All routes are under `/api`.

### Public
- `POST /api/auth/login`
- `POST /api/auth/reset-password`

### Authenticated (`require_auth`)
- `POST /api/auth/logout`
- `GET /api/auth/me`
- `POST /api/reservations`
- `GET /api/reservations`
- `GET /api/tickets/:id`
- `GET /api/assignments`

### Staff+ (`require_staff`)
- `GET /api/vehicles`
- `GET /api/vehicles/:id`
- `POST /api/vehicles`
- `PUT /api/vehicles/:id/status`
- `GET /api/bays`
- `POST /api/bays`
- `GET /api/stores`
- `GET /api/calendar`
- `POST /api/tickets/:id/redeem`
- `POST /api/tickets/:id/undo`
- `POST /api/uploads`
- `POST /api/assignments`

### PlatformOps+ (`require_ops`)
- `GET /api/exports`
- `GET /api/audit`

### Administrator (`require_admin`)
- `GET /api/admin/users`
- `POST /api/admin/users`
- `GET /api/admin/permissions`
- `POST /api/admin/permissions`
- `POST /api/admin/permissions/:id`
- `PUT /api/admin/users/:id/role`
- `PUT /api/admin/users/:id/active`
- `POST /api/admin/recovery-codes`
- `POST /api/backup`
- `POST /api/backup/restore`

## UI Verification Flow (End-to-End)

1. Login as `admin` in UI.
2. Vehicles: verify list renders, create vehicle, update status.
3. Reservations: create reservation and confirm ticket output.
4. Check-in (`/checkin`): redeem then undo a ticket.
5. Admin: list users, issue recovery code, create and restore backup.
6. Role checks: sign in as `customer1`, `photo1`, `merchant1`, `ops1` and verify forbidden/allowed screens.

## Manual Verifications with Observable Output

1. Concurrency contention:
   - run two parallel reservation creates for same asset/time
   - expected: one `201`, one `409 conflict`
2. QR camera scan:
   - scan generated ticket at `/checkin`
   - expected: `SUCCESS: Ticket redeemed successfully!`
3. Backup/restore:
   - create backup in Admin page
   - expected: success message + encrypted `.enc` file under mounted backup path
