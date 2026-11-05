# 知识库项目骨架与 Clean Architecture 设计

> **历史设计说明**：本文件记录骨架阶段采用的 Clean Architecture 设计。当前 `app/pages/index.vue` 与 `app/pages/workspace.vue` 已退化为最小占位页，原首页示例链路（`home-content`、`home-page` 及相关文件）已移除，四层目录保持空置以待真实业务接入。下方示例路径保留为历史参考，不代表当前源码状态。

## 目标

为一个长期维护、多人协作的类语雀知识库项目建立可持续的 Nuxt 4 + Vue 3 工程骨架。首轮工作不实现真实业务，只用一个无开发态文案的最小示例证明目录职责、依赖方向、代码组织方式和文档规约。

## 已确定的边界

- 前端框架继续使用 Nuxt 4、Vue 3、TypeScript 和 pnpm。
- Markdown 编辑最终使用 Milkdown；本次只锁定依赖和适配边界，不实现真实编辑业务。
- 采用 Clean Architecture 的四层模型：`domain`、`application`、`interface-adapters`、`infrastructure`。
- Nuxt 的约定式目录继续保留在 `app/`、`server/`、`shared/`，不把 Nuxt 页面和组件塞进领域层。
- 首屏只承担架构示例职责，内容使用静态示例数据，不引入后端、鉴权、持久化或真实知识库流程。
- 不增加 Markdown 语法检查、拼写检查或其他与当前交付目标无关的质量门禁。
- ESLint 和 TypeScript 使用 Nuxt 默认规则；只增加必要的脚本编排和提交信息校验。
- 提交钩子只做轻量检查；CI 做完整的 lint、类型检查、单元测试和生产构建；本次不配置 CD。

## 方案选择

### 方案 A：Nuxt 目录承载全部业务代码

将领域、用例、适配器和基础设施都放到 `app/` 下。优点是 Nuxt 约定最直接，缺点是框架目录和业务边界混在一起，未来服务端、脚本或测试复用领域代码时容易产生耦合。

### 方案 B：Nuxt 约定目录 + 根级 Clean Architecture 层（采用）

`app/` 只承担页面、组件、布局、组合式函数、插件等表现层职责；根级四层目录承担业务内核和外部适配；`server/` 承担 Nuxt 服务端入口；`shared/` 只放应用端和服务端都能安全复用的无业务副作用代码。

这个方案把框架细节推到外圈，同时保留 Nuxt 的自动发现能力，适合长期演进和多人协作。首页通过 `interface-adapters` 调用 `application` 用例，使用 `infrastructure` 提供的内存实现作为示例，页面本身不直接创建领域对象或读取基础设施。

### 方案 C：按业务模块复制完整四层目录

每个业务模块都包含自己的四层目录。它适合业务边界已经稳定的大型系统，但当前项目尚未形成稳定业务模块，首轮采用会制造重复目录和迁移成本，因此暂不选择。

## 目录职责

```text
app/                         # Nuxt 表现层和框架约定目录
├── app.vue                  # 应用根组件
├── assets/                  # 需要构建处理的静态资源
├── components/              # 可复用 Vue UI 组件
├── composables/             # Vue/Nuxt 组合式适配
├── layouts/                 # 页面布局
├── middleware/              # 路由中间件
├── pages/                   # 路由页面
├── plugins/                 # Nuxt 插件注册
├── stores/                  # Pinia 状态适配
├── types/                   # 仅表现层类型
└── utils/                   # 仅表现层无副作用工具

domain/                      # 企业规则，不能依赖外层
├── entities/                # 具有身份和生命周期的实体
├── value-objects/           # 不可变值对象
├── services/                # 跨实体且不属于单一实体的规则
├── repositories/            # 面向领域的仓储抽象
└── errors/                  # 领域错误

application/                 # 用例编排，依赖领域抽象
├── dto/                     # 用例输入输出数据结构
├── ports/                   # 外部能力端口
├── use-cases/               # 单一业务目标的用例
└── errors/                  # 应用层错误

interface-adapters/          # 把外部输入输出转换为内核模型
├── controllers/             # 页面或接口入口的协调器
├── presenters/              # 用例结果到视图模型的转换
├── view-models/             # 面向页面的稳定数据结构
└── mappers/                 # 外部模型和内核模型转换

infrastructure/             # 框架、驱动和第三方实现
├── config/                  # 运行时配置读取
├── http/                    # HTTP 客户端和 API 实现
├── markdown/                # Milkdown 编辑器/Markdown 适配
├── persistence/             # 数据库、缓存和内存实现
├── logging/                 # 日志实现
└── nuxt/                    # Nuxt 特有启动和运行时适配

server/                      # Nuxt 服务端约定目录
├── api/                     # API 路由
├── middleware/              # 服务端中间件
├── plugins/                 # 服务端插件
└── utils/                   # 服务端工具

shared/                      # app/server 都可复用的基础代码
├── constants/               # 跨端常量
├── schemas/                 # 跨端数据校验 schema
├── types/                   # 跨端类型
└── utils/                   # 无副作用通用工具
```

