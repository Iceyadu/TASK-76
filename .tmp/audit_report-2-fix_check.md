
Fixed

Ticket read isolation: GET /api/tickets/:id now checks ticket store/owner instead of allowing any authenticated non-customer to read arbitrary tickets. See routes (line 148) and tickets handler (line 32).
Recovery-code expiry: validation now parses expires_at as RFC3339 in Rust instead of comparing incompatible SQLite strings. See recovery_codes.rs (line 17).
Calendar route tiering: /api/calendar moved under staff routes instead of generic authenticated routes. See routes (line 158).
CSRF on logout: logout now requires CSRF. See auth handler (line 58).
Idle-time refresh wiring: frontend now calls /auth/me and updates the token periodically. See app.rs (line 11).
Default secret fallback removed: env secrets are now mandatory and length-checked. See main.rs (line 23).
Upload flow improved: the upload form now actually posts multipart data and is mounted on the vehicles page. See upload_form.rs (line 35) and vehicles page (line 24).
Backup/restore UI improved: backup and restore paths are now user-editable inputs, not a single hard-coded backup path. See admin page (line 92).
Check-in validity display added: the check-in UI now renders a ticket validity window. See tickets page (line 119).
QR generation upgraded: ticket display now uses qrcodegen rather than the old fake hash-grid SVG. See ticket_display.rs (line 3).

Still Open

Permission management is only partially fixed: admin CRUD endpoints now exist for permission rows, but I still found no evidence that request authorization actually consults the permissions table at runtime. Auth still appears role-hierarchy-based. See admin permission handlers (line 169), permissions repo (line 1), and the lack of any has_permission-style enforcement from the code search.
Test/coverage documentation is still overstated: API_tests remains prose-only, integration_tests.rs is still mostly service-level, and docs/test-coverage.md (line 11) still claims many tests that do not exist as executable cases. See integration_tests.rs (line 5) and API_tests (line 1).
Reservation validation status handling still looks wrong: invalid format/business-hours failures are still converted into ConflictResponse values by the engine, so the earlier 400 vs 409 concern still appears open. See reservation_engine.rs (line 13).

Previously reported issues fixed: many.
Previously reported issues still open: permission enforcement, stale test-verification artifacts, and reservation validation status semantics.


