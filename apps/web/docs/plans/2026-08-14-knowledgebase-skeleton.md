# Knowledgebase Skeleton Implementation Plan

> **历史计划说明**：本文件记录项目骨架阶段的实施计划。当前 `app/pages/index.vue` 与 `app/pages/workspace.vue` 已退化为最小占位页，首页示例链路（HomeHero、HomeCategoryList、HomeDocumentList 及相关 domain/application/interface-adapters/infrastructure 文件）已移除，工具链版本约束已放宽为范围声明。下方具体步骤保留为历史参考，不代表当前源码状态。

**Goal:** 建立 Nuxt 4 类语雀知识库的长期协作骨架，用 Clean Architecture 四层和一个最小首页示例明确代码边界、命名规则、工具链和文档治理。

**Architecture:** `app/`、`server/` 和 `shared/` 保留 Nuxt 约定式目录；`domain/`、`application/`、`interface-adapters/`、`infrastructure/` 位于根目录，分别承担领域规则、用例、转换适配和外部实现。页面通过组合根创建依赖，经过控制器、用例、领域仓储和 presenter 返回页面 view model；页面不直接读取基础设施。

**Tech Stack:** Nuxt ^4.4.8, Vue ^3.5.38, TypeScript ^6.0.3, Milkdown ^7.22.1, Vitest ^4.1.10, pnpm >=10, Node >=22.

---

## 文件地图

### 将创建或修改的核心文件

- Modify: `package.json` — 增加 Milkdown、Vitest、TypeScript、vue-tsc、检查脚本、pnpm 版本声明和只针对代码文件的 lint-staged。
- Modify: `pnpm-lock.yaml` — 由 pnpm 更新锁定依赖。
- Modify: `nuxt.config.ts` — 注册应用样式和 Clean Architecture 根目录别名。
- Modify: `app/app.vue` — 使用 Nuxt 页面和默认布局作为应用根。
- Create: `app/layouts/default.vue` — 页面公共壳层。
- Create: `app/pages/index.vue` — 首页入口，当前为最小占位页。
- Create: `app/composables/use-home-page.ts` — Nuxt 数据加载适配，不实现业务规则（当前已随示例链路移除）。
- Create: `app/components/home/HomeHero.vue` — 首页主视觉展示（当前已移除，首页为占位页）。
- Create: `app/components/home/HomeCategoryList.vue` — 分类入口展示（当前已移除）。
- Create: `app/components/home/HomeDocumentList.vue` — 最近文档展示（当前已移除）。
- Create: `app/assets/styles/main.css` — 全局设计变量和基础样式。
- Create: `domain/entities/home-content.ts` — 首页领域实体及构造函数。
- Create: `domain/repositories/home-content.ts` — 领域仓储抽象，只表达读取能力。
- Create: `application/dto/home-page.ts` — 用例边界 DTO。
- Create: `application/ports/home-page.ts` — 首页输入端口。
- Create: `application/use-cases/get-home-page-content.ts` — 获取首页内容的唯一用例。
- Create: `interface-adapters/controllers/home-page.ts` — 组合输入、执行用例并交给 presenter。
- Create: `interface-adapters/mappers/home-page.ts` — DTO 到页面模型的转换。
- Create: `interface-adapters/presenters/home-page.ts` — 输出页面 view model。
- Create: `interface-adapters/view-models/home-page.ts` — 页面稳定数据结构。
- Create: `infrastructure/persistence/in-memory-home-content.ts` — 当前阶段的内存仓储实现。
- Create: `infrastructure/nuxt/home-page.ts` — Nuxt 组合根，集中组装具体实现。
- Create: `tests/unit/get-home-page-content.test.ts` — 真实仓储和用例的单元测试。
- Create: `tests/unit/present-home-page.test.ts` — presenter 映射行为测试。
- Create: `tests/unit/validate-commit-message.test.ts` — 提交信息校验器行为测试。

