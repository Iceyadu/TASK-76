//! API Vehicle Tests
//!
//! GET /api/vehicles
//!   - MerchantStaff sees own store vehicles (masked VIN/plate)
//!   - MerchantStaff from store A cannot see store B vehicles
//!   - PlatformOps sees all stores
//!   - Customer -> 403
//!
//! GET /api/vehicles/:id
//!   - Own store vehicle -> 200, masked data
//!   - Other store vehicle -> 403
//!
//! POST /api/vehicles
//!   - Valid data -> 201, VIN/plate encrypted at rest, response masked
//!   - CSRF required -> 403
//!   - Other store -> 403
//!
//! PUT /api/vehicles/:id/status
//!   - available -> reserved (MerchantStaff) -> 200
//!   - available -> decommissioned (MerchantStaff) -> 403 (requires Admin)
//!   - available -> decommissioned (Admin) -> 200
//!   - decommissioned -> available -> 400 (invalid transition)
//!   - Status change creates audit entry
//!   - Version conflict -> 409
