//! HTTP API: cross-role authorization matrix for protected routes.
use axum::http::{header, HeaderValue};

use crate::http_helpers::{
    api_server, customer_token_and_csrf, merchant_token_and_csrf, ops_token_and_csrf,
    photographer_token_and_csrf,
};

#[tokio::test]
async fn api_authz_customer_forbidden_on_staff_ops_admin_routes() {
    let s = api_server();
    let (token, _) = customer_token_and_csrf(&s).await;
    let auth = HeaderValue::from_str(&format!("Bearer {}", token)).unwrap();

    s.get("/api/vehicles")
        .add_header(header::AUTHORIZATION, auth.clone())
        .await
        .assert_status(axum::http::StatusCode::FORBIDDEN);

    s.get("/api/exports")
        .add_header(header::AUTHORIZATION, auth.clone())
        .await
        .assert_status(axum::http::StatusCode::FORBIDDEN);

    s.get("/api/admin/users")
        .add_header(header::AUTHORIZATION, auth)
        .await
        .assert_status(axum::http::StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn api_authz_photographer_allowed_auth_routes_forbidden_staff_routes() {
    let s = api_server();
    let (token, _) = photographer_token_and_csrf(&s).await;
    let auth = HeaderValue::from_str(&format!("Bearer {}", token)).unwrap();

    let assignments = s.get("/api/assignments")
        .add_header(header::AUTHORIZATION, auth.clone())
        .await;
    assignments.assert_status_ok();
    let body = assignments.json::<serde_json::Value>();
    assert!(body["assignments"].is_array());

    s.get("/api/vehicles")
        .add_header(header::AUTHORIZATION, auth)
        .await
        .assert_status(axum::http::StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn api_authz_merchant_allowed_staff_forbidden_ops_admin() {
    let s = api_server();
    let (token, _) = merchant_token_and_csrf(&s).await;
    let auth = HeaderValue::from_str(&format!("Bearer {}", token)).unwrap();

    let vehicles = s.get("/api/vehicles")
        .add_header(header::AUTHORIZATION, auth.clone())
        .await;
    vehicles.assert_status_ok();
    let body = vehicles.json::<serde_json::Value>();
    assert!(body["vehicles"].is_array());
    assert!(body["total"].as_u64().unwrap_or(0) >= 1);

    s.get("/api/exports")
        .add_header(header::AUTHORIZATION, auth.clone())
        .await
        .assert_status(axum::http::StatusCode::FORBIDDEN);

    s.get("/api/admin/users")
        .add_header(header::AUTHORIZATION, auth)
        .await
        .assert_status(axum::http::StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn api_authz_ops_allowed_ops_forbidden_admin() {
    let s = api_server();
    let (token, _) = ops_token_and_csrf(&s).await;
    let auth = HeaderValue::from_str(&format!("Bearer {}", token)).unwrap();

    let exports = s.get("/api/exports")
        .add_header(header::AUTHORIZATION, auth.clone())
        .await;
    exports.assert_status_ok();
    let exports_body = exports.json::<serde_json::Value>();
    assert!(exports_body["vehicles"].is_array());
    assert!(exports_body["reservations"].is_array());

    let audit = s.get("/api/audit")
        .add_query_param("limit", "5")
        .add_header(header::AUTHORIZATION, auth.clone())
        .await;
    audit.assert_status_ok();
    let audit_body = audit.json::<serde_json::Value>();
    assert!(audit_body["entries"].is_array());

    s.get("/api/admin/users")
        .add_header(header::AUTHORIZATION, auth)
        .await
        .assert_status(axum::http::StatusCode::FORBIDDEN);
}
