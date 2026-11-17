# 任务：独立标签补全工作台

## 基线

- 分支与 HEAD：main / afaed378a30cc09dc7da4a686df9089c59bf2b5b。
- 未提交改动：保留 TASK-26 的后端/IPC/界面/测试和文档修改；包括未跟踪的新模块。未修改其他用户工作，不 commit。
- 工作区指纹：`1dd984e824d41958fcefd97d00f2497450450bd0d930bec306e6af7d684f0386`；最终门禁前后和归档后相同，排除本任务活动/归档记录及 docs/current-state.md。

## 状态

- 状态：完成（自动/浏览器门禁通过；Windows/真实外网未验收）
- 依赖与前置任务：当前工作区的 TASK-26 实现（本次不改其已归档历史验收）。
- 下一步：按要求归档后停止，不自动提交。
- 提交：待提交；不自动提交。
- 遗留事项的任务 ID：本记录列明。

## 范围

- 要改：一级 enrichment 路由、侧栏与面包屑；详情只保留跳转；双栏候选编辑、四类筛选、多选/全选、串行限流检索/应用、进度及取消；缺失艺术家时两种方向拆分查询。
- 不要改：解码器、音频回调、schema、真实用户库/文件/偏好、主 Tauri 配置、默认关闭的服务范围及 AI 工具列表。
- 涉及数据库/IPC/文件：新增只读批量完整度状态命令；继续复用确认写入；新增 dedicated_cover 选项，工作台默认逐曲 .cover.jpg，保持旧 cover.jpg 调用兼容。无新依赖。
- 搜索：缺艺术家时支持 - / － / —；紧凑横杠优先 title/artist、空格横杠优先 artist/title，来源无匹配时尝试相反方向。人工明确歌手时不拆标题。
- 缺失状态：艺术家/专辑、有效本地独立/共享/内嵌封面、同名 .lrc；不可读保留未知状态，不误称缺失。只在用户进入工作台时查询；无自动网络和写入。
- 批处理：固定任务快照、一次一个请求，曲目间限流；预览后点击批量应用才写入。取消停止后续任务，当前不可撤销 IPC 可完成；导航销毁后禁止启动后续任务。每曲保存候选/选项/错误，切换不串结果；默认不覆盖 sidecar 或写原音频。

## 验收

- [x] 自动测试：PASS；pnpm format、pwsh -File scripts/check.ps1 退出0；Svelte 0 errors/0 warnings，前端104/104，Rust fmt/clippy 通过、单元138通过/1 ignored、音频3/3、歌词6/6，Null输出。
- [x] 浏览器验证：PASS；更新阶段75/75退出0；最终 pnpm test:ui 不更新快照75/75退出0，更新后的基准零像素差异（maxDiffPixels=0 / threshold=0）。
- [ ] Windows Tauri 实机：NOT RUN
- [x] 未打开用户正在使用的曲库和设置

## 来源

- 任务 ID：TASK-27-DEDICATED-ENRICHMENT-WORKBENCH
- 时间：2026-11-17
- HEAD：afaed378a30cc09dc7da4a686df9089c59bf2b5b
- 工作区指纹：`1dd984e824d41958fcefd97d00f2497450450bd0d930bec306e6af7d684f0386`。命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-27-DEDICATED-ENRICHMENT-WORKBENCH.md,docs/tasks/done/TASK-27-DEDICATED-ENRICHMENT-WORKBENCH.md,docs/current-state.md`。
- 命令、工具版本、构建配置：pnpm 12.21.0、Node v24.14.0、cargo 1.99.0 (5f94df478 2026-08-27)、Playwright 1.65.1；pnpm format、pnpm test:ui、pwsh -File scripts/check.ps1，Rust tests/debug，MUSIC_MINE_NULL_OUTPUT=1。浏览器使用沙箱外执行以正常清理 Vite；Playwright 配置未改。
- 产物哈希：无 GUI exe
- 三层结果和未验证项：自动 PASS，浏览器 PASS，Windows Tauri 实机 NOT RUN，真实外网 NOT RUN。

## 结束报告

- 改动文件：Rust enrichment/enrichment_tests、commands/catalog、lib.rs、artwork/covers、platform/metadata_http；nav/Sidebar/TopBar/App/TrackDetails；EnrichmentView、受控 MetadataEnrichment、纯工作台调度器 enrichment-workbench.ts、music.ts；harness、服务/调度单测、7项工作台 UI 测试、8张侧栏快照；任务/现状/架构/IPC文档。没有新增依赖、schema 或权限。
- 三条验收结果：自动 PASS；浏览器 PASS；Windows Tauri 实机 NOT RUN。
- 未验证项：MusicBrainz/LRCLIB/CAA 实际服务/TLS/限流和内容覆盖；Windows WebView2、真实IPC、文件占用/权限、硬件/DPI；断电/崩溃多文件恢复。未构建或启动GUI exe，不打开用户现用数据。
- 回归过程：拆分/完整度/专属封面最初因实现缺失失败，补齐后通过；带前导曲号的文件名曾误将01当作歌手，受控测试失败后修复。详情关闭后读取 props 导致跳转失败，改为关闭前捕获ID；全选 checkbox 因计数混入可访问名称导致选择器失败，增加明确 aria-label。窄窗整页溢出测试先失败，再将页面改成受容器约束的双栏 flex/grid 后通过。已提交的写入遇到公共刷新失败不能显示未应用，新增红/绿回归并修复。
- 调度测试8项：本地加载/筛选、串行限流、取消与销毁、失败继续、冻结payload/安全默认值、取消在途写入、刷新失败仍报告已应用、205首分页完整加载。Rust 新增4项，含相反方向匹配和实际命中查询词；无真实外网。
- 快照：先查看1280×800及900×640双栏顶部/底部截图并验证整页不溢出，再按任务授权更新8张基准；使用 Playwright PNG 解码逐像素对比 HEAD，所有变动均在侧栏内（1280图 x≤201；900图 x≤165），主体区域像素保持不变。now-playing-dark.png 未改变。截图位于 tests/perf/out/task27-workbench-*.png。
- 日志：tests/perf/out/task27-format-final.log、task27-check-final.log、task27-ui-update.log、task27-ui-final.log（忽略产物）；既有 Cargo PDB 文件名冲突警告保留。
- Git：未提交；不自动 commit/push。
- 已移到 docs/tasks/done/，更新 current-state/architecture 后停止；三项指定门禁全部退出0，未提交或推送。
