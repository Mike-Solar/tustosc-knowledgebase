//! 数据库连接与迁移：同一份代码通过 `DATABASE_URL` 支持 PostgreSQL / MySQL / SQLite。

use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

use knowledgebase_migration::{Migrator, MigratorTrait};

/// 连接池最大连接数（内存 SQLite 只允许一个连接，见 [`is_in_memory_sqlite`]）。
const MAX_CONNECTIONS: u32 = 10;
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

/// 根据连接串建立数据库连接池。
///
/// SeaORM 会从 URL 的 scheme 推断方言：
/// - `postgres://user:pass@host:5432/db`
/// - `mysql://user:pass@host:3306/db`
/// - `sqlite://file.db` / `sqlite::memory:`
pub async fn connect(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut options = ConnectOptions::new(database_url.to_owned());

    if is_in_memory_sqlite(database_url) {
        // `sqlite::memory:` 的每个连接都是独立的库，连接池只能保持一个连接，
        // 否则迁移与查询会落在不同的内存库上。
        options
            .max_connections(1)
            .min_connections(1)
            .sqlx_logging(false);
    } else {
        options
            .max_connections(MAX_CONNECTIONS)
            .min_connections(1)
            .connect_timeout(CONNECT_TIMEOUT)
            .acquire_timeout(CONNECT_TIMEOUT);
    }

    let db = Database::connect(options).await?;

    tracing::info!(
        backend = ?db.get_database_backend(),
        in_memory = is_in_memory_sqlite(database_url),
        "database connected"
    );

    Ok(db)
}

/// 执行所有待应用的迁移。
pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(db, None).await
}

/// 判断连接串是否指向 SQLite 内存库。
pub fn is_in_memory_sqlite(database_url: &str) -> bool {
    database_url.starts_with("sqlite:")
        && (database_url.contains(":memory:") || database_url.contains("mode=memory"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_in_memory_sqlite() {
        assert!(is_in_memory_sqlite("sqlite::memory:"));
        assert!(is_in_memory_sqlite(
            "sqlite://file?mode=memory&cache=shared"
        ));
        assert!(!is_in_memory_sqlite("sqlite://knowledgebase.db"));
        assert!(!is_in_memory_sqlite("postgres://localhost/knowledgebase"));
    }
}
