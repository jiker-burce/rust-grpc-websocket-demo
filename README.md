# 实时聊天系统 (Real-time Chat System)

一个基于 Rust + Vue.js 的综合实时聊天系统，集成了 gRPC、WebSocket、MySQL、Redis 等技术栈。

## 🚀 项目特色

- **高性能**: 基于 Rust 的异步后端，支持高并发
- **实时通信**: WebSocket 实现即时消息推送
- **现代架构**: gRPC + HTTP API 双重接口
- **响应式设计**: Vue 3 + Element Plus 现代化界面
- **完整功能**: 用户管理、实时聊天、在线状态、消息历史

## 📁 项目架构

```
grpc_websocket/
├── backend/                 # Rust 后端服务
│   ├── src/
│   │   ├── main.rs         # 主程序入口
│   │   ├── grpc/           # gRPC 服务实现
│   │   ├── http/           # HTTP API 接口
│   │   ├── websocket/      # WebSocket 处理
│   │   ├── database/       # 数据库操作
│   │   ├── redis/          # Redis 缓存
│   │   └── models/         # 数据模型
│   ├── proto/              # Protocol Buffers 定义
│   ├── migrations/         # 数据库迁移
│   └── Cargo.toml
├── frontend/               # Vue.js 前端
│   ├── src/
│   │   ├── components/     # Vue 组件
│   │   ├── views/          # 页面视图
│   │   ├── stores/         # Pinia 状态管理
│   │   ├── services/       # API 服务
│   │   └── router/         # 路由配置
│   ├── package.json
│   └── vite.config.js
├── docker/                 # Docker 配置
│   ├── docker-compose.yml
│   └── mysql/
└── scripts/               # 部署脚本
    ├── start.sh           # 生产环境启动
    ├── dev.sh             # 开发环境启动
    ├── stop.sh            # 停止服务
    └── test.sh            # 系统测试
```

## 🛠 技术栈

### 后端 (Rust)
- **gRPC**: 高性能 RPC 通信
- **WebSocket**: 实时消息推送
- **MySQL**: 用户信息、聊天记录存储
- **Redis**: 在线状态、会话缓存
- **Tonic**: gRPC 框架
- **Tokio**: 异步运行时
- **SQLx**: 类型安全的数据库操作
- **Warp**: HTTP 服务器
- **JWT**: 用户认证

### 前端 (Vue.js)
- **Vue 3**: 现代前端框架
- **Vite**: 快速构建工具
- **Element Plus**: 企业级 UI 组件库
- **Pinia**: 状态管理
- **Vue Router**: 路由管理
- **Axios**: HTTP 请求库
- **WebSocket**: 实时通信

## ✨ 功能特性

- ✅ **用户系统**: 注册、登录、个人资料管理
- ✅ **实时聊天**: 即时消息发送和接收
- ✅ **多房间支持**: 支持多个聊天房间
- ✅ **在线状态**: 实时显示在线用户
- ✅ **消息历史**: 聊天记录持久化存储
- ✅ **响应式设计**: 适配各种设备尺寸
- ✅ **安全认证**: JWT Token 认证机制
- ✅ **会话管理**: Redis 缓存用户会话

## 🚀 快速开始

### 环境要求
- **Rust**: 1.70+
- **Node.js**: 18+
- **Docker**: 20.0+
- **Docker Compose**: 2.0+

### 方式一：Docker 一键部署（推荐）

```bash
# 克隆项目
git clone <repository-url>
cd grpc_websocket

# 一键启动所有服务
./scripts/start.sh
```

### 方式二：开发环境

```bash
# 启动开发环境
./scripts/dev.sh
```

### 方式三：手动启动

1. **启动数据库服务**
```bash
cd docker
docker-compose up -d mysql redis
```

2. **启动后端服务**
```bash
cd backend
cargo run
```

3. **启动前端服务**
```bash
cd frontend
npm install
npm run dev
```

## 📱 访问地址

- **前端界面**: http://localhost:3000
- **后端 API**: http://localhost:3001
- **健康检查**: http://localhost:3001/health
- **gRPC 服务**: localhost:50051
- **WebSocket**: ws://localhost:8080

## 🧪 系统测试

```bash
# 运行系统测试
./scripts/test.sh
```

## 📊 数据库设计

### 用户表 (users)

- `id`: 用户唯一标识
- `username`: 用户名
- `email`: 邮箱地址
- `password_hash`: 密码哈希
- `avatar`: 头像URL
- `is_online`: 在线状态
- `created_at`: 创建时间
- `updated_at`: 更新时间

### 消息表 (messages)

- `id`: 消息唯一标识
- `user_id`: 发送用户ID
- `username`: 发送用户名
- `content`: 消息内容
- `room_id`: 房间ID
- `message_type`: 消息类型
- `created_at`: 发送时间

### 房间表 (rooms)

- `id`: 房间唯一标识
- `name`: 房间名称
- `description`: 房间描述
- `is_public`: 是否公开
- `created_by`: 创建者ID
- `created_at`: 创建时间
- `updated_at`: 更新时间

## 🔧 API 接口

### 用户相关

- `POST /users/register` - 用户注册
- `POST /users/login` - 用户登录

### 聊天相关

- `POST /chat/messages` - 发送消息
- `GET /chat/rooms/{room_id}/messages` - 获取消息历史
- `GET /chat/rooms/{room_id}/users` - 获取在线用户

## 🐳 Docker 部署

### 生产环境部署

```bash
# 构建并启动所有服务
docker-compose -f docker/docker-compose.yml up --build -d

# 查看服务状态
docker-compose -f docker/docker-compose.yml ps

# 查看日志
docker-compose -f docker/docker-compose.yml logs -f

# 停止服务
docker-compose -f docker/docker-compose.yml down
```

## 🔒 安全配置

### 环境变量配置

创建 `backend/.env` 文件：

```env
DATABASE_URL=mysql://chat_user:chat_password@localhost:3306/chat_db
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-super-secret-jwt-key-change-in-production
```

### 生产环境注意事项

1. **更改默认密码**: 修改数据库和Redis的默认密码
2. **JWT密钥**: 使用强随机密钥替换默认JWT密钥
3. **HTTPS**: 生产环境建议使用HTTPS
4. **防火墙**: 配置适当的防火墙规则
5. **备份**: 定期备份数据库数据

## 📈 性能优化

- **连接池**: 数据库连接池优化
- **缓存策略**: Redis缓存热点数据
- **异步处理**: 全异步架构提升并发性能
- **消息队列**: 可扩展消息队列处理
- **负载均衡**: 支持水平扩展

## 🤝 贡献指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 🙏 致谢

感谢以下开源项目：

- [Rust](https://www.rust-lang.org/)
- [Vue.js](https://vuejs.org/)
- [Element Plus](https://element-plus.org/)
- [Tonic](https://github.com/hyperium/tonic)
- [SQLx](https://github.com/launchbadge/sqlx)

## 工作机会

- 有合适的rust岗位希望大家可以帮我推荐，以后会陆续推出各种demo项目共大家研究学习，共同进步。
- 邮箱：[storefee@163.com](storefee@163.com)
