# FleetReserve Operations Suite — Acceptance & Architecture Audit (Static + Remediation Summary)

**Date:** 2026-04-11  
**Repository:** `repo/` (under workspace root `TASK-w2t76/`)

## 1. Verdict

- **Overall conclusion:** **Partial Pass** (admissible for static review).
- **Prior static findings (2026-04-07 — 2026-04-09)** described **contract drift**: backend manifest vs `cargo test --lib`, weak or missing CSRF on some routes, backup/restore stubs, store/object authorization gaps, audit mutability, and **non-executable** `unit_tests` / `API_tests` artifacts.
- **Subsequent remediation** aligned: **`[lib]` + `src/lib.rs`**, migrations and seeds, handlers and engines, CSRF storage in `AppState`, route tiering (`require_auth` / `require_staff` / `require_ops` / `require_admin`), append-only audit triggers, ticket validity enforcement, encrypted backup path, **`run_tests.sh`** orchestration, and executable suites under **`backend/tests/`** (`unit/` via **`unit_tests_runner`**, HTTP routes in **`api/`** via **`axum-test`** + **`api_tests_runner`**).
- **Residual risk** is expected: **runtime verification** (browser Leptos, real QR scanners, production secrets in `.env`, concurrent HTTP against SQLite) remains recommended.

## 2. Scope and Verification Boundary

- **Reviewed:** `docs/` (design, api-spec, questions, test-coverage), `repo/backend` (Axum, SQLite, migrations), `repo/frontend` (Leptos), `repo/docker-compose.yml`, `repo/run_tests.sh`, `repo/backend/tests/` (integration, `unit/`, `api/`, runners).
- **Static boundary:** This document does not certify production load, WASM bundle size, or third-party scanner behavior.
- **Manual verification:** first-time Docker build, seeded admin login, backup file on real disk, check-in / QR in browser.

## 3. Repository / Requirement Mapping Summary

- **Auth / RBAC / store scope:** Argon2id passwords (`auth/password.rs`); HMAC-signed tokens with idle window (`auth/session.rs`); `UserRole` hierarchy (`models/mod.rs`); `require_*` middleware validates token **and** live DB role (`routes/mod.rs`).
- **CSRF:** Server-stored tokens keyed by session token (`app/state.rs`, `handlers/auth.rs`); required on state-changing reservation and related paths; integration + HTTP tests exercise login → CSRF → POST.
- **Reservations:** Transactional engine with optimistic concurrency, conflict reasons, alternative slots (`services/reservation_engine.rs`); handler wiring (`handlers/reservations.rs`).
- **Tickets:** Generation, redemption, duplicate block, supervised undo window + reason (`services/ticket_engine.rs`, `handlers/tickets.rs`).
- **Audit:** Append-only chain + periodic anchors (`audit/chain.rs`, `audit/anchors.rs`); SQLite triggers block UPDATE/DELETE on audit rows where applicable (`migrations/001_initial_schema.sql`).
- **Encryption / masking:** AES-GCM for sensitive columns (`security/encryption.rs`); masked DTOs (`security/masking.rs`).
- **Uploads:** MIME + magic bytes + size + fingerprint (`services/uploads.rs`, `handlers/uploads.rs`).
- **Backup / restore:** Encrypted payload path (`handlers/backup.rs`, `backup/mod.rs`, `services/crypto.rs`).
- **Frontend:** Leptos pages and API client under `repo/frontend/src/` (calendar, reservations, tickets, check-in).
- **Tests:** `cargo test --lib`, `--test integration_tests`, `--test unit_tests_runner`, `--test api_tests_runner` (HTTP route suite in `backend/tests/api/`); `run_tests.sh` runs the stack (host `cargo` or Docker with repo mounted).

## 4. Section-by-section Review (Updated)

### 4.1 Hard Gates

#### 4.1.1 Documentation and static verifiability

- **Conclusion:** **Partial Pass** — `repo/README.md` and `docs/design.md` / `docs/api-spec.md` describe stack and endpoints; `docs/test-coverage.md` maps risks to tests (maintain as suites evolve).

#### 4.1.2 Material deviation from prompt

- **Conclusion:** **Partial Pass** — Core prompt themes (offline auth, CSRF, reservations, tickets, audit, encryption, roles) are implemented in code. Remaining gaps are **operational** (browser UAT, concurrent load) rather than missing modules.

### 4.2 Delivery Completeness

#### 4.2.1 Core functional requirement coverage

- **Conclusion:** **Partial Pass** — Backend flows are strongly covered by tests; frontend is statically present with honest gaps noted in `docs/test-coverage.md` (e.g. QR encoder choice).

#### 4.2.2 End-to-end deliverable

- **Conclusion:** **Partial Pass** — Docker + README + test entrypoints are coherent; full facility-operator UAT is manual.

### 4.3 Engineering and Architecture Quality

#### 4.3.1 Structure and module decomposition

- **Conclusion:** **Partial Pass** — `handlers/`, `services/`, `repositories/`, `auth/`, `audit/`, `security/` separation is clear.

#### 4.3.2 Maintainability and extensibility

- **Conclusion:** **Partial Pass** — Role model and store scoping are centralized; permission table depth is a future enhancement.

### 4.4 Engineering Details and Professionalism

#### 4.4.1 Error handling, logging, validation, API design

