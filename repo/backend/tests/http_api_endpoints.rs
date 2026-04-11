//! HTTP-level API tests using the real Axum router (`fleetreserve_backend::routes::build_router`).
mod http_support;

use axum::http::{header, HeaderName, HeaderValue};
use axum_test::TestServer;
use http_support::{test_app_state, test_router, TEST_ADMIN_PASSWORD};
use serde_json::json;

fn server() -> TestServer {
    TestServer::new(test_router()).unwrap()
}

#[tokio::test]
async fn http_post_login_success_returns_token_and_csrf() {
    let s = server();
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
async fn http_post_login_invalid_password_is_unauthorized() {
    let s = server();
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
async fn http_get_me_returns_user_and_refreshed_token() {
    let state = test_app_state();
    let app = fleetreserve_backend::routes::build_router(state.clone());
    let s = TestServer::new(app).unwrap();

    let login = s
        .post("/api/auth/login")
        .json(&json!({
            "username": "admin",
            "password": TEST_ADMIN_PASSWORD,
        }))
        .await;
    let token = login.json::<serde_json::Value>()["token"]
        .as_str()
        .unwrap()
        .to_string();

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

#[tokio::test]
async fn http_post_reservation_created_with_csrf() {
    let state = test_app_state();
    let app = fleetreserve_backend::routes::build_router(state);
    let s = TestServer::new(app).unwrap();

    let login = s
        .post("/api/auth/login")
        .json(&json!({
            "username": "admin",
            "password": TEST_ADMIN_PASSWORD,
        }))
        .await;
    let body = login.json::<serde_json::Value>();
    let token = body["token"].as_str().unwrap().to_string();
    let csrf = body["csrf_token"].as_str().unwrap().to_string();

    let res = s
        .post("/api/reservations")
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        )
        .add_header(
            HeaderName::from_static("x-csrf-token"),
            HeaderValue::from_str(&csrf).unwrap(),
        )
        .json(&json!({
            "asset_type": "vehicle",
            "asset_id": "v1",
            "store_id": "store-001",
            "start_time": "2026-06-01T10:00:00",
            "end_time": "2026-06-01T11:00:00",
        }))
        .await;
    res.assert_status(axum::http::StatusCode::CREATED);
    let out = res.json::<serde_json::Value>();
    assert_eq!(out["reservation"]["status"], "confirmed");
    assert!(out["ticket"]["ticket_number"].as_str().unwrap().starts_with("FR-"));
}

#[tokio::test]
async fn http_get_stores_requires_staff_session() {
    let state = test_app_state();
    let app = fleetreserve_backend::routes::build_router(state);
    let s = TestServer::new(app).unwrap();

    let login = s
        .post("/api/auth/login")
        .json(&json!({
            "username": "admin",
            "password": TEST_ADMIN_PASSWORD,
        }))
        .await;
    let token = login.json::<serde_json::Value>()["token"]
        .as_str()
        .unwrap()
        .to_string();

    let res = s
        .get("/api/stores")
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        )
        .await;
    res.assert_status_ok();
    let body = res.json::<serde_json::Value>();
    let stores = body["stores"].as_array().expect("stores array");
    assert!(!stores.is_empty(), "GET /api/stores should list active stores");
}
