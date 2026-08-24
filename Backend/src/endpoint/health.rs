use crate::endpoint::api_error::ApiError;
use crate::state::AppState;
use actix_web::{HttpResponse, web};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub database: String,
    pub storage: String,
}

pub async fn get(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let db_ok = sqlx::query("SELECT 1").execute(&*state.pool).await.is_ok();

    let storage_ok = state.storage.check_health().await.is_ok();

    let database_status = if db_ok {
        "connected".to_string()
    } else {
        "disconnected".to_string()
    };
    let storage_status = if storage_ok {
        "connected".to_string()
    } else {
        "disconnected".to_string()
    };

    if db_ok && storage_ok {
        Ok(HttpResponse::Ok().json(HealthStatus {
            status: "ok".to_string(),
            database: database_status,
            storage: storage_status,
        }))
    } else {
        Ok(HttpResponse::ServiceUnavailable().json(HealthStatus {
            status: "degraded".to_string(),
            database: database_status,
            storage: storage_status,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_health_status_to_json() {
        let status = HealthStatus {
            status: "ok".to_string(),
            database: "connected".to_string(),
            storage: "connected".to_string(),
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"status\":\"ok\""));
    }
}
