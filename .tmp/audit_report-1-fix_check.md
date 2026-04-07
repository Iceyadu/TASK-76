Recheck Results

Fixed: static verifiability / crate shape.
Evidence: repo/backend/Cargo.toml (line 6) now declares [lib], repo/backend/src/lib.rs exists, repo/frontend/Cargo.toml (line 26) adds serde_wasm_bindgen, and repo/backend/Cargo.toml (line 19) enables rusqlite backup.

Fixed: backup / restore implementation.
Evidence: repo/backend/src/handlers/backup.rs (line 33) now writes an encrypted backup file and records size_bytes and sha256; repo/backend/src/handlers/backup.rs (line 109) decrypts, validates SQLite magic bytes, checks required tables, and restores via SQLite backup API.
Manual verification still required for a real filesystem backup/restore cycle.

Fixed: CSRF weakness / missing upload CSRF.
Evidence: repo/backend/src/handlers/auth.rs (line 34) stores CSRF tokens server-side, repo/backend/src/handlers/auth.rs (line 163) validates them against server state, and repo/backend/src/handlers/uploads.rs (line 16) now requires require_csrf_with_state.

Fixed: object-level / store authorization for the previously cited write flows.
Evidence: repo/backend/src/handlers/reservations.rs (line 37) verifies asset store and enforces store isolation, repo/backend/src/handlers/bays.rs (line 19) enforces store checks, repo/backend/src/handlers/tickets.rs (line 68) resolves the ticket then enforces ticket-store isolation, and repo/backend/src/handlers/uploads.rs (line 42) overrides caller-supplied store for non-elevated roles.

Partially Fixed: QR placeholder / ticket-number mismatch / datetime mismatch.
Evidence: repo/frontend/src/components/ticket_display.rs (line 91) now renders an SVG image instead of a placeholder, repo/backend/src/handlers/tickets.rs (line 9) resolves by ID or ticket number, and repo/backend/src/handlers/reservations.rs (line 9) plus repo/backend/src/services/reservation_engine.rs (line 19) now accept datetime-local style timestamps.
Remaining issue: the frontend itself says this is a deterministic “QR-like” SVG, not a standards-compliant QR encoder, so real scanner interoperability still cannot be confirmed statically. Evidence: repo/frontend/src/components/ticket_display.rs (line 3).

Fixed: calendar day/week/status support.
Evidence: repo/frontend/src/pages/calendar.rs (line 19) sends asset_status, repo/backend/src/handlers/calendar.rs (line 30) parses it, repo/backend/src/handlers/calendar.rs (line 39) expands week ranges, and repo/backend/src/handlers/calendar.rs (line 65) filters assets by status.

Fixed: audit trail append-only enforcement.
Evidence: repo/backend/migrations/001_initial_schema.sql (line 162) adds BEFORE UPDATE / BEFORE DELETE triggers, and repo/backend/src/audit/chain.rs (line 125) now tests those protections.

Partially Fixed: session handling / 12-hour idle timeout.
Evidence: repo/backend/src/auth/session.rs (line 8) now models idle timeout semantics, and repo/backend/src/handlers/auth.rs (line 86) reissues a refreshed token in /api/auth/me.
Remaining issue: I still do not see frontend code that calls /api/auth/me or stores refreshed_token, so the refresh path is not wired end-to-end in the client. Evidence: repo/frontend/src/api/client.rs (line 7) and the absence of any frontend consumer for refreshed_token.

Fixed: ticket validity window enforcement.
Evidence: repo/backend/src/services/ticket_engine.rs (line 76) now rejects redemption before valid_from and after valid_until.

Still Present: tests / docs still overstate executable coverage.
Evidence: repo/API_tests/*.rs remain comment-only specifications such as repo/API_tests/test_api_auth.rs, while docs/test-coverage.md (line 11) still maps many named API tests as if they were real automated coverage. docs/reviewer-notes.md (line 57) is improved and now matches the code much better, but the coverage doc is still overstated.