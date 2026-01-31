# AGENTS.md

## Project Overview
Ralph Desktop is a Tauri 2.x + Svelte 5 desktop app for orchestrating CLI coding agents (Claude Code/Codex/OpenCode).

## Repo Layout
- `src/`: Svelte frontend (components, stores, services, routes)
- `src-tauri/`: Rust backend (commands, engine, adapters, storage)

## Dev Commands
- Frontend dev: `pnpm dev`
- Tauri dev: `pnpm tauri dev`
- Frontend checks: `pnpm check`
- Rust tests: `cargo test` (run inside `src-tauri`)

## Conventions
- Keep UI aligned to the VS Code Dark Modern style (use the existing `bg-vscode-*`, `text-vscode-*`, `border-vscode` utility classes).
- All user-facing strings should use `svelte-i18n` keys from `src/lib/i18n/locales`.
- Brainstorm flow is AI-driven only; avoid re‑introducing preset/questionnaire flows unless explicitly requested.

## Data & Storage
- App data lives under `~/.ralph-desktop/` (config, projects, logs).

## Versioning
- Keep versions in sync across `package.json` and `src-tauri/tauri.conf.json`.
- Use `scripts/bump-version.mjs` when updating release versions.