### 将创建的治理与说明文件

- Create: `AGENTS.md` — 项目协作约束、目录职责、命名规则和验证命令。
- Modify: `CONTRIBUTING.md` — 同步新的提交信息和代码组织规则。
- Create: `docs/architecture/clean-architecture.md` — 面向新人的 Clean Architecture 说明。
- Create: `docs/architecture/directory-map.md` — 目录职责和依赖方向速查。
- Create: `docs/guides/development-workflow.md` — 日常开发、验证和提交流程。
- Create: `docs/adr/0001-clean-architecture.md` — 记录采用四层架构的决策。
- Create: `.husky/pre-commit` — 暂存代码文件的轻量检查。
- Create: `.husky/commit-msg` — 提交信息格式校验入口。
- Create: `.husky/pre-push` — 推送前的轻量 lint 和类型检查。
- Create: `scripts/validate-commit-message.mjs` — `type(scope): 中文摘要` 及 body 规则校验器。
- Create: `.github/workflows/ci.yml` — CI 中的全量 lint、typecheck、test、build。
- Modify: `commitlint.config.js` — 保留 Conventional Commits 基础解析规则。
- Modify: `eslint.config.mjs` — 使用 Nuxt 默认 ESLint 配置，不额外启用 stylistic/tooling 规则。

为了让空的 Nuxt 约定目录也能被 Git 跟踪，空目录使用 `README.md` 说明其职责；这些 `README.md` 是目录入口的特例，不用于代替业务代码。不会创建无意义的业务占位实现。

## 阶段与原子提交

每个阶段必须在验证通过后提交，提交成功后才能开始下一阶段。提交正文遵守项目规则：普通提交只有一行 header；破坏性变更使用 `!` 并提供 `BREAKING CHANGE:` body。

### Task 1: 依赖和目录骨架

**Files:** `package.json`, `pnpm-lock.yaml`, `nuxt.config.ts`, `app/`, `domain/`, `application/`, `interface-adapters/`, `infrastructure/`, `server/`, `shared/`, `tests/`

- [ ] **Step 1: 安装已核对的编辑器和测试依赖**

运行：

```bash
pnpm add @milkdown/core@^7.22.1 @milkdown/vue@^7.22.1 @milkdown/preset-commonmark@^7.22.1 @milkdown/preset-gfm@^7.22.1 @milkdown/plugin-history@^7.22.1
pnpm add -D vitest@^4.1.10
```

预期：`package.json` 中出现直接依赖，`pnpm-lock.yaml` 更新，退出码为 `0`。

- [ ] **Step 2: 增加可复现的包管理器声明和基础脚本**

在 `package.json` 中加入 `typescript@^6.0.3`、`vue-tsc@^3.3.9`，让 Nuxt 默认 ESLint 和 `nuxt typecheck` 能直接识别根级 TypeScript 文件；同时加入脚本（当前版本约束已改为 `engines` 范围声明，不再锁定具体 patch 版本）：

```json
{
  "engines": {
    "node": ">=22.0.0",
    "pnpm": ">=10.0.0"
  },
  "scripts": {
    "lint:check": "eslint .",
    "test": "vitest run"
  }
}
```

保留现有 `lint` 的自动修复用途；`lint:check` 专供 pre-push 和 CI，避免检查脚本隐式改文件。

- [ ] **Step 3: 配置 Clean Architecture 根目录别名**

在 `nuxt.config.ts` 中使用 `node:url` 的 `fileURLToPath`，为 `@domain`、`@application`、`@interface-adapters`、`@infrastructure` 提供绝对路径别名，并把 `~/assets/styles/main.css` 注册到 `css`。别名只解决路径，不改变依赖方向。保留仓库现有 `.prettierrc`，ESLint 使用 Nuxt 默认配置。

- [ ] **Step 4: 创建 Nuxt 约定目录和四层目录**

