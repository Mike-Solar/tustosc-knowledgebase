# API 设计规范

本文定义知识库的 HTTP API 约定，并给出接口清单。接口设计目标：**简洁、可预测、对齐语雀的直觉**——资源嵌套表达归属关系，写操作返回明确状态码，错误响应机器可读。

> **实现状态约定**：每个接口标题后标注「✅ 已实现」或「🚧 规划中」。当前仅健康检查接口已实现；其余接口为设计稿，实现时以本文为契约，如有变更必须同步修改本文。统一错误模型与路由骨架已实现（见 [`apps/server/src/error.rs`](../apps/server/src/error.rs)、[`apps/server/src/routes/mod.rs`](../apps/server/src/routes/mod.rs)）。

## 目录

- [基础约定](#基础约定)
- [错误响应](#错误响应)
- [分页、时间与 ID](#分页时间与-id)
- [鉴权](#鉴权)
- [已实现接口](#已实现接口)
- [规划接口：认证 Auth](#规划接口认证-auth)
- [规划接口：用户 Users](#规划接口用户-users)
- [规划接口：空间 Spaces](#规划接口空间-spaces)
- [规划接口：文档 Documents](#规划接口文档-documents)
- [规划接口：搜索 Search](#规划接口搜索-search)
- [状态码与错误码速查](#状态码与错误码速查)
- [接口实现顺序建议](#接口实现顺序建议)

## 基础约定

| 项 | 约定 |
| --- | --- |
| Base URL | `https://<host>/api/v1`；健康检查不带前缀：`/healthz`、`/readyz` |
| 传输 | 仅 HTTPS（生产）；本地允许 HTTP |
| 编码 | 请求/响应均为 `application/json; charset=utf-8` |
| 字段命名 | `snake_case`（与 Rust / 数据库一致，前端用 TS 类型描述） |
| 请求体 | 除 `GET` / `DELETE` 外必须有 JSON body；`DELETE` 需要参数时用查询串 |
| 未知字段 | 默认忽略；安全敏感接口（注册、重置密码）建议拒绝未知字段 |
| 幂等性 | `GET` / `PUT` / `DELETE` 幂等；`POST` 默认非幂等（创建时可用 `Idempotency-Key`，P2） |
| 版本化 | URL 路径版本（`/api/v1`）；破坏性变更升 `v2`，不做字段级版本 |
| 请求 ID | 客户端可传 `x-request-id`；服务端总是回传该头（缺失时自动生成） |
| 空值 | 可选字段未设置时返回 `null`，不省略字段（前端类型稳定） |

### 常用请求头

| Header | 说明 |
| --- | --- |
| `Authorization: Bearer <access_token>` | 需要登录的接口必填 |
| `Content-Type: application/json` | 有请求体时必填 |
| `If-Match: "<revision>"` | 文档保存的乐观锁（与 body 的 `base_rev` 二选一，见文档接口） |
| `x-request-id` | 可选，客户端自带链路 ID |
| `Accept-Language` | 可选，预留国际化（当前仅中文文案） |

### 统一响应结构

单资源直接返回对象（不再套一层 `data`）：

```json
{
  "id": "1839283746501234567",
  "title": "新人指南",
  "revision": 7
}
```

列表返回：

```json
{
  "items": [],
  "page": 1,
  "page_size": 20,
  "total": 0
}
```

删除类操作返回 `204 No Content`，无 body。

## 错误响应

所有非 2xx 响应 body 固定为：

```json
{
  "error": {
    "code": "not_found",
    "message": "文档不存在"
  }
}
```

- `code`：**稳定**的机器可读错误码，前端/脚本据此分支，不得随文案调整。
- `message`：面向用户的中文提示，可以调整；`500` 固定为「服务器内部错误」，不泄露内部细节。
- 校验失败时可在 `error` 下增加 `details` 数组（规划中）：

  ```json
  {
    "error": {
      "code": "bad_request",
      "message": "请求参数不合法",
      "details": [
        { "field": "email", "reason": "邮箱格式不正确" }
      ]
    }
  }
  ```

已实现的错误码（`apps/server/src/error.rs`）：

| HTTP | `code` | 触发场景 |
| --- | --- | --- |
| 400 | `bad_request` | 参数缺失、格式非法、业务前置条件不满足 |
| 401 | `unauthorized` | 未携带令牌、令牌过期/签名错误/已被吊销 |
| 403 | `forbidden` | 已登录但角色不足、资源不属于当前用户 |
| 404 | `not_found` | 资源不存在、路由不存在（统一 JSON 404） |
| 409 | `conflict` | 邮箱/标题重名、文档乐观锁版本冲突 |
| 500 | `internal_error` | 数据库异常、序列化异常等 |

规划中的错误码（**尚未实现**，实现时同步更新本表）：

| HTTP | `code` | 触发场景 |
| --- | --- | --- |
| 400 | `validation_failed` | 字段级校验失败，配合 `details` |
| 401 | `totp_required` | 密码正确但需要 TOTP 验证 |
| 410 | `challenge_expired` | 登录挑战过期或已被使用 |
| 422 | `unprocessable_entity` | 语义正确但无法处理（如回滚到不存在的版本） |
| 429 | `rate_limited` | 触发限流，响应带 `Retry-After` |
| 503 | `service_unavailable` | 依赖不可用（如数据库只读） |

## 分页、时间与 ID

### 分页

| 参数 | 类型 | 默认 | 约束 | 说明 |
| --- | --- | --- | --- | --- |
| `page` | integer | `1` | ≥ 1 | 页码，从 1 开始 |
| `page_size` | integer | `20` | 1～100 | 每页条数 |

响应固定包含 `items`、`page`、`page_size`、`total`。`total` 为满足条件的总条数；深分页性能不足时再引入游标（P2，接口不变，增加 `next_cursor`）。

### 时间

- 传输格式：**RFC 3339 / ISO 8601 带时区**，例如 `2026-11-05T12:34:56Z`、`2026-11-05T20:34:56+08:00`。
- 服务端统一用 UTC 存储；响应中时区偏移可保留客户端所在时区（v1 统一输出 `Z`）。
- 字段命名：`created_at`、`updated_at`、`saved_at`、`deleted_at`。

### ID

- 所有资源 ID 为**雪花 ID（64 位整数）**，在 JSON 中**序列化为字符串**（`"1839283746501234567"`），避免 JavaScript `Number` 超过 `2^53` 丢精度。
- 路径参数中的 ID 同样按字符串解析，服务端校验为合法整数。
- 不使用自增 ID，不暴露资源总量。

## 鉴权

| 项 | 约定 |
| --- | --- |
| 访问令牌 | JWT，`Authorization: Bearer <access_token>`，有效期建议 15 分钟 |
| 刷新令牌 | 不透明随机串，放在请求体（**不放 URL**，避免日志泄漏），有效期建议 30 天 |
| 令牌失效 | 用户改密码 / 被禁用 / 登出全部设备 → `token_version + 1` 或吊销全部 refresh token |
| 权限 | 空间级 RBAC：`OWNER` / `ADMIN` / `EDITOR` / `VIEWER`，判定在服务端 |
| 失效响应 | 401 `unauthorized`；前端应尝试一次静默刷新，失败再跳登录 |

令牌响应结构（登录/注册/刷新共用，规划中）：

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "9f2c8a7e1b4d...",
  "token_type": "Bearer",
  "expires_in": 900
}
```

## 已实现接口

### `GET /healthz` ✅ 已实现

存活探针，不访问数据库。用于容器 liveness / 网关心跳。

**响应 200**

```json
{
  "status": "ok",
  "service": "knowledgebase-server",
  "version": "0.1.0"
}
```

`version` 来自 `CARGO_PKG_VERSION`，随发版变化。

### `GET /readyz` ✅ 已实现

就绪探针，实际执行 `db.ping()`。

**响应 200（数据库可用）**

```json
{
  "status": "ready",
  "database": "up"
}
```

**响应 503（数据库不可用）**

```json
{
  "status": "degraded",
  "database": "down"
}
```

> 注意：这两个接口的响应结构与其他接口不同（没有 `error` 包装），因为它们面向基础设施探针而非业务客户端。

### 未匹配路由 ✅ 已实现

任意未定义路径返回统一 JSON 404，而不是纯文本：

```json
{
  "error": {
    "code": "not_found",
    "message": "接口不存在"
  }
}
```

## 规划接口：认证 Auth

前缀：`/api/v1/auth`。以下均为 **🚧 规划中**。

### `POST /auth/register` 🚧 规划中

**请求**

```json
{
  "email": "alice@example.com",
  "password": "S3cure-passw0rd!",
  "name": "Alice"
}
```

| 字段 | 类型 | 必填 | 规则 |
| --- | --- | --- | --- |
| `email` | string | 是 | 合法邮箱，最长 255，统一小写存储 |
| `password` | string | 是 | 8～72 字节（argon2 限制），至少包含字母和数字 |
| `name` | string | 是 | 1～32 字符 |

**响应 201**

```json
{
  "user": {
    "id": "1839283746501234567",
    "email": "alice@example.com",
    "name": "Alice",
    "avatar_url": "",
    "totp_enabled": false,
    "created_at": "2026-11-05T12:34:56Z"
  },
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "9f2c8a7e1b4d...",
  "token_type": "Bearer",
  "expires_in": 900
}
```

**错误**：`400 bad_request`（参数非法）、`409 conflict`（邮箱已注册）。

### `POST /auth/login` 🚧 规划中

**请求**

```json
{
  "email": "alice@example.com",
  "password": "S3cure-passw0rd!"
}
```

**响应 200（未启用 TOTP，直接登录成功）**

结构同注册成功的令牌响应。

**响应 202（已启用 TOTP，需要二次验证）**

```json
{
  "challenge_id": "1839283746509999999",
  "expires_at": "2026-11-05T12:39:56Z"
}
```

此时**不签发任何令牌**。客户端拿 `challenge_id` 调用 `/auth/login/totp`。

**错误**：`400 bad_request`、`401 unauthorized`（邮箱或密码错误，不区分提示）、`403 forbidden`（账号被禁用）。

### `POST /auth/login/totp` 🚧 规划中

**请求**

```json
{
  "challenge_id": "1839283746509999999",
  "code": "123456"
}
```

| 字段 | 类型 | 规则 |
| --- | --- | --- |
| `challenge_id` | string | 上一步返回的挑战 ID |
| `code` | string | 6 位数字 TOTP 验证码 |

**响应 200**：令牌响应（同登录成功）。

**错误**：`400 bad_request`（验证码格式错误）、`401 unauthorized`（验证码错误）、`410 challenge_expired`（挑战过期或已使用）。

### `POST /auth/refresh` 🚧 规划中

用 refresh token 换新的一对令牌（旧 refresh token 立即失效）。

**请求**

```json
{
  "refresh_token": "9f2c8a7e1b4d..."
}
```

**响应 200**：令牌响应（新的 `access_token` 与 `refresh_token`）。

**错误**：`401 unauthorized`（不存在、已吊销、已过期、重放）。

### `POST /auth/logout` 🚧 规划中

吊销当前 refresh token。

**请求**

```json
{
  "refresh_token": "9f2c8a7e1b4d..."
}
```

**响应**：`204 No Content`。始终返回 204（幂等），即使令牌已失效。

### `POST /auth/password/reset-request` 🚧 规划中（P1）

**请求**

```json
{
  "email": "alice@example.com"
}
```

**响应**：`202 Accepted`，统一返回（不暴露邮箱是否存在）：

```json
{
  "message": "如果该邮箱已注册，重置邮件将在几分钟内送达"
}
```

### `POST /auth/password/reset` 🚧 规划中（P1）

**请求**

```json
{
  "email": "alice@example.com",
  "reset_token": "a1b2c3",
  "new_password": "N3w-passw0rd!"
}
```

**响应**：`204 No Content`。成功后 `token_version + 1` 并吊销全部 refresh token。

**错误**：`400 bad_request`、`401 unauthorized`（令牌无效/过期）。

## 规划接口：用户 Users

前缀：`/api/v1/users`。以下均为 **🚧 规划中**。

### `GET /users/me` 🚧 规划中

需登录。返回当前用户资料（不含密码散列、TOTP 密钥）。

**响应 200**

```json
{
  "id": "1839283746501234567",
  "email": "alice@example.com",
  "name": "Alice",
  "avatar_url": "",
  "totp_enabled": false,
  "created_at": "2026-11-05T12:34:56Z",
  "updated_at": "2026-11-05T12:34:56Z"
}
```

### `PATCH /users/me` 🚧 规划中

**请求**（字段均可选，至少传一个）

```json
{
  "name": "Alice Chen",
  "avatar_url": "https://example.com/avatar.png"
}
```

**响应 200**：更新后的用户对象。

### `POST /users/me/password` 🚧 规划中

**请求**

```json
{
  "old_password": "S3cure-passw0rd!",
  "new_password": "N3w-passw0rd!"
}
```

**响应**：`204 No Content`。成功后旧令牌全部失效，客户端需重新登录。

### TOTP 管理 🚧 规划中（P1）

| 方法 | 路径 | 说明 | 成功响应 |
| --- | --- | --- | --- |
| `POST` | `/users/me/totp/setup` | 生成密钥与 otpauth URL（不落库明文） | `200 {secret, otpauth_url}` |
| `POST` | `/users/me/totp/enable` | 提交一次验证码完成绑定 | `200 {totp_enabled: true}` |
| `POST` | `/users/me/totp/disable` | 校验密码后关闭两步验证 | `204` |
| `POST` | `/users/me/totp/recovery-codes` | 生成或重置恢复码 | `200 {codes: [...]}` |

### 管理员接口 🚧 规划中（P2）

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` | `/admin/users?q=&page=&page_size=` | 用户列表 |
| `PATCH` | `/admin/users/{user_id}` | 禁用/启用用户，修改角色 |

## 规划接口：空间 Spaces

前缀：`/api/v1/spaces`。除 `GET /spaces` 与 `POST /spaces` 外，均要求当前用户在对应空间内有足够角色。以下均为 **🚧 规划中**。

### 空间对象

```json
{
  "id": "1839283746501111111",
  "name": "ACM 集训队",
  "description": "算法竞赛训练资料",
  "owner_id": "1839283746501234567",
  "role": "OWNER",
  "member_count": 12,
  "document_count": 87,
  "created_at": "2026-09-01T08:00:00Z",
  "updated_at": "2026-11-01T10:20:00Z"
}
```

`role` 为**当前用户**在该空间的角色；`member_count` / `document_count` 为展示用汇总，不保证强一致。

### `GET /spaces` 🚧 规划中

返回当前用户加入的空间（按 `updated_at` 倒序）。

**查询参数**：`page`、`page_size`。

**响应 200**

```json
{
  "items": [
    {
      "id": "1839283746501111111",
      "name": "ACM 集训队",
      "description": "算法竞赛训练资料",
      "role": "OWNER",
      "member_count": 12,
      "document_count": 87,
      "updated_at": "2026-11-01T10:20:00Z"
    }
  ],
  "page": 1,
  "page_size": 20,
  "total": 1
}
```

### `POST /spaces` 🚧 规划中

**请求**

```json
{
  "name": "ACM 集训队",
  "description": "算法竞赛训练资料"
}
```

**响应 201**：空间对象（`role` 为 `OWNER`）。创建时自动写入 `space_members(OWNER)`。

**错误**：`400 bad_request`、`409 conflict`（同一用户空间重名，规划）。

### `GET /spaces/{space_id}` 🚧 规划中

**响应 200**：空间对象。

**错误**：`401 unauthorized`、`403 forbidden`（非成员）、`404 not_found`。

### `PATCH /spaces/{space_id}` 🚧 规划中

需 `OWNER` 或 `ADMIN`。

**请求**

```json
{
  "name": "ACM 集训队（2026）",
  "description": "更新后的简介"
}
```

**响应 200**：更新后的空间对象。

### `DELETE /spaces/{space_id}` 🚧 规划中

需 `OWNER`。级联删除空间下所有文档与版本（数据库外键 `ON DELETE CASCADE`）。**不可恢复**，实现时建议要求请求体带 `{"confirm": "<space_name>"}` 二次确认。

**响应**：`204 No Content`。

### 成员管理 🚧 规划中

| 方法 | 路径 | 权限 | 说明 |
| --- | --- | --- | --- |
| `GET` | `/spaces/{space_id}/members` | 成员 | 成员列表（含用户昵称、邮箱、角色、加入时间） |
| `POST` | `/spaces/{space_id}/members` | OWNER / ADMIN | 添加成员 |
| `PATCH` | `/spaces/{space_id}/members/{user_id}` | OWNER / ADMIN | 修改角色 |
| `DELETE` | `/spaces/{space_id}/members/{user_id}` | OWNER / ADMIN | 移除成员 |

添加成员请求：

```json
{
  "email": "bob@example.com",
  "role": "EDITOR"
}
```

成员列表响应：

```json
{
  "items": [
    {
      "user_id": "1839283746501234567",
      "email": "alice@example.com",
      "name": "Alice",
      "avatar_url": "",
      "role": "OWNER",
      "joined_at": "2026-09-01T08:00:00Z"
    }
  ],
  "page": 1,
  "page_size": 20,
  "total": 1
}
```

约束：

- 不能移除或降级最后一名 `OWNER`；`OWNER` 转让需专用接口（P1）。
- `ADMIN` 不能修改 `OWNER` 的角色。
- 重复添加已在空间内的成员返回 `409 conflict`。

## 规划接口：文档 Documents

文档归属空间，路径统一嵌套在空间下，避免文档 ID 跨空间歧义：

```text
/api/v1/spaces/{space_id}/documents
```

以下均为 **🚧 规划中**。

### 文档对象

```json
{
  "id": "1839283746502222222",
  "space_id": "1839283746501111111",
  "parent_id": null,
  "title": "新人指南",
  "format": "markdown",
  "revision": 7,
  "created_by": "1839283746501234567",
  "updated_by": "1839283746501234567",
  "created_at": "2026-10-01T09:00:00Z",
  "updated_at": "2026-11-05T12:00:00Z",
  "deleted_at": null
}
```

列表对象**不含 `content`**；详情对象额外包含 `"content": "# 标题\n..."`。

### `GET /spaces/{space_id}/documents` 🚧 规划中

**查询参数**

| 参数 | 类型 | 默认 | 说明 |
| --- | --- | --- | --- |
| `parent_id` | string | — | 按父目录过滤；不传返回全部活跃文档（v1 平铺） |
| `q` | string | — | 标题关键词（P1） |
| `include_deleted` | boolean | `false` | 是否包含回收站文档（需 EDITOR 及以上） |
| `page` / `page_size` | integer | 1 / 20 | 分页 |

**响应 200**

```json
{
  "items": [
    {
      "id": "1839283746502222222",
      "space_id": "1839283746501111111",
      "parent_id": null,
      "title": "新人指南",
      "format": "markdown",
      "revision": 7,
      "created_by": "1839283746501234567",
      "updated_by": "1839283746501234567",
      "created_at": "2026-10-01T09:00:00Z",
      "updated_at": "2026-11-05T12:00:00Z",
      "deleted_at": null
    }
  ],
  "page": 1,
  "page_size": 20,
  "total": 1
}
```

**错误**：`401 unauthorized`、`403 forbidden`、`404 not_found`（空间不存在）。

### `POST /spaces/{space_id}/documents` 🚧 规划中

需 `EDITOR` 及以上。

**请求**

```json
{
  "title": "新人指南",
  "content": "# 新人指南\n\n欢迎加入……",
  "parent_id": null,
  "format": "markdown"
}
```

| 字段 | 必填 | 说明 |
| --- | --- | --- |
| `title` | 是 | 1～255 字符，同空间活跃文档内唯一 |
| `content` | 否 | 默认空串；创建时也可直接带内容 |
| `parent_id` | 否 | v1 必须为 `null`；v2 目录树启用 |
| `format` | 否 | 默认 `markdown`，v1 仅支持该值 |

**响应 201**：文档详情对象，其中 `revision` 从 0 开始。

**错误**：`400 bad_request`、`403 forbidden`、`404 not_found`、`409 conflict`（标题重复）。

### `GET /spaces/{space_id}/documents/{document_id}` 🚧 规划中

**响应 200**：文档详情（含 `content`）。可附带 `"role": "EDITOR"` 表示当前用户对该文档的权限（便于前端渲染按钮）。

**错误**：`401 unauthorized`、`403 forbidden`、`404 not_found`（含已在回收站且未指定恢复的情况）。

### `PUT /spaces/{space_id}/documents/{document_id}` 🚧 规划中

保存文档内容，**乐观锁**。需 `EDITOR` 及以上。

**请求**

```json
{
  "base_rev": 7,
  "title": "新人指南",
  "content": "# 新人指南\n\n更新后的内容……",
  "note": "补充常见问题"
}
```

| 字段 | 必填 | 说明 |
| --- | --- | --- |
| `base_rev` | 是 | 客户端加载时的 `revision`；也接受 `If-Match: "7"` 头，二者必须一致（若同时提供） |
| `title` | 否 | 不传则保留原标题 |
| `content` | 是 | 全文 Markdown |
| `note` | 否 | 版本备注，最长 255 |

**响应 200**

```json
{
  "id": "1839283746502222222",
  "revision": 8,
  "updated_at": "2026-11-05T12:05:00Z"
}
```

**错误**：

| HTTP | code | 场景 |
| --- | --- | --- |
| 400 | `bad_request` | 缺少 `base_rev`、`base_rev` 与 `If-Match` 不一致 |
| 403 | `forbidden` | 非 EDITOR 及以上 |
| 404 | `not_found` | 文档不存在或已删除 |
| 409 | `conflict` | 版本冲突（`revision` 已变化）或标题与其他文档重复 |
| 413 | — | 内容超过大小限制（规划：512 KB，由网关或应用层限制） |

> 冲突时返回 `409`，客户端应提示用户「文档已被他人修改」，并提供「重新加载」与「另存为新文档」两个操作。

### `PATCH /spaces/{space_id}/documents/{document_id}` 🚧 规划中

只改元数据（标题、父目录），不动正文，不产生版本快照。

**请求**

```json
{
  "title": "新人指南（2026 版）",
  "parent_id": "1839283746503333333"
}
```

**响应 200**：文档详情对象（不含 `content`）。

### `DELETE /spaces/{space_id}/documents/{document_id}` 🚧 规划中

软删除（进入回收站，写 `deleted_at`）。需 `EDITOR` 及以上（`EDITOR` 仅能删除自己创建的文档）。

**响应**：`204 No Content`。

**错误**：`403 forbidden`、`404 not_found`、`409 conflict`（已删除）。

### `POST /spaces/{space_id}/documents/{document_id}/restore` 🚧 规划中

从回收站恢复。需 `EDITOR` 及以上，且标题不能与当前活跃文档冲突。

**响应 200**：恢复后的文档对象（不含 `content`）。

**错误**：`404 not_found`（不在回收站）、`409 conflict`（标题被占用）。

### `DELETE /spaces/{space_id}/documents/{document_id}/purge` 🚧 规划中（P1）

永久删除文档及其所有版本，仅 `OWNER` / `ADMIN` 可用。**不可恢复**。

**响应**：`204 No Content`。

### 历史版本 🚧 规划中

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` | `/spaces/{space_id}/documents/{document_id}/versions` | 版本列表（不含 `content`），按 `version` 倒序 |
| `GET` | `/spaces/{space_id}/documents/{document_id}/versions/{version}` | 版本详情（含 `content`） |
| `POST` | `/spaces/{space_id}/documents/{document_id}/rollback` | 回滚到指定版本 |

版本列表响应：

```json
{
  "items": [
    {
      "version": 8,
      "title": "新人指南",
      "note": "补充常见问题",
      "size_bytes": 2048,
      "saved_by": "1839283746501234567",
      "saved_at": "2026-11-05T12:05:00Z"
    }
  ],
  "page": 1,
  "page_size": 20,
  "total": 8
}
```

回滚请求：

```json
{
  "version": 5,
  "note": "回滚到第 5 版"
}
```

回滚响应：

```json
{
  "id": "1839283746502222222",
  "revision": 9,
  "rolled_back_to": 5,
  "updated_at": "2026-11-05T12:10:00Z"
}
```

回滚**不覆盖历史**：会生成新版本 `9`，内容与 v5 相同，`note` 记录回滚来源。错误：`404 not_found`（版本不存在）、`409 conflict`（并发修改）。

### 回收站 🚧 规划中（P1）

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` | `/spaces/{space_id}/documents/trash` | 回收站列表，按 `deleted_at` 倒序，`items` 含 `deleted_at` |
| `DELETE` | `/spaces/{space_id}/documents/trash` | 清空回收站（OWNER / ADMIN） |

## 规划接口：搜索 Search

### `GET /search` 🚧 规划中（P1）

跨空间搜索当前用户有权访问的文档。

**查询参数**

| 参数 | 类型 | 默认 | 说明 |
| --- | --- | --- | --- |
| `q` | string | 必填 | 关键词，1～100 字符 |
| `space_id` | string | — | 限定空间；不传搜索全部可访问空间 |
| `page` / `page_size` | integer | 1 / 20 | 分页 |

**响应 200**

```json
{
  "items": [
    {
      "document_id": "1839283746502222222",
      "space_id": "1839283746501111111",
      "space_name": "ACM 集训队",
      "title": "新人指南",
      "snippet": "……欢迎加入<em>算法</em>竞赛训练……",
      "updated_at": "2026-11-05T12:00:00Z"
    }
  ],
  "page": 1,
  "page_size": 20,
  "total": 1
}
```

`q` 为空返回 `400 bad_request`。v1 使用数据库 `LIKE` / 全文索引实现，`snippet` 由服务端截取并转义 HTML；引入搜索引擎后接口保持不变。

## 状态码与错误码速查

| HTTP | 使用场景 |
| --- | --- |
| 200 | 查询成功、更新成功 |
| 201 | 创建成功（注册、空间、文档） |
| 202 | 已受理（登录需要 TOTP、密码重置邮件） |
| 204 | 删除/登出成功，无 body |
| 400 | 参数非法 |
| 401 | 未认证 / 令牌失效 / 密码错误 |
| 403 | 无权限 / 账号被禁用 |
| 404 | 资源或路由不存在 |
| 408 | 请求超时（`APP_REQUEST_TIMEOUT_SECS`，默认 30 秒） |
| 409 | 唯一约束冲突 / 乐观锁冲突 |
| 410 | 登录挑战过期（规划） |
| 413 | 请求体过大（规划） |
| 429 | 限流（规划） |
| 500 | 服务内部错误 |
| 503 | 依赖不可用（`/readyz` 已实现） |

> 例外：`408` 由 `TimeoutLayer` 在进入业务 handler 之前直接返回，响应体不保证是统一错误结构；`/healthz`、`/readyz` 的 `503` 使用各自探针的响应体（见上文）。业务接口的 `4xx`/`5xx` 一律遵循统一错误结构。

## 接口实现顺序建议

按依赖关系分阶段落地，每个阶段都能独立联调：

1. **认证闭环**：`register` → `login` → `refresh` → `logout`，先不做 TOTP；补 `me`。
2. **空间与成员**：`spaces` CRUD → `members` 增删改查 → 权限中间件真正生效。
3. **文档主流程**：列表 → 创建 → 详情 → 乐观锁保存。
4. **版本与回收站**：版本列表 → 版本详情 → 回滚 → 软删除 → 恢复。
5. **增强**：TOTP 登录挑战 → 密码重置 → 搜索 → 限流 → 目录树。

每实现一个接口，同步更新本文的 ✅/🚧 标记与示例，保持文档即契约。
