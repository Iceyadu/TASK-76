use fleetreserve_backend::services::uploads::validate_upload;

#[test]
fn unit_uploads_invalid_rejected() {
    let err = validate_upload(b"not image bytes", "x.txt").unwrap_err();
    assert!(err.contains("Magic bytes") || err.contains("valid image") || err.contains("too small"));
}

#[test]
fn unit_uploads_oversized_rejected() {
    let mut data = vec![0xFF, 0xD8, 0xFF, 0xE0];
    data.extend(vec![0u8; 10 * 1024 * 1024 + 1]);
    let err = validate_upload(&data, "big.jpg").unwrap_err();
    assert!(err.contains("10 MB"));
}
