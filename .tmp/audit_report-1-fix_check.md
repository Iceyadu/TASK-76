Recheck Results for `audit_report-1.md`

Date: 2026-04-14  
Type: Static-only verification  
Scope: Re-validated Section 5 issue list, Section 6 security summary, Section 7 tests/logging summary, and Section 8 coverage mapping in `.tmp/audit_report-1.md`.

## Overall Recheck Result

Previously reported Section 5 issues resolved: **11/11**  
Section 6 security summary partial-pass findings reconciled: **5/5**  
Section 7 tests/logging partial-pass findings reconciled: **4/4**  
Section 8 coverage mappings reconciled: **4/4**  
Remaining unresolved items from that report: **0**

## A) Issues from Section 5

1) **Issue 5.1**  
**Title:** Backend library crate missing vs `cargo test --lib` / integration imports  
**Previous status:** Remediated  
**Recheck status:** Fixed  
**Evidence:** `repo/backend/Cargo.toml`, `repo/backend/src/lib.rs`  
**Conclusion:** Library target and import surface are present and aligned.

2) **Issue 5.2**  
**Title:** Backup/restore non-functional or misleading  
**Previous status:** Remediated  
**Recheck status:** Fixed  
**Evidence:** `repo/backend/src/handlers/backup.rs`, `repo/backend/Cargo.toml` (`rusqlite` backup feature)  
**Conclusion:** Encrypted backup and restore flow is implemented in handlers and dependency config.

3) **Issue 5.3**  
**Title:** CSRF not session-bound or missing on mutating routes  
**Previous status:** Remediated  
**Recheck status:** Fixed  
**Evidence:** `repo/backend/src/handlers/auth.rs`, `repo/backend/src/handlers/reservations.rs`, `repo/backend/src/handlers/uploads.rs`, `repo/backend/src/handlers/tickets.rs`  
**Conclusion:** CSRF is session-bound and enforced on covered mutating endpoints.

4) **Issue 5.4**  
**Title:** Store/object authorization trusting client `store_id`  
**Previous status:** Remediated  
**Recheck status:** Fixed  
**Evidence:** `repo/backend/src/handlers/auth.rs`, `repo/backend/src/handlers/reservations.rs`, `repo/backend/src/handlers/uploads.rs`, `repo/backend/src/handlers/tickets.rs`  
**Conclusion:** Store isolation is enforced in handler logic and no longer trusts client-only scope.

5) **Issue 5.5**  
**Title:** Calendar API ignoring view/filters  
**Previous status:** Remediated  
**Recheck status:** Fixed  
**Evidence:** `repo/backend/src/handlers/calendar.rs`  
**Conclusion:** Calendar query handling includes view and filter-driven behavior.

6) **Issue 5.6**  
**Title:** Audit table not append-only  
**Previous status:** Remediated  
**Recheck status:** Fixed  
**Evidence:** `repo/backend/migrations/001_initial_schema.sql`, `repo/backend/tests/integration_tests.rs`  
**Conclusion:** Append-only protections and test assertions are present.

7) **Issue 5.7**  
**Title:** Ticket redemption without validity window  
**Previous status:** Remediated  
**Recheck status:** Fixed  
**Evidence:** `repo/backend/src/services/ticket_engine.rs`, `repo/backend/tests/integration_tests.rs`  
**Conclusion:** Validity window checks are implemented and covered by tests.

8) **Issue 5.8**  
**Title:** Session semantics (idle vs absolute-only)  
**Previous status:** Remediated  
**Recheck status:** Fixed  
**Evidence:** `repo/backend/src/auth/session.rs`, `repo/backend/src/handlers/auth.rs`  
**Conclusion:** Idle-time semantics and refresh path are implemented consistently.

9) **Issue 5.9**  
**Title:** Frontend datetime vs backend parse format mismatch  
**Previous status:** Remediated  
**Recheck status:** Fixed  
**Evidence:** `repo/backend/src/handlers/reservations.rs`, `repo/backend/src/services/reservation_engine.rs`  
**Conclusion:** Datetime normalization/parsing alignment is implemented.

10) **Issue 5.10**  
**Title:** Unit/API tests existed as scattered or non-executable artifacts  
**Previous status:** Remediated  
**Recheck status:** Fixed  
**Evidence:** `repo/backend/tests/unit_tests_runner.rs`, `repo/backend/tests/unit/`, `repo/backend/tests/api_tests_runner.rs`, `repo/backend/tests/api/`  
**Conclusion:** Tests are consolidated and runner-wired under `backend/tests`.

11) **Issue 5.11**  
**Title:** HTTP-level API proof missing  
**Previous status:** Remediated  
**Recheck status:** Fixed  
**Evidence:** `repo/backend/tests/api/http_support.rs`, `repo/backend/tests/api/test_api_*.rs`, `repo/backend/tests/api_tests_runner.rs`  
**Conclusion:** Real route-level HTTP test suite is present via `axum-test`.

