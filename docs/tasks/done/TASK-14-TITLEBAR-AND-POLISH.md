# 任务：一体化标题栏、歌词层级与首页艺术家封面

## 基线

- 分支与 HEAD：main，bad7f2ec8cf744adc04edd4627b9f701a5ee0d1a。
- 未提交改动：已有 TASK-13、歌词窗口修复、视觉精修及 9 张快照变更，全部保留。
- 开工工作区指纹：3ee53097855ee1f7c4a8783a526dc1ac8575333ec5c9db6bf5ce01b16031a23b（无排除项）。

## 状态

- 状态：完成
- 依赖与前置任务：当前未提交工作区。
- 下一步：等待用户检视。
- 提交：待提交；本任务不提交。
- 遗留事项的任务 ID：无。

## 范围

- 要改：主窗口装饰和最小窗口权限，两个顶部区域共用窗口按钮，歌词焦点和减少动画，艺术家封面与首字母回退。
- 不要改：音频引擎、数据库、歌词子窗口权限、用户数据目录。
- IPC：仅使用 Tauri 官方窗口 API；不新增业务命令。

## 验收

- [x] 自动测试：pnpm format、scripts/check.ps1 均退出 0；Svelte 0 error / 0 warning，74 前端单测，Rust 92 passed / 1 ignored，音频 3/3、歌词 6/6。
- [x] 浏览器验证：33/33，退出 0；1280×800、900×640 控件布局通过，审查后更新 9 张预期快照，复跑零差异。
- [x] Windows Tauri 实机：隔离 exe 的无边框/可缩放属性、两个标题区最大化/还原、最小化、歌词窗口创建及主窗口退出通过；实际鼠标拖边缩放、拖拽、Snap Layouts、多 DPI 和真实音频 NOT RUN。
- [x] 未打开用户正在使用的曲库和设置

## 来源

- 任务 ID：TASK-14-TITLEBAR-AND-POLISH
- 时间：2026-11-16
- HEAD：同基线
- 工作区指纹：见 current-state.md（排除本记录及 current-state.md 防止自引用）。
- 命令：pnpm format、pnpm test:ui、pwsh -File scripts/check.ps1；pnpm 12.21.0、Node 24.14.0、Cargo 1.99.0。实机通过 scripts/build-isolated.ps1 使用已忽略的 lyrics-probe-isolation.json，专用 TEMP 数据目录，MUSIC_MINE_NULL_OUTPUT=1。
- 产物 SHA-256：39E5F2ADC962A650DE731870C441E3CCC2DB4ABC57F5D8D4CDE1DE582C72ADA1；哈希绑定来源在 tests/perf/out/isolation-sources/，实机结果在 tests/perf/out/task14-native-result.json。
- 三层结果和未验证项：见验收。

## 结束报告

- 改动：WindowControls、TopBar、NowPlaying、HomeView、tokens、主窗口配置及权限、desktop.rs 退出路径、harness、UI 测试/快照及对应文档。没有改动业务 IPC、数据库或音频引擎。
- 三条验收结果：自动门禁通过；浏览器通过；实机部分覆盖通过。
- 首次门禁遇到既有音频暂停时钟 5ms 抖动；另一次因隔离 exe 正运行而无法覆盖产物失败。停止测试实例并修复正常退出后，完整门禁退出 0；未改音频代码或测试断言。
- Git：不提交
