# 任务：TASK-12-SMTC-THUMBNAIL — Windows SMTC 专辑封面

## 基线

- 分支与 HEAD：main / 416d4a1e7bde5a0e3ead4120aed3fd1b1375b9ec
- 未提交改动：无，工作区干净。
- 工作区指纹：`bb873a50b245b596ce513991374611d23959a5af72c77b28a7ab08809e7ec6da`，验收前后相同，排除 current-state 和本任务的进行中/归档位置。

## 状态

- 状态：完成
- 依赖与前置任务：现有 covers::Store、SMTC 元数据去重；windows 0.58 仅补充 Storage_Streams feature。
- 下一步：等待用户检视。
- 提交：未提交
- 遗留事项的任务 ID：无新增。

## 范围

- 要改：SMTC 复用 320px 封面缓存、按路径缓存 WinRT 引用，封面失败降级为文本，停止/无封面时清除旧图，更新连接状态文案。
- 不要改：音频引擎、回调、数据库结构、用户真实文件/曲库/偏好、IPC 形状、既有截图。
- 是否涉及音频、数据库、IPC 或用户数据目录：媒体集成和隔离缓存读写；不改音频或数据库。

## 验收

- [x] 自动测试：PASS，scripts/check.ps1 退出 0；前端 74/74、Svelte 0 error / 0 warning、Prettier / Rust fmt / clippy 通过；Rust 90 通过、1 ignored，音频/歌词集成 3/6 通过。新增 3 项 SMTC 测试覆盖路径去重/清理、320px 准备与不改文件、WinRT 引用生命周期。
- [x] 浏览器验证：PASS，pnpm test:ui 22/22、退出 0；既有 8 张快照零像素差异，不更新快照。
- [x] Windows Tauri 实机：PASS（系统会话 API 范围）；隔离 exe 的真实媒体会话中封面出现、暂停保留、无封面切歌/停止清除、切回恢复；连续轮询 Store 请求 6→6。NULL_OUTPUT 未打开真实音频设备。
- [x] 未打开用户正在使用的曲库和设置

实机必须使用按 docs/isolation.md 构建且绑定哈希来源的 exe，不修改主配置。

## 来源

- 任务 ID：TASK-12-SMTC-THUMBNAIL
- 时间：2026-11-16
- HEAD：416d4a1e7bde5a0e3ead4120aed3fd1b1375b9ec
- 工作区指纹：`bb873a50b245b596ce513991374611d23959a5af72c77b28a7ab08809e7ec6da`，命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/current-state.md,docs/tasks/TASK-12-SMTC-THUMBNAIL.md,docs/tasks/done/TASK-12-SMTC-THUMBNAIL.md`；已忽略验收产物不计入。
- 命令、工具版本、构建配置：pnpm 12.21.0、Node 24.14.0、rustc 1.99.0、WebView2 157.0.4341.46。定向 cargo test --lib --locked 与 clippy --all-targets --locked 退出 0；依次 scripts/check.ps1 → pnpm test:ui 均退出 0。windows 0.58 只补 Storage_Streams，锁文件已有版本无需变动。随后按 `scripts/build-isolated.ps1 -Config tests/perf/out/task12-isolation.json -TestRoot <专用 TEMP 根>` 构建（含 pnpm build），退出 0。数据目录为 `C:\Users\myuli\AppData\Local\Temp\music-mine-task12-1beda1cc6e794a568aee5f4cd24a9aa6\data`；不改主配置，进程启用 NULL_OUTPUT 和 WebView2 CDP 9239。按 PID/路径/启动时间核对后测试进程已关闭。
- 产物哈希：exe SHA-256 `29C601E5EC5A27D3D1F94483552DA36CAAF629548363070C73C62519EF7A5D29`，配置 SHA-256 `EAC1BFF7AA2E1CCDAB704A1FA748C2984A23F92F6D8D0402AB4F3C01773C24C1`；产物路径 `src-tauri/target/debug/music-mine-desktop.exe`，来源 `tests/perf/out/isolation-sources/<exe SHA-256>.json`，最终 check-source 返回 OK。`node tests/perf/out/task12-native.mjs` 退出 0；结果 `tests/perf/out/task12-native-result.json`。
- 实机证据：PowerShell WinRT 全局媒体会话 API 只检查 source 为 music-mine-desktop.exe 的会话。带图和暂停时 Thumbnail 非空；无图曲目、停止后为空；返回带图曲目恢复；标题同时保持正确。字节长度/图像尺寸由 Rust/WinRT 单测验证；系统会话 API 验收不声称面板像素渲染已目视通过。初次验收脚本直接读取 COM 投影 Size 得到空值，定位后改为系统引用存在性断言，产品代码未因此改动。
- 文件完整性：在 TEMP 内复制 tone.flac 并制作带图/无图测试标签，未打开用户库。验收前后两文件 SHA-256 分别为 `b09b3499fcd300a1d10b9c4ab140363d3d087caddee45e36cf3b649fba9bf734`、`4458d499630131d8e3e1dbe52baa8cd41723b7b1762b6bd7ee9b64465f368bd2`；仓库原 fixture 内容也未变。
- 三层结果和未验证项：自动 / 浏览器 / Windows 系统媒体会话 API 均 PASS；系统面板人工目视、实体媒体键、实际出声、DPI/托盘 NOT RUN。保留已有 Cargo PDB 文件名冲突警告。

## 结束报告

- 改动文件：platform/media.rs、Cargo.toml、rust-guidelines.md、current-state.md 和任务单；没有前端样式或 IPC 变更。
- 三条验收结果：自动 PASS；浏览器 PASS；Windows Tauri 系统会话 API PASS。
- 未验证项：系统面板人工目视、实体媒体键、真实出声与 DPI/托盘。GIF/WebP 透传不保证尺寸，因此不送入 SMTC，仍展示文本元数据。
- Git：未提交，不自动 commit/push。
- 已完成，归档到 docs/tasks/done/。
