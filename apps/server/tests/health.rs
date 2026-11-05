//! 冒烟测试：验证路由装配、健康检查与迁移在 SQLite 内存库上可用。

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use knowledgebase_server::{app::build_router, config::AppConfig, db, state::AppState};
use tower::ServiceExt;

const TEST_DATABASE_URL: &str = "sqlite::memory:";

/// 构造一个完成迁移的内存 SQLite 应用状态。
async fn test_state() -> AppState {
    let db = db::connect(TEST_DATABASE_URL)
        .await
        .expect("SQLite 内存库连接失败");
    db::migrate(&db).await.expect("SQLite 迁移失败");
    AppState::new(db, AppConfig::for_testing(TEST_DATABASE_URL))
}

async fn body_json(response: axum::response::Response) -> serde_json::Value {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("读取响应体失败")
        .to_bytes();
    serde_json::from_slice(&bytes).expect("响应体不是合法 JSON")
}

#[tokio::test]
async fn healthz_returns_ok() {
    let app = build_router(test_state().await);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let json = body_json(response).await;
    assert_eq!(json["status"], "ok");
    assert_eq!(json["service"], "knowledgebase-server");
}

#[tokio::test]
async fn readyz_reports_database_up() {
    let app = build_router(test_state().await);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let json = body_json(response).await;
    assert_eq!(json["status"], "ready");
    assert_eq!(json["database"], "up");
}

#[tokio::test]
async fn unknown_route_returns_json_404() {
    let app = build_router(test_state().await);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/definitely-not-a-route")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let json = body_json(response).await;
    assert_eq!(json["error"]["code"], "not_found");
}

#[tokio::test]
async fn request_id_header_is_echoed() {
    let app = build_router(test_state().await);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.headers().contains_key("x-request-id"),
        "响应应回传 x-request-id 头"
    );
}
