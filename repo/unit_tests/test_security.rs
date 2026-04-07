//! Security unit tests
//! Run via: cd backend && cargo test
//!
//! Corresponding inline tests:
//!   backend/src/security/masking.rs::tests
//!   backend/src/security/encryption.rs::tests
//!   backend/src/audit/chain.rs::tests

// Test: mask_vin - "*************3456" for 17-char VIN
// Test: mask_license_plate - "*****34" for 7-char plate
// Test: mask_username - "j***" for "johndoe"
// Test: encrypt_decrypt_roundtrip - Decrypt(Encrypt(x)) == x
// Test: ciphertext_not_plaintext - encrypted != original
// Test: wrong_key_fails - decrypt with different key errors
// Test: audit_chain_integrity - valid chain passes verification
// Test: audit_chain_detects_tampering - modified record fails verification
// Test: hash_chain_links_previous - entry N's previous_hash == entry N-1's current_hash
