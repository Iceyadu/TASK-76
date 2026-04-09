use fleetreserve_backend::models::CreateReservationRequest;
use fleetreserve_backend::services::reservation_engine::create_reservation;
use rusqlite::Connection;

fn setup_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
    conn.execute_batch(include_str!("../backend/migrations/001_initial_schema.sql")).unwrap();
    conn.execute_batch(include_str!("../backend/migrations/002_seed_data.sql")).unwrap();
    conn.execute("UPDATE users SET active = 1 WHERE id = 'user-admin-001'", []).unwrap();
    conn.execute(
        "INSERT INTO vehicles (id, vin_encrypted, vin_hash, license_plate_encrypted, license_plate_hash, make, model, store_id, status, insurance_expiry, version) VALUES ('v1', 'enc', 'h', 'enc', 'h', 'T', 'V', 'store-001', 'available', '2100-01-01T00:00:00', 1)",
        [],
    ).unwrap();
    conn
}

#[test]
fn api_reservation_happy_path_returns_ticket() {
    let conn = setup_db();
    let req = CreateReservationRequest {
        asset_type: "vehicle".into(),
        asset_id: "v1".into(),
        store_id: "store-001".into(),
        start_time: "2026-05-01T09:00:00".into(),
        end_time: "2026-05-01T10:00:00".into(),
    };
    let result = create_reservation(&conn, "user-admin-001", "admin", &req).unwrap();
    assert_eq!(result.reservation.status, "confirmed");
    assert!(result.ticket.ticket_number.starts_with("FR-"));
}