创建 `app/{assets/styles,components/home,composables,layouts,middleware,pages,plugins,stores,types,utils}`、`server/{api,middleware,plugins,utils}`、`shared/{constants,schemas,types,utils}` 以及四层目录的完整子目录。空目录添加职责说明 `README.md`，不得写入业务逻辑。

- [ ] **Step 5: 检查目录骨架和依赖声明**

运行：

```bash
rg --files app server shared domain application interface-adapters infrastructure tests
pnpm exec nuxt prepare
git diff --check
```

预期：四层目录和 Nuxt 约定目录均可发现，Nuxt 类型目录生成成功，差异无空白错误。

- [ ] **Step 6: 原子提交**

```bash
git add package.json pnpm-lock.yaml nuxt.config.ts app server shared domain application interface-adapters infrastructure tests
git diff --cached --check
git commit -m "chore(scaffold): 搭建项目目录骨架"
```

预期：提交成功且工作区只剩本地忽略的 `.learnings/`。

### Task 2: 领域、用例和适配器示例（TDD）

**Files:** `tests/unit/get-home-page-content.test.ts`, `tests/unit/present-home-page.test.ts`, `domain/entities/home-content.ts`, `domain/repositories/home-content.ts`, `application/dto/home-page.ts`, `application/ports/home-page.ts`, `application/use-cases/get-home-page-content.ts`, `interface-adapters/controllers/home-page.ts`, `interface-adapters/mappers/home-page.ts`, `interface-adapters/presenters/home-page.ts`, `interface-adapters/view-models/home-page.ts`, `infrastructure/persistence/in-memory-home-content.ts`, `infrastructure/nuxt/home-page.ts`

- [ ] **Step 1: 写第一个失败测试**

在 `tests/unit/get-home-page-content.test.ts` 中先写真实依赖测试：实例化内存内容源和 `GetHomePageContent`，执行 `execute()`，断言返回中文标题、三个分类和最近文档。测试只通过公开用例接口验证行为，不测试私有实现。

- [ ] **Step 2: 运行并确认 RED**

运行：`pnpm test -- tests/unit/get-home-page-content.test.ts`

预期：测试因为被导入的领域和用例文件不存在而失败；如果出现配置错误，先修复测试环境直到得到“缺少实现”的失败。

- [ ] **Step 3: 写最小领域和应用实现**

实现不可变的 `HomeContent` 实体、`HomeContentReader` 仓储抽象、`HomePageData` DTO、`HomePageInputPort` 输入端口、`GetHomePageContent` 用例和 `InMemoryHomeContent`。内存数据只作为示例，不在页面文件中出现。

- [ ] **Step 4: 运行并确认 GREEN**

运行：`pnpm test -- tests/unit/get-home-page-content.test.ts`

预期：该测试通过，输出无错误和警告。

- [ ] **Step 5: 写 presenter 的失败测试并确认 RED**

在 `tests/unit/present-home-page.test.ts` 中构造一个 `HomePageData`，断言 `presentHomePage` 返回页面需要的 `hero`、`documentTotal`、`categories` 和 `documents` 结构；先运行测试，确认 presenter 尚不存在或行为不正确。

- [ ] **Step 6: 实现 controller、mapper、presenter 和组合根**

让 mapper 负责 DTO 到 view model 的字段转换，presenter 只负责调用 mapper，controller 只负责执行输入端口并把结果交给 presenter；`infrastructure/nuxt/home-page.ts` 集中创建内存仓储、用例和 controller。禁止页面直接 `new InMemoryHomeContent()`。

- [ ] **Step 7: 运行单元测试并做依赖方向检查**

运行：

```bash
pnpm test
rg -n "from ['\"](vue|nuxt|@milkdown|pinia)" domain application
```

预期：所有单元测试通过；领域和应用层没有 Vue、Nuxt、Milkdown、Pinia 导入。

- [ ] **Step 8: 原子提交**

```bash
git add domain application interface-adapters infrastructure tests
git diff --cached --check
git commit -m "feat(architecture): 添加分层首页示例"
```