## B) Section 6 — Security Review Summary

**6.1 Authentication**  
**Previous status:** Partial Pass  
**Recheck status:** Resolved  
**Evidence:** `repo/backend/src/auth/password.rs`, `repo/backend/src/auth/session.rs`, `repo/backend/src/handlers/auth.rs`  
**Conclusion:** Authentication flows and session handling match Report 1 remediation intent.

**6.2 Route-level authorization**  
**Previous status:** Partial Pass  
**Recheck status:** Resolved  
**Evidence:** `repo/backend/src/routes/mod.rs` (`require_auth`, `require_staff`, `require_ops`, `require_admin`)  
**Conclusion:** Route groups are explicitly tiered by role.

**6.3 Object-level authorization**  
**Previous status:** Partial Pass  
**Recheck status:** Resolved  
**Evidence:** `repo/backend/src/handlers/auth.rs` (`enforce_store_isolation`), reservation/ticket/upload handlers  
**Conclusion:** High-risk object/store paths enforce scope checks.

**6.4 Tenant/store isolation**  
**Previous status:** Partial Pass  
**Recheck status:** Resolved  
**Evidence:** `repo/backend/src/handlers/*` store checks, `repo/backend/tests/integration_tests.rs` store isolation assertions  
**Conclusion:** Store isolation is implemented and test-backed in core flows.

**6.5 Admin/internal protection**  
**Previous status:** Partial Pass  
**Recheck status:** Resolved  
**Evidence:** `repo/backend/src/routes/mod.rs`, `repo/backend/src/handlers/backup.rs`, `repo/backend/src/handlers/admin.rs`  
**Conclusion:** Admin-sensitive operations are behind administrator-only gates.

## C) Section 7 — Tests and Logging Review

**7.1 Unit tests**  
**Previous status:** Partial Pass  
**Recheck status:** Resolved  
**Evidence:** `repo/backend/tests/unit/`, `repo/backend/tests/unit_tests_runner.rs`, frontend `#[cfg(test)]` modules  
**Conclusion:** Unit coverage is present and explicitly wired.

**7.2 API/integration tests**  
**Previous status:** Partial Pass  
**Recheck status:** Resolved  
**Evidence:** `repo/backend/tests/integration_tests.rs`, `repo/backend/tests/api/`, `repo/backend/tests/api_tests_runner.rs`  
**Conclusion:** Integration and HTTP suites are both present and scoped.

**7.3 HTTP tests**  
**Previous status:** Partial Pass  
**Recheck status:** Resolved  
**Evidence:** `repo/backend/tests/api/test_api_auth.rs`, `test_api_reservations.rs`, `test_api_tickets.rs`, `test_api_uploads.rs`, `test_api_vehicles.rs`, `test_api_audit.rs`, `test_api_stores.rs`  
**Conclusion:** HTTP route verification exists for key backend surfaces.

**7.4 Logging/observability**  
**Previous status:** Partial Pass  
**Recheck status:** Resolved  
**Evidence:** `repo/backend/src/audit/chain.rs`, `repo/backend/src/errors/mod.rs`, `repo/backend/src/main.rs` (tracing setup)  
**Conclusion:** Logging/audit structure remains aligned with report expectations.

## D) Coverage Confirmations from Section 8

1) **Auth + CSRF coverage**  
**Previous status:** Partial Pass  
**Recheck status:** Fixed  
**Evidence:** `repo/backend/tests/integration_tests.rs`, `repo/backend/tests/api/test_api_auth.rs`, `repo/backend/tests/api/test_api_reservations.rs`  
**Conclusion:** Auth and CSRF assertions are present across integration + HTTP tests.

2) **Reservations + conflicts coverage**  
**Previous status:** Partial Pass  
**Recheck status:** Fixed  
**Evidence:** `repo/backend/src/services/reservation_engine.rs`, `repo/backend/tests/unit/test_reservation_engine.rs`, `repo/backend/tests/api/test_api_reservations.rs`  
**Conclusion:** Reservation and conflict logic is covered in unit and API layers.

3) **Tickets + undo coverage**  
**Previous status:** Partial Pass  
**Recheck status:** Fixed  
**Evidence:** `repo/backend/src/services/ticket_engine.rs`, `repo/backend/tests/unit/test_ticket_engine.rs`, `repo/backend/tests/api/test_api_tickets.rs`  
**Conclusion:** Ticket redeem/undo behavior is covered and wired.

4) **Audit append-only coverage**  
**Previous status:** Partial Pass  
**Recheck status:** Fixed  
**Evidence:** `repo/backend/migrations/001_initial_schema.sql`, `repo/backend/tests/integration_tests.rs`  
**Conclusion:** Append-only trigger behavior is represented in schema and tests.

## Final Determination

Based on static evidence in the current repository, all Section 5 issues from `.tmp/audit_report-1.md`, all Section 6 and Section 7 partial-pass findings, and the Section 8 coverage mappings are reconciled as fixed in this recheck.
