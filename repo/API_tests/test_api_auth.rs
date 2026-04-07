//! API Authentication Tests
//!
//! POST /api/auth/login
//!   - Valid credentials -> 200, returns {token, csrf_token, user}
//!   - Invalid password -> 401
//!   - Unknown user -> 401
//!   - Missing fields -> 400
//!
//! GET /api/auth/me
//!   - With valid token -> 200, returns masked user
//!   - Without token -> 401
//!   - With expired token -> 401
//!
//! POST /api/admin/recovery-codes
//!   - As admin -> 200, returns {code, expires_at}
//!   - As non-admin -> 403
//!   - For nonexistent user -> 404
//!
//! POST /api/auth/reset-password
//!   - Valid code within 30 min -> 200
//!   - Expired code -> 400
//!   - Wrong code -> 400
//!   - Already-used code -> 400
//!
//! POST /api/auth/logout
//!   - Valid token -> 200, audit entry created
