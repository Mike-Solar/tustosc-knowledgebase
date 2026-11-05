# ADR-0001：采用前后端单仓 Monorepo 与技术栈

- 状态：已接受
- 日期：2026-11-05
- 决策者：协会知识库项目组

## 背景

本项目要做一套面向天津科技大学开放原子开源协会的「类语雀」知识库：空间、文档、目录、历史版本、权限与协作编辑缺一不可。项目由社团同学长期接力维护，参与者多为刚入门的开发者，因此工程结构必须同时满足三点：

1. **一眼能看懂**：新人克隆仓库后，知道前端、后端、数据库分别在哪里。
2. **改得动**：前端和后端可以独立演进，但不至于拆成多个仓库后版本对不齐。
3. **跑得起**：一台普通笔记本、一个 `docker compose up -d` 就能完成本地联调。

仓库最初只有前端（Nuxt 4 + Vue 3 + Milkdown），后端从零开始。我们需要在「拆成两个仓库」与「单仓多包」之间做一次长期决策，同时确定后端技术栈。

## 决策

采用 **单仓 Monorepo**，并使用 Cargo workspace + pnpm workspace 分别管理 Rust 与 Node 两个生态：

```text
tustosc-knowledgebase/
├── Cargo.toml            # Rust virtual workspace
├── package.json          # pnpm workspace 根（前端包通过 workspace 协议管理）
├── pnpm-workspace.yaml
├── apps/
│   ├── server/           # Rust + Axum + SeaORM 后端（knowledgebase-server）
│   └── web/              # Nuxt 4 + Vue 3 + Milkdown 前端（knowledgebase-web）
├── crates/
│   └── migration/        # SeaORM 迁移（knowledgebase-migration）
└── docs/                 # 全项目文档
```

后端技术栈固定为 **Rust + Axum 0.8 + SeaORM 2（PostgreSQL / MySQL / SQLite 三种驱动同时编译）**；前端沿用已有的 **Nuxt 4 + Vue 3 + Milkdown 7**，包管理统一使用 **pnpm**。数据库迁移放在独立的 `crates/migration` 中，以独立二进制运行，而不是塞进 server 进程里。

## 备选方案

### 方案 A：前后端拆成两个仓库

- 优点：权限和 CI 完全独立；前端可复用原有仓库历史。
- 缺点：接口契约、数据库迁移、文档和 Issue 分散在两个仓库；社团新人需要同时关注两个仓库和两套 CI；跨仓库联调要手动保持版本一致。
- 结论：否决。当前团队规模小、接口变更频繁，拆仓的协作成本远高于收益。

### 方案 B：单仓 Monorepo（采用）

- 优点：一次 clone 拿到全部代码；接口改动可以在一个 PR 里同时改前后端；文档、ADR、Issue 集中；CI 可以按路径过滤（规划中）。
- 缺点：仓库体积随时间增长；Rust 与 Node 两套工具链需要在一台机器上共存。
- 结论：采用。

### 方案 C：后端也用 TypeScript（Nuxt server / Nitro）

- 优点：前后端同语言，新人学习成本最低。
- 缺点：本项目定位是「对标语雀级别」的长期工程，后端需要严格的类型、并发与内存安全，以及成熟的 ORM/迁移生态；Rust + Axum 更符合协会希望锻炼系统编程能力的目标。
- 结论：否决，前端继续使用 TypeScript。

### 方案 D：后端 Rust 但只支持 PostgreSQL

- 优点：实现最简单，无方言兼容成本。
- 缺点：社团成员本地环境差异大，部分同学没有 Docker 或没有生产级 PG 实例；SQLite 可以零依赖跑通全部功能，MySQL 则覆盖常见的云厂商免费/学生数据库。
- 结论：否决，改为**三种数据库同时支持**，代价是需要处理数据库方言差异（见 `docs/architecture.md` 的「多数据库策略」）。

## 后果

### 正面结果

- 一个仓库、一次 clone 即可参与全栈开发，新人上手路径清晰。
- 前后端接口约定（`docs/api.md`）和数据库结构（`crates/migration`）在同一处演进，评审时能看到完整变更。
- 根 `Cargo.toml` 使用 virtual workspace，新增 Rust crate 不需要改动 `apps/server` 的依赖结构。
- 前端的 `knowledgebase-web` 通过 pnpm workspace 管理，实现与 `apps/web` 原有 Nuxt 工程无缝衔接。

### 需要承担的成本

- 仓库同时需要 Rust 工具链与 Node/pnpm 工具链，CI 配置比单语言仓库复杂。
- 前端原有的 `apps/web/pnpm-lock.yaml` 需要迁移到根目录统一管理（迁移期间两套锁文件可能短暂共存）。
- 数据库方言兼容要求迁移与实体代码保持克制：不能使用某个数据库独有的 SQL 特性，除非有明确的降级方案。
- 文档需要同时覆盖前端（`apps/web/docs/`）与后端（根 `docs/`），本 ADR 所在目录是二者的共同入口。

## 约束

- 前端包名固定为 `knowledgebase-web`，后端 crate 名为 `knowledgebase-server`，迁移 crate 名为 `knowledgebase-migration`。
- 不新增第三方前端包管理器（npm / yarn / bun）；根目录只允许 pnpm。
- Rust 代码统一使用 `cargo fmt`、`cargo clippy`，提交前必须通过；具体命令见 `docs/development.md`。
- 后端分层与模块边界见 `docs/architecture.md`；接口约定见 `docs/api.md`。
- 本决策的回滚成本较高（拆仓需要迁移历史），若未来团队规模显著扩大，可另立 ADR 讨论按包拆仓。
