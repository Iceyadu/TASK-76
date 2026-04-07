use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::Json;
use crate::app::state::AppState;
use crate::errors::AppError;
use crate::handlers::auth::*;
use crate::models::*;

/// Resolve a ticket by ID or by ticket number (FR-XXXXXXXX)
fn resolve_ticket(db: &rusqlite::Connection, id_or_number: &str) -> Result<Ticket, AppError> {
    // Try by ID first
    if let Ok(Some(t)) = crate::repositories::tickets::find_by_id(db, id_or_number) {
        return Ok(t);
    }
    // Try by ticket number
    if let Ok(Some(t)) = crate::repositories::tickets::find_by_number(db, id_or_number) {
        return Ok(t);
    }
    Err(AppError::NotFound("Ticket not found".to_string()))
}

/// Get the store_id for a ticket via its reservation
fn get_ticket_store(db: &rusqlite::Connection, reservation_id: &str) -> Result<String, AppError> {
    db.query_row(
        "SELECT store_id FROM reservations WHERE id = ?1",
        [reservation_id],
        |row| row.get(0),
    )
    .map_err(|_| AppError::Internal("Reservation not found for ticket".to_string()))
}

pub async fn get_ticket(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<Ticket>, AppError> {
    let claims = extract_claims_required(&headers, &state.hmac_secret)?;
    let db = state.db.lock().map_err(|e| AppError::Internal(e.to_string()))?;

    let ticket = resolve_ticket(&db, &id)?;
    let ticket_store = get_ticket_store(&db, &ticket.reservation_id)?;
    let role = UserRole::from_str(&claims.role).ok_or_else(|| AppError::Forbidden("Invalid role".to_string()))?;

    match role {
        // Customer can only see own tickets
        UserRole::Customer => {
            let reservation: Result<String, _> = db.query_row(
                "SELECT user_id FROM reservations WHERE id = ?1", [&ticket.reservation_id], |row| row.get(0),
            );
            if let Ok(owner_id) = reservation {
                if owner_id != claims.user_id {
                    return Err(AppError::Forbidden("Access denied".to_string()));
                }
            } else {
                return Err(AppError::Forbidden("Access denied".to_string()));
            }
        }
        // Non-admin non-ops roles are store-scoped
        UserRole::Photographer | UserRole::MerchantStaff => enforce_store_isolation(&claims, &ticket_store)?,
        UserRole::PlatformOps | UserRole::Administrator => {}
    }

    Ok(Json(ticket))
}

pub async fn redeem_ticket(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let claims = extract_claims_required(&headers, &state.hmac_secret)?;
    require_role(&claims, &UserRole::MerchantStaff)?;
    require_csrf_with_state(&headers, &claims, &state)?;

    let db = state.db.lock().map_err(|e| AppError::Internal(e.to_string()))?;

    // Resolve ticket by ID or number
    let ticket = resolve_ticket(&db, &id)?;

    // Store isolation: staff can only redeem tickets from their store
    let ticket_store = get_ticket_store(&db, &ticket.reservation_id)?;
    enforce_store_isolation(&claims, &ticket_store)?;

    match crate::services::ticket_engine::redeem_ticket(&db, &ticket.id, &claims.user_id, &claims.username) {
        Ok(redeemed_at) => {
            Ok(Json(serde_json::json!({"message": "Ticket redeemed", "redeemed_at": redeemed_at})))
        }
        Err(msg) => Err(AppError::Validation(msg)),
    }
}

pub async fn undo_redemption(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(req): Json<UndoRedemptionRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let claims = extract_claims_required(&headers, &state.hmac_secret)?;
    require_role(&claims, &UserRole::MerchantStaff)?;
    require_csrf_with_state(&headers, &claims, &state)?;

    let db = state.db.lock().map_err(|e| AppError::Internal(e.to_string()))?;

    // Resolve ticket
    let ticket = resolve_ticket(&db, &id)?;

    // Store isolation
    let ticket_store = get_ticket_store(&db, &ticket.reservation_id)?;
    enforce_store_isolation(&claims, &ticket_store)?;

    match crate::services::ticket_engine::undo_redemption(&db, &ticket.id, &claims.user_id, &claims.username, &req.reason) {
        Ok(()) => Ok(Json(serde_json::json!({"message": "Redemption undone"}))),
        Err(msg) => Err(AppError::Validation(msg)),
    }
}
