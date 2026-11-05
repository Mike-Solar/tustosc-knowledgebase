# ADR-0001：采用 Clean Architecture 四层结构

- 状态：已接受
- 日期：2026-08-14

## 背景

项目需要长期维护并由多人协作，未来会扩展知识文档、搜索、权限、协作和 Markdown 编辑。Nuxt 页面、第三方编辑器、API 和持久化实现的变化频率不同，直接混写会使测试和替换成本快速增加。

## 决策

采用根级四层结构：

```text
domain → application → interface-adapters → app/server
infrastructure 实现 application ports 和 domain repositories
```

实际依赖方向从外向内，domain 只包含企业规则；application 负责用例；interface-adapters 做转换；infrastructure 放框架、驱动和第三方实现。Nuxt 的约定目录保留在 `app/`、`server/` 和 `shared/`。

## 备选方案

### 全部放进 app/

放置简单，但表现层和业务内核没有清晰边界，未来服务端或脚本复用领域代码时容易耦合。否决。

### 每个业务模块复制完整四层

适合业务边界成熟后的大型模块化系统。当前项目还在骨架阶段，复制完整层级会制造重复和迁移成本。暂不采用。

## 后果

正面结果：

- 业务规则可以脱离 Nuxt 和浏览器测试。
- API、数据库和 Milkdown 可以在 infrastructure 内替换。
- 新人可以通过固定目录找到职责和边界。
- controller、presenter、mapper 和 view model 形成明确的输入输出转换链。

需要承担的成本：

- 一个简单功能可能需要经过多个文件。
- 组合根和端口需要团队理解后才能正确使用。
- 需要持续维护架构文档，避免目录只剩形式。

## 约束

目录上下文不重复出现在文件名中。比如使用 `controllers/document-detail.ts`，而不是 `controllers/document-detail-controller.ts`；只有第三方实现或必要消歧时才增加限定词。
