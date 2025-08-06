# Debug Build Workflow

## 概述

这个工作流用于生成带有调试信息的 Claude Suite 构建版本。与正式发布版本不同，debug build 包含：

- 完整的调试符号
- 开发者工具支持（devtools feature）
- 更详细的日志输出
- 未优化的代码（便于调试）

## 如何触发

### 方法 1: 提交消息触发
在提交消息中包含 `[debug]` 标签：

```bash
git commit -m "fix: 修复设置页面问题 [debug]"
git push origin main
```

### 方法 2: Pull Request 标题触发
在 Pull Request 标题中包含 `[debug]`：

```
[debug] Fix settings page issue
```

## 构建产物

工作流将为以下平台生成 debug 版本：
- `claude-suite-debug-windows-latest`: Windows debug 版本
- `claude-suite-debug-macos-latest`: macOS debug 版本  
- `claude-suite-debug-ubuntu-latest`: Linux debug 版本

### 构建路径
- Windows: `src-tauri/target/debug/bundle/` 和 `src-tauri/target/debug/claude-suite.exe`
- macOS: `src-tauri/target/debug/bundle/` 和 `src-tauri/target/debug/claude-suite`
- Linux: `src-tauri/target/debug/bundle/` 和 `src-tauri/target/debug/claude-suite`

## 特性

### 1. 条件触发
- 仅在提交消息或 PR 标题包含 `[debug]` 时运行
- 支持 push 和 pull_request 事件
- 跨平台构建（Windows, macOS, Linux）

### 2. 工件保留
- 构建产物保留 7 天（相比正式版本的 30 天）
- 自动上传到 GitHub Actions 工件存储

### 3. 调试功能
- 启用 Tauri devtools 特性
- 包含完整调试符号
- 显示详细构建信息

## 与正式构建的区别

| 特性 | Debug Build | Release Build |
|------|-------------|---------------|
| 触发条件 | `[debug]` 标签 | 正常 push/PR |
| 优化级别 | 未优化 | 完全优化 |
| 调试符号 | 包含 | stripped |
| 文件大小 | 较大 | 较小 |
| 启动速度 | 较慢 | 较快 |
| 调试能力 | 完整支持 | 有限 |
| 保留时间 | 7 天 | 30 天 |

## 使用建议

### 何时使用 debug build
- 调试复杂问题时
- 需要详细错误信息时
- 开发新功能需要调试时
- 用户报告问题需要深入分析时

### 注意事项
- Debug 版本文件较大，下载时间较长
- 运行速度比 release 版本慢
- 仅用于调试目的，不建议日常使用
- 工件保留时间较短（7天），请及时下载

## 工作流文件

工作流定义文件：`.github/workflows/debug-build.yml`

如需修改工作流行为，请编辑此文件。