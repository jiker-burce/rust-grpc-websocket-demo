#!/bin/bash

# 实时聊天系统启动脚本

set -e

echo "🚀 启动实时聊天系统..."

# 检查Docker是否安装
if ! command -v docker &> /dev/null; then
    echo "❌ Docker未安装，请先安装Docker"
    exit 1
fi

if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose未安装，请先安装Docker Compose"
    exit 1
fi

# 创建必要的目录
echo "📁 创建必要的目录..."
mkdir -p docker/mysql/data
mkdir -p docker/redis/data

# 设置环境变量
echo "🔧 设置环境变量..."
export COMPOSE_PROJECT_NAME=chat-system

# 停止现有容器
echo "🛑 停止现有容器..."
docker-compose -f docker/docker-compose.yml down

# 构建并启动服务
echo "🔨 构建并启动服务..."
docker-compose -f docker/docker-compose.yml up --build -d

# 等待服务启动
echo "⏳ 等待服务启动..."
sleep 10

# 检查服务状态
echo "🔍 检查服务状态..."
docker-compose -f docker/docker-compose.yml ps

# 显示访问信息
echo ""
echo "✅ 实时聊天系统启动成功！"
echo ""
echo "📱 访问地址："
echo "   前端界面: http://localhost:3000"
echo "   后端健康检查: http://localhost:3001/health"
echo "   gRPC服务: localhost:50051"
echo "   WebSocket服务: ws://localhost:8080"
echo ""
echo "📊 数据库信息："
echo "   MySQL: localhost:3306"
echo "   Redis: localhost:6379"
echo ""
echo "🔧 管理命令："
echo "   查看日志: docker-compose -f docker/docker-compose.yml logs -f"
echo "   停止服务: docker-compose -f docker/docker-compose.yml down"
echo "   重启服务: docker-compose -f docker/docker-compose.yml restart"
echo ""
