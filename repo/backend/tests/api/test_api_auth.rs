//! HTTP API: `/api/auth/*`
use axum::http::{header, HeaderValue};
use serde_json::json;

use crate::http_helpers::{admin_token_and_csrf, api_server};
use crate::http_support::TEST_ADMIN_PASSWORD;

#[tokio::test]
async fn api_route_post_login_returns_token_and_csrf() {
    let s = api_server();
    let res = s
        .post("/api/auth/login")
        .json(&json!({
            "username": "admin",
            "password": TEST_ADMIN_PASSWORD,
        }))
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert!(body["token"].as_str().unwrap_or("").len() > 10);
    assert!(body["csrf_token"].as_str().unwrap_or("").len() > 4);
}

#[tokio::test]
async fn api_route_post_login_invalid_password_unauthorized() {
    let s = api_server();
    let res = s
        .post("/api/auth/login")
        .json(&json!({
            "username": "admin",
            "password": "wrong-password-xxxxxxxx",
        }))
        .await;
    res.assert_status(axum::http::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn api_route_get_me_returns_user_and_refreshed_token() {
    let s = api_server();
    let (token, _) = admin_token_and_csrf(&s).await;

    let res = s
        .get("/api/auth/me")
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        )
        .await;
    res.assert_status_ok();
    let body = res.json::<serde_json::Value>();
    assert_eq!(body["user"]["role"], "Administrator");
    assert!(body["refreshed_token"].as_str().unwrap_or("").len() > 10);
}

