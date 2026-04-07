//! API Upload Tests
//!
//! POST /api/uploads
//!   - Valid JPEG (magic bytes FF D8 FF) -> 201
//!   - Valid PNG (magic bytes 89 50 4E 47) -> 201
//!   - Invalid file type (GIF, PDF, etc.) -> 400, "Magic bytes do not match"
//!   - File > 10MB -> 400, "exceeds maximum size"
//!   - Duplicate fingerprint -> 409, "Duplicate file"
//!   - CSRF required -> 403
//!   - MerchantStaff+ required -> 403 for Customer
