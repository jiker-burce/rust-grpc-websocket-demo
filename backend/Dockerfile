# 使用官方Rust镜像作为构建环境
FROM rust:1.70 as builder

# 设置工作目录
WORKDIR /app

# 复制Cargo文件
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./

# 创建虚拟的main.rs来缓存依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 构建依赖
RUN cargo build --release

# 删除虚拟main.rs并复制实际源码
RUN rm src/main.rs
COPY src ./src
COPY proto ./proto
COPY migrations ./migrations

# 构建应用
RUN cargo build --release

# 使用轻量级镜像作为运行环境
FROM debian:bullseye-slim

# 安装必要的系统依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 创建非root用户
RUN useradd -r -s /bin/false appuser

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/chat-backend /app/chat-backend

# 复制迁移文件
COPY --from=builder /app/migrations /app/migrations

# 更改文件所有者
RUN chown -R appuser:appuser /app

# 切换到非root用户
USER appuser

# 暴露端口
EXPOSE 50051 8080 3001

# 启动应用
CMD ["./chat-backend"]
