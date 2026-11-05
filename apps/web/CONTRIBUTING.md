# 贡献指南

感谢您对天津科技大学开放原子开源协会知识库项目的关注！我们欢迎任何形式的贡献。

## 如何贡献

### 1. Fork 项目

- 点击项目右上角的 "Fork" 按钮，将项目复制到您的 GitHub 账号下。

### 2. 克隆项目

```bash
git clone https://github.com/your-username/knowledgebase-ui.git
cd knowledgebase-ui
```

### 3. 创建分支

```bash
git checkout -b feature/your-feature-name
# 或者修复 bug
git checkout -b fix/your-bug-fix
```

### 4. 安装依赖

```bash
pnpm install
```

### 5. 开发

- 启动开发服务器：`pnpm dev`
- 确保代码符合项目规范：
  - 运行 lint：`pnpm lint`
  - 运行类型检查：`pnpm typecheck`
  - 运行测试：`pnpm test`

### 6. 提交代码

- 使用规范的提交信息（见下方提交规范）。
- 确保所有测试通过。
- 推送到您的 Fork：`git push origin feature/your-feature-name`

### 7. 创建 Pull Request

- 在原始仓库中创建 Pull Request。
- 填写清晰的 PR 描述，说明改动内容和原因。
- 等待代码审查。

## 提交规范

我们使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范。提交信息格式如下：

```
type(scope): 中文摘要
```

`type` 使用小写，`scope` 使用小写字母、数字和短横线，摘要必须包含中文。普通提交只写一行 header，不写 body。

只有破坏性提交可以写 body。破坏性提交需要在 header 中使用 `!`，并在 body 中写明 `BREAKING CHANGE:`：

```text
feat(api)!: 调整文档接口

BREAKING CHANGE: 文档详情响应结构发生变化
```

### type 类型

- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `style`: 代码格式调整（不影响逻辑）
- `refactor`: 代码重构
- `perf`: 性能优化
- `test`: 测试相关
- `chore`: 构建工具或辅助工具的变动
- `ci`: CI 配置变动
- `revert`: 回滚提交

### 示例

```
feat(auth): 添加用户登录功能
```

提交时会依次经过 `pre-commit`、`commit-msg` 和 `pre-push`。普通提交不会运行 Markdown 语法检查或拼写检查；CI 才会执行完整的 lint、类型检查、测试和构建。

## 代码风格

- 使用 Nuxt 默认 ESLint 规则和项目现有 Prettier 配置。
- Vue 组件使用 `<script setup lang="ts">`，组件文件名使用 PascalCase。
- 普通 TypeScript 文件使用 kebab-case。
- 目录表达上下文，文件名表达职责；不要写 `controllers/document-detail-controller.ts` 这种重复命名，应写成 `controllers/document-detail.ts`。
- Clean Architecture 的详细边界见 [架构说明](docs/architecture/clean-architecture.md)。

## 报告问题

- 使用 GitHub Issues 报告 bug。
- 提供清晰的复现步骤、期望行为和实际行为。
- 包含相关环境信息（操作系统、Node.js 版本、浏览器版本等）。

## 行为准则

- 尊重每位参与者。
- 保持专业和友善的交流。
- 欢迎不同背景和经验水平的贡献者。

## 联系我们

如有任何问题，请通过以下方式联系：

- GitHub Issues
- 协会邮箱：openatom@tust.edu.cn（示例）

再次感谢您的贡献！
