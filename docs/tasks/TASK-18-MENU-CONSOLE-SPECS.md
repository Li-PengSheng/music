# 任务：顶部工具菜单、运行控制台与专业音频参数

## 基线

- 分支与 HEAD：`main` / `ef60136108699cd30a7cb1d00057b3fa1ee0bada`
- 未提交改动：用户已删除根目录两张图标、repomix-output.md、Android/iOS 图标文件；全部保留。
- 工作区指纹：`8268a45cff86e57acbe8bf731400ad1cad1d016c1b9fc38d79c86dfb7bb5fe5f`；最终验收前后相同，排除本任务单与 docs/current-state.md。

## 状态

- 状态：待审阅（实现与本任务验收完成）
- 依赖与前置任务：现有 Modal、openMenu、播放快照与 LogBuffer。
- 下一步：用户审阅 diff；本单按要求保留在 docs/tasks/，审阅后归档。
- 提交：待提交；不自动 commit。
- 遗留事项的任务 ID：TASK-18 的真实音频硬件、原生选择/扫描与 DPI 扩展验收；不扩大本次结论。

## 范围

- 要改：AudioInfo 位深/字节大小/累计播放次数；顶部菜单与 Console；本地内置命令；IPC、mock 与测试。
- 不要改：解码器、音频回调、用户数据、主 Tauri 配置、依赖与无关 UI。
- 涉及数据库只读查询与 IPC。当前 schema 是 listening_history.plays，沿用它而非新增 play_history；不迁移 schema。
- 文件属性移到 metadata；异步 command 在阻塞工作线程读取；未知位深不伪造。
- Console 共用引擎有界日志，清空仅影响内存；日志出 IPC 前脱敏绝对路径。补充扫描与 SMTC 状态日志。
- Console 只在打开期间每两秒刷新，销毁清理定时器；内置指令有校验，播放结果走 control/accept，不接入 AI 或 shell。
- 正在播放按完整路径查曲库 track ID，切歌取消旧结果；无库内 ID、读取失败时静默隐藏规格。
- 播放累计数由 Rust 写后事件 library-listen-recorded 单独更新；先订阅再读取参数，以最大值合并事件与查询结果，避免首次查询早于后台统计时一直显示旧次数，不重复读取静态文件规格。

## 验收

- [x] 自动测试：PASS，pnpm format、pwsh -File scripts/check.ps1 均退出 0；TS 0 error / 0 warning、前端 83/83、Rust 单元 98 passed / 1 ignored、音频集成 3/3、歌词集成 6/6。脚本使用 MUSIC_MINE_NULL_OUTPUT，不代表硬件出声。
- [x] 浏览器验证：PASS，pnpm test:ui 42/42，退出 0；新增 7 项菜单、Console、错误/清空/复制/销毁、参数/未知值/切歌竞态与计数事件乱序测试。1280×800 / 900×640 无挤压；保留既有 960×700 快照覆盖，明暗主题通过。9 张受影响快照经查看后更新，最终复跑零像素差异。
- [x] Windows Tauri 实机：PASS（限定下述范围）；构建出的隔离 exe / WebView2 / 1280×850 / 100%，测试仅使用复制到临时目录的三首仓库 fixture 和新建 SQLite。真实 IPC 已验证菜单、日志读清、SMTC 状态、封面统计、曲库规模、vol 30 权威快照、theme/palette、详情 WAV 16 bit / 529.3 KB / 播放 2 次。NowPlaying 已挂载并显示 2 次后开始播放，同路径通过真实 Rust 写后事件更新为 3 次，再暂停；验证了实际计数事件而非重新挂载后查询。使用空音频输出；实际扬声器、原生选择器/扫描、DPI 125%/150% 为 NOT RUN。
- [x] 未打开用户正在使用的曲库和设置

实机必须使用构建出的 exe。会写数据时按 `docs/isolation.md`。不要修改 `src-tauri/tauri.conf.json`。

## 来源

