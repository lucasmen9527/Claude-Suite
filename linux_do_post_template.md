# Claude Suite macOS 修复版 - 解决 DMG 打包路径识别问题

## 前言说明

基于 L 站 [@xiniah](https://linux.do/u/xiniah) 大佬的帖子 [【已停止更新，请看最新消息】Claude Suite: 基于claudia 一体化管理claude code. 支持直接管理API中转站，管理令牌，查询日志，快速切换配置](https://linux.do/t/topic/838591/136) 开源项目 [claude-suite](https://github.com/xinhai-ai/claude-suite)，修复了 macOS 打包 DMG 之后路径识别不到的 bug。

**重要说明**：
- 原项目作者 [@xiniah](https://linux.do/u/xiniah) 已停止维护，原仓库已关闭（This repository was archived by the owner on Sep 15, 2025. It is now read-only.）
- 该仓库仅仅是完善 macOS DMG 打包问题，后续看情况可能不会再维护此项目
- 发这个帖子主要就是方便 macOS 用户下载使用
- **Windows 用户请看原作者 L 站 [@anyme](https://linux.do/u/anyme) 大佬的帖子** [[8月2日更新] Claude Workbench - 让 windows 使用 Claude Code更加高效便捷！](https://linux.do/t/topic/799521) GitHub仓库：https://github.com/anyme123/claude-workbench

## 下载地址

🔗 **GitHub Release**: https://github.com/lucasmen9527/claude-suite/releases/tag/v1.1.5

## 主要修复内容

这次修复主要解决了一个很恼人的问题：**macOS DMG 应用无法检测 Claude CLI 版本**

### 问题描述
使用 unix 二进制文件运行时，可以正常检测到 Claude CLI 版本，但是打包成 DMG 后就检测不到了。这是因为：
- macOS 应用从 Finder 启动时不会继承 shell 环境变量
- NVM 安装的 Node.js 和 Claude CLI 路径不在默认的 PATH 中
- DMG 应用无法访问到 `~/.nvm/versions/node/*/bin/claude` 路径

### 解决方案
- 实现了智能 PATH 构建机制，自动扫描多种安装路径
- 支持 NVM、Homebrew、系统路径等多种环境
- 优化了默认安装选择逻辑，优先选择 NVM 安装的版本

## 主要功能

看图说话，先上几张截图：

### 应用界面展示

**主界面 - 简洁现代的设计**
![主界面](https://github.com/lucasmen9527/claude-suite/blob/main/screenshots/01-main-interface.png?raw=true)

**项目管理 - 一目了然的项目列表**
![项目管理](https://github.com/lucasmen9527/claude-suite/blob/main/screenshots/02-project-management.png?raw=true)

**Claude 交互界面 - 专业的 AI 开发工具**
![Claude 交互](https://github.com/lucasmen9527/claude-suite/blob/main/screenshots/03-claude-chat-interface.png?raw=true)

**使用统计仪表板 - 详细的费用和使用分析**
![使用统计](https://github.com/lucasmen9527/claude-suite/blob/main/screenshots/04-usage-dashboard.png?raw=true)

**设置页面 - 完整的环境变量管理**
![设置页面](https://github.com/lucasmen9527/claude-suite/blob/main/screenshots/05-settings-environment.png?raw=true)

### 详细功能介绍

### Claude CLI 管理
- **智能项目管理**：可视化显示所有 Claude Code 项目，支持项目路径和活动时间追踪
- **一键会话创建**：快速创建新的 Claude Code 会话
- **实时 AI 交互**：流式显示 Claude 响应，支持 Markdown 渲染和代码高亮
- **15+ 专业工具**：支持 Task、Bash、Glob、Grep、Read、Edit、Write、MultiEdit、NotebookEdit 等开发工具
- **MCP 协议支持**：完整的 Model Context Protocol 支持，多达 39 个服务

### 使用统计与分析
- **费用监控**：实时显示 API 使用费用和成本分析
- **Token 统计**：详细的输入/输出 Token 使用量统计
- **使用趋势**：会话数量、模型使用频率等趋势分析
- **项目热度**：热门项目和使用习惯分析

### 配置管理
- **环境变量管理**：可视化管理所有 Claude CLI 相关环境变量
- **API 配置**：支持 ANTHROPIC_AUTH_TOKEN、ANTHROPIC_BASE_URL 等配置
- **一键切换**：快速切换不同的 API 提供商配置
- **安全存储**：本地加密存储所有敏感配置信息

### macOS 特别优化
- 智能 PATH 检测：自动扫描 NVM、Homebrew 等开发环境路径
- 环境变量增强：解决 macOS 应用启动时的环境变量限制
- DMG 安装包：提供标准的 macOS 安装体验
- 原生权限管理：正确的 macOS 安全权限配置

## 安装使用

### 系统要求
- macOS 11.0+ (Big Sur 或更高版本)
- Node.js 18.0+ (推荐通过 NVM 安装)
- 已安装 Claude CLI：`npm install -g @anthropic-ai/claude-code`

### 安装步骤
1. 下载 DMG 文件：[Claude Suite_1.1.5_aarch64.dmg](https://github.com/lucasmen9527/claude-suite/releases/tag/v1.1.5)
2. 双击 DMG 文件，将应用拖拽到 Applications 文件夹
3. 首次运行时，在"系统偏好设置 → 安全性与隐私"中允许应用运行
4. 启动应用，会自动检测 Claude CLI 安装

### 故障排除
如果遇到 "Claude CLI not found" 错误：
```bash
# 检查 Claude CLI 是否安装
which claude
claude --version

# 重新安装（推荐）
npm install -g @anthropic-ai/claude-code
```

也可以在应用设置中手动指定 Claude CLI 路径。

## 技术架构

**前端**：React 18 + TypeScript + Tailwind CSS 4
**后端**：Tauri 2 + Rust + SQLite
**特色**：完全本地化，无需联网即可管理配置

## 致谢

特别感谢 Linux.do 社区的贡献者：

- [@xiniah](https://linux.do/u/xiniah) 大佬：原创项目作者，提供了完整的功能设计和实现基础
- [@anyme](https://linux.do/u/anyme) 大佬：Windows 版本开发者，提供了宝贵的跨平台开发经验
- [@getAsterisk/claudia](https://github.com/getAsterisk/opcode)：提供了基础架构和设计思路

## 最后

这个版本主要就是解决 macOS DMG 的路径识别问题，让 Mac 用户也能愉快地使用 Claude Suite。

如果这个工具对你有帮助，欢迎到 [GitHub](https://github.com/lucasmen9527/claude-suite) 给个 ⭐

**再次提醒**：Windows 用户请使用 [@anyme](https://linux.do/u/anyme) 大佬的版本 [Claude Workbench](https://github.com/anyme123/claude-workbench)，功能更完善！