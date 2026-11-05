//! 健康检查接口，供容器探针与网关使用。

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use serde_json::json;

use crate::state::AppState;

/// `GET /healthz` 的响应体。
#[derive(Debug, Serialize)]
pub struct LivenessResponse {
    /// 固定为 `ok`。
    pub status: &'static str,
    /// 服务名。
    pub service: &'static str,
    /// 版本号（来自 Cargo）。
    pub version: &'static str,
}

/// `GET /healthz`：存活探针，不访问数据库。
pub async fn live() -> impl IntoResponse {
    Json(LivenessResponse {
        status: "ok",
        service: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
    })
}

/// `GET /readyz`：就绪探针，探测数据库连通性。
pub async fn ready(State(state): State<AppState>) -> impl IntoResponse {
    match state.db().ping().await {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({ "status": "ready", "database": "up" })),
        ),
        Err(error) => {
            tracing::error!(%error, "database ping failed");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({ "status": "degraded", "database": "down" })),
            )
        }
    }
}
