# 开发流程

## 安装和启动

```bash
pnpm install
pnpm dev
```

项目要求 Node >=22 和 pnpm >=10。不要在同一个项目中混用 npm/yarn 生成锁文件。

## 页面入口

- `/` 和 `/workspace` 当前都是最小占位页，只保留公共顶部导航。
- 顶部导航使用 `app/components/CardNav/CardNav.vue`：收起状态展示品牌标识和工作台入口，展开状态展示三组导航卡片；卡片文案和链接在 `app/layouts/default.vue` 中声明。
- 真实知识空间、收藏和最近访问功能后续再接入 Clean Architecture 用例，不在当前页面实现。
- 首页和工作台共享公共品牌壳层，页面署名固定为“天津科技大学开放原子开源协会”。

## 日常修改流程

1. 从特性分支开始工作。
2. 阅读 `AGENTS.md`、相关架构文档和现有测试。
3. 先为新行为写一个能表达预期的失败测试。
4. 写最小实现让测试通过，再做小范围重构。
5. 运行 `pnpm run lint:check`、`pnpm typecheck` 和 `pnpm test`。
6. 如果涉及页面，启动 `pnpm dev`，确认页面和窄屏布局，再停止服务。
7. 检查 `git diff --check` 和 `git diff --stat`。
8. 每个独立阶段创建一个原子提交。

## 验证层级

| 时机       | 检查                            | 目的                         |
| ---------- | ------------------------------- | ---------------------------- |
| pre-commit | 暂存代码文件的 lint-staged      | 快速发现格式和 lint 问题     |
| commit-msg | 提交格式校验                    | 保证提交历史可读、可自动处理 |
| pre-push   | lint + typecheck                | 推送前尽快发现跨文件问题     |
| CI         | lint + typecheck + test + build | 对合并请求做全量验证         |

Hook 不运行 Markdown 检查、拼写检查或生产构建。CI 当前也不包含部署步骤。

## 提交信息

普通提交只有一行：

```text
feat(search): 添加文档搜索用例
```

摘要必须包含中文。破坏性提交用 `!` 并写 body：

```text
feat(api)!: 调整文档接口

BREAKING CHANGE: 文档详情响应结构发生变化
```

普通提交不能写 body。`commit-msg` hook 会同时运行 commitlint 和项目自定义校验器。

## 文档更新

- 目录边界或依赖方向变化：更新 `docs/architecture/`。
- 新人操作步骤变化：更新 `docs/guides/`。
- 重要取舍：新增或更新 `docs/adr/`。
- 稳定协作规则变化：更新 `AGENTS.md`。
- 项目入口和常用命令变化：更新 `README.md`。

## 不能做的事

- 不把业务规则直接写进页面和组件。
- 不让 domain/application 依赖 Nuxt、Vue、Milkdown、数据库或 HTTP 客户端。
- 不把数据库记录直接作为页面 props。
- 不为了“顺手整理”修改无关模块。
- 不在没有验证的情况下宣称构建、测试或类型检查通过。
