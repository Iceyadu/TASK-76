use crate::models::Reservation;
use rusqlite::Connection;

pub fn find_by_user(conn: &Connection, user_id: &str) -> Result<Vec<Reservation>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, asset_type, asset_id, store_id, user_id, start_time, end_time, status, ticket_id, version FROM reservations WHERE user_id = ?1 ORDER BY start_time"
    )?;
    let rows = stmt.query_map([user_id], |row| row_to_reservation(row))?;
    let items: Vec<Reservation> = rows.filter_map(|r| r.ok()).collect();
    Ok(items)
}

pub fn find_by_store(conn: &Connection, store_id: &str) -> Result<Vec<Reservation>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, asset_type, asset_id, store_id, user_id, start_time, end_time, status, ticket_id, version FROM reservations WHERE store_id = ?1 ORDER BY start_time"
    )?;
    let rows = stmt.query_map([store_id], |row| row_to_reservation(row))?;
    let items: Vec<Reservation> = rows.filter_map(|r| r.ok()).collect();
    Ok(items)
}

pub fn find_all(conn: &Connection) -> Result<Vec<Reservation>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, asset_type, asset_id, store_id, user_id, start_time, end_time, status, ticket_id, version FROM reservations ORDER BY start_time"
    )?;
    let rows = stmt.query_map([], |row| row_to_reservation(row))?;
    let items: Vec<Reservation> = rows.filter_map(|r| r.ok()).collect();
    Ok(items)
}

fn row_to_reservation(row: &rusqlite::Row) -> Result<Reservation, rusqlite::Error> {
    Ok(Reservation {
        id: row.get(0)?, asset_type: row.get(1)?, asset_id: row.get(2)?,
        store_id: row.get(3)?, user_id: row.get(4)?, start_time: row.get(5)?,
        end_time: row.get(6)?, status: row.get(7)?, ticket_id: row.get(8)?, version: row.get(9)?,
    })
}
