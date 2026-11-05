//! HTTP 层：路由、中间件与错误响应统一在这里装配。

use std::time::Duration;

use axum::{
    Router,
    http::{HeaderName, HeaderValue, Method, header},
    routing::get,
};
use tower_http::{
    cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer, ExposeHeaders},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::{routes, state::AppState};

/// 请求 ID 头，贯穿日志与响应，便于排查问题。
pub const REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-request-id");

/// 装配完整的 HTTP 应用。
pub fn build_router(state: AppState) -> Router {
    let cors = cors_layer(&state.config.cors_origins);
    let timeout = TimeoutLayer::with_status_code(
        axum::http::StatusCode::REQUEST_TIMEOUT,
        Duration::from_secs(state.config.request_timeout_secs),
    );

    Router::new()
        .route("/healthz", get(routes::health::live))
        .route("/readyz", get(routes::health::ready))
        .nest("/api/v1", routes::api_router())
        .fallback(routes::not_found)
        // 链路追踪：为每个请求生成 span 与访问日志。
        .layer(TraceLayer::new_for_http())
        // 请求 ID：没有传入时自动生成，并在响应头回传。
        .layer(PropagateRequestIdLayer::new(REQUEST_ID_HEADER.clone()))
        .layer(SetRequestIdLayer::new(
            REQUEST_ID_HEADER.clone(),
            MakeRequestUuid,
        ))
        .layer(timeout)
        .layer(cors)
        .with_state(state)
}

/// 跨域策略：未配置来源时允许任意来源（本地开发方便），配置后只放行白名单。
fn cors_layer(origins: &[String]) -> CorsLayer {
    let methods = vec![
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::PATCH,
        Method::DELETE,
        Method::OPTIONS,
    ];
    let headers = vec![
        header::ACCEPT,
        header::AUTHORIZATION,
        header::CONTENT_TYPE,
        header::ORIGIN,
        REQUEST_ID_HEADER.clone(),
    ];

    let layer = CorsLayer::new()
        .allow_methods(AllowMethods::list(methods))
        .allow_headers(AllowHeaders::list(headers))
        .expose_headers(ExposeHeaders::list([REQUEST_ID_HEADER.clone()]));

    if origins.is_empty() {
        layer.allow_origin(AllowOrigin::any())
    } else {
        let allowed: Vec<HeaderValue> = origins
            .iter()
            .filter_map(|origin| origin.parse::<HeaderValue>().ok())
            .collect();
        layer.allow_origin(AllowOrigin::list(allowed))
    }
}
