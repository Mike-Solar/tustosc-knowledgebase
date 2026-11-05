//! 路由模块：新增业务模块时在 [`api_router`] 中挂载。

use axum::{Json, Router, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::state::AppState;

pub mod health;

/// `/api/v1` 下的业务路由。
///
/// 当前为空骨架，后续按模块挂载：
/// - `auth`：注册 / 登录 / 刷新令牌 / 登出
/// - `spaces`：空间与成员管理
/// - `documents`：文档 CRUD、回收站、历史版本
pub fn api_router() -> Router<AppState> {
    Router::new()
}

/// 未匹配路由的统一 404 响应（JSON，而不是纯文本）。
pub async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "error": {
                "code": "not_found",
                "message": "接口不存在"
            }
        })),
    )
}
