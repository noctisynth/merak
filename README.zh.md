# Merak · 天璇

> The Open-Source Distributed Project Collaboration Platform

Merak（天璇）是一个**开源、可自托管、分布式**的项目协作平台。你可以把它理解为开源版的飞书项目 / Notion + Jira，但更轻量、更灵活、完全由你掌控。

## 项目状态

**v0.1.0-alpha · 早期阶段**

目前已完成：
- [x] 用户注册 / 登录 / 登出 / Token 刷新
- [x] JWT 双密钥认证（Access Token + Refresh Token Rotation）
- [x] Argon2id 密码加密与强度校验
- [x] SurrealDB 数据库集成 + 自定义 ORM（过程宏驱动）
- [x] OpenAPI 3.0 自动生成文档 + Redoc UI
- [x] TypeScript API 客户端自动生成（@hey-api/openapi-ts）
- [x] 前端登录 / 注册页面 + 国际化（中/英）
- [x] shadcn/ui 组件库 + Tailwind CSS v4 + 暗色主题
- [x] CI（Rust 测试 + Clippy，前端 Lint + Build）
- [ ] **核心业务功能（等你来写）**

## 技术栈

| 层 | 技术 |
|---|---|
| **前端** | React 19 + TypeScript + Vite |
| **样式** | Tailwind CSS v4 + shadcn/ui（base-maia） |
| **后端** | Rust + Axum 0.8 |
| **数据库** | SurrealDB 2.5（多模：文档 + 图 + 关系） |
| **ORM** | 自定义过程宏（merak-macros + merak-core） |
| **认证** | JWT（HS256）+ Argon2id |
| **API 文档** | OpenAPI 3.0 + utoipa + Redoc |
| **前端客户端** | 自动生成（@hey-api/openapi-ts）+ Axios |
| **国际化** | i18next + react-i18next（中 / 英） |
| **CI** | GitHub Actions（Unit Test + Semifold） |

## 架构一览

```
merak/
├── src/                 # 前端 React SPA
│   ├── pages/           # 页面（landing, login, register）
│   ├── components/      # UI 组件
│   ├── client/          # 自动生成的 API 客户端
│   └── i18n/            # 国际化文案
├── crates/
│   ├── merak/           # 后端 Axum 服务器
│   │   ├── routes/      # HTTP 路由处理器
│   │   ├── services/    # 业务逻辑层
│   │   ├── models/      # 数据模型
│   │   └── common/      # 通用工具（错误码、响应格式）
│   ├── core/            # ORM 核心：Model trait + CRUD 封装
│   └── macros/          # 过程宏：#[derive(Model)]
```

## 如何运行

### 前置条件

- Rust（最新 stable）
- pnpm（v10+）
- SurrealDB（v2.5+）

### 启动后端

```bash
# 1. 启动 SurrealDB
surreal start --user root --pass root

# 2. 配置环境变量（参考 .env.example）
export SURREAL_URL="ws://127.0.0.1:5070/rpc"
export SURREAL_NS="merak"
export SURREAL_DB="merak"
export SURREAL_USER="root"
export SURREAL_PASS="root"
export JWT_ACCESS_SECRET="your-access-secret"
export JWT_REFRESH_SECRET="your-refresh-secret"

# 3. 启动服务器（默认 :8080）
cargo run
```

### 启动前端

```bash
pnpm install
pnpm dev    # 默认 :5173
```

### 测试

```bash
# Rust 后端
cargo test --workspace --all-targets --all-features

# 前端
pnpm lint
pnpm build
```

## 接下来要做什么

这是给你的**开发路线图**，建议按顺序推进：

### 第一阶段：组织与团队（Org & Team）

> 项目协作的基础——你需要先建立组织模型，让用户能创建团队、邀请成员。

- [ ] **后端 Model**：`Organization`、`OrganizationMember`、`Team`、`TeamMember`
  - 创建 `crates/merak/src/models/org.rs`
  - 使用 `#[derive(Model)]` 声明模型
- [ ] **后端 Route + Service**：组织 CRUD、成员管理（邀请、角色、退出）
  - 创建 `crates/merak/src/routes/org.rs`
  - 创建 `crates/merak/src/services/org.rs`
- [ ] **前端页面**：组织创建页、组织设置页、成员管理页
- [ ] 注册错误码 `code.rs`：Org 模块（`03MM`）

### 第二阶段：项目管理（Project）

> 核心业务——项目是协作的容器。

- [ ] **后端 Model**：`Project`、`ProjectMember`
- [ ] **后端 Route + Service**：项目 CRUD、成员权限、归档/删除
- [ ] **前端页面**：项目列表（Dashboard）、项目详情页、项目设置
- [ ] 注册错误码：Project 模块（`04MM`）

### 第三阶段：工作流与节点（Workflow & Node）

> 灵活的工作流引擎——这是 Merak 的核心差异化能力。

- [ ] **后端 Model**：`Workflow`、`WorkflowNode`、`NodeLink`
- [ ] **后端 Route + Service**：工作流 CRUD、节点编排、状态流转
- [ ] **前端页面**：工作流画布 / 看板视图
- [ ] 注册错误码：Workflow 模块（`05MM`）、Node 模块（`06MM`）

### 第四阶段：任务与子任务（Task & Subtask）

> 具体执行单元。

- [ ] **后端 Model**：`Task`、`Subtask`
- [ ] **后端 Route + Service**：任务 CRUD、分配、状态、优先级、截止日期
- [ ] **前端页面**：任务列表、任务详情、拖拽排序
- [ ] 注册错误码：Task 模块（`07MM`）

### 第五阶段：文档与评论（Doc & Comment）

> 知识沉淀与协作沟通。

- [ ] **后端 Model**：`Doc`、`Comment`
- [ ] **后端 Route + Service**：文档 CRUD、富文本存储、评论/回复
- [ ] **前端页面**：文档编辑器、评论区
- [ ] 注册错误码：Doc 模块（`08MM`）、Comment 模块（`09MM`）

### 第六阶段：通知与动态（Notification & Feed）

> 让协作有感知。

- [ ] **后端 Model**：`Notification`、`ActivityFeed`
- [ ] **后端 Route + Service**：通知推送、已读/未读、活动流聚合
- [ ] **前端页面**：通知中心、动态侧边栏
- [ ] 注册错误码：Notification 模块（`10MM`）

### 第七阶段：搜索与集成（Search & Integration）

> 让平台可用、好用。

- [ ] 全局搜索（全文检索）
- [ ] Webhook / API 集成
- [ ] 导入/导出（CSV, JSON, Markdown）
- [ ] 注册错误码：Integration 模块（`11MM`）

### 第八阶段：完善与打磨（Polish & Production）

> 从能用到好用。

- [ ] 前端状态管理（Zustand）
- [ ] Docker Compose 一键部署
- [ ] 完整的单元测试 + 集成测试覆盖
- [ ] 性能优化、错误处理完善
- [ ] 更多 i18n 语言

## 如何贡献

1. 从第一阶段开始，每完成一个模块提交一个 PR
2. 遵循现有的代码风格（Biome + cargo fmt）
3. 为新功能编写测试
4. 遵循 CMMRR 错误码体系
5. 保持前后端 API 同步，用 `pnpm generate-client` 重新生成客户端

---

**准备好了吗？Merak 的未来在你手中。**
