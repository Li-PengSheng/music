# 任务：TASK-15-CHINESE-THEMES — 中式双模配色预设

## 基线

- 分支与 HEAD：main / bad7f2ec8cf744adc04edd4627b9f701a5ee0d1a
- 未提交改动：开工时已有 TASK-13、TASK-14 和歌词修复，全部保留。
- 工作区指纹：`71eb641c8fcf9b17f877c4aca3a9105bab35cf4d6f2b0216c813de50ff608ec7`，验收前后相同。排除 current-state 和本任务的进行中/归档位置。

## 状态

- 状态：完成
- 依赖与前置任务：现有明暗主题、`public/theme-init.js`、设置页 Select。
- 下一步：等待用户检视。
- 提交：未提交
- 遗留事项的任务 ID：无新增。实机配色仍是 NOT RUN。

## 范围

- 要改：四套色系与深浅模式正交。`celadon` 沿用原色。其余三套写入 `tokens.css`。设置页可立即切换，并写入 `localStorage`。
- 不要改：`theme` 的取值、Rust 主题偏好、IPC、音频、数据库和用户真实数据。
- 是否涉及音频、数据库、IPC 或用户数据目录：否。配色只在浏览器本地存储。

## 验收

- [x] 自动测试：PASS。`pnpm format` 与 `pwsh -File scripts/check.ps1` 退出 0。Svelte 0 error / 0 warning，前端 83/83，Rust fmt / clippy 通过，Rust 单元 92 通过、1 ignored，音频/歌词集成 3/6 通过。八组配色的主次文字对比度不低于 4.5:1。
- [x] 浏览器验证：PASS。`pnpm test:ui` 34/34、退出 0。朱砂深浅切换不改变音量和进度，刷新后仍在。默认青瓷下除设置页外的快照为零像素差异。只更新了 `settings-dark.png`。
- [ ] Windows Tauri 实机：NOT RUN
- [x] 未打开用户正在使用的曲库和设置

实机必须使用按 `docs/isolation.md` 构建的 exe。本次没有构建或启动 exe，也没有修改 `src-tauri/tauri.conf.json`。

## 来源

- 任务 ID：TASK-15-CHINESE-THEMES
- 时间：2026-11-16
- HEAD：bad7f2ec8cf744adc04edd4627b9f701a5ee0d1a
- 工作区指纹：`71eb641c8fcf9b17f877c4aca3a9105bab35cf4d6f2b0216c813de50ff608ec7`。命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/current-state.md,docs/tasks/TASK-15-CHINESE-THEMES.md,docs/tasks/done/TASK-15-CHINESE-THEMES.md`。
- 命令、工具版本、构建配置：pnpm 12.21.0、Node v24.14.0、rustc 1.99.0。`pnpm format`、`pnpm test:ui`、`pwsh -File scripts/check.ps1` 均退出 0。检查脚本包含 Prettier、svelte-check、前端单测、`cargo fmt --check`、`cargo clippy --all-targets -- -D warnings` 和 `MUSIC_MINE_NULL_OUTPUT=1` 的 `cargo test`。
- 产物哈希：无。
- 三层结果和未验证项：自动 PASS，浏览器 PASS，Windows Tauri 实机 NOT RUN。标题栏不随色系变色。

## 结束报告

- 改动文件：`tokens.css`、`theme.ts`、`theme-init.js`、`preferences.ts`、`ui.svelte.ts`、`SettingsView.svelte`、主题对比度测试、`tests/ui/palette.spec.ts`、`settings-dark.png`。
- 三条验收结果：自动 PASS；浏览器 PASS；Windows Tauri 实机 NOT RUN。
- 未验证项：构建出的 exe 上的四套配色，以及标题栏是否仍只跟随深浅模式。
- Git：未提交，不自动 commit/push。
- 已完成，归档到 `docs/tasks/done/`。Codex 留下的未闭合色块已补上。
