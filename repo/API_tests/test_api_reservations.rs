//! API Reservation Tests
//!
//! POST /api/reservations
//!   - Happy path -> 201, returns {reservation, ticket}
//!   - Overlapping conflict -> 409, returns ConflictResponse with plain-language reasons
//!   - In-repair vehicle -> 409, conflict with in_repair_hold reason
//!   - Response includes 2 alternative time slots
//!   - Response includes alternate assets when applicable
//!   - Without CSRF token -> 403
//!   - Outside business hours -> 400
//!   - Invalid asset_type -> 400
//!
//! GET /api/reservations
//!   - Customer sees only own reservations
//!   - MerchantStaff sees store reservations
//!   - PlatformOps sees all reservations
//!
//! Conflict response format:
//!   {"conflict": true, "reasons": [{"code": "...", "message": "..."}], "alternative_slots": [...], "alternate_assets": [...]}
