//! API Audit Tests
//!
//! GET /api/audit
//!   - PlatformOps+ required -> 403 for Customer/MerchantStaff
//!   - Returns recent audit entries
//!   - Filters by resource_type and resource_id
//!
//! Audit chain verification:
//!   - Each entry's previous_hash matches the prior entry's current_hash
//!   - Recomputed hash matches stored current_hash
//!   - Tampered entry breaks chain validation
//!
//! Actions that create audit entries:
//!   - LOGIN, LOGOUT
//!   - CREATE/UPDATE/DELETE on all resources
//!   - STATUS_CHANGE on vehicles
//!   - REDEEM, UNDO on tickets
//!   - EXPORT
//!   - BACKUP, RESTORE
//!   - PERMISSION_CHANGE
