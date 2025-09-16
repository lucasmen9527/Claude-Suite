# GitHub Release 发版信息

## 基本信息
- **Release Tag**: `v1.1.5`
- **Release Title**: `Claude Suite v1.1.5 - macOS DMG 修复版`
- **Target Branch**: `main`

## Release 描述内容

### 🎯 **重大修复 - macOS DMG 应用 Claude CLI 检测问题**

本版本专门解决了 **macOS DMG 应用无法检测 Claude CLI 版本** 的关键问题，为 macOS 用户提供完整的 Claude CLI 桌面管理体验。

---

## 🔧 **核心修复内容**

### 🚨 关键问题修复
- **修复 macOS DMG 应用无法检测 Claude CLI 版本的问题**
  - 解决了 Finder 启动应用时环境变量缺失的根本原因
  - 实现智能 PATH 构建，自动扫描多种安装路径

### 🔍 环境检测增强
- **智能 PATH 检测机制**
  - 自动扫描 NVM 安装路径 (`~/.nvm/versions/node/*/bin/claude`)
  - 支持 Homebrew 安装路径 (`/opt/homebrew/bin`, `/usr/local/bin`)
  - 兼容全局 npm 安装和系统路径

### 🎯 默认选择优化
- **改善 NVM 安装的 Claude CLI 识别逻辑**
  - 优先选择 NVM 安装的版本
  - 智能版本比较和源偏好设置
  - 解决手动切换安装源的问题

### 🍎 macOS 系统适配
- **完善 macOS 打包和分发流程**
  - 优化 DMG 安装体验
  - 更新 macOS 安全权限配置
  - 原生 macOS 用户界面优化

---

## ✨ **功能特性**

### 🎯 核心功能
- **项目管理**: 可视化管理 Claude 项目，支持会话历史和检查点
- **实时交互**: 流式显示 Claude 响应，支持 Markdown 渲染和语法高亮
- **智能代理**: Agent 系统支持 GitHub 集成和自动化任务执行
- **MCP 支持**: 完整的 Model Context Protocol 服务器管理

### 🔧 代理商管理
- **一键切换**: 静默切换不同的 Claude API 代理商，无弹窗干扰
- **隐私安全**: 本地存储配置，零硬编码敏感信息
- **自由配置**: 完整的 CRUD 操作界面，支持自定义代理商
- **立即生效**: 自动重启 Claude 进程，配置立即生效

### 🍎 macOS 特别优化
- **智能 PATH 检测**: 自动扫描 NVM、Homebrew 等开发环境路径
- **环境变量增强**: 解决 macOS 应用启动时的环境变量限制
- **DMG 安装包**: 提供标准的 macOS 安装体验
- **原生权限管理**: 正确的 macOS 安全权限配置

---

## 📦 **系统要求**

- **操作系统**: macOS 11.0+ (Big Sur 或更高版本)
- **架构**: Apple Silicon (M1/M2/M3) 或 Intel
- **Node.js**: 18.0+ (推荐通过 NVM 安装)
- **Claude CLI**: 需要预先安装 (`npm install -g @anthropic-ai/claude-code`)

---

## 🚀 **安装说明**

### 快速安装
1. 下载 `Claude Suite_1.1.5_aarch64.dmg` (Apple Silicon) 或 `Claude Suite_1.1.5_x64.dmg` (Intel)
2. 双击 DMG 文件，将应用拖拽到 Applications 文件夹
3. 首次运行时，在"系统偏好设置 → 安全性与隐私"中允许应用运行
4. 启动应用，自动检测 Claude CLI 安装

### Claude CLI 安装
如果尚未安装 Claude CLI：
```bash
# 推荐通过 NVM 安装
npm install -g @anthropic-ai/claude-code

# 验证安装
claude --version
```

---

## 🔧 **故障排除**

### Claude CLI 检测问题
如果应用显示"Claude CLI not found"：

1. **检查安装**:
   ```bash
   which claude
   claude --version
   ```

2. **通过 NVM 重新安装**:
   ```bash
   npm install -g @anthropic-ai/claude-code
   ```

3. **手动指定路径**: 在设置中选择正确的 Claude CLI 路径

4. **刷新检测**: 点击左上角状态指示器的刷新按钮

---

## 🛠️ **技术架构**

### 前端
- React 18.3.1 + TypeScript 5.6.2
- Tailwind CSS 4.1.8 + Framer Motion
- i18next 国际化支持

### 后端
- Tauri 2.1.1 + Rust 2021
- SQLite 数据库
- macOS 原生 API 集成

---

## 🏗️ **构建信息**

- **构建工具**: Tauri 2.1.1 + Vite 6.0.3
- **包管理器**: Bun (推荐) / npm
- **目标平台**: macOS (Apple Silicon + Intel)
- **包格式**: DMG 安装包

---

## 🙏 **致谢**

### 社区贡献者
特别感谢 Linux.do 社区的杰出贡献者：

- **[xiniah](https://linux.do/u/xiniah)** - 原创项目作者，提供完整功能设计基础
- **[anyme](https://linux.do/u/anyme)** - Windows 版本开发者，提供跨平台技术参考
- **[@getAsterisk/claudia](https://github.com/getAsterisk/opcode)** - 原始架构来源

### 开源项目
- [Claude](https://claude.ai/) - 强大的 AI 助手
- [Tauri](https://tauri.app/) - 现代桌面应用框架
- [React](https://react.dev/) - 用户界面库
- [Rust](https://rust-lang.org/) - 系统编程语言

---

## 📞 **支持与反馈**

- **GitHub Issues**: [报告问题](https://github.com/lucasmen9527/claude-suite/issues)
- **GitHub Discussions**: [社区讨论](https://github.com/lucasmen9527/claude-suite/discussions)
- **项目文档**: [README.md](https://github.com/lucasmen9527/claude-suite#readme)

---

## 🔗 **相关链接**

- **项目主页**: https://github.com/lucasmen9527/claude-suite
- **下载页面**: [Releases](https://github.com/lucasmen9527/claude-suite/releases)
- **技术文档**: [CLAUDE.md](https://github.com/lucasmen9527/claude-suite/blob/main/CLAUDE.md)

---

**🍎 专为 macOS 用户优化的 Claude CLI 管理工具**

**Made with ❤️ for macOS users**