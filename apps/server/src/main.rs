//! 应用入口：加载 `.env` 与配置，然后把控制权交给 [`knowledgebase_server::run`]。

use anyhow::Context;

use knowledgebase_server::{config::AppConfig, run};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 本地开发时从仓库根目录的 `.env` 读取配置；生产环境用真实环境变量。
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env().context("加载应用配置失败")?;
    run(config).await
}
