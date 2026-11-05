//! 配置加载：所有配置项都来自环境变量，便于容器化部署与多环境切换。
//!
//! | 变量 | 默认值 | 说明 |
//! | --- | --- | --- |
//! | `APP_HOST` | `127.0.0.1` | HTTP 监听地址 |
//! | `APP_PORT` | `8080` | HTTP 监听端口 |
//! | `DATABASE_URL` | 无（必填） | PostgreSQL / MySQL / SQLite 连接串 |
//! | `APP_RUN_MIGRATIONS` | `true` | 启动时是否自动执行迁移 |
//! | `APP_CORS_ORIGINS` | 空（允许任意来源） | 允许的跨域来源，逗号分隔 |
//! | `APP_REQUEST_TIMEOUT_SECS` | `30` | 请求超时时间（秒） |
//! | `RUST_LOG` | `info` | 日志过滤表达式 |

use std::net::IpAddr;
use std::{env, net::SocketAddr};

use thiserror::Error;

/// 配置加载失败的原因。
#[derive(Debug, Error)]
pub enum ConfigError {
    /// 缺少必填环境变量。
    #[error("缺少必填环境变量 `{0}`")]
    Missing(&'static str),
    /// 环境变量存在但无法解析。
    #[error("环境变量 `{name}` 的值 `{value}` 无法解析: {reason}")]
    Invalid {
        name: &'static str,
        value: String,
        reason: String,
    },
}

/// 应用运行期配置。
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// HTTP 监听地址。
    pub host: IpAddr,
    /// HTTP 监听端口。
    pub port: u16,
    /// 数据库连接串（`postgres://` / `mysql://` / `sqlite:`）。
    pub database_url: String,
    /// 启动时是否自动执行 SeaORM 迁移。
    pub run_migrations: bool,
    /// `tracing` 的日志过滤表达式。
    pub log_filter: String,
    /// 允许的跨域来源；为空时允许任意来源（仅建议本地开发使用）。
    pub cors_origins: Vec<String>,
    /// 单个请求的超时时间（秒）。
    pub request_timeout_secs: u64,
}

impl AppConfig {
    /// 从环境变量读取配置；`DATABASE_URL` 为必填项。
    pub fn from_env() -> Result<Self, ConfigError> {
        let host = match env::var("APP_HOST") {
            Ok(value) => value
                .parse::<IpAddr>()
                .map_err(|error| ConfigError::Invalid {
                    name: "APP_HOST",
                    value,
                    reason: error.to_string(),
                })?,
            Err(_) => IpAddr::from([127, 0, 0, 1]),
        };

        let port = parse_env("APP_PORT", 8080_u16)?;
        let run_migrations = parse_env("APP_RUN_MIGRATIONS", true)?;
        let request_timeout_secs = parse_env("APP_REQUEST_TIMEOUT_SECS", 30_u64)?;

        let database_url =
            env::var("DATABASE_URL").map_err(|_| ConfigError::Missing("DATABASE_URL"))?;
        if database_url.trim().is_empty() {
            return Err(ConfigError::Missing("DATABASE_URL"));
        }

        let cors_origins = env::var("APP_CORS_ORIGINS")
            .unwrap_or_default()
            .split(',')
            .map(str::trim)
            .filter(|origin| !origin.is_empty())
            .map(ToOwned::to_owned)
            .collect();

        Ok(Self {
            host,
            port,
            database_url,
            run_migrations,
            log_filter: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned()),
            cors_origins,
            request_timeout_secs,
        })
    }

    /// 构造测试用配置（SQLite 内存库，默认关闭自动迁移之外的副作用）。
    pub fn for_testing(database_url: impl Into<String>) -> Self {
        Self {
            host: IpAddr::from([127, 0, 0, 1]),
            port: 0,
            database_url: database_url.into(),
            run_migrations: false,
            log_filter: "warn".to_owned(),
            cors_origins: Vec::new(),
            request_timeout_secs: 30,
        }
    }

    /// 最终的监听地址。
    pub fn bind_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}

fn parse_env<T>(name: &'static str, default: T) -> Result<T, ConfigError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    match env::var(name) {
        Ok(value) => value.parse::<T>().map_err(|error| ConfigError::Invalid {
            name,
            value,
            reason: error.to_string(),
        }),
        Err(_) => Ok(default),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bind_addr_uses_config_host_and_port() {
        let mut config = AppConfig::for_testing("sqlite::memory:");
        config.port = 9000;
        assert_eq!(config.bind_addr().to_string(), "127.0.0.1:9000");
    }

    #[test]
    fn cors_origins_are_split_and_trimmed() {
        // 直接验证解析逻辑，避免依赖真实环境变量（环境变量在测试间共享）。
        let origins: Vec<String> = " http://localhost:3000 , https://example.com ,,"
            .split(',')
            .map(str::trim)
            .filter(|origin| !origin.is_empty())
            .map(ToOwned::to_owned)
            .collect();
        assert_eq!(
            origins,
            vec!["http://localhost:3000", "https://example.com"]
        );
    }
}