### Task 3: 首屏表现层示例

**Files:** `app/app.vue`, `app/layouts/default.vue`, `app/pages/index.vue`, `app/composables/use-home-page.ts`（当前已移除）, `app/components/home/HomeHero.vue`（当前已移除）, `app/components/home/HomeCategoryList.vue`（当前已移除）, `app/components/home/HomeDocumentList.vue`（当前已移除）, `app/assets/styles/main.css`

- [ ] **Step 1: 创建应用根、布局和 composable**

`app/app.vue` 渲染 `NuxtRouteAnnouncer`、`NuxtLayout` 和 `NuxtPage`；`use-home-page.ts` 调用组合根 controller 并通过 `useAsyncData` 暴露 view model；页面不创建仓储、不调用 HTTP、不定义领域规则。

- [ ] **Step 2: 创建最小展示组件和首页**

首页只传递 `HomePageViewModel` 的明确字段给三个展示组件。可见内容使用“开源知识库”“让知识持续生长”等正式产品文案，不出现“开发中”“示例”“TODO”“架构层”等开发态文字。

- [ ] **Step 3: 添加低耦合视觉样式**

在 `main.css` 中定义颜色、间距、字体和响应式断点；组件只保留结构和局部状态，不在组件内复制数据或业务判断。

- [ ] **Step 4: 启动并验证首屏**

运行：`pnpm dev --host 127.0.0.1`，在浏览器打开首页，确认首屏能加载、窄屏不发生横向溢出、控制台无运行时错误。随后停止开发服务器。

- [ ] **Step 5: 运行类型检查和构建**

运行：`pnpm typecheck`、`pnpm run build`。

预期：两条命令都以退出码 `0` 完成。

- [ ] **Step 6: 原子提交**

```bash
git add app
git diff --cached --check
git commit -m "feat(home): 完成首页分层示例"
```

### Task 4: 项目规约、文档治理和提交钩子

**Files:** `AGENTS.md`, `CONTRIBUTING.md`, `docs/architecture/clean-architecture.md`, `docs/architecture/directory-map.md`, `docs/guides/development-workflow.md`, `docs/adr/0001-clean-architecture.md`, `.husky/pre-commit`, `.husky/commit-msg`, `.husky/pre-push`, `scripts/validate-commit-message.mjs`, `commitlint.config.js`, `package.json`

- [ ] **Step 1: 更新 package scripts 和 lint-staged**

将 lint-staged 限定为 Vue、JS、TS 和 CSS 文件，只运行 `eslint --fix`、`prettier --write`；删除 Markdown、JSON、YAML 文件的 hook 任务。移除 ESLint 配置中额外的 stylistic/tooling 开关，保留 Nuxt 默认规则。增加：

```json
{
  "scripts": {
    "check:pre-push": "pnpm run lint:check && pnpm run typecheck"
  }
}
```

- [ ] **Step 2: 实现提交信息校验器**

先在 `tests/unit/validate-commit-message.test.ts` 写测试，再实现 `validate-commit-message.mjs`。脚本导出可测试的 `validateCommitMessage`，并在 CLI 模式读取传入文件。它必须接受 `feat(auth): 添加登录`，拒绝没有中文摘要、缺少 scope、scope 含大写或非破坏性提交带 body 的消息；`feat(api)!: 调整接口` 只有在 body 含 `BREAKING CHANGE: ...` 时接受。

- [ ] **Step 3: 配置三个 hook**

`.husky/pre-commit` 只执行 `pnpm exec lint-staged`；`.husky/commit-msg` 依次执行 `pnpm exec commitlint --edit "$1"` 和自定义校验；`.husky/pre-push` 执行 `pnpm run check:pre-push`。不写入旧版 Husky 已废弃的 shim 两行。

- [ ] **Step 4: 写项目规约和新人文档**

