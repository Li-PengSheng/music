# 任务：TASK-11-AUDIO-OUTPUT-DEVICE — 音频输出设备选择与热切换

## 基线

- 分支与 HEAD：main / 10e733cb13267e7e5ae34bd08d29d567254cff24
- 未提交改动：无，工作区干净。
- 工作区指纹：`d9cdff6c14ea2b728ce3a2aa050ea9bfc9b2d7df61f7f15a683e490b769215b3`，验收前后相同，排除 current-state 与本任务两种归档位置。

## 状态

- 状态：完成
- 依赖与前置任务：现有 cpal 共享输出、音频线程、Controller 同步屏障与 desktop-v1.json。
- 下一步：等待用户检视。
- 提交：未提交
- 遗留事项的任务 ID：无新增。

## 范围

- 要改：设备枚举及首选设备回退；播放/暂停精确位置恢复；独立 IPC 与偏好；设置页和契约测试。
- 不要改：数据库结构、解码器、共享输出模式、回调实时安全边界、用户真实文件及设置、既有截图。
- 是否涉及音频、数据库、IPC 或用户数据目录：音频与 IPC，偏好仅在隔离数据目录和临时测试目录验收。

## 验收

- [x] 自动测试：PASS；pnpm format 和 scripts/check.ps1 退出 0，格式通过、Svelte 0 error / 0 warning、前端 74/74、Rust fmt / clippy 通过、Rust 87 通过与 1 ignored、音频/歌词集成 3/6 通过。Rust 测试启用 MUSIC_MINE_NULL_OUTPUT，不作为扬声器出声证明。
- [x] 浏览器验证：PASS；pnpm test:ui 22/22、退出 0，既有 8 张截图零像素差异，未更新快照。新增选择/默认、断开项、切换错误和枚举失败恢复。
- [x] Windows Tauri 实机：PASS（本任务静音范围）；隔离 exe 的真实 IPC 与 Select、两个实际 cpal 共享输出切换、未知设备回退、播放/暂停进度保持与偏好/会话重启恢复通过。
- [x] 未打开用户正在使用的曲库和设置

实机必须使用构建出的 exe。会写数据时按 docs/isolation.md，不修改主 Tauri 配置。

## 来源

- 任务 ID：TASK-11-AUDIO-OUTPUT-DEVICE
- 时间：2026-11-16
- HEAD：10e733cb13267e7e5ae34bd08d29d567254cff24
- 工作区指纹：`d9cdff6c14ea2b728ce3a2aa050ea9bfc9b2d7df61f7f15a683e490b769215b3`。命令：`pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/current-state.md,docs/tasks/TASK-11-AUDIO-OUTPUT-DEVICE.md,docs/tasks/done/TASK-11-AUDIO-OUTPUT-DEVICE.md`。已忽略的隔离产物不计入。
- 命令、工具版本、构建配置：pnpm 12.21.0、Node 24.14.0、rustc 1.99.0、WebView2 157.0.4341.46。依次 pnpm format → pnpm test:ui → pwsh -File scripts/check.ps1，全部退出 0。综合门禁完成后通过 `scripts/build-isolated.ps1 -Config tests/perf/out/task11-isolation.json -TestRoot <专用 TEMP 根>` 构建（包含 pnpm build），退出 0，不改主配置。新建数据目录为 `C:\Users\myuli\AppData\Local\Temp\music-mine-task11-91e838e267154c97869b9e5352131a64\data`。`task11-app.ps1` 启动前校验来源，使用进程级 WebView2 CDP 9238，未设置 MUSIC_MINE_NULL_OUTPUT；按 PID/路径/启动时间关闭和重启测试进程，最终已关闭。
- 产物哈希：exe SHA-256 `F49EF5F9DB8E6D8EC3DE71E561310E97A4E469EC383AED3A6E6287764BBFB41B`；config SHA-256 `F242724C6D2CEACD5C3D980037A7034F38404A868ABD697A11CF64258C76E5DF`。exe 路径为 `src-tauri/target/debug/music-mine-desktop.exe`；记录位于 `tests/perf/out/isolation-sources/<exe SHA-256>.json`，最终 check-source 为 OK。`node tests/perf/out/task11-native.mjs`、重启后 `--restart`、`task11-alternate.mjs` 均退出 0；完整结果在 `tests/perf/out/task11-native-result.json`。
- 实机细节：枚举到 `Speakers (Realtek(R) Audio)` 和 `Realtek Digital Output (Realtek(R) Audio)`。指定 Speakers、未知设备回退、播放中切换、暂停位置保持均通过；实际设置页选择数字输出后快照的 actual device 等于所选设备，再恢复 Speakers。偏好不被窗口设置覆盖。重启恢复 Speakers / paused / 530ms / volume 0。仅在专用 TEMP 库插入 fixture 行并播放复制的 tone.flac，SHA-256 始终为 `6a04ea578be1751513ce4ed20aeedb5212a2fe2e179c3c7353463e89a8d077da`。
- 三层结果和未验证项：自动 / 浏览器 / 隔离 Windows 静音实机均 PASS；物理拔插、实际出声、DPI/托盘 NOT RUN。保留既有 Cargo PDB 命名冲突警告，Clippy 与 Svelte 无警告。

## 结束报告

- 改动文件：audio/player.rs、playback/controller.rs、platform/desktop.rs、lib.rs；music.ts、SettingsView.svelte、mock-ipc.js；IPC 契约、当前状态、本任务记录和新增前端/Playwright 测试。
- 三条验收结果：自动测试 PASS，浏览器 PASS，Windows Tauri 隔离实机 PASS（静音管理功能）。
- 未验证项：物理拔插、实际出声、DPI/托盘；不扩展为完整发布验收。
- Git：不自动 commit 或 push。
- 已完成，归档到 docs/tasks/done/。
