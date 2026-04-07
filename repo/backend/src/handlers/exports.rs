use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::Json;
use crate::app::state::AppState;
use crate::errors::AppError;
use crate::handlers::auth::*;
use crate::models::*;

pub async fn export_data(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<ExportQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let claims = extract_claims_required(&headers, &state.hmac_secret)?;
    require_role(&claims, &UserRole::PlatformOps)?;

    let db = state.db.lock().map_err(|e| AppError::Internal(e.to_string()))?;

    let reservations = if let Some(ref store_id) = params.store_id {
        crate::repositories::reservations::find_by_store(&db, store_id)
    } else {
        crate::repositories::reservations::find_all(&db)
    }.map_err(|e| AppError::Internal(e.to_string()))?;

    let vehicles = if let Some(ref store_id) = params.store_id {
        crate::repositories::vehicles::find_by_store(&db, store_id)
    } else {
        crate::repositories::vehicles::find_all(&db)
    }.map_err(|e| AppError::Internal(e.to_string()))?;

    // Mask vehicle sensitive data in exports
    let masked_vehicles: Vec<serde_json::Value> = vehicles.iter().map(|v| {
        serde_json::json!({
            "id": v.id, "make": v.make, "model": v.model,
            "status": v.status, "store_id": v.store_id,
        })
    }).collect();

    let _ = crate::audit::chain::append_audit_log(
        &db, &claims.user_id, &claims.username, "EXPORT", "export", "",
        &serde_json::json!({"store_id": params.store_id, "type": params.export_type}),
    );

    Ok(Json(serde_json::json!({
        "export_type": params.export_type.unwrap_or("all".into()),
        "reservations": reservations,
        "vehicles": masked_vehicles,
        "exported_at": chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    })))
}
