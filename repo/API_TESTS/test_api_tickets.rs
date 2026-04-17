//! HTTP API: `/api/tickets/*`
use axum::http::{header, HeaderName, HeaderValue};
use serde_json::json;

use crate::http_helpers::{admin_token_and_csrf, api_server};

#[tokio::test]
async fn api_route_get_ticket_redeem_undo_roundtrip() {
    let s = api_server();
    let (token, csrf) = admin_token_and_csrf(&s).await;

    let create = s
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
            "start_time": "2020-01-01T12:00:00",
            "end_time": "2099-12-31T13:00:00",
        }))
        .await;
    create.assert_status(axum::http::StatusCode::CREATED);
    let created = create.json::<serde_json::Value>();
    let ticket_id = created["ticket"]["id"].as_str().unwrap().to_string();

    let (tok2, _) = admin_token_and_csrf(&s).await;
    let get = s
        .get(&format!("/api/tickets/{}", ticket_id))
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", tok2)).unwrap(),
        )
        .await;
    get.assert_status_ok();
    let ticket = get.json::<serde_json::Value>();
    assert_eq!(ticket["id"], ticket_id);
    assert!(!ticket["redeemed"].as_bool().unwrap());

    let (tok3, csrf3) = admin_token_and_csrf(&s).await;
    let redeem = s
        .post(&format!("/api/tickets/{}/redeem", ticket_id))
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", tok3)).unwrap(),
        )
        .add_header(
            HeaderName::from_static("x-csrf-token"),
            HeaderValue::from_str(&csrf3).unwrap(),
        )
        .await;
    redeem.assert_status_ok();

    let (tok4, csrf4) = admin_token_and_csrf(&s).await;
    let undo = s
        .post(&format!("/api/tickets/{}/undo", ticket_id))
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", tok4)).unwrap(),
        )
        .add_header(
            HeaderName::from_static("x-csrf-token"),
            HeaderValue::from_str(&csrf4).unwrap(),
        )
        .json(&json!({ "reason": "operator correction within window" }))
        .await;
    undo.assert_status_ok();
}
