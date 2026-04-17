use axum::http::{header, HeaderName, HeaderValue};
use axum_test::TestServer;
use fleetreserve_backend::{app::state::AppState, auth::password, routes::build_router};
use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tempfile::NamedTempFile;

#[path = "../src/api/types.rs"]
mod api_types;

const ADMIN_PASSWORD: &str = "FleetReserveFrontendE2E#2026";

fn app_state() -> AppState {
    let tmp = NamedTempFile::new().expect("temp db");
    let conn = Connection::open(tmp.path()).expect("open db");
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
        .expect("pragma");
    conn.execute_batch(include_str!("../../backend/migrations/001_initial_schema.sql"))
        .expect("schema");
    conn.execute_batch(include_str!("../../backend/migrations/002_seed_data.sql"))
        .expect("seed");
    conn.execute_batch(
        "UPDATE stores SET business_hours_start='00:00', business_hours_end='23:59' WHERE id IN ('store-001','store-002');",
    )
    .expect("hours");
    let admin_hash = password::hash_password(ADMIN_PASSWORD).expect("hash admin");
    conn.execute(
        "UPDATE users SET active = 1, password_hash = ?1 WHERE id = 'user-admin-001'",
        [&admin_hash],
    )
    .expect("admin");
    conn.execute(
        "INSERT INTO vehicles (id, vin_encrypted, vin_hash, license_plate_encrypted, license_plate_hash, make, model, store_id, status, insurance_expiry, version) VALUES ('v1', 'enc', 'h', 'enc', 'h', 'T', 'V', 'store-001', 'available', '2100-01-01T00:00:00', 1)",
        [],
    )
    .expect("vehicle");

    let upload_dir = tempfile::tempdir().expect("upload dir");
    let upload_path = upload_dir.path().to_string_lossy().into_owned();
    std::mem::forget(upload_dir);
    AppState {
        db: Arc::new(Mutex::new(conn)),
        encryption_key: "x".repeat(32),
        hmac_secret: "y".repeat(32),
        upload_dir: upload_path,
        csrf_tokens: Arc::new(Mutex::new(HashMap::new())),
    }
}

fn server() -> TestServer {
    TestServer::new(build_router(app_state())).expect("test server")
}

#[tokio::test]
async fn frontend_backend_e2e_login_list_vehicles_and_create_reservation() {
    let s = server();

    // FE login flow: deserialize backend payload to frontend LoginResponse shape.
    let login = s
        .post("/api/auth/login")
        .json(&serde_json::json!({
            "username": "admin",
            "password": ADMIN_PASSWORD,
        }))
        .await;
    login.assert_status_ok();
    let login_body = login.json::<api_types::LoginResponse>();
    assert_eq!(login_body.user.role, "Administrator");
    assert!(!login_body.token.is_empty());
    assert!(!login_body.csrf_token.is_empty());

    // FE vehicles page flow: call backend and parse frontend vehicle DTOs.
    let vehicles = s
        .get("/api/vehicles")
        .add_query_param("store_id", "store-001")
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", login_body.token)).unwrap(),
        )
        .await;
    vehicles.assert_status_ok();
    let vehicles_body = vehicles.json::<serde_json::Value>();
    let parsed: Vec<api_types::MaskedVehicle> =
        serde_json::from_value(vehicles_body["vehicles"].clone()).expect("frontend vehicle DTOs");
    assert!(!parsed.is_empty());
    assert_eq!(parsed[0].store_id, "store-001");

    // FE reservation create flow with CSRF.
    let reserve = s
        .post("/api/reservations")
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", login_body.token)).unwrap(),
        )
        .add_header(
            HeaderName::from_static("x-csrf-token"),
            HeaderValue::from_str(&login_body.csrf_token).unwrap(),
        )
        .json(&serde_json::json!({
            "asset_type": "vehicle",
            "asset_id": "v1",
            "store_id": "store-001",
            "start_time": "2026-10-01T10:00:00",
            "end_time": "2026-10-01T11:00:00",
        }))
        .await;
    reserve.assert_status(axum::http::StatusCode::CREATED);
    let reserve_body = reserve.json::<serde_json::Value>();
    let reservation: api_types::Reservation =
        serde_json::from_value(reserve_body["reservation"].clone()).expect("frontend reservation DTO");
    assert_eq!(reservation.asset_type, "vehicle");
}
