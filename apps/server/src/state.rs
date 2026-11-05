//! 共享应用状态：`axum` 的 handler 通过 `State<AppState>` 取用。

use sea_orm::DatabaseConnection;

use crate::config::AppConfig;

/// 所有 handler 共享的只读状态，克隆代价很低。
#[derive(Clone)]
pub struct AppState {
    /// SeaORM 连接池。
    pub db: DatabaseConnection,
    /// 运行期配置。
    pub config: AppConfig,
}

impl AppState {
    /// 由连接池与配置构造状态。
    pub fn new(db: DatabaseConnection, config: AppConfig) -> Self {
        Self { db, config }
    }

    /// 数据库连接池。
    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    /// 运行期配置。
    pub fn config(&self) -> &AppConfig {
        &self.config
    }
}
