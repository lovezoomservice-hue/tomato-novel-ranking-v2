#!/bin/bash
# 构建带榜单功能的番茄小说下载器 Docker 镜像
# 用法: ./build-ranking.sh [镜像名]
# 示例: ./build-ranking.sh tomato-novel-ranking:latest

set -e

IMAGE_NAME="${1:-tomato-novel-ranking:latest}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR"

echo "============================================"
echo "  番茄小说下载器 - 榜单批量下载版"
echo "  构建镜像: $IMAGE_NAME"
echo "============================================"

# 检查 Docker
if ! command -v docker &> /dev/null; then
    echo "错误: 未找到 docker，请先安装 Docker"
    exit 1
fi

# 检查 Tomato-Novel-Official-API 是否存在
OFFICIAL_API_DIR="$(dirname "$PROJECT_DIR")/Tomato-Novel-Official-API"
if [ ! -d "$OFFICIAL_API_DIR" ]; then
    echo ""
    echo "警告: 未找到 Tomato-Novel-Official-API 依赖！"
    echo ""
    echo "请先获取 Tomato-Novel-Official-API："
    echo "  1. 联系原作者获取仓库访问权限"
    echo "  2. 将仓库 clone 到以下路径："
    echo ""
    echo "    $(dirname "$PROJECT_DIR")/Tomato-Novel-Official-API"
    echo ""
    echo "  目录结构应该是："
    echo "    $(dirname "$PROJECT_DIR")/"
    echo "    ├── Tomato-Novel-Downloader/"
    echo "    └── Tomato-Novel-Official-API/"
    echo ""
    exit 1
fi

echo "找到 Tomato-Novel-Official-API: $OFFICIAL_API_DIR"

# 执行构建
echo ""
echo "开始构建..."
cd "$PROJECT_DIR"

docker build \
    -f Dockerfile.webui \
    -t "$IMAGE_NAME" \
    --progress=plain \
    .

echo ""
echo "============================================"
echo "  构建完成！"
echo "  镜像: $IMAGE_NAME"
echo "============================================"
echo ""
echo "启动示例："
echo ""
echo "  docker run -d \\"
echo "    --name tomato-novel-ranking \\"
echo "    --restart unless-stopped \\"
echo "    -p 18423:18423 \\"
echo "    -v \$(pwd)/data:/data \\"
echo "    -e TOMATO_WEB_ADDR=0.0.0.0:18423 \\"
echo "    -e TOMATO_WEB_PASSWORD=你的密码 \\"
echo "    $IMAGE_NAME"
echo ""
