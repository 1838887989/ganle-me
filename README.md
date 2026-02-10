# 干了么 (GanLe Me)

一款简洁高效的桌面待办任务管理应用，基于 Vue 3 + Tauri 2 构建。

## 功能特性

- **任务管理** — 创建、编辑、删除、完成任务，支持子任务
- **智能清单** — 今日待办、近 7 天、全部任务、已完成、收藏
- **项目分类** — 按项目组织任务，自定义颜色和图标
- **标签系统** — 多标签管理，按标签筛选任务
- **重复任务** — 支持每日/每周/每月自动重复
- **到期提醒** — 任务到期前 5 分钟系统通知提醒
- **拖拽排序** — 自由拖拽调整任务顺序
- **极简模式** — 小窗口悬浮模式，支持窗口置顶和透明度调节
- **深色主题** — 浅色/深色主题一键切换
- **系统托盘** — 关闭窗口最小化到托盘，支持开机自启

## 技术栈

| 层级 | 技术 |
|------|------|
| 前端框架 | Vue 3 + TypeScript |
| 状态管理 | Pinia |
| UI 组件 | Naive UI + Tailwind CSS |
| 桌面框架 | Tauri 2 (Rust) |
| 数据存储 | SQLite (rusqlite) |
| 构建工具 | Vite 5 |

## 项目架构

```
src/                    # 前端源码
├── components/         # Vue 组件
├── stores/             # Pinia 状态管理
├── styles/             # 全局样式
└── types/              # TypeScript 类型定义

src-tauri/src/          # Rust 后端（四层架构）
├── models/             # 数据结构定义
├── db/                 # 数据库层（SQLite CRUD）
├── services/           # 业务逻辑层
├── commands/           # Tauri 命令层（API）
└── lib.rs              # 应用入口（托盘、窗口）
```

## 安装使用

### 下载安装包

前往 [Releases](../../releases) 页面下载对应平台的安装包：

| 平台 | 文件 |
|------|------|
| Windows | `干了么_x.x.x_x64-setup.exe` |
| macOS (Intel) | `干了么_x.x.x_x64.dmg` |
| macOS (Apple Silicon) | `干了么_x.x.x_aarch64.dmg` |
| Linux (deb) | `干了么_x.x.x_amd64.deb` |
| Linux (AppImage) | `干了么_x.x.x_amd64.AppImage` |

### 从源码构建

**前置要求：**
- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://www.rust-lang.org/tools/install) >= 1.70
- 系统依赖（Linux）：`sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf`

```bash
# 克隆仓库
git clone https://github.com/1838887989/ganle-me.git
cd ganle-me

# 安装依赖
npm install

# 开发模式
npm run tauri:dev

# 构建安装包
npm run tauri:build
```

## 开源协议

[MIT License](LICENSE)
