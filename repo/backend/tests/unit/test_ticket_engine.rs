use fleetreserve_backend::services::ticket_engine::{generate_ticket, redeem_ticket};
use rusqlite::Connection;

fn setup_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
    conn.execute_batch(include_str!("../../migrations/001_initial_schema.sql")).unwrap();
    conn.execute("INSERT INTO stores (id, name, location) VALUES ('s1','S','L')", []).unwrap();
    conn.execute("INSERT INTO users (id, username, password_hash, display_name, role) VALUES ('u1','u1','x','U1','Customer')", []).unwrap();
    conn.execute("INSERT INTO users (id, username, password_hash, display_name, role) VALUES ('staff-1','staff1','x','S1','MerchantStaff')", []).unwrap();
    conn.execute(
        "INSERT INTO reservations (id, asset_type, asset_id, store_id, user_id, start_time, end_time, status) VALUES ('r1','vehicle','v1','s1','u1','2026-05-01T09:00:00','2026-05-01T10:00:00','confirmed')",
        [],
    ).unwrap();
    conn
}

#[test]
fn unit_ticket_engine_redeem_works() {
    let conn = setup_db();
    let t = generate_ticket(&conn, "r1", "2000-01-01T00:00:00", "2099-01-01T00:00:00").unwrap();
    assert!(redeem_ticket(&conn, &t.id, "staff-1", "staff1").is_ok());
}
