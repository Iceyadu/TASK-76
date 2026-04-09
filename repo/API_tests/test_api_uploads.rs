use fleetreserve_backend::services::uploads::validate_upload;

#[test]
fn api_upload_rejects_non_image() {
    let bad = b"not an image payload";
    let err = validate_upload(bad, "x.txt").unwrap_err();
    assert!(err.contains("Magic bytes") || err.contains("valid image") || err.contains("too small"));
}

#[test]
fn api_upload_rejects_oversized_blob() {
    let mut big = vec![0xFF, 0xD8, 0xFF, 0xE0];
    big.extend(vec![0u8; 10 * 1024 * 1024 + 1]);
    let err = validate_upload(&big, "big.jpg").unwrap_err();
    assert!(err.contains("10 MB"));
}
