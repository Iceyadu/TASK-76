//! Authentication unit tests
//! Run via: cd backend && cargo test
//!
//! These tests verify:
//! - Password hash and verify round-trip (Argon2id)
//! - Wrong password fails verification
//! - Session token creation contains correct claims
//! - Expired token is rejected
//! - Token with wrong signature is rejected
//! - CSRF token generation produces unique values
//! - CSRF validation rejects mismatched tokens
//!
//! Corresponding inline tests:
//!   backend/src/auth/password.rs::tests
//!   backend/src/auth/session.rs::tests
//!   backend/src/auth/csrf.rs::tests

// Test: hash_password produces argon2id hash
// Assertion: hash starts with "$argon2id$"
// See: backend/src/auth/password.rs:test_hash_contains_argon2id

// Test: verify_password succeeds for correct password
// Assertion: verify_password("pass", hash_password("pass")) == true
// See: backend/src/auth/password.rs:test_hash_and_verify

// Test: verify_password fails for wrong password
// Assertion: verify_password("wrong", hash_password("pass")) == false
// See: backend/src/auth/password.rs:test_hash_and_verify

// Test: create_token and validate_token round-trip
// Assertion: claims match input (user_id, role, store_id)
// See: backend/src/auth/session.rs:test_create_and_validate_token

// Test: expired token rejected
// Fixture: token with exp=1 (epoch second 1)
// Assertion: validate_token returns None
// See: backend/src/auth/session.rs:test_expired_token_rejected

// Test: wrong signature rejected
// Assertion: validate_token with different secret returns None
// See: backend/src/auth/session.rs:test_invalid_signature_rejected

// Test: CSRF tokens are unique
// Assertion: generate_csrf_token() != generate_csrf_token()
// See: backend/src/auth/csrf.rs:test_csrf_token_uniqueness

// Test: CSRF validation
// Assertion: validate_csrf_token(t, t) == true, validate_csrf_token(t, "wrong") == false
// See: backend/src/auth/csrf.rs:test_csrf_validation
