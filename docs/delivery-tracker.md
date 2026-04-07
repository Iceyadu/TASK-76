# FleetReserve Operations Suite - Delivery Tracker

## Current Stage
Phase 4 - Final Hardening, Docker, Test Entrypoints, and Package Cleanup

## Current Focus
- Complete all source files for backend and frontend
- Ensure Docker deployment coherence
- Finalize test suites and documentation alignment

## Hard-Fail Risks
1. **Reservation concurrency**: Must demonstrate transactional optimistic concurrency with retry (up to 3) and deterministic conflict reasons from committed snapshot
2. **Auth bypass**: No hardcoded auth, no skipped permission checks, no unprotected admin routes
3. **Audit chain integrity**: Append-only log with hash chain must be tamper-evident
4. **Ticket double-redemption**: Must statically prove redemption blocks re-entry
5. **Undo window enforcement**: 2-minute supervised undo must be enforced server-side, not client-only
6. **Store isolation**: Merchant/Staff must never see cross-store data
7. **Photographer isolation**: Must only see assignments tied to their jobs
8. **Encryption at rest**: VIN, license plate, user identifiers must be encrypted in SQLite
9. **Upload validation**: Must check magic bytes, not just Content-Type header
10. **Recovery code expiry**: 30-minute window must be enforced server-side

## Required Deliverables
- [x] Package structure per specification
- [x] prompt.md with original prompt
- [x] metadata.json with correct field values
- [x] docs/design.md - full architecture
- [x] docs/api-spec.md - all endpoints
- [x] docs/questions.md - business ambiguities
- [x] docs/test-coverage.md - risk-first plan
- [x] docs/reviewer-notes.md - static audit guide
- [x] repo/backend/ - Axum + SQLite backend
- [x] repo/frontend/ - Leptos WASM frontend
- [x] repo/docker-compose.yml
- [x] repo/run_tests.sh
- [x] repo/README.md
- [x] repo/unit_tests/
- [x] repo/API_tests/
- [x] sessions/ placeholder files

## Open Business Ambiguities
See docs/questions.md for full list. Key items:
- Photographer assignment creation flow (who creates assignments?)
- Undo supervision model (does supervisor authenticate separately?)
- Backup encryption key lifecycle (rotation, escrow?)
- Hash anchor periodicity (time-based or count-based?)
- Export format and scope
- Bay capacity definition (concurrent reservations or physical limit?)

## Packaging Exclusions
- target/ (Rust build cache)
- node_modules/
- .venv, __pycache__, .pytest_cache
- .vscode, .idea, .opencode, .codex
- *.db (runtime database files)
- uploads/ (runtime upload directory)
- backup files
- temp files
- .DS_Store
