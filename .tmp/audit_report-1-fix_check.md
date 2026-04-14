# Fix check — `.tmp/audit_report-1.md`

**Date:** 2026-04-11  
**Paired document:** `.tmp/audit_report-1.md` (same heading numbers and issue IDs in §5).

For each audit row: **Status** = remediation vs the cited finding; **Evidence** = where to verify in the repo today. When the audit narrative was written before a code fix, this file records the **current** repo state; update the audit body in the same change when conclusions change materially.

**Same issues?** Yes — `audit_report-1.md` **§5** numbered items **1–11** map one-to-one to **§5** in this file; **§4** subsections and **§6–§9** align with the audit headings.

**Convention:** Rows below use **Pass** where the cited gap is **remediated in tree** as of this date; **Partial** is used only where the audit itself keeps residual scope (e.g. manual browser proof).

---

## 1. Verdict (audit §1)

| Audit text | Fix-check status | Evidence |
|------------|------------------|----------|
| Overall **Partial Pass**; prior contract drift; remediation in library shape, CSRF, backup, authz, audit, tests | **Pass** | Matches `audit_report-1.md` §1: `Cargo.toml` `[lib]`, `src/lib.rs`, `routes/mod.rs`, `handlers/*`, `run_tests.sh`, `backend/tests/api/`, `backend/tests/api_tests_runner.rs` |

---

## 2. Scope (audit §2)

| Audit text | Fix-check status | Evidence |
|------------|------------------|----------|
| Static boundary; manual verification for runtime / Docker / QR | **Pass** | Scope unchanged; fix-check documents code alignment for listed artifacts under `repo/` and `docs/` |

---

## 3. Repository mapping summary (audit §3)

| Audit text | Fix-check status | Evidence |
|------------|------------------|----------|
| Auth, CSRF, reservations, tickets, audit, encryption, uploads, backup, tests | **Pass** | `auth/`, `services/reservation_engine.rs`, `services/ticket_engine.rs`, `audit/chain.rs`, `security/encryption.rs`, `services/uploads.rs`, `handlers/backup.rs`, `repo/run_tests.sh` |

---

## 4. Section-by-section review (audit §4)

### 4.1 Hard Gates

#### 4.1.1 Documentation and static verifiability

| Audit concern | Status | Evidence |
|---------------|--------|----------|
| README / docs vs empty or broken tree | **Pass** | `repo/README.md`, `docs/design.md`, `docs/api-spec.md`, `docs/test-coverage.md` |
| Cannot run `cargo test --lib` | **Pass** | `repo/backend/Cargo.toml` `[lib]`, `src/lib.rs` |

#### 4.1.2 Material deviation from prompt

| Audit concern | Status | Evidence |
|---------------|--------|----------|
| Missing core modules (auth, reservations, tickets) | **Pass** | `repo/backend/src/` module layout and migrations `001_initial_schema.sql` / `002_seed_data.sql` |

### 4.2 Delivery Completeness

#### 4.2.1 Core explicit requirements

| Audit concern | Status | Evidence |
|---------------|--------|----------|
| Reservation / ticket / audit behavior unimplemented | **Pass** | Engines + handlers + `integration_tests.rs` |

#### 4.2.2 End-to-end vs fast harness

| Audit concern | Status | Evidence |
|---------------|--------|----------|
| No runnable test entrypoint | **Pass** | `repo/run_tests.sh`; Docker path `-v repo:/app` `-w /app/backend` |

### 4.3 Engineering and Architecture Quality

#### 4.3.1 Structure and modular decomposition

| Audit conclusion | Status | Evidence |
|------------------|--------|----------|
| Partial Pass — layered backend | **Pass** | `handlers/`, `services/`, `repositories/` |

#### 4.3.2 Maintainability and extensibility

| Audit concern | Status | Evidence |
|---------------|--------|----------|
| Role / store model unclear | **Pass** | `models/mod.rs` `UserRole`, store fields on user + handlers |

### 4.4 Engineering Details and Professionalism

#### 4.4.1 Error handling / validation / API

| Audit concern | Status | Evidence |
|---------------|--------|----------|
| Ad-hoc errors only | **Pass** | `errors/mod.rs`, Axum `IntoResponse` |

#### 4.4.2 Product-grade vs demo-grade

| Audit concern | Status | Evidence |
|---------------|--------|----------|
| Single-file or stub backend | **Pass** | Full migration set + multiple integration tests |

### 4.5 Prompt understanding and requirement fit

#### 4.5.1 Business goal / constraints

| Audit concern | Status | Evidence |
|---------------|--------|----------|
| CSRF / offline auth not reflected | **Pass** | `handlers/auth.rs`, `auth/session.rs`, CSRF map on `AppState` |

### 4.6 Aesthetics

#### 4.6.1 Visual and interaction quality

