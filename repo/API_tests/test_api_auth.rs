use fleetreserve_backend::auth::{csrf, password, session};

#[test]
fn api_auth_password_hash_and_verify() {
    let hash = password::hash_password("fleetreserve-pass").unwrap();
    assert!(password::verify_password("fleetreserve-pass", &hash));
    assert!(!password::verify_password("wrong-pass", &hash));
}

#[test]
fn api_auth_token_roundtrip_and_csrf() {
    let token = session::create_token(
        "user-1",
        "alice",
        "MerchantStaff",
        Some("store-001"),
        "test-hmac-secret-32-bytes-minimum!!",
    );
    let claims = session::validate_token(&token, "test-hmac-secret-32-bytes-minimum!!").unwrap();
    assert_eq!(claims.user_id, "user-1");
    assert_eq!(claims.role, "MerchantStaff");

    let csrf_token = csrf::generate_csrf_token();
    assert!(csrf::validate_csrf_token(&csrf_token, &csrf_token));
    assert!(!csrf::validate_csrf_token(&csrf_token, "mismatch"));
}