- 任务 ID：TASK-18-MENU-CONSOLE-SPECS
- 时间：2026-11-17
- HEAD：`ef60136108699cd30a7cb1d00057b3fa1ee0bada`
- 工作区指纹：同基线；命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-18-MENU-CONSOLE-SPECS.md,docs/current-state.md`。最终 pnpm format 后、完整 UI/check.ps1 前采集；结束后再次相同。
- 命令、工具版本、构建配置：pnpm 12.21.0、Node v24.14.0、Playwright 1.65.1、Cargo 1.99.0、PowerShell 7.6.10。运行 pnpm format、pnpm test:ui、pwsh -File scripts/check.ps1（全部退出 0）；快照更新使用 pnpm test:ui --update-snapshots 后再普通 test:ui 验证。沙箱内 UI 断言通过但 Vite 清理挂起，外部执行后正常退出。
- 隔离构建：`pwsh -File scripts/build-isolated.ps1 -Config <test-root>/isolation.json -TestRoot <test-root>`；仅 CLI 合并绝对路径配置，未改主 tauri.conf.json。测试根 `%TEMP%/music-mine-task18-03ef83be8e784a70b96b366af9c37a41`，数据目录 `<test-root>/data`；另用 `<test-root>/webview` 隔离 WebView2 用户数据。启动前 Test-IsolationSource 核对哈希与配置，MUSIC_MINE_NULL_OUTPUT=1，CDP 9338；结束已按 PID/启动时间/exe 路径核对关闭。
- exe：`src-tauri/target/debug/music-mine-desktop.exe`；SHA-256 `43437B56B0C3F93A691703160CB0224A978BB0744506ABB02D699D540C40825E`。
- 配置 SHA-256：`A572580D35628DF7F8CB494F3EFDC582DE974DB0BBE14981386AAC69C4DC5769`；哈希绑定来源记录 `tests/perf/out/isolation-sources/43437B56B0C3F93A691703160CB0224A978BB0744506ABB02D699D540C40825E.json`。
- 本地验收产物（已忽略）：tests/perf/out/task18-run.json 定位测试根；<test-root>/desktop-result.json、console-real.png、details-real.png、now-playing-real.png。脚本 tests/perf/out/task18-desktop.mjs 通过实际 WebView2 UI 操作，无 mock。
- 三层结果：如上；早期实机脚本立即暂停导致未赶上既有 200ms 播放记录轮询。审阅另发现首次参数查询早于写入后可能一直显示旧计数，已通过写后事件修复；最终 exe 在 NowPlaying 已打开、路径不变的情况下确认 2→3。产品计数规则未修改。清空后的日志不重建启动行，复测按当前 SMTC 摘要断言。
- 审阅：只读代理发现的两个 P2 已修复并复核通过：合法括号/中括号/中文逗号和单引号路径的目录脱敏，以及累计次数的查询/写入竞态。对应回归先失败再通过；新增事件只给主窗口，不改变进度轮询或计数规则。
- 工具差异：PowerShell 7.6 的 ConvertFrom-Json 将 process.json 时间解析为日期，原清理辅助函数误判身份；本次使用 -DateKind String 重新核对后关闭，没有修改无关隔离脚本。

## 结束报告

- 改动文件：metadata.rs/db.rs/commands/catalog.rs、catalog/mod.rs 的写后计数返回与 catalog_tests；logging.rs/lib.rs、Player/Controller 只增加日志 Arc 访问器、media.rs 初始状态日志；music.ts、TrackDetails/NowPlaying、ConsoleModal、TopBar、ui store、App、格式工具、mock、7 项 UI 测试与 9 张快照；IPC 契约与 current-state。
- 三条验收结果：自动测试 PASS；浏览器 PASS；Windows Tauri 实机 PASS（限定范围，空音频输出）。git diff --check 通过。
- 未验证项：真实音频设备出声/拔插、原生文件选择与扫描、125%/150% DPI；历史验收缺口不因本次自动测试而消除。
- Git：未提交；保留用户的全部既有删除，未改依赖、Cargo.lock、主配置、音频回调、数据库 schema 或真实用户数据。
