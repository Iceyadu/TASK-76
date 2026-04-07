//! Authorization unit tests
//! Run via: cd backend && cargo test
//!
//! These tests verify role hierarchy and access control.

// Test: customer_level < photographer_level < merchant_level < ops_level < admin_level
// Test: has_at_least(Customer, Customer) == true
// Test: has_at_least(Customer, MerchantStaff) == false
// Test: has_at_least(Administrator, Customer) == true
// Test: vehicle_status_transition_requires_admin for decommission
// Test: valid transitions: available->reserved, reserved->on-rent, etc.
// Test: invalid transitions: decommissioned->available
// Note: Full API-level store isolation and photographer isolation tests are in API_tests/
