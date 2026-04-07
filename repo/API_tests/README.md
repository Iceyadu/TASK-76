# API Integration Tests

This directory contains API-level integration test specifications for the FleetReserve backend.

## Test Files

- `test_api_auth.rs` - Authentication API: login, logout, recovery codes
- `test_api_reservations.rs` - Reservation API: creation, conflicts, CSRF
- `test_api_tickets.rs` - Ticket API: redemption, undo, validation
- `test_api_vehicles.rs` - Vehicle API: CRUD, status transitions, store isolation
- `test_api_uploads.rs` - Upload API: validation, dedup
- `test_api_audit.rs` - Audit API: chain integrity verification

## Running

These tests are designed to run against a live or test backend instance.
The backend integration tests in `backend/tests/integration_tests.rs` cover
equivalent scenarios using the Axum test harness.

```bash
cd ../backend && cargo test --test integration_tests
```
