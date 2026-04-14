# Fix check — `.tmp/audit_report-2.md`

**Date:** 2026-04-11  
**Paired document:** `.tmp/audit_report-2.md` (same heading numbers and issue IDs).

For each audit row: **Status** = remediation vs the cited finding; **Evidence** = where to verify in the repo today. When the audit narrative was written before a code fix, this file records the **current** repo state; update the audit body in the same commit when conclusions change materially.

**Same issues?** Yes — `audit_report-2.md` **§5** numbered items **1–13** map one-to-one to **§5** in this file; **§4–§9** subsections use the same references as the audit.

**Convention:** Rows use **Pass** where the cited gap is remediated or accepted as documented scope; **Partial** only where the audit explicitly leaves follow-on (e.g. permission table depth, reservation edge matrix).

---

## 1. Verdict (audit §1)

| Audit text | Fix-check status | Evidence |
|------------|------------------|----------|
| Second-pass **Partial Pass**; hardening on tickets, recovery, calendar, CSRF, secrets, tests | **Pass** | Aligns with `audit_report-2.md` §1; evidence across `handlers/tickets.rs`, `repositories/recovery_codes.rs`, `routes/mod.rs`, `backend/tests/api/` |

---

## 2. Scope (audit §2)

| Audit text | Fix-check status | Evidence |
|------------|------------------|----------|
| Static boundary; manual browser / scanner | **Pass** | Scope explicit in audit §2; this file maps code to each §5 item |

---

## 3. Repository mapping summary (audit §3)

| Audit text | Fix-check status | Evidence |
|------------|------------------|----------|
| Delta: ticket isolation, recovery, calendar tier, CSRF logout, secrets, UI alignment, QR | **Pass** | Files cited in audit §3; verified in tree |

---

## 4. Section-by-section review (audit §4)

### 4.1 Hard Gates

#### 4.1.1 Documentation and route inventory

| Audit concern | Status | Evidence |
|---------------|--------|----------|
| api-spec drift | **Pass** | `docs/api-spec.md` + `routes/mod.rs` cross-check (maintain on route changes) |

#### 4.1.2 Material deviation (delta)

| Audit concern | Status | Evidence |
|---------------|--------|----------|
| Second-pass findings unaddressed | **Pass** | §5 items 1–9, 11, 13 remediated per evidence below |

### 4.2 Delivery Completeness

#### 4.2.1 Core requirements

| Audit concern | Status | Evidence |
|---------------|--------|----------|
| Validity / isolation / expiry untested | **Pass** | `integration_tests.rs` ticket + recovery + store tests |

#### 4.2.2 E2E vs harness

| Audit concern | Status | Evidence |
|---------------|--------|----------|
| No HTTP proof | **Pass** | `backend/tests/api/` via `api_tests_runner` |

### 4.3 Engineering and Architecture Quality

#### 4.3.1 Structure

| Audit conclusion | Status | Evidence |
|------------------|--------|----------|
| Partial Pass | **Pass** | No regression vs Report 1 |

#### 4.3.2 Extensibility

| Audit concern | Status | Evidence |
|---------------|--------|----------|
| Permission table as future work | **Partial** | Role-based `require_*` today; `repositories/permissions.rs` for evolution |

### 4.4 Engineering Details and Professionalism

#### 4.4.1 Validation / API

| Audit concern | Status | Evidence |
|---------------|--------|----------|
| Error semantics inconsistent | **Pass** | `errors/mod.rs` + handler patterns |

#### 4.4.2 Product vs demo

| Audit concern | Status | Evidence |
|---------------|--------|----------|
| UI stubs only | **Pass** | Leptos pages + backend parity |

### 4.5 Prompt understanding and requirement fit

#### 4.5.1 Business constraints

| Audit concern | Status | Evidence |
|---------------|--------|----------|
| Undo / duplicate redemption | **Pass** | `ticket_engine` tests + integration |

### 4.6 Aesthetics

#### 4.6.1 UI quality

| Audit conclusion | Status | Evidence |
|------------------|--------|----------|
| Partial Pass | **Partial** | Browser polish manual |

---

## 5. Severity-rated issues (audit §5) — one row per numbered item

| # | Title (from audit) | Status | Evidence |
|---|---------------------|--------|----------|
| 1 | Ticket read isolation | **Pass** | `handlers/tickets.rs`, `repositories/tickets.rs`, isolation tests |
| 2 | Recovery code expiry | **Pass** | `repositories/recovery_codes.rs`, time-based tests |
| 3 | Calendar route tiering | **Pass** | `routes/mod.rs`, `handlers/calendar.rs` |
| 4 | Logout + CSRF | **Pass** | `handlers/auth.rs`, CSRF map lifecycle |
| 5 | Secret bootstrap strength | **Pass** | `main.rs` env validation, README |
| 6 | Upload CSRF / store checks | **Pass** | `handlers/uploads.rs`, tests |
| 7 | Backup/restore UI vs API | **Pass** | Frontend backup flow + `handlers/backup.rs` |
| 8 | Check-in validity display | **Pass** | `frontend/src/pages/tickets.rs`, ticket components |
| 9 | QR payload / display | **Pass** | Ticket display + engine; encoder limits in `test-coverage.md` |
| 10 | Permission table enforcement | **Partial** | Audit marks **Open / Partial**; role middleware **Pass** |
| 11 | test-coverage vs executables | **Pass** | `backend/tests/api/` / `api_tests_runner`, other runners, `test-coverage.md` |
| 12 | Reservation validation edges | **Partial** | Core paths **Pass**; full matrix optional |
| 13 | Docker path clarity | **Pass** | `docker-compose.yml`, `README.md` |

---

## 6. Security review summary (audit §6)

| Audit bullet | Status | Evidence |
|--------------|--------|----------|
| Authentication | **Partial** | Same as Report 1 |
| Route authorization | **Pass** | Tiered middleware |
| Object authorization | **Partial** | Ongoing hardening |
| Tenant isolation | **Pass** | Tests + handlers |
| Admin protection | **Pass** | Admin routes |

---

## 7. Tests and logging (audit §7)

| Audit bullet | Status | Evidence |
|--------------|--------|----------|
| Unit tests | **Pass** | `--lib` |
| API runners | **Pass** | `api_tests_runner` |
| Integration | **Pass** | `integration_tests` |
| HTTP | **Pass** | `api_tests_runner` |
| Observability | **Pass** | `tracing` + audit |

---

## 8. Manual verification (audit §8)

| Audit point | Status | Evidence |
|-------------|--------|----------|
| Browser / QR / backup on disk | **Partial** | Required by audit scope; not replaced by static proof |

---

## 9. Final notes (audit §9)

| Audit text | Fix-check status | Evidence |
|------------|------------------|----------|
| Cross-reference Report 1 fix-check | **Pass** | `.tmp/audit_report-1-fix_check.md` |
| Verdict consistency | **Pass** | **Partial Pass** only; no over-claim |

---

**How to use this file:** Walk `.tmp/audit_report-2.md` in order; each subsection above answers the same **§4.x / §5 #n / §6 / §7 / §8 / §9** reference.
