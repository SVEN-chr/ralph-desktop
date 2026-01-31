# Ralph Desktop

**从模糊开始，Ralph Loop 到完美。**

一个 AI 编程助手的可视化控制器，帮助你梳理需求并通过持续迭代执行任务。

[English](./README.md)

---

## 为什么选择 Ralph Desktop？

### 痛点

使用 Claude Code 或 Codex 等 AI 编程助手时，你可能遇到过这些问题：

| 痛点 | 结果 |
|------|------|
| **"我不会写 prompt"** | 给出模糊的指令，AI 产出一堆垃圾 |
| **"AI 跑一次就停了"** | 反复手动重试，祈祷这次能好一点 |
| **"配置 Ralph Loop 太麻烦"** | 方法论是好的，但配置 bash while 循环和状态管理对新手来说太折腾 |

### 解决方案

Ralph Desktop 一次性解决这三个问题：

1. **AI Brainstorm** — 不用写 prompt。只需描述你模糊的想法，AI 会通过对话帮你理清需求，自动生成高质量的 prompt。

2. **Ralph Loop 执行** — 一键启动，AI 自主迭代直到任务完成或达到你设置的上限。

3. **可视化控制** — 实时日志，随时暂停/恢复/停止。你始终掌控全局。

---

## 什么是 Ralph Loop？

**Ralph Loop** 是由 [Geoffrey Huntley](https://ghuntley.com/) 提出的 AI 编程方法论。名字来源于《辛普森一家》中的 Ralph Wiggum —— 一个看起来简单但坚持不懈的角色 —— 这个方法体现了"暴力美学"的精髓。

### 工作原理

不同于让 AI 跑一次就结束，Ralph Loop：

1. **持续迭代** — AI 执行、审查自己的输出、修复错误、重复
2. **每次迭代使用新上下文** — 避免"上下文腐烂"，每次循环都以规格说明为真理来源
3. **完成信号检测** — 检测任务何时真正完成（通过 `<done>COMPLETE</done>` 等标记）

### 为什么有效

- **量变产生质变** — 类似蒙特卡洛树搜索，大量尝试找到更优解
- **自我纠错** — AI 在多次迭代中发现并修复自己的错误
- **处理复杂任务** — 将超出单次上下文窗口的大问题分解处理

> *"傻循环，聪明结果。"*

---

## 工作流程

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   1. 描述想法    │ ──▶ │  2. AI 头脑风暴  │ ──▶ │   3. 循环执行    │ ──▶ │   4. 可视化控制  │
│                 │     │                 │     │                 │     │                 │
│  "我想做一个..." │     │  AI 提问引导，   │     │  AI 反复执行    │     │  查看日志，     │
│                 │     │  生成完美 prompt │     │  直到完成       │     │  随时暂停/停止  │
└─────────────────┘     └─────────────────┘     └─────────────────┘     └─────────────────┘
```

1. **描述** — 从模糊的想法开始，不需要完美的 prompt
2. **头脑风暴** — AI 通过苏格拉底式对话引导你，然后生成详细的任务规格
3. **循环** — 点击"开始执行"，Ralph Loop 接管，迭代直到任务完成
4. **控制** — 实时监控进度，随时暂停、恢复或停止

---

## 功能特性

- **AI Brainstorm** — 对话式需求收集，自动生成 prompt
- **Ralph Loop 引擎** — 持续迭代，完成信号检测
- **多 CLI 支持** — 兼容 Claude Code、Codex、OpenCode
- **可视化面板** — 实时日志，支持 ANSI 颜色
- **执行控制** — 随时暂停 / 恢复 / 停止
- **项目管理** — 创建、切换、管理多个项目
- **任务恢复** — 检测并恢复中断的任务
- **快捷键** — Cmd+N（新项目）、Cmd+,（设置）、Cmd+?（帮助）
- **主题支持** — 浅色 / 深色 / 跟随系统
- **多语言 UI** — 支持 12 种语言（见下方）

---

## 支持的 CLI

Ralph Desktop 可与任何支持无头执行的 AI 编程 CLI 配合使用：

| CLI | 状态 | 安装方式 |
|-----|------|----------|
| [Claude Code](https://github.com/anthropics/claude-code) | ✅ 已支持 | `npm install -g @anthropic-ai/claude-code` |
| [Codex](https://github.com/openai/codex) | ✅ 已支持 | `npm install -g @openai/codex` |
| [OpenCode](https://github.com/opencode-ai/opencode) | 🚧 即将支持 | — |

**前置要求：** 使用 Ralph Desktop 前，你必须至少安装上述 CLI 中的一个。

---

## 支持的语言

Ralph Desktop 开箱即支持 12 种语言：

| 语言 | 代码 | 语言 | 代码 |
|------|------|------|------|
| English | `en` | Português（葡萄牙语） | `pt` |
| 简体中文 | `zh-CN` | Русский（俄语） | `ru` |
| 繁體中文 | `zh-TW` | 日本語 | `ja` |
| Español（西班牙语） | `es` | Deutsch（德语） | `de` |
| हिन्दी（印地语） | `hi` | Français（法语） | `fr` |
| العربية（阿拉伯语） | `ar` | বাংলা（孟加拉语） | `bn` |

在 **设置**（Cmd+,）中切换语言。AI Brainstorm 对话会自动适配你的语言。

---

## 安装

下载适合你平台的最新版本：

| 平台 | 下载 |
|------|------|
| macOS（Intel / Apple 芯片通用） | [.dmg](https://github.com/liuxiaopai-ai/ralph-desktop/releases/latest) |
| Windows 10/11（64 位） | [.exe](https://github.com/liuxiaopai-ai/ralph-desktop/releases/latest) |
| Linux（x86_64） | [.AppImage](https://github.com/liuxiaopai-ai/ralph-desktop/releases/latest) |

> **注意：** 应用未进行代码签名。在 macOS 上，右键点击 App 选择“打开”，或进入“系统设置 → 隐私与安全性 → 仍要打开”。在 Windows 上，点击“更多信息” → “仍要运行”。

**Linux AppImage（首次运行）:**
```bash
chmod +x Ralph.Desktop_*.AppImage
./Ralph.Desktop_*.AppImage
```

---

## 快速开始

1. **安装 CLI** — 确保已安装 Claude Code、Codex 或 OpenCode
2. **下载 Ralph Desktop** — 获取适合你平台的安装包
3. **创建项目** — 点击"新建项目"（Cmd+N）并选择工作目录
4. **头脑风暴** — 描述你想做什么，让 AI 帮你理清需求
5. **开始循环** — 点击"开始执行"，见证奇迹发生

---

## 开发

### 前置要求

- Node.js 18+
- pnpm
- Rust 1.70+
- Tauri CLI

### 设置

```bash
# 安装依赖
pnpm install

# 开发模式运行
pnpm tauri dev

# 生产构建
pnpm tauri build
```

### 技术栈

- **前端：** Svelte 5 + TypeScript + Tailwind CSS 4
- **后端：** Rust + Tauri 2.x
- **构建：** Vite + Cargo

---

## 数据存储

应用数据存储位置：

| 平台 | 路径 |
|------|------|
| macOS | `~/Library/Application Support/com.ralph.desktop/` |
| Windows | `%APPDATA%/com.ralph.desktop/` |
| Linux | `~/.config/com.ralph.desktop/` |

---

## 致谢

- **Ralph Loop 方法论** — [Geoffrey Huntley](https://ghuntley.com/)
- **构建工具** — [Tauri](https://tauri.app/)、[Svelte](https://svelte.dev/)、[Rust](https://www.rust-lang.org/)

---

## 许可证

MIT
