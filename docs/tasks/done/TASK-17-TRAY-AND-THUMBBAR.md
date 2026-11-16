# 任务：托盘右键菜单与 Windows 任务栏缩略图按钮

## 基线

- 分支与 HEAD：`main`，`f7cdaf4055eec1160dfed55d8a303094bd8869e8`
- 未提交改动：开工时工作区已有未提交的 TASK-16（侧栏标记、`tauri.conf.json` 图标列表、生成图标和 8 张快照）。本次没有回退这些文件。
- 工作区指纹：开工前 `4e03a04c9449a407d48a4dd7848511550db86827331d26ee413ef9a2491abd88`（当时还排除了本任务将改的 `docs/architecture.md`、`docs/ipc-contract.md`、`docs/rust-guidelines.md`）。验收指纹 `c9ba3bae8e94dad90d0b1f1f024dd95daaf26b2f54dbc95868175f29b5bcaa49`。命令为 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/current-state.md,docs/tasks/TASK-17-TRAY-AND-THUMBBAR.md,docs/tasks/done/TASK-17-TRAY-AND-THUMBBAR.md`。两枚指纹排除范围不同，不能直接对比。

## 状态

- 状态：完成
- 依赖与前置任务：现有托盘、桌面歌词窗口、播放控制器和 Windows SMTC
- 下一步：在隔离构建的 exe 里看真实托盘菜单和任务栏按钮
- 提交：待提交
- 遗留事项的任务 ID：库外打开、WAV 经界面打开仍没有单独任务 ID。托盘和任务栏按钮的实机验收没有新任务 ID，仍记为 NOT RUN

## 范围

- 要改：托盘菜单、Tooltip、播放模式勾选、桌面歌词菜单项、Windows `ITaskbarList3` 缩略图按钮、`playback_mode` 的可见性、`windows` 0.58 的 Win32 feature
- 不要改：音频回调、解码器、曲库数据库、前端界面、`tauri.conf.json`、IPC 命令名单
- 是否涉及音频、数据库、IPC 或用户数据目录：播放命令仍进入现有 `Controller`。不新增 IPC。不打开用户的曲库、歌词或设置

## 验收

- [x] 自动测试：PASS
- [x] 浏览器验证：PASS
- [ ] Windows Tauri 实机：NOT RUN
- [x] 未打开用户正在使用的曲库和设置

实机必须使用构建出的 exe。会写数据时按 `docs/isolation.md`。不要修改 `src-tauri/tauri.conf.json`。

## 来源

- 任务 ID：TASK-17-TRAY-AND-THUMBBAR
- 时间：2026-11-16
- HEAD：`f7cdaf4055eec1160dfed55d8a303094bd8869e8`
- 工作区指纹：`c9ba3bae8e94dad90d0b1f1f024dd95daaf26b2f54dbc95868175f29b5bcaa49`
- 命令、工具版本、构建配置：`pnpm format`；`pnpm test:ui`；`pwsh -File scripts/check.ps1`。Node v24.14.0，pnpm 12.21.0，rustc 1.99.0，cargo 1.99.0。`windows` crate 仍是 0.58.0。`Cargo.lock` 没有变化，因为没有新 crate，也没有升级版本。额外的 `Win32_System_Com` 用来调用 `CoCreateInstance`，Shell feature 不包含它。
- 产物哈希：未构建 exe，无产物哈希
- 三层结果和未验证项：自动测试与浏览器 harness 通过。真实托盘、任务栏按钮和系统媒体键没有在 exe 里看过。

## 结束报告

- 改动文件：`src-tauri/src/platform/desktop.rs`、`src-tauri/src/platform/thumbbar.rs`、`src-tauri/src/playback/controller.rs`、`src-tauri/Cargo.toml`、`docs/architecture.md`、`docs/ipc-contract.md`、`docs/rust-guidelines.md`、`docs/current-state.md`
- 三条验收结果：
  - 自动测试 PASS。`pnpm format` 与 `pwsh -File scripts/check.ps1` 退出 0。Prettier 通过，Svelte 0 error / 0 warning，前端 83/83。Rust fmt / clippy 通过。Rust 单元 93 通过、1 ignored。音频集成 3 通过，歌词集成 6 通过。既有 Cargo PDB 文件名冲突警告仍在。
  - 浏览器验证 PASS。`pnpm test:ui` 34/34，退出 0。本次没有改前端，既有快照零像素差异。
  - Windows Tauri 实机 NOT RUN。
- 未验证项：没有启动构建出的 exe，因此没有看见真实托盘 Tooltip、右键菜单、左键隐藏，也没有看见任务栏上的四个按钮。未访问用户数据。
- Git：未提交
- 完成后把本记录移到 `docs/tasks/done/`。进行中的任务不要和历史记录放在一起
