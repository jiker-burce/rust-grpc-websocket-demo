#!/bin/bash

# 实时聊天系统停止脚本

set -e

echo "🛑 停止实时聊天系统..."

# 停止所有服务
docker-compose -f docker/docker-compose.yml down

echo "✅ 实时聊天系统已停止"
