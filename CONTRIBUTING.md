# 贡献指南

欢迎加入天津科技大学开放原子开源协会的知识库项目！

这个项目要做的是「类语雀」的社团知识库：空间、文档、目录、历史版本、权限、协作编辑。它同时也是一个**学习型项目**——我们更在意你能否在这里学会读代码、写测试、发 PR，而不是一次写多少行。第一次参与开源？完全没问题，照下面的步骤做即可。

> 仓库地址：<https://github.com/TUST-Open-Source-Club/knowledgebase>
> 组织：TUST-Open-Source-Club
> 许可证：AGPL-3.0（贡献即视为同意以该许可证发布）

## 目录

- [先读什么](#先读什么)
- [找一个任务](#找一个任务)
- [准备开发环境](#准备开发环境)
- [分支与提交](#分支与提交)
- [发起 Pull Request](#发起-pull-request)
- [代码评审标准](#代码评审标准)
- [本地验证清单](#本地验证清单)
- [遇到问题怎么办](#遇到问题怎么办)

## 先读什么

按顺序花 30 分钟读完，能省下后面几小时的摸索：

| 顺序 | 文档 | 你会知道 |
| --- | --- | --- |
| 1 | [`README.md`](README.md) | 项目是什么、怎么跑起来 |
| 2 | [`docs/architecture.md`](docs/architecture.md) | 后端分层、数据模型、关键流程 |
| 3 | [`docs/development.md`](docs/development.md) | 本地环境、迁移、调试、测试 |
| 4 | [`docs/api.md`](docs/api.md) | 接口约定，哪些已实现、哪些在规划中 |
| 5 | [`apps/web/AGENTS.md`](apps/web/AGENTS.md) | 前端目录边界与协作规则 |

如果你只改前端，第 1、2、5 篇必读；只改后端，第 1～4 篇必读。

## 找一个任务

1. 打开 [Issues](https://github.com/TUST-Open-Source-Club/knowledgebase/issues)，筛选 `good first issue` 标签——这些任务我们保证有明确的验收标准，且改动范围可控。
2. 想做什么但 Issue 里没有？先开一个 Issue 描述你的想法，等维护者确认后再动手，避免白做。
3. 在 Issue 下留言「我来做」，维护者会把 Issue 分配给你。**一次只认领一个**，做完再认领下一个。
4. 如果一周内没有进展，Issue 会被释放给其他人——不是惩罚，只是让任务流动起来；随时可以重新认领。

常用标签：

| 标签 | 含义 |
| --- | --- |
| `good first issue` | 适合第一次贡献，指导完整 |
| `help wanted` | 欢迎任何人认领 |
| `bug` | 行为不符合预期 |
| `enhancement` | 新功能或改进 |
| `docs` | 纯文档改动 |
| `backend` / `frontend` | 涉及的技术侧 |
| `blocked` | 依赖其他任务，先别动 |

## 准备开发环境

完整步骤见 [`docs/development.md`](docs/development.md)，最短路径：

```bash
# 1. Fork 后克隆你自己的仓库
git clone https://github.com/<你的用户名>/knowledgebase.git
cd tustosc-knowledgebase

# 2. 添加主仓库为上游，方便同步
git remote add upstream https://github.com/TUST-Open-Source-Club/knowledgebase.git

# 3. 安装前端依赖（根目录执行）
pnpm install

# 4. 启动数据库
docker compose up -d

# 5. 启动后端
cp .env.example .env
cargo run -p knowledgebase-server

# 6. 另开终端启动前端
pnpm --filter knowledgebase-web dev
```

需要安装：Rust（stable，通过 [rustup](https://rustup.rs)）、Node.js ≥ 22、pnpm ≥ 11、Docker Desktop。

## 分支与提交

### 分支命名

从最新的 `main` 切出，命名格式 `类型/简短英文描述`：

```bash
git fetch upstream
git checkout -b feat/document-version-api upstream/main
```

| 前缀 | 用途 | 示例 |
| --- | --- | --- |
| `feat/` | 新功能 | `feat/space-members` |
| `fix/` | 修 bug | `fix/refresh-token-rotation` |
| `docs/` | 文档 | `docs/api-examples` |
| `refactor/` | 重构 | `refactor/error-model` |
| `test/` | 补测试 | `test/document-repository` |
| `chore/` | 工具链、依赖 | `chore/bump-sea-orm` |

### 提交信息

提交信息统一使用 **Conventional Commits + 中文摘要**，格式：

```text
type(scope): 中文摘要
```

- 普通提交**只写一行 header，不写 body**。
- 破坏性变更在 type 后加 `!`，并必须写 `BREAKING CHANGE:` body。
- 摘要用中文，动词开头，不超过 50 个字符，结尾不加句号。
- `scope` 用模块名：`server`、`web`、`migration`、`docs`、`api`、`auth`、`space`、`document`。

示例：

```text
feat(server): 添加文档乐观锁保存接口
fix(web): 修复编辑器切换文档时内容未重置
docs(architecture): 补充版本回滚流程
feat(api)!: 调整文档详情响应结构

BREAKING CHANGE: 文档详情响应由 data.document 改为直接返回文档对象
```

允许的 type：`feat`、`fix`、`docs`、`style`、`refactor`、`perf`、`test`、`build`、`ci`、`chore`、`revert`。

> 根目录的 `husky` 钩子会对**所有提交**执行 `commitlint` 与自定义校验器（`apps/web/scripts/validate-commit-message.mjs`）；`pre-commit` 只对暂存的前端代码运行 lint-staged，`pre-push` 运行前端 lint + typecheck。Rust 侧目前没有提交钩子，但 CI 会强制执行 `cargo fmt`、`clippy`、`test`。

### 保持分支同步

```bash
git fetch upstream
git rebase upstream/main
```

优先使用 rebase 而不是 merge，保持提交历史线性；如果 rebase 冲突处理不了，在 Issue 里求助，不要直接 force-push 到主仓库（你也没有权限）。

## 发起 Pull Request

1. 推送你的分支到自己的 fork：

   ```bash
   git push origin feat/document-version-api
   ```

2. 在 GitHub 上向 `TUST-Open-Source-Club/knowledgebase` 的 `main` 发起 PR，标题遵循提交信息格式，例如 `feat(server): 添加文档历史版本接口`。
3. PR 描述请包含：
   - **改了什么**：一句话说明。
   - **为什么**：关联 Issue，写 `Closes #12`。
   - **怎么验证的**：贴出你跑过的命令与结果（见下方清单）。
   - **截图/录屏**：涉及界面改动时必填。
   - **遗留问题**：没做完的部分明确写出来，不要藏着。
4. PR 保持小而聚焦。一个 PR 只做一件事；如果顺手改了无关文件，评审时会被要求拆开。
5. 收到评审意见后，**继续在同一个分支追加提交**，不要关闭 PR 重开。评审通过后由维护者 squash 合并（或按仓库约定合并）。

## 代码评审标准

评审不是挑刺，是两个人一起确认「这段代码三个月后还能被维护」。我们会重点看：

| 维度 | 我们关心什么 |
| --- | --- |
| 正确性 | 边界条件、错误处理、并发与事务、是否真的修好了问题 |
| 一致性 | 命名、分层、错误码、返回结构是否与现有代码一致 |
| 可读性 | 新人能否只看名字和注释就理解意图；注释是否解释「为什么」 |
| 可测试性 | 是否有测试覆盖新增行为；测试是否验证行为而非实现细节 |
| 安全性 | 密码/令牌是否泄漏进日志；权限校验是否在服务端完成；SQL 是否参数化 |
| 文档 | 行为、命令、接口变化是否同步更新了 `README.md` / `docs/` |

评审意见通常分三类：

- **必须改**（blocking）：正确性、安全、破坏约定。改完才能合并。
- **建议改**（non-blocking）：可读性、更好的写法。可以讨论，双方达成一致即可。
- **提问**：评审者没看懂的地方。请补充解释或改代码，不要只回「已修改」。

作为 PR 作者，你也有权礼貌地讨论——如果不同意某条意见，说明理由，而不是默默照改或直接忽略。

## 本地验证清单

提交 PR 前，**在本地跑通与改动相关的命令**，并把结果贴到 PR 描述里。`[x]` 表示已通过。

### 后端（Rust）

```bash
cargo fmt --all -- --check                 # 格式检查
cargo clippy --workspace --all-targets -- -D warnings   # 静态检查
cargo test --workspace                     # 全部测试
```

数据库相关改动还需要验证迁移可上升、可回滚（用 SQLite 最省事）：

```bash
DATABASE_URL="sqlite://knowledgebase-dev.db?mode=rwc" cargo run -p knowledgebase-migration -- up
DATABASE_URL="sqlite://knowledgebase-dev.db?mode=rwc" cargo run -p knowledgebase-migration -- down
```

### 前端（TypeScript / Vue）

```bash
pnpm install --frozen-lockfile   # 确认锁文件干净
pnpm lint:check                  # ESLint 只检查不修改
pnpm typecheck                   # vue-tsc 类型检查
pnpm test                        # Vitest 单元测试
pnpm build                       # 生产构建（改动较大时）
```

### 接口自测

后端启动后，至少确认健康检查正常：

```bash
curl -s http://127.0.0.1:8080/healthz
curl -s http://127.0.0.1:8080/readyz
```

新增接口时，在 PR 描述里贴出 `curl` 请求与响应示例。

### 通用

```bash
git diff --check    # 检查空白符错误
git status          # 确认没有误提交 .env、数据库文件、构建产物
```

## 遇到问题怎么办

- **不知道该改哪里**：在 Issue 下提问，或直接 @ 维护者。提问不丢人，瞎改才浪费时间。
- **环境跑不起来**：先看 [`docs/development.md`](docs/development.md) 的「常见问题」，再带上完整报错信息提问。
- **发现安全问题**：不要开公开 Issue，私信维护者。
- **想长期参与**：从 `good first issue` 做起，熟悉后可以认领 `help wanted`，再往后可以成为某个模块的 reviewer。

最后一条：**不要在没有验证的情况下宣称「已通过」**。跑挂了也是有效信息，如实写出来，我们一起看。
