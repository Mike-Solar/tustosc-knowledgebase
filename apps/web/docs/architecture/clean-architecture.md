# Clean Architecture 上手说明

## 为什么采用四层结构

知识库会长期增加文档、搜索、权限、协作、评论和 Markdown 编辑能力。如果页面直接调用 API、数据库或编辑器实例，功能增长后会把框架细节和业务规则纠缠在一起。

本项目采用四层结构，把变化频率不同的代码隔离：

```text
┌─────────────────────────────────────────────┐
│ app / server                                │  Nuxt 入口与生命周期
├─────────────────────────────────────────────┤
│ interface-adapters                          │  输入输出转换
├─────────────────────────────────────────────┤
│ application                                 │  用例与端口
├─────────────────────────────────────────────┤
│ domain                                      │  企业规则
└─────────────────────────────────────────────┘
        infrastructure 在外圈实现端口和驱动
```

越靠内层越稳定，越不能知道外层技术。外层可以替换内层的实现，但内层不应该为了适配外层而改变规则。

## 四层职责

### domain

领域层回答“知识库业务上是什么”和“哪些规则必须成立”。这里放实体、值对象、领域服务、领域错误和仓储抽象。

- 实体有身份和生命周期，例如知识文档、知识空间。
- 值对象通过值表达概念，例如文档路径、版本号、成员角色。
- 领域服务处理不自然属于单个实体的规则。
- 仓储目录只声明领域需要的能力，不包含数据库、HTTP 或 ORM 细节。

### application

应用层回答“用户要完成什么目标”。一个用例只负责一个业务目标，负责调用领域对象和抽象端口，输出 DTO。

- `dto/` 定义用例边界数据。
- `ports/` 定义输入端口和外部能力抽象。
- `use-cases/` 实现单一业务目标。
- `errors/` 定义应用边界错误。

应用层不读取 `window`、不调用 `useFetch`、不实例化数据库客户端，也不导入 Vue 组件。

### interface-adapters

适配器层把外部输入转换为用例输入，把用例输出转换为页面或接口能使用的形状。

- controller 协调入口和用例。
- mapper 负责字段和格式转换。
- presenter 负责输出边界。
- view-model 是页面稳定依赖的展示数据结构。

页面不应该从用例返回的 DTO 自己拼日期、数量文案或权限状态，这些转换属于 mapper/presenter。

### infrastructure

基础设施层实现外部细节，包括 HTTP、持久化、日志、Milkdown 和 Nuxt 组合根。

组合根是具体依赖的组装点。例如文档详情页可以在 `infrastructure/nuxt/document-detail.ts` 创建仓储、用例和 controller；未来替换 API 时，只改变组合根和仓储实现，不影响 domain/application。

## 典型调用链

当前 `/` 与 `/workspace` 都是最小占位页，尚未接入真实业务链路。未来新增功能时，推荐形成如下调用链：

```text
app/pages/document.vue
  ↓ useDocumentDetail
app/composables/use-document-detail.ts
  ↓ createDocumentDetail
infrastructure/nuxt/document-detail.ts
  ↓ createDocumentDetailController
interface-adapters/controllers/document-detail.ts
  ↓ execute
application/use-cases/get-document-detail.ts
  ↓ read
domain/repositories/document.ts
  ↑ implements
infrastructure/persistence/document.ts
  ↓ DocumentDetailData
interface-adapters/presenters/document-detail.ts
  ↓ DocumentDetailViewModel
app/components/document/*.vue
```

这条链路中的每一层都有明确替换点：

1. 页面可以替换为另一个页面而不改变用例。
2. 内存仓储可以替换为 API 或数据库而不改变 domain/application。
3. view model 可以调整展示格式而不改变领域实体。
4. Milkdown 可以在基础设施中被封装和替换，而不污染内核。

## 新增功能的落点

以“文档详情”作为例子：

1. 在 `domain/entities` 定义文档需要的业务状态和不变量。
2. 在 `domain/repositories` 声明读取文档的领域能力。
3. 在 `application/use-cases` 创建 `get-document-detail.ts`，返回详情 DTO。
4. 在 `interface-adapters/controllers` 创建 `document-detail.ts`，转换路由参数。
5. 在 `interface-adapters/presenters` 和 `view-models` 定义详情页面输出。
6. 在 `infrastructure/persistence` 或 `infrastructure/http` 实现具体数据源。
7. 在 `app/pages` 组合 composable 和展示组件。

注意：目录已经表达了 controller、repository、presenter 等上下文，文件名只写文档详情这个职责，不重复写 `-controller`、`-repository`、`-presenter`。

## 常见错误

### 页面直接调用 API

```ts
// 不要在 app/pages 中直接调用具体客户端
const result = await $fetch('/api/documents');
```

页面应该调用 composable，由 composable 进入 controller 和 use case。

### 用例返回数据库记录

数据库记录属于 infrastructure。用例应该将领域对象转换成 DTO，再由 presenter 转成 view model。

### 把 Milkdown 实例放进领域层

Milkdown 是编辑器驱动，不是知识库业务规则。它只能出现在 `infrastructure/markdown` 或表现层的专用适配器中。

### 文件名重复目录语义

```text
不推荐：interface-adapters/controllers/document-detail-controller.ts
推荐：interface-adapters/controllers/document-detail.ts
```

文件名的目标是让读者知道“它负责什么”，目录已经回答了“它属于哪类边界”。
