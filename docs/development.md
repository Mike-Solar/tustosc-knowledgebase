# 开发指南

本文面向第一次在本仓库写代码的同学，从零把项目跑起来，并说明日常开发、迁移、测试和提交的流程。架构设计请看 [`architecture.md`](architecture.md)，接口约定请看 [`api.md`](api.md)。

## 目录

- [环境准备](#环境准备)
- [数据库准备](#数据库准备)
- [环境变量](#环境变量)
- [启动后端](#启动后端)
- [启动前端](#启动前端)
- [数据库迁移工作流](#数据库迁移工作流)
- [调试与日志](#调试与日志)
- [代码规范](#代码规范)
- [测试策略](#测试策略)
- [目录约定与「新功能该放哪」](#目录约定与新功能该放哪)
- [新人上手路线](#新人上手路线)
- [常见问题](#常见问题)

## 环境准备

| 工具 | 版本要求 | 检查命令 | 安装方式 |
| --- | --- | --- | --- |
| Rust | stable（edition 2024） | `rustc --version` | [rustup.rs](https://rustup.rs) |
| Cargo | 随 rustup 安装 | `cargo --version` | 同上 |
| Node.js | ≥ 22.0.0 | `node --version` | [nodejs.org](https://nodejs.org) 或 nvm |
| pnpm | ≥ 11.0.0 | `pnpm --version` | `corepack enable` 或 `npm i -g pnpm` |
| Docker | 任意较新版本 | `docker --version` | Docker Desktop |
| curl / jq（可选） | 任意 | `curl --version` | 系统包管理器 |

Rust 工具链建议补齐格式化与 lint 组件：

```bash
rustup component add rustfmt clippy
```

> 本仓库的 `Cargo.toml` 使用 `edition = "2024"`，需要 Rust 1.85+。如果 `cargo` 报 edition 不支持，先执行 `rustup update stable`。

## 数据库准备

后端同时支持 PostgreSQL / MySQL / SQLite，本地推荐 PostgreSQL（与生产一致），没装 Docker 时可以用 SQLite。

### PostgreSQL（推荐）

```bash
# 根目录执行
docker compose up -d
```

默认参数：

| 项 | 值 |
| --- | --- |
| 服务名 | `postgres` |
| 镜像 | `postgres:17-alpine` |
| 端口 | `5432` |
| 数据库 | `knowledgebase` |
| 用户名 | `knowledgebase` |
| 密码 | `knowledgebase` |

对应的连接串（也是 `.env.example` 的默认值）：

```text
DATABASE_URL=postgres://knowledgebase:knowledgebase@localhost:5432/knowledgebase
```

### MySQL

```bash
docker compose --profile mysql up -d
```

连接串：

```text
DATABASE_URL=mysql://knowledgebase:knowledgebase@localhost:3306/knowledgebase
```

### SQLite（零依赖）

不需要容器，连接串指向一个本地文件即可。文件不存在时需要 `?mode=rwc` 让 SQLite 自动创建：

```text
DATABASE_URL=sqlite://knowledgebase.db?mode=rwc
```

内存库仅用于测试（每个连接是独立数据库，连接池会被限制为 1 个连接）：

```text
DATABASE_URL=sqlite::memory:
```

### 清理数据库

```bash
docker compose down          # 停止容器，保留数据卷
docker compose down -v       # 停止并删除数据卷（数据全部丢失）
rm knowledgebase.db          # 删除 SQLite 文件
```

## 环境变量

配置全部来自环境变量，定义见 [`apps/server/src/config.rs`](../apps/server/src/config.rs)。本地开发时：

```bash
cp .env.example .env
```

`dotenvy` 会在启动时自动加载根目录的 `.env`；生产环境直接注入真实环境变量。

| 变量 | 默认值 | 说明 |
| --- | --- | --- |
| `APP_HOST` | `127.0.0.1` | HTTP 监听地址。容器内运行时设为 `0.0.0.0` |
| `APP_PORT` | `8080` | HTTP 监听端口 |
| `DATABASE_URL` | **无（必填）** | `postgres://` / `mysql://` / `sqlite:` 连接串 |
| `APP_RUN_MIGRATIONS` | `true` | 启动时是否自动执行迁移；生产环境建议改为 `false` 并用迁移命令单独执行 |
| `APP_CORS_ORIGINS` | 空（允许任意来源） | 允许的跨域来源，逗号分隔，例如 `http://localhost:3000` |
| `APP_REQUEST_TIMEOUT_SECS` | `30` | 单个请求超时时间（秒） |
| `RUST_LOG` | `info` | `tracing` 过滤表达式，如 `info,knowledgebase_server=debug` |

> `DATABASE_URL` 缺失或为空时服务会直接启动失败，并打印「缺少必填环境变量 `DATABASE_URL`」。

## 启动后端

```bash
# 根目录
cp .env.example .env          # 首次运行
docker compose up -d          # 首次运行
cargo run -p knowledgebase-server
```

看到以下日志说明启动成功：

```text
 INFO knowledgebase_server: starting knowledgebase-server host=127.0.0.1 port=8080 database=postgres://***@localhost:5432/knowledgebase
 INFO knowledgebase_server: database connected backend=Postgres
 INFO knowledgebase_server: database migrations applied
 INFO knowledgebase_server: listening on http://127.0.0.1:8080
```

验证：

```bash
curl -s http://127.0.0.1:8080/healthz
# {"status":"ok","service":"knowledgebase-server","version":"0.1.0"}

curl -s http://127.0.0.1:8080/readyz
# {"status":"ready","database":"up"}
```

`/healthz` 不访问数据库，用于存活探针；`/readyz` 会真正 `ping` 数据库，失败时返回 `503` 与 `{"status":"degraded","database":"down"}`。

停止：`Ctrl+C`。服务收到 `SIGINT` / `SIGTERM` 后会优雅停机。

## 启动前端

前端位于 `apps/web`，由根 pnpm workspace 管理，包名为 `knowledgebase-web`。

```bash
# 根目录
pnpm install
pnpm --filter knowledgebase-web dev
```

默认访问 <http://localhost:3000>。根目录也提供了快捷脚本：

```bash
pnpm dev          # 等价于 pnpm --filter knowledgebase-web dev
pnpm typecheck
pnpm lint:check
pnpm test
```

前端与后端联调时注意：

- 后端默认监听 `127.0.0.1:8080`，前端默认 `localhost:3000`。
- 本地开发 `.env` 里 `APP_CORS_ORIGINS=http://localhost:3000`；留空会允许任意来源，仅建议本地使用。
- 健康检查在 `/healthz`、`/readyz`，业务接口统一在 `/api/v1` 下。

## 数据库迁移工作流

迁移代码位于 `crates/migration`，crate 名为 `knowledgebase-migration`。迁移是独立二进制，不依赖 server 进程：

```bash
cargo run -p knowledgebase-migration -- up       # 应用所有未执行的迁移
cargo run -p knowledgebase-migration -- status   # 查看已应用/未应用
cargo run -p knowledgebase-migration -- down -n 1 # 回滚最近 1 个迁移
```

带自定义连接串（例如临时用 SQLite 验证）：

```bash
DATABASE_URL="sqlite://knowledgebase-dev.db?mode=rwc" cargo run -p knowledgebase-migration -- up
DATABASE_URL="sqlite://knowledgebase-dev.db?mode=rwc" cargo run -p knowledgebase-migration -- down
```

### 新增迁移的步骤

1. 在 `crates/migration/src/` 新建文件，命名格式：

   ```text
   m{YYYYMMDD}_{6位序号}_{动作}.rs
   ```

   例如 `m20261110_000008_create_comments.rs`。序号递增，不要复用已有编号。

2. 实现 `MigrationTrait` 的 `up` 与 `down`，`up` 里创建表/索引，`down` 里按相反顺序删除。

3. 在 `crates/migration/src/lib.rs` 的 `mod` 列表和 `Migrator::migrations()` 中注册，顺序必须与依赖顺序一致（被引用的表先建）。

4. 本地验证上升 + 回滚：

   ```bash
   DATABASE_URL="sqlite://knowledgebase-dev.db?mode=rwc" cargo run -p knowledgebase-migration -- up
   DATABASE_URL="sqlite://knowledgebase-dev.db?mode=rwc" cargo run -p knowledgebase-migration -- down
   ```

5. 如果新增/修改表，同步更新 `apps/server/src/entities/` 中对应的 SeaORM 实体，以及 [`docs/architecture.md`](architecture.md) 的数据模型章节。

### 迁移规范

- **一次迁移只做一件事**：建表、加索引、改字段分开，便于回滚定位。
- **`down` 必须可用**：不允许只写 `up`，否则本地回滚与故障恢复会卡住。
- **保持方言兼容**：三种数据库都要能跑。避免使用数据库独有语法；确需使用时必须在其他方言上提供等价方案并注释说明。典型例子是 `documents` 的「同空间活跃标题唯一」：PostgreSQL / SQLite 用部分唯一索引，MySQL 用 STORED 生成列 + 唯一索引（见 `m20261008_000006_create_documents.rs`）。
- **时间字段**统一 `timestamp_with_time_zone`，默认值用 `Expr::current_timestamp()`。
- **主键**统一应用层生成雪花 ID（`big_integer`），不使用自增。
- **外键命名** `fk_<表名>_<字段名>`，索引命名 `idx_<表名>_<字段列表>`，唯一索引 `uniq_<表名>_<含义>`。

## 调试与日志

### 日志级别

通过 `RUST_LOG` 控制，支持按模块设置：

```bash
RUST_LOG=debug cargo run -p knowledgebase-server
RUST_LOG=info,knowledgebase_server=debug,sqlx=warn cargo run -p knowledgebase-server
```

推荐日常开发使用 `.env` 中的 `RUST_LOG=info,knowledgebase_server=debug`。

### 请求 ID

每个请求由 `tower-http` 自动生成或透传 `x-request-id`，并回写到响应头。排查问题时先拿请求 ID，再在日志里搜索对应记录。

### 健康检查与探针

| 接口 | 用途 | 依赖数据库 | 失败状态码 |
| --- | --- | --- | --- |
| `GET /healthz` | 存活探针 | 否 | — |
| `GET /readyz` | 就绪探针 | 是（`db.ping()`） | `503` |

### 常见调试入口

- 配置加载：`AppConfig::from_env()`，出错信息会指出具体环境变量。
- 数据库连接：`db::connect()` 会打印后端类型与是否内存库。
- 统一错误：任意 handler 返回 `AppError`，会自动转成 `{"error":{"code","message"}}`；`internal_error` 只写日志、不向客户端泄漏细节。

## 代码规范

### Rust

```bash
cargo fmt --all                 # 格式化（提交前必须执行）
cargo fmt --all -- --check      # 只检查
cargo clippy --workspace --all-targets -- -D warnings   # 静态检查，警告视为错误
```

约定：

- 注释与文档注释使用中文，解释「为什么」和边界约束，不重复代码字面意思。
- 分层依赖只允许从外向内：`routes` → `service`（规划中）→ `repository`（规划中）→ `entities`；`config` / `db` / `error` / `state` / `telemetry` 属于基础设施，可被各层使用。
- 错误统一使用 `AppError`，不要把 `anyhow::Error` 直接暴露成 HTTP 响应。
- 新增公开函数尽量写文档注释（`///`），模块顶部写 `//!` 说明职责。

### 前端

配置沿用 `apps/web` 现有规则：2 空格、单引号、分号、100 列、ES5 trailing comma（`.prettierrc`）；ESLint 使用 Nuxt 默认配置。

```bash
pnpm lint          # ESLint 自动修复
pnpm lint:check    # 只检查
pnpm format        # Prettier 格式化
```

### 提交信息

统一 `type(scope): 中文摘要`，普通提交只写一行；破坏性变更用 `!` 并写 `BREAKING CHANGE:` body。详细规则与示例见 [`CONTRIBUTING.md`](../CONTRIBUTING.md)。

## 测试策略

### 后端

```bash
cargo test --workspace                 # 全部测试
cargo test -p knowledgebase-server     # 只测 server
cargo test -p knowledgebase-migration  # 只测迁移相关
```

现有测试分两类：模块内的 `#[cfg(test)] mod tests`（配置解析、数据库 URL 判定、错误模型等），以及 `apps/server/tests/` 下的集成冒烟测试（`health.rs` 覆盖路由装配、`/healthz`、`/readyz`、JSON 404、`x-request-id` 回传）。

- **单元测试**：纯逻辑，不连数据库。例如 `config`、`error`、`db::is_in_memory_sqlite`。
- **集成测试**：需要数据库时优先使用 SQLite 内存库：

  ```rust
  let db = db::connect("sqlite::memory:").await?;
  db::migrate(&db).await?;
  ```

  注意内存库连接池只有 1 个连接（见 `db.rs`），测试中不要并发开多个 pool。
- **HTTP 层测试**：`tower::ServiceExt::oneshot` 直接对 `Router` 发请求，不占端口。`AppConfig::for_testing()` 提供了测试配置；可参考 `apps/server/tests/health.rs` 的写法。
- **未来**：需要验证 PostgreSQL/MySQL 方言时引入 `testcontainers`，在 CI 中按标签运行，不要求本地默认跑（规划中）。

### 前端

```bash
pnpm test                                                    # Vitest 单次运行
pnpm --filter knowledgebase-web exec vitest                  # 本地监听（watch 模式）
pnpm --filter knowledgebase-web exec vitest run tests/unit   # 只跑某个目录
```

前端测试与业务代码同层放置或放在 `apps/web/tests/`，详见 [`apps/web/AGENTS.md`](../apps/web/AGENTS.md)。

## 目录约定与「新功能该放哪」

```text
apps/server/src/
├── main.rs          # 入口：加载 .env → 配置 → run()
├── lib.rs           # 组装启动流程、优雅停机
├── app.rs           # 路由装配 + 中间件（CORS/Trace/RequestId/Timeout）
├── config.rs        # 环境变量 → AppConfig
├── db.rs            # 连接池与迁移执行
├── error.rs         # AppError 统一错误模型
├── state.rs         # AppState（db + config）
├── telemetry.rs     # tracing 初始化
├── routes/          # HTTP 入口
│   ├── mod.rs       # /api/v1 路由挂载
│   └── health.rs    # /healthz、/readyz
└── entities/        # SeaORM 实体，与迁移表一一对应
```

新增一个功能的推荐落点：

| 要做的事 | 放哪 |
| --- | --- |
| 新增 HTTP 接口 | `apps/server/src/routes/<模块>.rs`，在 `routes/mod.rs` 挂到 `/api/v1` |
| 新增数据库表 | `crates/migration/src/m{日期}_{序号}_*.rs` + `lib.rs` 注册 |
| 新增表对应实体 | `apps/server/src/entities/<表名>.rs` + `entities/mod.rs` 导出 |
| 新增错误码 | `apps/server/src/error.rs`，同时更新 [`api.md`](api.md) 错误码表 |
| 新增环境变量 | `apps/server/src/config.rs` + `.env.example` + 本文档环境变量表 |
| 新增前端页面 | `apps/web/app/pages/`（遵循 `apps/web/AGENTS.md` 的分层规则） |
| 新增前端业务用例 | `apps/web/{domain,application,interface-adapters,infrastructure}/` |

> 注意：`routes` 层只做参数解析与响应组装；业务逻辑放在 `service` 层，数据访问放在 `repository` 层（这两层随业务模块逐步引入，见 [`architecture.md`](architecture.md) 的分层设计）。

## 新人上手路线

按由浅入深的顺序，三个练习方向（动手前先在 Issue 里认领，避免和别人的改动冲突）：

### 练习 1：文档与可观测性（1～2 小时）

- 给 `GET /healthz` 增加一个字段（例如 `uptime_seconds`），同步更新测试与 [`api.md`](api.md)。
- 学习点：读 `routes/health.rs`、写 `#[cfg(test)]` 测试、理解「文档与代码必须同步」。
- 验证：`cargo test -p knowledgebase-server`，`curl` 看到新字段。

### 练习 2：迁移与实体（半天）

- 新增一张小表（例如 `document_comments`），写 `up`/`down`、注册到 `Migrator`、补 SeaORM 实体。
- 学习点：三种数据库方言差异、外键与索引、实体与迁移的一致性。
- 验证：在 PostgreSQL 与 SQLite 上分别 `up` → `status` → `down`。

### 练习 3：一个完整的只读接口（1～2 天）

- 实现 `GET /api/v1/users/me` 的骨架（鉴权可以先返回固定测试用户），包括路由、DTO、错误处理与集成测试。
- 学习点：分层职责、`AppState` 注入、错误模型、`tower::oneshot` 测试、CORS 与请求 ID。
- 验证：`cargo test`，并用前端或 `curl` 实际请求。

## 常见问题

**Q：启动时报「缺少必填环境变量 `DATABASE_URL`」？**
A：根目录没有 `.env`，或 `.env` 里没有 `DATABASE_URL`。执行 `cp .env.example .env` 后重试。

**Q：`readyz` 返回 503？**
A：数据库没起来或连接串不对。`docker compose ps` 看容器状态，再确认 `DATABASE_URL` 的端口、库名、用户名密码。

**Q：端口被占用？**
A：改 `.env` 里的 `APP_PORT`（后端）或用 `pnpm --filter knowledgebase-web dev --port 3001`（前端），同时调整 `APP_CORS_ORIGINS`。

**Q：`pnpm install` 报 Node/pnpm 版本不满足？**
A：根 `package.json` 的 `engines` 要求 Node ≥ 22、pnpm ≥ 11。用 `node -v` / `pnpm -v` 核对，必要时 `corepack enable` 后重装。

**Q：SQLite 迁移后查询报「no such table」？**
A：`sqlite::memory:` 每次连接都是新库。确认用的是 `db::connect` 返回的同一个连接池，或改用文件库 `sqlite://knowledgebase.db?mode=rwc`。

**Q：`cargo clippy` 出现大量 node_modules 相关报错？**
A：Rust workspace 只包含 `apps/server` 和 `crates/migration`，不会扫描前端目录。若确实出现，检查是否在错误目录执行了 cargo。

**Q：改了迁移文件，本地 `up` 没生效？**
A：已应用的迁移不会重复执行。开发阶段可以 `down` 后重新 `up`，或删除 SQLite 文件 / `docker compose down -v` 重建数据库。
