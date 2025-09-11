#!/bin/bash

# 实时聊天系统测试脚本

set -e

echo "🧪 开始测试实时聊天系统..."

# 检查服务是否运行
echo "🔍 检查服务状态..."

# 检查HTTP健康检查
echo "检查HTTP健康检查..."
if curl -s http://localhost:3001/health | grep -q "ok"; then
    echo "✅ HTTP服务正常"
else
    echo "❌ HTTP服务异常"
    exit 1
fi

# 检查WebSocket连接
echo "检查WebSocket连接..."
if nc -z localhost 8080; then
    echo "✅ WebSocket服务正常"
else
    echo "❌ WebSocket服务异常"
    exit 1
fi

# 检查gRPC服务
echo "检查gRPC服务..."
if nc -z localhost 50051; then
    echo "✅ gRPC服务正常"
else
    echo "❌ gRPC服务异常"
    exit 1
fi

# 检查数据库连接
echo "检查数据库连接..."
if nc -z localhost 3306; then
    echo "✅ MySQL服务正常"
else
    echo "❌ MySQL服务异常"
    exit 1
fi

# 检查Redis连接
echo "检查Redis连接..."
if nc -z localhost 6379; then
    echo "✅ Redis服务正常"
else
    echo "❌ Redis服务异常"
    exit 1
fi

echo ""
echo "🎉 所有服务测试通过！"
echo ""
echo "📱 可以访问以下地址进行测试："
echo "   前端界面: http://localhost:3000"
echo "   API文档: http://localhost:3001/health"
echo ""
