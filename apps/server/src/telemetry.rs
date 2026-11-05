//! 日志与可观测性初始化。

use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// 按 `RUST_LOG`（或传入的默认过滤表达式）初始化 `tracing` 订阅器。
///
/// 重复初始化会被忽略，方便在测试中多次调用。
pub fn init(default_filter: &str) {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter));

    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(false)
                .with_file(false),
        )
        .try_init();
}
