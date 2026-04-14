# FleetReserve Audit Verification

Date: 2026-04-14
Workspace: `/Users/mac/Documents/EaglePoint/TASK-w2t76`
Reviewer: Codex

## Audit Target

- Source report: `.tmp/audit_report-1.md`
- Purpose: verify status of each numbered issue from the report

## Executive Summary

- All issues listed in `audit_report-1.md` are fixed.
- Verification is mapped issue-by-issue below.

## Item-by-Item Verification

| # | Issue from `audit_report-1.md` | Status | Evidence |
|---|---|---|---|
| 1 | Backend library crate missing vs `cargo test --lib` / integration imports | Fixed | `repo/backend/Cargo.toml`, `repo/backend/src/lib.rs` |
| 2 | Backup / restore non-functional or misleading | Fixed | `repo/backend/src/handlers/backup.rs`, `repo/backend/Cargo.toml` |
| 3 | CSRF not session-bound or missing on mutating routes | Fixed | `repo/backend/src/handlers/auth.rs`, `repo/backend/src/handlers/reservations.rs`, `repo/backend/src/handlers/uploads.rs`, `repo/backend/src/handlers/tickets.rs` |
| 4 | Store / object authorization trusting client `store_id` | Fixed | `repo/backend/src/handlers/auth.rs`, `repo/backend/src/handlers/reservations.rs`, `repo/backend/src/handlers/uploads.rs`, `repo/backend/src/handlers/tickets.rs` |
| 5 | Calendar API ignoring view / filters | Fixed | `repo/backend/src/handlers/calendar.rs` |
| 6 | Audit table not append-only | Fixed | `repo/backend/migrations/001_initial_schema.sql`, `repo/backend/tests/integration_tests.rs` |
| 7 | Ticket redemption without validity window | Fixed | `repo/backend/src/services/ticket_engine.rs`, `repo/backend/tests/integration_tests.rs` |
| 8 | Session semantics (idle vs absolute-only) | Fixed | `repo/backend/src/auth/session.rs`, `repo/backend/src/handlers/auth.rs` |
| 9 | Frontend datetime vs backend parse format | Fixed | `repo/backend/src/handlers/reservations.rs`, `repo/backend/src/services/reservation_engine.rs` |
| 10 | Unit tests scattered (`repo/unit_tests` and `backend/tests`) | Fixed | `repo/backend/tests/unit_tests_runner.rs`, `repo/backend/tests/unit/`, `repo/backend/tests/api_tests_runner.rs`, `repo/backend/tests/api/` |
| 11 | HTTP-level API proof missing | Fixed | `repo/backend/tests/api/http_support.rs`, `repo/backend/tests/api/test_api_*.rs`, `repo/backend/tests/api_tests_runner.rs` |

--|
| Docker README vs compose | Fixed | `repo/README.md`, `repo/docker-compose.yml` |
| `rusqlite` backup feature | Fixed | `repo/backend/Cargo.toml` |
| Sensitive field masking | Fixed | `repo/backend/src/handlers/vehicles.rs`, `repo/backend/src/security/masking.rs` |
| Recovery code expiry | Fixed | `repo/backend/src/repositories/recovery_codes.rs`, `repo/backend/src/handlers/auth.rs` |
