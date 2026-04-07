//! API Ticket Tests
//!
//! GET /api/tickets/:id
//!   - Ticket owner -> 200
//!   - Other customer -> 403
//!   - MerchantStaff -> 200
//!
//! POST /api/tickets/:id/redeem
//!   - Valid ticket, first redeem -> 200, {redeemed_at}
//!   - Already redeemed -> 400, "already been redeemed"
//!   - MerchantStaff+ required -> 403 for Customer
//!   - CSRF required -> 403 without token
//!
//! POST /api/tickets/:id/undo
//!   - Within 2 min, with reason -> 200
//!   - Within 2 min, empty reason -> 400, "required"
//!   - Within 2 min, missing reason -> 400
//!   - After 2 min -> 400, "expired"
//!   - Not redeemed -> 400, "not been redeemed"
//!   - Already undone -> 400, "already been undone"
