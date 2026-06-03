# 榜单批量下载功能 - 构建指南

本项目基于 [Tomato-Novel-Downloader](https://github.com/zhongbai2333/Tomato-Novel-Downloader) 二次开发，新增了"榜单批量下载"功能。

## 新增功能

- **榜单批量下载**：支持男生榜、女生榜、畅销榜等榜单小说的批量下载
- **多选下载**：在榜单页面勾选多本书籍，一键批量创建下载任务
- **Web UI 无终端**：通过 Docker 部署，无终端窗口，浏览器操作

## 快速开始

### 方式一：Docker 部署（推荐）

#### 准备工作

Docker 构建需要准备私有依赖 `Tomato-Novel-Official-API`。获取方式：
1. 联系原作者获取仓库访问权限
2. 将 `Tomato-Novel-Official-API` 仓库 clone 到本项目的**同级目录**

目录结构应该是：
```
Tomato-Novel-Downloader/   <- 本项目
Tomato-Novel-Official-API/ <- 私有依赖（同级目录）
```

#### 构建镜像

```sh
cd Tomato-Novel-Downloader
docker build -f Dockerfile.webui -t tomato-novel-ranking:latest .
```

#### 启动服务

```sh
docker run -d \
  --name tomato-novel-ranking \
  --restart unless-stopped \
  -p 18423:18423 \
  -v $(pwd)/data:/data \
  -e TOMATO_WEB_ADDR=0.0.0.0:18423 \
  -e TOMATO_DATA_DIR=/data \
  -e TOMATO_WEB_PASSWORD=你的密码 \
  tomato-novel-ranking:latest
```

#### 访问

- 本机浏览器：`http://127.0.0.1:18423/`
- 局域网其他设备：`http://<本机IP>:18423/`

### 方式二：本地源码构建

```sh
# 安装 Rust（如果尚未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 克隆项目
git clone https://github.com/zhongbai2333/Tomato-Novel-Downloader.git
cd Tomato-Novel-Downloader

# 准备私有依赖（同级目录）
git clone https://github.com/zhongbai2333/Tomato-Novel-Official-API.git ../Tomato-Novel-Official-API

# 构建
cargo build --release --features official-api,tts

# 运行
./target/release/tomato-novel-downloader --server
```

## 使用榜单批量下载

1. 启动服务后，浏览器打开 `http://127.0.0.1:18423/`
2. 点击左侧导航的"**榜单下载**"
3. 选择频道（男生/女生/畅销）
4. 选择榜单分类（如"玄幻"）
5. 勾选想要下载的书籍（支持全选）
6. 点击"**批量下载已选**"
7. 自动跳转到任务列表查看进度

## API 接口说明

### 榜单分类列表
```
GET /api/ranking/categories
```

响应示例：
```json
{
  "channels": [
    {
      "channel_id": 1,
      "channel_name": "男生",
      "ranks": [
        { "rank_id": 1, "rank_name": "玄幻", "cover_imgs": [] },
        { "rank_id": 3, "rank_name": "都市", "cover_imgs": [] }
      ]
    },
    {
      "channel_id": 0,
      "channel_name": "女生",
      "ranks": [
        { "rank_id": 21, "rank_name": "现代言情", "cover_imgs": [] }
      ]
    },
    {
      "channel_id": -1,
      "channel_name": "畅销",
      "ranks": [
        { "rank_id": 100, "rank_name": "畅销总榜", "cover_imgs": [] }
      ]
    }
  ]
}
```

### 榜单书籍列表
```
GET /api/ranking/books?channel_id=1&rank_id=1&page=1&size=50
```

响应示例：
```json
{
  "items": [
    {
      "book_id": "1234567890",
      "title": "书名",
      "author": "作者",
      "description": "简介",
      "cover_url": "https://...",
      "word_count": "100万字",
      "category": "玄幻"
    }
  ],
  "total": 50,
  "page": 1,
  "size": 50
}
```

## 文件结构

```
src/
├── ranking/                      # 新增：榜单模块
│   ├── mod.rs                   # 模块入口
│   ├── api.rs                  # 榜单 API 调用
│   └── models.rs               # 数据结构定义
├── ui/web/
│   ├── routes/
│   │   └── ranking.rs           # 新增：榜单 API 路由
│   └── templates/
│       ├── index.html          # 新增：榜单导航 + 页面
│       ├── app.css             # 新增：榜单样式
│       └── app.js              # 新增：榜单交互逻辑
└── main.rs                     # 新增：mod ranking
```

## 注意事项

1. **番茄小说 API 可能变化**：榜单 API 基于 `fanqienovel.com/api/author/library/book_list/v0/` 接口，如遇接口失效需重新抓包分析
2. **榜单接口归属第三方接口**：部分第三方接口地址和 token 并不开源，可能需要定期维护
3. **单任务限制**：同时只允许 1 个下载任务，防止 API 被滥用
4. **数据安全**：Docker 部署时务必设置密码保护（`TOMATO_WEB_PASSWORD`）
