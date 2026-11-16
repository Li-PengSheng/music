# 任务：TASK-13-DESKTOP-LYRICS — 独立置顶桌面悬浮歌词窗口

## 基线

- 分支与 HEAD：main / bad7f2ec8cf744adc04edd4627b9f701a5ee0d1a
- 未提交改动：开工时工作区干净。
- 工作区指纹：`62d24cc707af61fd14f749dbd7a65d8c310485b20e2fead9c101957ec6eacc4f`，验收前后相同。排除 current-state 和本任务的进行中/归档位置。

## 状态

- 状态：完成
- 依赖与前置任务：现有歌词分组 `groupLines` / `activeLine`、播放栏和正在播放页、`desktop-v1.json`。
- 下一步：等待用户检视。
- 提交：未提交
- 遗留事项的任务 ID：无新增。实机窗口行为仍是 NOT RUN，没有单独任务 ID。

## 范围

- 要改：可复用的 `lyrics-overlay` 窗口，显示/隐藏与鼠标穿透，位置和锁定偏好，主窗口播放栏、正在播放页和托盘解锁入口，悬浮歌词视图。
- 不要改：音频引擎、数据库、用户真实曲库和设置、主窗口 `capabilities/default.json`、`tauri.conf.json`。
- 是否涉及音频、数据库、IPC 或用户数据目录：新增 IPC `desktop_lyrics_window`，并写入 `desktop-v1.json` 的两个新字段。旧文件缺字段时保持原行为。验收没有打开用户数据。

## 验收

- [x] 自动测试：PASS。`pnpm format` 与 `pwsh -File scripts/check.ps1` 退出 0。Svelte 0 error / 0 warning，前端 74/74，Rust fmt / clippy 通过，Rust 单元 92 通过、1 ignored，音频/歌词集成 3/6 通过。新增旧偏好反序列化兼容和窗口边界测试。
- [x] 浏览器验证：PASS。`pnpm test:ui` 27/27、退出 0。覆盖歌词路由分流、双语偏移高亮、无歌词和未播放、播放栏开关与解锁。相对旧图只有新增图标；更新 9 张快照后复查为零像素差异。
- [ ] Windows Tauri 实机：NOT RUN。
- [x] 未打开用户正在使用的曲库和设置

实机必须使用按 `docs/isolation.md` 构建且绑定哈希来源的 exe。本次没有构建或启动 exe，也没有修改 `src-tauri/tauri.conf.json`。

## 来源

- 任务 ID：TASK-13-DESKTOP-LYRICS
- 时间：2026-11-16
- HEAD：bad7f2ec8cf744adc04edd4627b9f701a5ee0d1a
- 工作区指纹：`62d24cc707af61fd14f749dbd7a65d8c310485b20e2fead9c101957ec6eacc4f`。命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/current-state.md,docs/tasks/TASK-13-DESKTOP-LYRICS.md,docs/tasks/done/TASK-13-DESKTOP-LYRICS.md`。验收前后相同。
- 命令、工具版本、构建配置：pnpm 12.21.0、Node v24.14.0、rustc 1.99.0。`pnpm format` 退出 0。`pnpm test:ui` 退出 0。`pwsh -File scripts/check.ps1` 退出 0，其中包含 Prettier 检查、svelte-check、前端单测、`cargo fmt --check`、`cargo clippy --all-targets -- -D warnings` 和 `MUSIC_MINE_NULL_OUTPUT=1` 的 `cargo test`。
- 产物哈希：无。没有构建 exe。
- 三层结果和未验证项：自动 PASS，浏览器 PASS，Windows Tauri 实机 NOT RUN。置顶、拖动、鼠标穿透、跳过任务栏和进程退出时关闭窗口都没有在 WebView2 里操作。

## 结束报告

- 改动文件：`platform/desktop.rs`、`lib.rs`、`capabilities/lyrics-overlay.json`、生成的 `capabilities.json`、`App.svelte`、`DesktopLyrics.svelte`、`DesktopLyricsButton.svelte`、`PlayerBar.svelte`、`NowPlaying.svelte`、`music.ts`、偏好与 UI store、`tokens.css`、harness、Playwright、IPC/架构/约定文档和 9 张快照。
- 三条验收结果：自动 PASS；浏览器 PASS；Windows Tauri 实机 NOT RUN。
- 未验证项：构建出的 exe 上的窗口创建、拖动、穿透锁定、托盘解锁、置顶和退出清理。浏览器 harness 不是 Rust 窗口。
- Git：未提交，不自动 commit/push。
- 已完成，归档到 `docs/tasks/done/`。
