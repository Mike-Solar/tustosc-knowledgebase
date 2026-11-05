//! 统一错误模型：业务错误 → HTTP 状态码 + 稳定的错误码。
//!
//! 响应体固定为：
//! ```json
//! { "error": { "code": "not_found", "message": "文档不存在" } }
//! ```
//! 前端与 SDK 依赖 `code` 做判断，`message` 仅用于展示与日志。

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

/// 应用层统一错误类型。
#[derive(Debug, Error)]
pub enum AppError {
    /// 请求参数非法（400）。
    #[error("{0}")]
    BadRequest(String),
    /// 未认证或凭证失效（401）。
    #[error("{0}")]
    Unauthorized(String),
    /// 已认证但无权限（403）。
    #[error("{0}")]
    Forbidden(String),
    /// 资源不存在（404）。
    #[error("{0}")]
    NotFound(String),
    /// 资源冲突，例如标题重名、乐观锁版本不一致（409）。
    #[error("{0}")]
    Conflict(String),
    /// 服务内部错误（500），细节只写日志，不返回给客户端。
    #[error("服务器内部错误")]
    Internal(#[from] anyhow::Error),
}

impl AppError {
    /// 稳定的机器可读错误码。
    pub fn code(&self) -> &'static str {
        match self {
            Self::BadRequest(_) => "bad_request",
            Self::Unauthorized(_) => "unauthorized",
            Self::Forbidden(_) => "forbidden",
            Self::NotFound(_) => "not_found",
            Self::Conflict(_) => "conflict",
            Self::Internal(_) => "internal_error",
        }
    }

    /// 对应的 HTTP 状态码。
    pub fn status(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// 错误响应体。
#[derive(Debug, Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: ErrorBody,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();
        let code = self.code();
        let message = self.to_string();

        if status.is_server_error() {
            tracing::error!(error = ?self, status = status.as_u16(), "request failed");
        } else {
            tracing::debug!(status = status.as_u16(), code, "request rejected");
        }

        let body = Json(ErrorResponse {
            error: ErrorBody { code, message },
        });

        (status, body).into_response()
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(error: sea_orm::DbErr) -> Self {
        Self::Internal(anyhow::Error::new(error))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        Self::Internal(anyhow::Error::new(error))
    }
}

/// 业务处理函数的返回类型别名。
pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_errors_keep_status_and_code() {
        let error = AppError::NotFound("文档不存在".to_owned());
        assert_eq!(error.status(), StatusCode::NOT_FOUND);
        assert_eq!(error.code(), "not_found");
    }

    #[test]
    fn internal_errors_do_not_leak_details() {
        let error = AppError::Internal(anyhow::anyhow!("连接池耗尽，密码是 hunter2"));
        assert_eq!(error.to_string(), "服务器内部错误");
        assert_eq!(error.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
