# 目录职责速查

```text
app/
├── app.vue                         # Nuxt 应用根
├── assets/styles/                  # 构建处理的全局样式
├── components/                    # Vue 展示组件
│   └── home/                      # 首页展示组件
├── composables/                   # Nuxt/Vue 生命周期适配
├── layouts/                       # 页面公共壳层
├── middleware/                    # 路由中间件
├── pages/                         # 路由入口
├── plugins/                       # Nuxt 客户端插件
├── stores/                        # Pinia 表现层适配
├── types/                         # 表现层类型
└── utils/                         # 表现层无副作用工具

domain/
├── entities/                      # 实体
├── value-objects/                 # 值对象
├── services/                      # 领域服务
├── repositories/                 # 仓储抽象
└── errors/                        # 领域错误

application/
├── dto/                           # 用例边界 DTO
├── ports/                         # 输入端口和外部能力端口
├── use-cases/                     # 单一业务目标
└── errors/                        # 应用层错误

interface-adapters/
├── controllers/                   # 输入协调
├── presenters/                    # 输出边界
├── view-models/                   # 页面稳定数据结构
└── mappers/                       # 跨层字段转换

infrastructure/
├── config/                        # 运行时配置适配
├── http/                          # HTTP 驱动
├── markdown/                      # Milkdown 驱动
├── persistence/                   # 持久化和内存实现
├── logging/                       # 日志驱动
└── nuxt/                          # Nuxt 组合根

server/
├── api/                           # 服务端 API 入口
├── middleware/                    # 服务端中间件
├── plugins/                       # 服务端插件
└── utils/                         # 服务端工具

shared/
├── constants/                     # 跨端常量
├── schemas/                       # 跨端校验 schema
├── types/                         # 跨端类型
└── utils/                         # 跨端无副作用工具

tests/
└── unit/                          # 领域、用例、适配器和脚本单元测试

docs/
├── architecture/                 # 架构说明和目录地图
├── guides/                       # 新人和日常工作指南
├── adr/                          # 架构决策记录
├── plans/                        # 阶段性实施计划
└── specs/                        # 阶段性设计文档
```

## 文件命名规则

目录已经提供上下文时，文件名只表达职责：

```text
controllers/document-detail.ts
repositories/document.ts
presenters/document-detail.ts
use-cases/get-document-detail.ts
```

不要把目录名机械拼回文件名。只有实现差异、协议或第三方技术需要消歧时，才使用额外限定词：

```text
persistence/in-memory-document.ts
markdown/milkdown.ts
http/client.ts
```