`AGENTS.md` 写入目录职责、依赖方向、文件名职责命名规则、中文注释要求、默认 lint/typecheck、无 Markdown/拼写门禁、hooks/CI 规则和阶段性原子提交要求。`clean-architecture.md` 用首页链路说明实体、用例、端口、适配器、基础设施和组合根；`directory-map.md` 给出完整树；`development-workflow.md` 给出从创建分支到提交的命令；ADR 记录采用该架构的原因和被否决的方案。

- [ ] **Step 5: 同步贡献指南**

删除现有“普通提交带 body/footer”的示例，改为普通提交一行 header；仅破坏性提交使用 `!` 和 `BREAKING CHANGE:` body。补充目录命名规则、轻量 hook 和 CI 的区别。

- [ ] **Step 6: 验证 hook 规则**

运行：

```bash
node --input-type=module -e "import { validateCommitMessage } from './scripts/validate-commit-message.mjs'; validateCommitMessage('feat(auth): 添加登录'); try { validateCommitMessage('feat(auth): add login'); process.exit(1); } catch {}"
node --input-type=module -e "import { validateCommitMessage } from './scripts/validate-commit-message.mjs'; validateCommitMessage('feat(api)!: 调整接口\n\nBREAKING CHANGE: 返回结构改变');"
pnpm exec lint-staged --help
```

预期：合法消息命令退出 `0`，非法消息被捕获，lint-staged 能正常加载配置。

- [ ] **Step 7: 原子提交**

```bash
git add AGENTS.md CONTRIBUTING.md docs .husky scripts commitlint.config.js package.json
git diff --cached --check
git commit -m "chore(workflow): 配置协作规约与提交钩子"
```

### Task 5: CI 全量检查

**Files:** `.github/workflows/ci.yml`, `docs/guides/development-workflow.md`

- [ ] **Step 1: 创建 CI workflow**

使用 `pnpm/action-setup@v4` 的 `10.33.2` 和 `actions/setup-node@v4` 的 Node `24`，开启 pnpm cache，执行 `pnpm install --frozen-lockfile`。workflow 依次执行 `pnpm run lint:check`、`pnpm run typecheck`、`pnpm test`、`pnpm run build`。

- [ ] **Step 2: 保证 CI 不配置 CD**

workflow 只响应 push 和 pull_request，不包含部署 action、环境密钥、production environment 或发布步骤。

- [ ] **Step 3: 验证 CI 配置和本地全量检查**

运行：

```bash
git diff --check
pnpm run lint:check
pnpm run typecheck
pnpm test
pnpm run build
```

预期：所有命令退出码为 `0`；如果环境缺失浏览器或网络依赖，记录确切失败原因，不把未验证内容报告为通过。

- [ ] **Step 4: 原子提交**

```bash
git add .github/workflows/ci.yml docs/guides/development-workflow.md
git diff --cached --check
git commit -m "ci(checks): 配置全量持续集成"
```

## 最终验收清单

- [ ] `rg --files` 能看到 Nuxt 约定目录和 Clean Architecture 四层目录。
- [ ] `domain`、`application` 没有导入 Vue、Nuxt、Pinia 或 Milkdown。
- [ ] 页面只通过 composable/controller 获得 view model，不直接创建基础设施。
- [ ] 文件名表达职责，不机械重复目录上下文；特殊限定词有明确理由。
- [ ] README 使用 AGPL-3.0，贡献指南和 AGENTS 规则一致。
- [ ] Milkdown 直接依赖锁定在 `7.22.1` 系列，Vitest 直接开发依赖锁定在 `4.1.10` 系列。
- [ ] pre-commit、commit-msg、pre-push 都可执行，普通提交 body 被拒绝，破坏性 body 被接受。
- [ ] CI 包含 lint、typecheck、test、build，不包含 CD。
- [ ] `pnpm run lint:check`、`pnpm run typecheck`、`pnpm test`、`pnpm run build` 均已获得新鲜的退出码 `0`。
