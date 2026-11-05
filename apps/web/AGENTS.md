# 项目协作规约

## 项目定位

这是一个基于 Nuxt 4、Vue 3 和 TypeScript 的类语雀社团知识库。项目面向长期维护和多人协作，当前阶段优先建设稳定的目录边界、代码组织方式和文档治理，不在基础骨架中提前实现真实业务。

## 开始工作前

1. 先阅读本文件和 [Clean Architecture 说明](docs/architecture/clean-architecture.md)。
2. 根据任务阅读 [目录速查](docs/architecture/directory-map.md) 和 [开发流程](docs/guides/development-workflow.md)。
3. 修改既有行为前，先搜索相关测试和调用方；不要只根据文件名推断职责。
4. 使用 pnpm，不切换到 npm、yarn 或其他包管理器。

## 目录与依赖方向

```text
app/                  Nuxt 表现层：页面、组件、布局、composable
server/               Nuxt 服务端入口：API、中间件、插件
shared/               app/server 都能安全复用的无副作用代码
domain/               企业规则：实体、值对象、领域服务、仓储抽象
application/          用例：DTO、端口、用例编排
interface-adapters/   输入输出转换：controller、presenter、mapper、view-model
infrastructure/       外部实现：HTTP、持久化、Markdown、日志、Nuxt 组合根
tests/                单元、集成和端到端测试
docs/                 长期说明、指南、ADR 和阶段性计划
```

依赖只能向内：

```text
app/server → interface-adapters → application → domain
infrastructure → application ports / domain repositories
```

- `domain` 不得导入 Vue、Nuxt、Pinia、Milkdown、HTTP 客户端或数据库驱动。
- `application` 只依赖 domain 和抽象端口，不依赖页面、路由、浏览器 API 或具体驱动。
- `interface-adapters` 负责输入输出转换和展示模型，不实现企业规则。
- `infrastructure` 可以依赖框架和第三方库，但具体实现必须通过端口或仓储抽象接入。
- `app/pages` 只组合 composable 和组件；具体依赖集中在 infrastructure 的组合根。

## 文件和目录命名

目录表达上下文，文件名表达职责。目录已经声明上下文时，文件名不要机械重复目录语义：

```text
推荐：controllers/document-detail.ts
不推荐：controllers/document-detail-controller.ts

推荐：repositories/document.ts
不推荐：repositories/document-repository.ts
```

文件名可以在表达协议、第三方实现、运行时差异或必要消歧时保留限定词，例如 `persistence/in-memory-document.ts`、`markdown/milkdown.ts`、`http/client.ts`。文件名必须让读者只看名字就能知道它承担什么职责，而不是只能知道它位于哪个目录。

## 代码风格

- TypeScript 和 Vue 使用项目现有 `.prettierrc`：2 空格、单引号、分号、100 列、ES5 trailing comma。
- ESLint 使用 Nuxt 默认配置，TypeScript 通过 `typescript` 和 `vue-tsc` 直接开发依赖启用；不额外引入严格规则。
- 不添加 Markdown 语法检查、拼写检查或与当前目标无关的格式门禁。
- Vue 组件使用 `<script setup lang="ts">`；页面负责组合，组件负责展示，composable 负责 Nuxt 生命周期适配。
- 一个文件只承担一个清晰职责。函数名使用动作，领域类型使用领域名词，避免 `helper`、`utils2` 等无语义名称。
- 跨层数据必须通过 DTO、mapper 或 presenter 转换，不能把领域实体、数据库记录或第三方响应直接暴露给页面。
- 注释使用中文，重点解释为什么存在、依赖边界、替换点和不可违反的约束，不重复显而易见的语法。
- 新增函数和行为先写失败测试；配置文件、纯目录说明和生成文件不强制套用 TDD。

## Markdown 与编辑器

Markdown 编辑和渲染统一规划使用 Milkdown。Milkdown 相关包已经作为直接依赖锁定，但编辑器具体业务流程必须放到 `infrastructure/markdown`，不能把 Milkdown 类型或实例带入 domain/application。

## 文档治理

- `README.md`：项目定位、安装、常用命令和入口链接，面向第一次接触项目的人。
- `AGENTS.md`：稳定、可执行的协作规则和工具链约束。
- `docs/architecture/`：目录、分层和依赖方向的长期说明。
- `docs/guides/`：新人上手和日常开发流程。
- `docs/adr/`：需要长期追溯的架构决策及其取舍。
- `docs/specs/`、`docs/plans/`：阶段性的设计和实施记录，不替代长期架构文档。
- 代码行为、命令、公开流程或架构发生变化时，同一阶段更新对应文档。

## 验证命令

```bash
pnpm dev
pnpm lint              # 自动修复
pnpm run lint:check    # 只检查，不修改文件
pnpm typecheck
pnpm test
pnpm build
```

提交前由 hooks 自动执行轻量检查：

- `pre-commit`：只对暂存的 Vue/JS/TS/CSS 文件运行 lint-staged；不检查 Markdown、拼写、构建。
- `commit-msg`：校验提交格式和 body 规则。
- `pre-push`：运行 `lint:check` 和 `typecheck`，不运行构建。

CI 执行完整的 `lint:check`、`typecheck`、`test` 和 `build`。本项目当前不配置 CD。

## 提交信息

提交 header 必须使用：

```text
type(scope): 中文摘要
```

允许的 type：`feat`、`fix`、`docs`、`style`、`refactor`、`perf`、`test`、`build`、`ci`、`chore`、`revert`。

普通提交只写一行 header，不写 body：

```text
docs(architecture): 补充目录命名说明
```

破坏性变更使用 `!`，并且必须写 body：

```text
feat(api)!: 调整文档接口

BREAKING CHANGE: 文档详情响应字段发生变化
```

每完成一个独立阶段，先完成验证、检查 diff，再创建一个原子提交，提交成功后才能进入下一阶段。未经明确要求，不 push 到默认分支，不 force-push，不重写历史。

<!-- OPENWIKI:START -->

## OpenWiki

This repository has a generated `openwiki/` evidence index. It is optional just-in-time context, not required startup reading.

- Treat source code and tests as authoritative. A brief's unknowns and review items are verification gaps, not automatic requirements.
- Prefer the narrowest quiet validation that proves the changed behavior. Preserve complete failure output.

The scheduled OpenWiki GitHub Actions workflow refreshes the repository wiki. Do not hand-edit generated OpenWiki pages unless explicitly asked; prefer updating source code/docs and letting OpenWiki regenerate.

<!-- OPENWIKI:END -->
