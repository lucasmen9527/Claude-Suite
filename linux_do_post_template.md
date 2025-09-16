# Claude Suite v1.1.5 macOS 修复版本发布 🍎

## 项目简介

Claude Suite 是一款专为 macOS 优化的 Claude CLI 桌面管理工具，提供可视化的项目管理、实时交互、智能代理和MCP支持。

**GitHub 仓库**: https://github.com/lucasmen9527/claude-suite
**最新版本**: v1.1.5 macOS 修复版

## 🎯 本次重大修复

本版本专门解决了 **macOS DMG 应用无法检测 Claude CLI 版本** 的关键问题！

### 核心问题
之前的版本在构建为 DMG 后，由于 macOS 应用启动时的环境变量限制，无法正确检测到通过 NVM 或 Homebrew 安装的 Claude CLI。

### 解决方案
✅ **智能 PATH 检测**: 自动扫描 NVM、Homebrew 等开发环境路径
✅ **环境变量增强**: 解决 macOS 应用启动时的环境变量限制
✅ **DMG 安装包**: 提供标准的 macOS 安装体验
✅ **原生权限管理**: 正确的 macOS 安全权限配置

## 🚀 主要特性

### 🎯 核心功能
- **项目管理**: 可视化管理 Claude 项目，支持会话历史和检查点
- **实时交互**: 流式显示 Claude 响应，支持 Markdown 渲染和语法高亮
- **智能代理**: Agent 系统支持 GitHub 集成和自动化任务执行
- **MCP 支持**: 完整的 Model Context Protocol 服务器管理

### 🔧 代理商管理（主要功能）
- **一键切换**: 静默切换不同的 Claude API 代理商，无弹窗干扰
- **隐私安全**: 本地存储配置，零硬编码敏感信息
- **自由配置**: 完整的 CRUD 操作界面，支持自定义代理商
- **立即生效**: 自动重启 Claude 进程，配置立即生效

### 🍎 macOS 特别优化
- **智能 PATH 检测**: 自动扫描 NVM、Homebrew 等开发环境路径
- **环境变量增强**: 解决 macOS 应用启动时的环境变量限制
- **DMG 安装包**: 提供标准的 macOS 安装体验
- **原生权限管理**: 正确的 macOS 安全权限配置

## 📦 下载和安装

### 系统要求
- **操作系统**: macOS 11.0+ (Big Sur 或更高版本)
- **Node.js**: 18.0+ (推荐通过 NVM 安装)
- **Claude CLI**: 需要预先安装 (`npm install -g @anthropic-ai/claude-code`)

### 安装步骤
1. 前往 [Releases 页面](https://github.com/lucasmen9527/claude-suite/releases)
2. 下载 macOS 安装包：`Claude Suite_x.x.x_aarch64.dmg`
3. 双击 DMG 文件，将应用拖拽到 Applications 文件夹
4. 首次运行时，可能需要在"系统偏好设置 → 安全性与隐私"中允许应用运行

## 🛠️ 技术架构

### 前端技术栈
- **React 18** - 现代化的用户界面框架
- **TypeScript** - 类型安全的开发体验
- **Tailwind CSS 4** - 实用优先的 CSS 框架
- **Framer Motion** - 流畅的动画效果
- **i18next** - 国际化支持

### 后端技术栈
- **Tauri 2** - 现代化的桌面应用框架 (macOS 优化)
- **Rust** - 高性能的系统编程语言
- **SQLite** - 嵌入式数据库
- **macOS API** - 原生 macOS 系统集成

## 🙏 致谢

### 社区贡献和启发

本项目基于 Linux.do 社区的杰出贡献者们的工作：

#### 🚀 核心架构来源
- **原项目架构**: [@getAsterisk/claudia](https://github.com/getAsterisk/opcode) - 提供了基础架构和设计思路

#### 🏆 Linux.do 社区贡献者

**[xiniah](https://linux.do/u/xiniah) 大佬** - 原创项目作者
- 📝 社区帖子：[【已停止更新，请看最新消息】Claude Suite: 基于claudia 一体化管理claude code](https://linux.do/t/topic/838591/136)
- 🔗 原始开源项目：[claude-suite](https://github.com/xinhai-ai/claude-suite)
- 💡 为本项目提供了完整的功能设计和实现基础

**[anyme](https://linux.do/u/anyme) 大佬** - Windows 版本开发者
- 📝 社区帖子：[[8月2日更新] Claude Workbench - 让 windows 使用 Claude Code更加高效便捷！](https://linux.do/t/topic/799521)
- 🔗 GitHub 仓库：[claude-workbench](https://github.com/anyme123/claude-workbench)
- 🛠️ 为跨平台开发提供了宝贵的技术参考

### 🍎 macOS 特别说明

本版本专门针对 macOS 平台进行了深度优化：
- 解决了 DMG 应用环境变量检测的关键技术难题
- 改善了 NVM 和 Homebrew 环境下的 Claude CLI 识别
- 提供了原生的 macOS 用户体验

## 📞 联系方式

- **GitHub Issues**: [Issues 页面](https://github.com/lucasmen9527/claude-suite/issues)
- **GitHub Discussions**: [讨论页面](https://github.com/lucasmen9527/claude-suite/discussions)

---

### 开源精神
感谢所有为开源社区做出贡献的开发者们。正是因为大家的无私分享，才让我们能够站在巨人的肩膀上，创造更好的工具。

**如果这个项目对您有帮助，请考虑给我们一个 ⭐**

🍎 专为 macOS 用户优化的 Claude CLI 管理工具
Made with ❤️ for macOS users

---

**备注**: 发布后请在此处添加实际的 Release 下载链接和版本号。