## 依赖方向

依赖只能从外向内指向抽象，不能反向污染内核：

```text
app / server
    ↓
interface-adapters
    ↓
application
    ↓
domain

infrastructure ─────实现─────> application ports / domain repositories
```

- `domain` 不导入 Vue、Nuxt、Pinia、Milkdown、HTTP 客户端或数据库包。
- `application` 可以依赖 `domain`，但不能依赖 Vue 组件、路由、浏览器 API 或具体基础设施。
- `interface-adapters` 负责调用用例和转换数据，不直接实现业务规则。
- `infrastructure` 实现端口和仓储，允许依赖 Nuxt、Milkdown、数据库驱动等外部细节。
- `app/pages` 只负责页面组合和生命周期绑定；业务动作通过控制器、组合式函数或 store 进入用例。

## 首屏示例的最小链路

未来以“文档详情”为例，使用 `GetDocumentDetail` 用例读取一个内存仓储：

```text
app/pages/document.vue
  → interface-adapters/controllers/document-detail.ts
  → application/use-cases/get-document-detail.ts
  → domain/repositories/document.ts
  → infrastructure/persistence/in-memory-document.ts
  → interface-adapters/presenters/document-detail.ts
  → app/pages/document.vue
```

页面只接收 `DocumentDetailViewModel`，不感知实体内部结构，也不直接依赖内存仓储。真实 API 或数据库接入时，只替换基础设施实现和组合入口。

## 文档治理

- `README.md` 面向首次使用项目的人，只描述项目定位、启动方式和常用命令。
- `AGENTS.md` 面向协作代理和开发者，记录稳定的工程约束、目录职责、验证方式和提交规则。
- `docs/architecture/` 记录架构、依赖方向和边界决策。
- `docs/guides/` 记录新人操作指南和常见开发流程。
- `docs/adr/` 记录需要长期追溯的架构决策。
- `docs/specs/` 和 `docs/plans/` 保存阶段性设计和实施计划，不替代项目长期文档。
- 代码注释解释“为什么”和边界，不重复显而易见的语法；公共层接口和跨层转换必须有中文说明。

## 代码风格

- 使用 TypeScript 严格类型表达边界，优先 `type`、不可变数据和窄类型。
- 一个文件只承担一个清晰职责；类和函数命名使用动词或领域名词，避免 `utils2`、`helper` 等无语义名称。
- 目录负责表达上下文，文件名负责表达职责；当目录已经声明了 `controllers`、`repositories`、`presenters`、`ports` 等上下文时，文件名不得机械重复 `controller`、`repository`、`presenter`、`interface` 等目录语义。例如使用 `controllers/document-detail.ts`、`repositories/document.ts`，不使用 `controllers/document-detail-controller.ts`、`repositories/document-repository.ts`。
- 只有在表达协议、第三方实现、运行时差异或必要消歧时，才允许文件名保留额外限定词，例如 `persistence/in-memory-document.ts`、`markdown/milkdown.ts`、`http/client.ts`。
- Vue 组件使用 `<script setup lang="ts">`，页面只负责组合，不放跨页面业务规则。
- 用例保持单一业务目标；端口使用接口或类型定义，具体实现放在基础设施层。
- 跨层数据必须经过 DTO、mapper 或 presenter，禁止把数据库记录直接暴露给页面。
- 注释用中文，优先说明依赖方向、设计原因、替换点和不可违反的约束。
- 不使用 Markdown lint、拼写检查和无明确收益的强制格式规则。

## 工具链规则

- `pre-commit`：只运行 lint-staged，对暂存的代码文件执行 ESLint 和 Prettier；不检查 Markdown，不执行构建。
- `commit-msg`：校验 `type(scope): 中文摘要`，非破坏性提交不允许 body；破坏性提交必须使用 `!` 并提供 `BREAKING CHANGE:` body。
- `pre-push`：运行轻量的全项目 ESLint 和 TypeScript 类型检查，不运行生产构建。
- CI：安装锁定依赖后运行 lint、typecheck、unit test 和 build。
- 不配置 CD；部署由后续阶段另行决策。
