//! Shared `TestServer` helpers for HTTP API route tests (`tests/api/`).
use axum_test::TestServer;
use fleetreserve_backend::routes::build_router;
use serde_json::json;

use crate::http_support::{test_app_state, TEST_ADMIN_PASSWORD};

pub fn api_server() -> TestServer {
    TestServer::new(build_router(test_app_state())).unwrap()
}

pub async fn admin_token_and_csrf(s: &TestServer) -> (String, String) {
    let res = s
        .post("/api/auth/login")
        .json(&json!({
            "username": "admin",
            "password": TEST_ADMIN_PASSWORD,
        }))
        .await;
    res.assert_status_ok();
    let body = res.json::<serde_json::Value>();
    (
        body["token"].as_str().unwrap().to_string(),
        body["csrf_token"].as_str().unwrap().to_string(),
    )
}
