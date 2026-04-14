# FleetReserve Operations Suite — Second-Pass Audit (Static + Hardening Summary)

**Date:** 2026-04-11  
**Repository:** `repo/` (under workspace root `TASK-w2t76/`)  
**Paired remediation log:** `.tmp/audit_report-2-fix_check.md`

## 1. Verdict

- **Overall conclusion:** **Partial Pass** (consistent with §1 of `.tmp/audit_report-1.md`; this pass focuses on **hardening** and **test realism**).
- **Focus:** ticket isolation, recovery flows, calendar tiering, CSRF completeness, secret bootstrap, upload/backup UX alignment with backend, QR/check-in presentation, and **executable** regression coverage.
- **Residual:** fine-grained permission tables vs role-only checks, full-browser E2E, and hardware QR scans remain **manual or future automation**.

## 2. Scope and Verification Boundary

- **Reviewed:** Same corpus as Report 1, with emphasis on `handlers/tickets.rs`, `handlers/calendar.rs`, `handlers/admin.rs`, `repositories/recovery_codes.rs`, `frontend/src/pages/tickets.rs`, `frontend/src/components/ticket_display.rs`, and **`backend/tests/api/`** (HTTP route coverage via `api_tests_runner`).
- **Static boundary:** No certification of production traffic or WASM performance.
- **Manual verification:** logout + CSRF refresh paths in browser, backup file permissions, scanner devices.

## 3. Repository / Requirement Mapping Summary (Delta)

- **Ticket read isolation:** List/get ticket paths respect role and store assignment where implemented; cross-store denial covered by tests where applicable.
- **Recovery codes:** Issue and redeem paths use **30-minute** expiry semantics aligned with repositories and tests.
- **Calendar routes:** Tiered auth on calendar endpoints matches operator vs customer needs (`routes/mod.rs`).
- **CSRF on logout:** State-changing logout invalidates session-side CSRF map as required.
- **Secrets:** `main.rs` requires minimum-length `ENCRYPTION_KEY` and `HMAC_SECRET` (or documented test-only paths).
- **Upload / backup UI:** Frontend calls align with backend multipart and JSON contracts.
- **QR / validity display:** Check-in UI shows validity window; QR payload is structured for static review (full encoder may still be flagged in `test-coverage.md`).

## 4. Section-by-section Review (Updated)

### 4.1 Hard Gates

#### 4.1.1 Documentation and route inventory

- **Conclusion:** **Partial Pass** — `docs/api-spec.md` remains the contract; update when routes change.

#### 4.1.2 Material deviation from prompt (delta)

- **Conclusion:** **Partial Pass** — Second-pass gaps (ticket isolation, expiry, CSRF completeness) addressed in code; remaining items are **enhancement** or **manual**.

### 4.2 Delivery Completeness

#### 4.2.1 Core explicit requirements

- **Conclusion:** **Partial Pass** — Redemption, undo window, and validity rules have integration tests.

#### 4.2.2 End-to-end vs fast harness

- **Conclusion:** **Partial Pass** — HTTP tests complement fast unit tests; Leptos E2E still optional.

### 4.3 Engineering and Architecture Quality

#### 4.3.1 Structure

- **Conclusion:** **Partial Pass**

#### 4.3.2 Extensibility

- **Conclusion:** **Partial Pass** — Role-centric checks are clear; DB-backed permission matrix is a follow-on.

### 4.4 Engineering Details and Professionalism

#### 4.4.1 Validation and API behavior

- **Conclusion:** **Partial Pass** — Error types and status codes are centralized in `errors/mod.rs`.

#### 4.4.2 Product vs demo

- **Conclusion:** **Partial Pass**

### 4.5 Prompt understanding and requirement fit

#### 4.5.1 Business constraints

- **Conclusion:** **Partial Pass** — Supervised undo and duplicate redemption are enforced in engine + tests.

### 4.6 Aesthetics

#### 4.6.1 UI quality

- **Conclusion:** **Partial Pass** — Static review of components; runtime polish not proven here.

## 5. Severity-Rated Issues (Second Pass) — Status at 2026-04-11

| # | Title | Status | Evidence (indicative) |
|---|--------|--------|------------------------|
| 1 | Ticket **read** paths leaking cross-store data | **Remediated** | `handlers/tickets.rs`, `repositories/tickets.rs`, isolation tests |
| 2 | **Recovery code** expiry incorrect or bypassable | **Remediated** | `repositories/recovery_codes.rs`, integration tests |
| 3 | **Calendar** routes under wrong auth tier | **Remediated** | `routes/mod.rs`, `handlers/calendar.rs` |
| 4 | **Logout** missing CSRF or stale token handling | **Remediated** | `handlers/auth.rs`, CSRF map updates |
| 5 | **Secrets** defaulting to weak or empty values in prod path | **Remediated** | `main.rs` validation; `.env` / README guidance |
| 6 | **Upload** handler CSRF / store checks | **Remediated** | `handlers/uploads.rs`, tests |
| 7 | **Backup/restore** UI vs API mismatch | **Remediated** | Frontend backup pages + `handlers/backup.rs` |
| 8 | **Check-in** validity display incomplete | **Remediated** | `frontend/src/pages/tickets.rs`, ticket components |
| 9 | **QR** placeholder vs structured payload | **Remediated** | `ticket_display` / ticket engine QR data (see coverage doc) |
| 10 | **Permission table** as sole source of truth | **Open / Partial** | Role checks exist; granular `permissions` table enforcement is incremental |
| 11 | **docs/test-coverage** vs executable tests drift | **Remediated** | `backend/tests/api/` + `api_tests_runner`; maintain table ongoing |
| 12 | **Reservation** validation edge cases | **Partial** | Core paths tested; extended matrix optional |
| 13 | **Docker** developer path clarity | **Remediated** | `docker-compose.yml`, `README.md` |

## 6. Security Review Snapshot (Updated)

- **Authentication:** **Partial Pass**
- **Route authorization:** **Partial Pass**
- **Object authorization:** **Partial Pass**
- **Tenant / store isolation:** **Partial Pass**
- **Admin / internal protection:** **Pass** (for documented admin-only operations)

## 7. Test and Logging Snapshot (Updated)

- **Unit tests:** **Partial Pass**
- **API tests (runners):** **Partial Pass**
- **Integration tests:** **Partial Pass**
- **HTTP router tests:** **Partial Pass**
- **Observability:** **Partial Pass**

## 8. Manual Verification Required

- Browser flows: login, calendar, reservation, ticket QR display, check-in.
- Backup file written to host volume from Docker.
- Concurrent reservation attempts from multiple clients (SQLite limits).

## 9. Final Notes

- **Cross-reference:** Issues **#1–#11** in Report 1 and **#1–#13** here are traced in `.tmp/audit_report-1-fix_check.md` and `.tmp/audit_report-2-fix_check.md` respectively.
- **Verdict consistency:** **Partial Pass** overall; no section claims **full production certification**.
