#!/bin/bash

# 开发环境启动脚本

set -e

echo "🔧 启动开发环境..."

# 检查Node.js是否安装
if ! command -v node &> /dev/null; then
    echo "❌ Node.js未安装，请先安装Node.js 18+"
    exit 1
fi

# 检查Rust是否安装
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust未安装，请先安装Rust"
    exit 1
fi

# 启动数据库服务
echo "🗄️ 启动数据库服务..."
docker-compose -f docker/docker-compose.yml up -d mysql redis

# 等待数据库启动
echo "⏳ 等待数据库启动..."
sleep 5

# 设置环境变量
export DATABASE_URL="mysql://chat_user:chat_password@localhost:3306/chat_db"
export REDIS_URL="redis://localhost:6379"
export JWT_SECRET="dev-secret-key"

# 启动后端服务
echo "🦀 启动Rust后端服务..."
cd backend
cargo run &
BACKEND_PID=$!
cd ..

# 等待后端启动
sleep 3

# 启动前端服务
echo "🎨 启动Vue.js前端服务..."
cd frontend
npm install
npm run dev &
FRONTEND_PID=$!
cd ..

echo ""
echo "✅ 开发环境启动成功！"
echo ""
echo "📱 访问地址："
echo "   前端界面: http://localhost:3000"
echo "   后端健康检查: http://localhost:3001/health"
echo ""
echo "🔧 停止开发环境："
echo "   按 Ctrl+C 停止所有服务"
echo ""

# 等待用户中断
trap "echo '🛑 停止开发环境...'; kill $BACKEND_PID $FRONTEND_PID 2>/dev/null; docker-compose -f docker/docker-compose.yml down; exit 0" INT

# 保持脚本运行
wait
