//! Ticket engine unit tests
//! Run via: cd backend && cargo test
//!
//! Corresponding inline tests:
//!   backend/src/services/ticket_engine.rs::tests

// Test: ticket_number_format - FR-XXXXXXXX, 11 chars
// Test: generate_ticket - valid ticket created
// Test: redeem_ticket - marks redeemed
// Test: double_redemption_blocked - second redeem returns error
// Test: undo_within_window - succeeds when within 2 minutes
// Test: undo_without_reason_rejected - empty reason fails
// Test: undo_after_window_rejected - past 2 minutes fails