| Audit conclusion | Status | Evidence |
|------------------|--------|----------|
| Partial Pass — runtime UI | **Partial** | Leptos pages under `repo/frontend/src/`; browser smoke still manual |

---

## 5. Severity-rated issues (audit §5) — one row per numbered item

| # | Title (from audit) | Status | Evidence |
|---|---------------------|--------|----------|
| 1 | Library crate / `cargo test --lib` | **Pass** | `repo/backend/Cargo.toml`, `src/lib.rs` |
| 2 | Backup / restore | **Pass** | `handlers/backup.rs`, `backup/mod.rs`, `test_backup_encryption_roundtrip` |
| 3 | CSRF session binding | **Pass** | `app/state.rs`, `handlers/auth.rs`, `test_csrf_token_session_binding` |
| 4 | Store / object authorization | **Pass** | Reservation/ticket/upload paths, `test_store_isolation_enforcement` |
| 5 | Calendar view / filters | **Pass** | `handlers/calendar.rs`, `routes/mod.rs` calendar routes |
| 6 | Audit append-only | **Pass** | `migrations/001_initial_schema.sql` triggers, `test_audit_log_*` |
| 7 | Ticket validity window | **Pass** | `services/ticket_engine.rs`, `test_ticket_validity_window_enforcement` |
| 8 | Session idle semantics | **Pass** | `auth/session.rs`, `test_idle_timeout_window`, `/api/auth/me` |
| 9 | Datetime format consistency | **Pass** | `reservation_engine` parse + frontend reservation pages |
| 10 | Executable unit + API test dirs | **Pass** | `backend/tests/unit_tests_runner.rs`, `backend/tests/api_tests_runner.rs`, `backend/tests/unit/`, `backend/tests/api/` |
| 11 | HTTP-level API tests | **Pass** | `backend/tests/api/*.rs`, `backend/tests/api/http_support.rs`, `backend/tests/api_tests_runner.rs` |

### Additional items table (audit §5 “Additional items”)

| Topic | Status | Evidence |
|--------|--------|----------|
| Docker / README | **Pass** | `docker-compose.yml`, `repo/README.md` |
| rusqlite backup | **Pass** | `Cargo.toml` `backup` feature |
| Masking | **Pass** | `security/masking.rs`, vehicle handlers |
| Recovery expiry | **Pass** | `repositories/recovery_codes.rs`, integration tests |

---

## 6. Security review summary (audit §6)

| Audit bullet | Status | Evidence |
|--------------|--------|----------|
| Authentication | **Partial** | Argon2 + tokens: `auth/password.rs`, `auth/session.rs`; seeded admin documented |
| Route authorization | **Pass** | `routes/mod.rs` `require_*` layers |
| Object authorization | **Partial** | Handlers + tests; granular permission matrix optional |
| Tenant / store isolation | **Pass** | `test_store_isolation_enforcement`, merchant paths |
| Admin protection | **Pass** | `require_admin`, backup/restore handlers |

---

## 7. Tests and logging (audit §7)

| Audit bullet | Status | Evidence |
|--------------|--------|----------|
| Unit tests | **Pass** | `cargo test --lib` modules under `src/` |
| API / integration | **Pass** | `integration_tests.rs`, runners |
| HTTP tests | **Pass** | `api_tests_runner` (`backend/tests/api/`) |
| Logging | **Pass** | `tracing`, audit append |

---

## 8. Test coverage assessment (audit §8)

### 8.1 Overview

| Audit point | Status | Evidence |
|-------------|--------|----------|
| Layered tests + honest gaps doc | **Pass** | `docs/test-coverage.md` + runners |

### 8.2 Coverage mapping (representative)

| Requirement | Status | Evidence |
|-------------|--------|----------|
| Auth + CSRF | **Pass** | Integration + `backend/tests/api/` |
| Reservations + conflicts | **Pass** | `services/reservation_engine.rs` tests + integration |
| Tickets + undo | **Pass** | `services/ticket_engine.rs` + integration |
| Audit triggers | **Pass** | `test_audit_log_append_only_triggers` |

### 8.3 Final coverage judgment

| Audit text | Fix-check status | Evidence |
|------------|------------------|----------|
| Partial Pass overall coverage | **Pass** | Static blockers from §5 cleared; manual gaps listed in `test-coverage.md` |

---

## 9. Final notes (audit §9)

| Audit “final notes” theme | Status |
|-----------------------------|--------|
| Session JSON vs trajectories | **Pass** | `sessions/*.json` normalized; `sessions/SESSION_INDEX.md` |
| Partial Pass admission | **Pass** | Verdicts consistent; no claim of full prod cert |

---

**How to use this file:** Walk `.tmp/audit_report-1.md` in order; each subsection above answers the same **§4.x / §5 #n / §6 / §7 / §8** reference.
