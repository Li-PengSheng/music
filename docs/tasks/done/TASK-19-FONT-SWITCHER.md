# 任务：本机字体预设与设置页实时切换

## 基线

- 分支与 HEAD：`main` / `ef60136108699cd30a7cb1d00057b3fa1ee0bada`。
- 未提交改动：完整保留两轮 TASK-18 的实现、记录与快照，以及用户既有 38 项图标/repomix 删除。开工 git status 与快照哈希保存在已忽略的 `.checkpoints/task19/`。
- 工作区指纹：`5bc4dd47b0cfe1cdc63b3394abf9ea4ba64f3475c9693f79ab2bd32d04e82fae`；最终验收前后相同，排除本任务单与 docs/current-state.md。

## 状态

- 状态：待审阅（实现与自动/浏览器验收完成）。
- 依赖与前置任务：现有主题首屏脚本、字体 token、preferences、Select；字体文件由系统提供。
- 下一步：用户审阅本次差异；不自动提交。
- 提交：待提交，不自动 commit。
- 遗留事项的任务 ID：TASK-19 的 Windows Tauri 隔离 exe 字体与 DPI 验收。

## 范围

- 要改：harmony/puhuiti/serif/kaiti/system 五套本机字体预设；默认 harmony；FONT_KEY=music-mine-font、parseFont/FONTS、ui.font、首屏与偏好读写/data-font；设置页配色下方选择器。
- 字体按用户提供的族名和顺序回退，不复制/下载字体，不新增依赖。系统预设保留原栈；等宽栈增加 Cascadia Code / Ubuntu Mono。
- 页面大标题、侧栏品牌、正在播放和桌面歌词使用 display token；serif 保持小字列表无衬线，桌面双语副歌词同样使用 display。补充同源 storage 监听，让已打开的歌词窗口实时响应；销毁时清理监听。
- 不要改：Rust、音频引擎、数据库、业务 IPC、原生主题 API、用户现有数据、主 Tauri 配置和此前入口精简。
- 方案：沿用现有 CSS token 与 localStorage 偏好通路；不额外构造字体加载服务或系统字体扫描命令。播放快照不受字体操作影响。

## 验收

- [x] 自动测试：PASS；pnpm format、pwsh -File scripts/check.ps1 均退出 0。Svelte 0 error / 0 warning，前端 85/85（字体相关单元 8/8），Rust fmt/clippy 通过、单元 98 passed / 1 ignored、音频集成 3/3、歌词集成 6/6。检查使用空音频输出；既有 Cargo PDB 警告不变。
- [x] 浏览器验证：PASS；pnpm test:ui 46/46，退出 0。新增 4 项字体 UI 测试，覆盖五套实时切换/重载、存储不可用、主题/配色/音量/进度保持、字体切换无 native/player IPC、宋体标题与双语歌词、已打开歌词窗口的 storage 同步及移除存储键后的默认恢复、1280×800 / 900×640 明暗主题长标签显示。9 张既有基线确认布局后更新，普通复跑零像素差异，未用 harness 字体覆盖掩盖默认变化。
- [ ] Windows Tauri 实机：NOT RUN；浏览器不能替代构建出的 exe。
- [x] 未打开用户正在使用的曲库和设置。

## 来源

- 任务 ID：TASK-19-FONT-SWITCHER。
- 时间：2026-11-17。
- HEAD：同基线。
- 工作区指纹：同基线；命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-19-FONT-SWITCHER.md,docs/current-state.md`。快照更新完成后、最终普通 UI/check.ps1 前采集，结束后相同。
- 命令、工具版本、构建配置：pnpm 12.21.0、Node v24.14.0、Playwright 1.65.1、PowerShell 7.6.10；pnpm format、pnpm test:ui、pwsh -File scripts/check.ps1 全部退出 0。字体单元与浏览器默认值回归先失败、实现后通过；字体与 Console 相关 UI 11/11；pnpm test:ui --update-snapshots 全量 46/46 后再普通复跑 46/46 验零差异。Playwright 沿用沙箱外执行方式，避免 Vite 子进程清理挂起。
- 字体来源：Chromium CDP 的 CSS.getPlatformFontsForNode 对品牌文字实际命中（均 isCustomFont=false）：harmony=HarmonyOS Sans SC / HarmonyOS_Sans_SC_Bold；puhuiti=Alibaba PuHuiTi 3.0 / AlibabaPuHuiTi_3_85_Bold；serif=Noto Serif SC / Noto-Serif-SC-Bold；kaiti=STKaiti；system=Segoe UI Variable / Segoe-UI-Variable-Display-Bold。这是本机浏览器证据，不代替 WebView2 实机。
- 本地验收产物（已忽略）：`test-results/font-presets-five-fonts-pr-ced73--theme-palette-and-playback/rendered-local-fonts.json`；`test-results/font-presets-long-font-lab-924f4-window-sizes-in-both-themes/` 包含五套字体 × 两种尺寸 × 明暗的 20 张截图。
- 产物哈希：不适用，本轮未构建或启动 exe。
- 三层结果和未验证项：自动测试 PASS；浏览器 PASS；Windows Tauri 实机/DPI NOT RUN。只读代码审阅通过，未发现重要可复现问题。

## 结束报告

- 改动文件：theme.ts、theme-init.js、tokens.css、preferences.ts、ui.svelte.ts、SettingsView.svelte；App.svelte/DesktopLyrics.svelte 安装和清理字体监听，桌面双语副歌词增加 display 字体；tests/v091-theme.test.mjs、新 tests/ui/font-presets.spec.ts、9 张快照；本单与 docs/current-state.md。无字体文件、依赖、Rust、业务 IPC、主配置变更。
- 三条验收结果：自动测试 PASS；浏览器 PASS；Windows Tauri 实机 NOT RUN。git diff --check 通过，用户原有删除与两轮 TASK-18 工作完整保留。
- 未验证项：Windows Tauri 实机与 DPI。
- Git：未提交，保留原有工作，任务单待审阅。
