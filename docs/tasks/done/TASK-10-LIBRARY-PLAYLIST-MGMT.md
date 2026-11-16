# 任务：TASK-10-LIBRARY-PLAYLIST-MGMT — 曲库文件夹移除与歌单内排序

## 基线

- 分支与 HEAD：`main` / `2f0f0170ebabf8fe9cd06f64a239be069fbac86e`
- 未提交改动：原有未跟踪 `repomix-output.md`，保留。
- 工作区指纹：`8c1a6104f3628122fc577eb3fd0a40856005038395d540ca14f90f211be0623e`，最终验收前后相同；排除本任务两个位置的记录和 current-state（命令见来源）。

## 状态

- 状态：完成
- 依赖与前置任务：复用现有 SQLite 删除目录、歌单移动和 Bits UI 确认/菜单能力。
- 下一步：等待用户检视；无自动提交。
- 提交：未提交
- 遗留事项的任务 ID：无新增发布事项，Windows Tauri 实机结果单独记录。

## 范围

- 要改：复用 remove_folder / move_in_playlist，新增 library_remove_folder 和 playlist reorder；确认删除仅影响数据库记录，歌单上下移支持边界禁用与 Alt+方向键。
- 不要改：数据库表结构、音频引擎、用户真实曲库/音乐/歌词/设置、现有快照。
- 是否涉及音频、数据库、IPC 或用户数据目录：涉及 SQLite 记录与 IPC；只在临时测试库和 harness 验证，不访问系统用户数据目录。

## 验收

- [x] 自动测试：PASS，format / format:check / Svelte 0 error 0 warning / 73 项前端测试 / Rust fmt / clippy / Rust 单元 82 通过、1 ignored / 音频与歌词集成 3+6 通过；scripts/check.ps1 退出 0。
- [x] 浏览器验证：PASS，pnpm test:ui 19/19，全部既有快照零像素差异且未更新；覆盖取消/失败/成功、边界与搜索禁用、焦点/播报、跨第 100 首分页索引。
- [x] Windows Tauri 实机：PASS（本任务范围），隔离构建中的真实 IPC、快捷键/焦点与目录移除 UI；空歌单、缺参、越界、重复移除被拒绝。专用 TEMP 数据库以 fixture 行播种；三个复制自仓库的 tone.flac 字节哈希不变。无真实曲库扫描或播放。
- [x] 未打开用户正在使用的曲库和设置；无数据库表结构变更，未修改磁盘上的真实音乐文件。

实机必须使用构建出的 exe。会写数据时按 `docs/isolation.md`。不要修改 `src-tauri/tauri.conf.json`。

## 来源

算法见 `docs/testing.md`。指纹排除验收记录本身，源码/测试/IPC 文档及原有 repomix-output.md 仍计入。

- 任务 ID：TASK-10-LIBRARY-PLAYLIST-MGMT
- 时间：2026-11-16
- HEAD：`2f0f0170ebabf8fe9cd06f64a239be069fbac86e`
- 工作区指纹：`8c1a6104f3628122fc577eb3fd0a40856005038395d540ca14f90f211be0623e`，最终验收前后相同；排除本任务两个位置的记录和 current-state（命令见来源）。
- 命令、工具版本、构建配置：pnpm 12.21.0、Node 24.14.0、rustc 1.99.0、WebView2 157.0.4341.46。`pnpm format` → `pnpm test:ui` → `pwsh -File scripts/check.ps1` 均退出 0；另 `pnpm test` 73/73。`pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/current-state.md,docs/tasks/TASK-10-LIBRARY-PLAYLIST-MGMT.md,docs/tasks/done/TASK-10-LIBRARY-PLAYLIST-MGMT.md`。实机由 `scripts/build-isolated.ps1 -Config tests/perf/out/task10-isolation.json -TestRoot <专用 TEMP 根>` 构建，不改主配置，使用 `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9237` 与 `MUSIC_MINE_NULL_OUTPUT=1`。`node tests/perf/out/task10-native.mjs` 退出 0；结束后核对 PID/路径/启动时间并关闭测试进程。
- 产物哈希：exe SHA-256 `25AB016177D4115D38961834C6BB9DAF742D23577A5EB657CEE10EE6D021EF1B`，config SHA-256 `247D235D4C66BF98F10400FB26BD4639FD2B06ED2859DBC238B408281B876955`。来源记录为 `tests/perf/out/isolation-sources/25AB016177D4115D38961834C6BB9DAF742D23577A5EB657CEE10EE6D021EF1B.json`，综合门禁后重建隔离产物并复验 IPC/UI，最终 check-source 返回 OK；结果 `tests/perf/out/task10-native-result.json`。数据目录 `C:\Users\myuli\AppData\Local\Temp\music-mine-task10-af4d74b507e04b3aaaf59db8c7599a5a\data`。三个 fixture 的 SHA-256 均为 `6a04ea578be1751513ce4ed20aeedb5212a2fe2e179c3c7353463e89a8d077da`。
- 三层结果和未验证项：自动测试 / 浏览器 / 本任务隔离实机均 PASS；真实音频、DPI、托盘不在本任务验收范围。保留既有 Cargo PDB 文件名冲突警告；Clippy 与 Svelte 无警告。

## 结束报告

- 改动文件：Rust catalog/db.rs、collections.rs、commands/catalog.rs、catalog/mod.rs、lib.rs；前端 music.ts、actions.ts、TrackList.svelte、SettingsView.svelte；mock-ipc.js、前端单测、Playwright 新用例；IPC 契约与 current-state。
- 三条验收结果：自动测试 PASS；浏览器 PASS；Windows Tauri 隔离实机 PASS（管理功能）。
- 未验证项：真实音频、真实曲库规模、DPI/托盘；用户真实数据未参与。
- Git：未提交，原有 repomix-output.md 保留。
- 已完成，归档至 docs/tasks/done/TASK-10-LIBRARY-PLAYLIST-MGMT.md。
