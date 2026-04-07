//! Reservation engine unit tests
//! Run via: cd backend && cargo test
//!
//! These tests verify:
//! - Happy path: reservation created with ticket
//! - Overlapping reservation detected
//! - Vehicle in-repair conflict
//! - Expired insurance conflict
//! - Bay capacity exceeded
//! - Alternative slot computation returns nearest 2
//! - Alternate vehicle suggestion
//! - Optimistic concurrency retry
//! - Deterministic conflict reasons
//!
//! Corresponding inline tests:
//!   backend/src/services/reservation_engine.rs::tests

// Test: reservation_happy_path
// Fixture: available vehicle v1, store-001
// Assertion: result.is_ok(), reservation.status == "confirmed", ticket starts with "FR-"
// See: backend/src/services/reservation_engine.rs:test_reservation_happy_path

// Test: overlapping_reservation_conflict
// Fixture: existing reservation on v1 9:00-10:00, attempt 9:30-10:30
// Assertion: result.is_err(), conflict.reasons contains "overlapping_reservation"
// See: backend/src/services/reservation_engine.rs:test_overlapping_reservation_conflict

// Test: in_repair_conflict
// Fixture: vehicle v-repair with status "in-repair"
// Assertion: conflict.reasons contains "in_repair_hold"
// See: backend/src/services/reservation_engine.rs:test_in_repair_conflict

// Test: alternative_slots_returned
// Fixture: conflict on v1
// Assertion: conflict.alternative_slots.len() <= 2, slots within business hours
// See: backend/src/services/reservation_engine.rs:test_alternative_slots_returned

// Test: alternate_vehicle_suggested
// Fixture: vehicle v-repair unavailable, v1 and v2 available
// Assertion: conflict.alternate_assets is not empty
// See: backend/src/services/reservation_engine.rs:test_alternate_vehicle_suggested