- **Conclusion:** **Partial Pass** — `AppError` mapping, `tracing`, structured audit entries; API shapes align with `docs/api-spec.md` at a high level.

#### 4.4.2 Product-grade organization vs demo

- **Conclusion:** **Partial Pass** — Not a single-file demo; migrations and tests support review.

### 4.5 Prompt Understanding and Requirement Fit

#### 4.5.1 Business goal and constraint fidelity

- **Conclusion:** **Partial Pass** — Fleet scheduling, e-tickets, multi-role, offline-first assumptions reflected in implementation choices documented in `docs/questions.md`.

### 4.6 Aesthetics (frontend / full-stack)

#### 4.6.1 Visual / interaction quality

- **Conclusion:** **Partial Pass** — Leptos UI structure is reviewable; pixel-perfect and device QA are runtime.

## 5. Original Severity-Rated Issues — Remediation Status

| # | Title | Status | Evidence (indicative) |
|---|--------|--------|------------------------|
| 1 | Backend **library crate** missing vs `cargo test --lib` / integration imports | **Remediated** | `repo/backend/Cargo.toml` `[lib]`, `src/lib.rs`, `tests/integration_tests.rs` |
| 2 | **Backup / restore** non-functional or misleading | **Remediated** | `handlers/backup.rs`, `backup/mod.rs`, `test_backup_encryption_roundtrip` |
| 3 | **CSRF** not session-bound or missing on mutating routes | **Remediated** | `app/state.rs` `csrf_tokens`, `handlers/auth.rs`, uploads/reservations where required |
| 4 | **Store / object authorization** trusting client `store_id` | **Remediated** | Reservation/ticket/upload handlers + engines; `test_store_isolation_enforcement` |
| 5 | **Calendar** API ignoring view / filters | **Remediated** | `handlers/calendar.rs`, `models` calendar request types (verify against `docs/api-spec.md`) |
| 6 | **Audit** table not append-only | **Remediated** | Triggers in `migrations/001_initial_schema.sql`, `test_audit_log_*_blocked_by_trigger` |
| 7 | **Ticket redemption** without validity window | **Remediated** | `services/ticket_engine.rs`, `test_ticket_validity_window_enforcement` |
| 8 | **Session** semantics (idle vs absolute-only) | **Remediated** | `auth/session.rs` idle window + `test_idle_timeout_window`, `/api/auth/me` refresh path |
| 9 | **Frontend datetime** vs backend parse format | **Remediated** | Aligned reservation/calendar payloads (see `reservation_engine` parse + frontend pages) |
| 10 | **unit_tests / API_tests** comment-only | **Remediated** | `backend/tests/unit_tests_runner.rs`, `backend/tests/api_tests_runner.rs`, `backend/tests/unit/*.rs`, `backend/tests/api/*.rs` |
| 11 | **HTTP-level** API proof missing | **Remediated** | `backend/tests/api/*.rs`, `backend/tests/api/http_support.rs`, `backend/tests/api_tests_runner.rs`, `run_tests.sh` |

### Additional items (parity with follow-on audits)

| Topic | Status | Evidence |
|--------|--------|----------|
| Docker README vs compose | **Remediated** | `repo/docker-compose.yml`, `repo/README.md` |
| `rusqlite` backup feature | **Remediated** | `Cargo.toml` `features = [\"bundled\", \"backup\"]` |
| Sensitive field masking | **Remediated** | `security/masking.rs`, vehicle list/get handlers |
| Recovery code expiry | **Remediated** | `repositories/recovery_codes.rs`, integration tests |

## 6. Security Review Summary (Updated)

- **Authentication:** **Partial Pass** — Argon2id + signed tokens; seeded admin is a documented bootstrap risk.
- **Route-level authorization:** **Partial Pass** — Layered `require_*` middleware on route groups.
- **Object-level authorization:** **Partial Pass** — Store checks on critical paths; expand with granular permission rows over time.
- **Tenant / store isolation:** **Partial Pass** — Tests enforce merchant store boundaries.
- **Admin / internal protection:** **Partial Pass** — Admin-only backup/restore and sensitive admin handlers.

## 7. Tests and Logging Review

- **Unit tests:** **Partial Pass** — Broad inline `#[cfg(test)]` coverage in backend crates.
- **API tests:** **Partial Pass** — `backend/tests/api/` modules execute against real DB fixtures via runners.
- **HTTP tests:** **Partial Pass** — `api_tests_runner` / `backend/tests/api/` hit real routes (auth, reservations, tickets, vehicles, uploads, audit, CSRF).
- **Logging / observability:** **Partial Pass** — `tracing` + audit chain; avoid logging raw secrets (review env setup).

## 8. Test Coverage Assessment

- **Overall:** **Partial Pass** — Layered strategy (lib → integration → standalone runners → HTTP) mitigates former “spec-only” gaps; `docs/test-coverage.md` lists honest manual gaps.

## 9. Final Notes

- **Admission:** **Partial Pass** is consistent: static evidence is strong; runtime and concurrency proof are explicitly bounded.
- **Submissions:** Package **`sessions/*.json`** as **plain chat exports** (string `content` only); use `sessions/SESSION_INDEX.md` for IDs ↔ files. Do not submit raw agent trajectories as “sessions.”
- **Tooling:** Implementation work may be attributed to the IDE/agent stack used in your course policy (e.g. Cursor); this audit is product-agnostic.
