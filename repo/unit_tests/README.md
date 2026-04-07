# Unit Tests

This directory contains standalone unit test files that test individual modules of the FleetReserve backend.

## Test Files

- `test_auth.rs` - Authentication: password hashing, token creation/validation, CSRF
- `test_reservation_engine.rs` - Reservation engine: conflicts, retries, alternatives
- `test_ticket_engine.rs` - Ticket: generation, redemption, undo window, reason validation
- `test_uploads.rs` - Upload: magic bytes, MIME, size, fingerprint dedup
- `test_security.rs` - Security: masking, encryption, audit chain, recovery codes
- `test_authorization.rs` - Authorization: role checks, store isolation, photographer isolation

## Running

These tests are structured as Rust test modules that reference the backend crate.
They are also present as inline `#[cfg(test)]` modules within the backend source files.

Run all backend tests:
```bash
cd ../backend && cargo test
```

The inline tests in `backend/src/` cover the same assertions. These standalone files
serve as a reviewer-friendly reference to the test structure.
