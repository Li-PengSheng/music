# 任务：移除顶栏冗余菜单，侧栏直达控制台

## 基线

- 分支与 HEAD：`main` / `ef60136108699cd30a7cb1d00057b3fa1ee0bada`。
- 未提交改动：保留完整 TASK-18-MENU-CONSOLE-SPECS 实现及文档、快照；保留用户既有 38 项图标/repomix 删除。本任务开始时的三份组件/测试副本和 git status 保存在已忽略的 `.checkpoints/task18-refine/`。
- 工作区指纹：`97dfadc0ad50775b0eb131040b7040184c611859457f9212f3a0c6e317d7fe84`，最终验收前后相同，排除本任务单与 docs/current-state.md。

## 状态

- 状态：待审阅（实现和自动/浏览器验收完成）。
- 依赖与前置任务：TASK-18-MENU-CONSOLE-SPECS 的 ConsoleModal 与 ui.consoleOpen。
- 下一步：用户审阅本次差异；不自动提交。
- 提交：待提交；不自动 commit。
- 遗留事项的任务 ID：本任务 Windows Tauri 实机验收。

## 范围

- 要改：删除 TopBar 菜单及其专用导入/样式；Sidebar 的设置上方增加 Terminal 图标「控制台」按钮，直接打开现有 Modal，随 consoleOpen 高亮；更新对应 UI 测试与受影响快照。
- 不要改：Console 行为、IPC、Rust、音频引擎、数据库、其他已有直达入口。
- 不涉及用户数据目录、依赖或主 Tauri 配置。

## 验收

- [x] 自动测试：PASS；pnpm format 与 pwsh -File scripts/check.ps1 均退出 0。Svelte 0 error / 0 warning；前端 83/83；Rust fmt/clippy 通过，单元 98 passed / 1 ignored，音频集成 3/3、歌词集成 6/6。检查使用空音频输出。既有 Cargo PDB 警告不变。
- [x] 浏览器验证：PASS；pnpm test:ui 42/42，退出 0。1280×800 / 900×640 已检查截图与控件边界，侧栏控制台和设置完整可见；直达、键盘 Enter、active/aria-expanded、Escape 焦点恢复、当前页面保持及本地模式共享激活状态均通过。8 张受影响快照经查看布局后更新，普通复跑零像素差异。
- [ ] Windows Tauri 实机：NOT RUN；上一轮 exe 不能代表这次入口布局。
- [x] 未打开用户正在使用的曲库和设置。

## 来源

- 任务 ID：TASK-18-REFINE-SIDEBAR-CONSOLE。
- 时间：2026-11-17。
- HEAD：同基线。
- 工作区指纹：同基线；命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-18-REFINE-SIDEBAR-CONSOLE.md,docs/current-state.md`。最终快照更新完成后、普通 UI/check.ps1 之前采集，结束后相同。
- 命令、工具版本、构建配置：pnpm 12.21.0、Node v24.14.0、Playwright 1.65.1、PowerShell 7.6.10；pnpm format、pnpm test:ui、pwsh -File scripts/check.ps1 全部退出 0。先运行侧栏入口测试确认缺失按钮导致失败，实现后相关 7 项测试通过；pnpm test:ui --update-snapshots 全量通过，随后普通 pnpm test:ui 42/42 再验零差异。Playwright 沿用上一轮沙箱外执行方式，以保证 Vite 子进程正常清理。
- 产物哈希：不适用，本轮未构建或启动 exe。浏览器截图位于已忽略的 test-results/menu-console-specs-minimal-68348-ith-keyboard-focus-restored/。
- 三层结果和未验证项：自动测试 PASS、浏览器 PASS、Windows Tauri 实机 NOT RUN。此前 TASK-18 的 exe 来源和结果仅属于此前指纹，不覆盖当前入口调整。

## 结束报告

- 改动文件：src/lib/layout/TopBar.svelte（删除菜单及专用导入/样式）、Sidebar.svelte（设置上方增加控制台按钮）、tests/ui/menu-console-specs.spec.ts（替换菜单相关用例）；8 张快照：home-dark/light、songs-dark/narrow、albums-dark、settings-dark、queue-dark、dialog-dark；本单与 docs/current-state.md。正在播放快照本轮未变。
- 三条验收结果：自动测试 PASS；浏览器 PASS；Windows Tauri 实机 NOT RUN。git diff --check 通过。
- 未验证项：本轮 Windows Tauri 实机。
- Git：未提交；保留原有工作，任务单待审阅。
