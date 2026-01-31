# Ralph Desktop

**Start vague. Ralph Loop until perfect.**

A visual controller for AI coding agents that helps you brainstorm requirements and execute tasks through persistent iteration.

[中文文档](./README.zh-CN.md)

---

## Why Ralph Desktop?

### The Problem

Using AI coding agents like Claude Code or Codex can be frustrating:

| Pain Point | What Happens |
|------------|--------------|
| **"I don't know how to write prompts"** | You give vague instructions, AI produces garbage |
| **"AI runs once and stops"** | You manually retry over and over, hoping for better results |
| **"Setting up Ralph Loop is hard"** | The methodology exists, but configuring bash while loops and managing state is tedious for newcomers |

### The Solution

Ralph Desktop solves all three:

1. **AI Brainstorm** — Don't write prompts. Just describe your vague idea, and AI will interview you to clarify requirements and generate a high-quality prompt automatically.

2. **Ralph Loop Execution** — One click to start, then AI iterates autonomously until the task is complete or hits your configured limit.

3. **Visual Control** — Real-time logs, pause/resume/stop anytime. You're always in control.

---

## What is Ralph Loop?

**Ralph Loop** is an AI coding methodology introduced by [Geoffrey Huntley](https://ghuntley.com/). Named after Ralph Wiggum from *The Simpsons* — a character who seems simple but persists relentlessly — the approach embodies "brute-force elegance."

### How It Works

Instead of running an AI agent once and hoping for the best, Ralph Loop:

1. **Iterates continuously** — The AI executes, reviews its own output, fixes mistakes, and repeats
2. **Uses fresh context each iteration** — Avoids "context rot" by treating each loop as a new conversation with the spec as source of truth
3. **Stops on completion signal** — Detects when the task is truly done (via `<done>COMPLETE</done>` or similar markers)

### Why It's Effective

- **Quantity breeds quality** — Like Monte Carlo tree search, many attempts find better solutions
- **Self-correction** — AI catches and fixes its own errors over multiple passes
- **Works for complex tasks** — Breaks down large problems that exceed a single context window

> *"Dumb loop. Smart results."*

---

## How It Works

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   1. DESCRIBE   │ ──▶ │  2. BRAINSTORM  │ ──▶ │    3. LOOP      │ ──▶ │   4. CONTROL    │
│                 │     │                 │     │                 │     │                 │
│  "I want to     │     │  AI asks smart  │     │  AI executes    │     │  Watch logs,    │
│   build a..."   │     │  questions,     │     │  repeatedly     │     │  pause/resume,  │
│                 │     │  generates      │     │  until done     │     │  stop anytime   │
│                 │     │  perfect prompt │     │                 │     │                 │
└─────────────────┘     └─────────────────┘     └─────────────────┘     └─────────────────┘
```

1. **Describe** — Start with a vague idea. No need for perfect prompts.
2. **Brainstorm** — AI interviews you through Socratic dialogue, then generates a detailed task specification.
3. **Loop** — Click "Start" and Ralph Loop takes over, iterating until the task is complete.
4. **Control** — Monitor progress in real-time. Pause, resume, or stop at any point.

---

## Features

- **AI Brainstorm** — Conversational requirement gathering with automatic prompt generation
- **Ralph Loop Engine** — Persistent iteration with completion detection
- **Multi-CLI Support** — Works with Claude Code, Codex, and OpenCode
- **Visual Dashboard** — Real-time logs with ANSI color support
- **Execution Control** — Pause / Resume / Stop at any time
- **Project Management** — Create, switch, and manage multiple projects
- **Task Recovery** — Detect and resume interrupted tasks
- **Keyboard Shortcuts** — Cmd+N (new project), Cmd+, (settings), Cmd+? (help)
- **Theme Support** — Light / Dark / System
- **Multi-language UI** — 12 languages supported (see below)

---

## Supported CLIs

Ralph Desktop works with any AI coding CLI that supports headless execution:

| CLI | Status | Installation |
|-----|--------|--------------|
| [Claude Code](https://github.com/anthropics/claude-code) | ✅ Supported | `npm install -g @anthropic-ai/claude-code` |
| [Codex](https://github.com/openai/codex) | ✅ Supported | `npm install -g @openai/codex` |
| [OpenCode](https://github.com/opencode-ai/opencode) | 🚧 Coming Soon | — |

**Prerequisites:** You must have at least one of the above CLIs installed before using Ralph Desktop.

---

## Supported Languages

Ralph Desktop supports 12 languages out of the box:

| Language | Code | Language | Code |
|----------|------|----------|------|
| English | `en` | Português | `pt` |
| 简体中文 | `zh-CN` | Русский | `ru` |
| 繁體中文 | `zh-TW` | 日本語 | `ja` |
| Español | `es` | Deutsch | `de` |
| हिन्दी | `hi` | Français | `fr` |
| العربية | `ar` | বাংলা | `bn` |

Change language in **Settings** (Cmd+,). The AI Brainstorm conversation automatically adapts to your language.

---

## Installation

Download the latest release for your platform:

| Platform | Download |
|----------|----------|
| macOS (Intel / Apple Silicon) | [.dmg](https://github.com/liuxiaopai-ai/ralph-desktop/releases/latest) |
| Windows 10/11 (64-bit) | [.exe](https://github.com/liuxiaopai-ai/ralph-desktop/releases/latest) |
| Linux (x86_64) | [.AppImage](https://github.com/liuxiaopai-ai/ralph-desktop/releases/latest) |

> **Note:** The app is not code-signed. On macOS, right-click the app and select "Open", or go to System Settings → Privacy & Security → Open Anyway. On Windows, click "More info" → "Run anyway" when SmartScreen appears.

**Linux AppImage (first run):**
```bash
chmod +x Ralph.Desktop_*.AppImage
./Ralph.Desktop_*.AppImage
```

---

## Quick Start

1. **Install a CLI** — Make sure you have Claude Code, Codex, or OpenCode installed
2. **Download Ralph Desktop** — Get the installer for your platform
3. **Create a Project** — Click "New Project" (Cmd+N) and select a working directory
4. **Brainstorm** — Describe what you want to build, let AI help you clarify
5. **Start Loop** — Click "Start Execution" and watch the magic happen

---

## Development

### Prerequisites

- Node.js 18+
- pnpm
- Rust 1.70+
- Tauri CLI

### Setup

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

### Tech Stack

- **Frontend:** Svelte 5 + TypeScript + Tailwind CSS 4
- **Backend:** Rust + Tauri 2.x
- **Build:** Vite + Cargo

---

## Data Storage

Application data is stored at:

| Platform | Path |
|----------|------|
| macOS | `~/Library/Application Support/com.ralph.desktop/` |
| Windows | `%APPDATA%/com.ralph.desktop/` |
| Linux | `~/.config/com.ralph.desktop/` |

---

## Credits

- **Ralph Loop Methodology** — [Geoffrey Huntley](https://ghuntley.com/)
- **Built with** — [Tauri](https://tauri.app/), [Svelte](https://svelte.dev/), [Rust](https://www.rust-lang.org/)

---

## License

MIT
