# 任务：歌曲详情在线标签、高清封面与 LRC 补全

## 基线

- 分支与 HEAD：main / afaed378a30cc09dc7da4a686df9089c59bf2b5b
- 未提交改动：开工时干净。
- 工作区指纹：`de1d64b5b90c1831f26f48a9bdc3667d88bae2d999aa59cbeadb24683b8ae15b`；排除本任务的活动/归档记录和 docs/current-state.md。最终门禁前后已核对相同，归档后再次核对。

## 状态

- 状态：完成（自动/浏览器门禁通过；Windows/真实外网未验收）
- 依赖与前置任务：现有 Catalog、lofty、ureq/Rustls、封面缓存、歌曲详情。
- 下一步：按用户要求归档后停止；实机/真实服务验收缺口单独保留。
- 提交：待提交；不自动 commit。
- 遗留事项的任务 ID：本任务内记录。

## 范围

- 要改：显式打开后检索 MusicBrainz / LRCLIB；预览字段对比、封面和同步歌词；用户选择后确认写入。
- 不要改：解码器、AI/ListenBrainz 范围、用户现用数据、主 Tauri 配置、产品版本、既有截图基准。
- 涉及：Catalog SQLite/文件、两个 IPC、歌曲详情及播放元数据刷新（不重启播放）。
- 当前 schema 无 albums/artists 表；曲库标签在 tracks 事务更新后沿用现有动态分组。
- 默认不修改音频；同目录 LRC/cover.jpg 只在确认且勾选后写；已有文件默认拒绝，覆盖须独立授权。
- HTTP：10秒、MusicMine/0.1.0、有界 JSON/图片、固定错误码。CAA HTTPS 重定向逐跳校验，图片≤5MB且真实解码 JPEG/PNG；预览通过 data URL 保持 CSP。
- 写入：Catalog.operation 内重新校验；网络在锁外；同目录 create_new 临时文件、sync、rename；失败回滚已发布文件，数据库事务最后提交。音频只修改副本，保留其他标签。

## 实施计划

1. 新增离线解析/边界/临时库与文件测试，观察失败后实现候选解析和安全写入。
2. 实现固定来源检索与独立服务错误；解析文件名补全缺失查询；最高5个标签、3个有效同步歌词。
3. 注册 metadata_search_candidates / metadata_apply_enrichment，补齐 services、mock 与契约；刷新 Rust 当前/队列文本，不改变传输状态。
4. 歌曲详情加入折叠检索、候选选择与六字段对比、保守写入选项；旧请求和关闭保护；成功刷新曲库/封面/歌词。
5. 浏览器覆盖正常、空、错误、勾选、刷新及窄屏；执行 pnpm format、pnpm test:ui、pwsh -File scripts/check.ps1；既有截图零差异。

## 验收

- [x] 自动测试：PASS；pnpm format 和 pwsh -File scripts/check.ps1 退出0，Svelte 0 errors/0 warnings，前端96/96；Rust fmt/clippy 通过，单元134通过/1 ignored、音频3/3、歌词6/6，Null输出。
- [x] 浏览器验证：PASS；最终 pnpm test:ui 74/74，退出0，既有快照零像素差异，基准文件无 diff、未更新。新增6项功能用例，1280×800深色 / 900×640浅色、键盘/选择/空/失败/收起旧响应、应用后列表/播放器/歌词刷新。
- [ ] Windows Tauri 实机：NOT RUN
- [x] 未打开用户正在使用的曲库和设置

## 来源

- 任务 ID：TASK-26-METADATA-ENRICHMENT
- 时间：2026-11-17
- HEAD：afaed378a30cc09dc7da4a686df9089c59bf2b5b
- 工作区指纹：`de1d64b5b90c1831f26f48a9bdc3667d88bae2d999aa59cbeadb24683b8ae15b`。命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-26-METADATA-ENRICHMENT.md,docs/tasks/done/TASK-26-METADATA-ENRICHMENT.md,docs/current-state.md`。
- 命令、工具版本、构建配置：pnpm 12.21.0、Node v24.14.0、cargo 1.99.0 (5f94df478 2026-08-27)、Playwright 1.65.1；Rust debug/tests，MUSIC_MINE_NULL_OUTPUT=1；未构建 GUI exe。
- 产物哈希：无 exe
- 三层结果和未验证项：自动 PASS、浏览器 PASS、Windows Tauri 实机 NOT RUN；真实外网 NOT RUN。
- API 参考：[CAA 官方 API](https://musicbrainz.org/doc/Cover_Art_Archive/API)、[LRCLIB 官方 API](https://lrclib.net/docs)。

## 结束报告

- 改动文件：catalog/enrichment.rs 与 enrichment_tests.rs，metadata/mod.rs；platform/metadata_http.rs 与 atomic_files.rs；commands/catalog.rs、lib.rs、playback/controller.rs、artwork/covers.rs、Cargo.toml；MetadataEnrichment/TrackDetails、music.ts、library store、App/NowPlaying/DesktopLyrics；mock-ipc.js、服务单测、6项浏览器测试；IPC/架构/Rust约定及现状文档。
- 三条验收结果：自动 PASS；浏览器 PASS；Windows Tauri 实机 NOT RUN。
- 未验证项：真实 MusicBrainz/LRCLIB/CAA 服务/TLS/限流/重定向可用性，真实 WebView2 IPC、正在播放的 Windows 文件占用、硬件/DPI，断电或进程崩溃中的多文件恢复。只在独立 fixture/临时库验证，不打开用户现用数据。
- 回归过程：初始测试因缺失实现失败；音频写入回读曾因 .tmp 扩展名失败，改用内容探测后通过；缺失音频仍创建歌词的测试曾失败，增加锁内源文件校验后通过。最终14项补全 Rust 测试，加有界读取/固定错误码测试和播放元数据保留时序测试。
- 浏览器过程：首轮两个断言因两个 status 区匹配歧义失败，收窄测试选择器后通过；沙箱内5项断言通过但 Vite 清理不退出，记录为中止/无成功退出码，沙箱外重跑5/5退出0；最终完整74项门禁沿用可正常清理的沙箱外环境，不修改 Playwright 配置或快照。
- 既有 Cargo PDB 文件名冲突警告保留，不影响退出码；真实音频设备测试在 Null 环境不作为扬声器出声证明。
- Git：未提交
- 三项指定门禁全部退出0，归档至 docs/tasks/done/，没有提交或推送。
- 最终门禁日志（忽略产物）：tests/perf/out/task26-format.log、task26-check-final.log、task26-ui-final.log。窄屏补全面板截图位于 tests/perf/out/task26-enrichment-narrow-*.png；不作为既有截图基准。
