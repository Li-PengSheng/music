# 任务：标签补全多源检索与手动目标编辑

## 基线

- 分支与 HEAD：main / afaed378a30cc09dc7da4a686df9089c59bf2b5b。
- 未提交改动：保留 TASK-26 / TASK-27 全部代码、文档、测试及8张已授权侧栏快照；不修改已归档任务记录。
- 工作区指纹：`36e2d219b05964687c72267d36ba305f793e6106fe7d43fb9a59efbbc4f4a462`；最终门禁前后及归档后相同，排除本任务活动/归档记录及 docs/current-state.md。

## 状态

- 状态：完成（自动/浏览器门禁通过；Windows和真实服务未验收）
- 依赖与前置任务：当前工作区独立标签补全工作台。
- 下一步：归档、更新现状后按要求停止，不自动提交。
- 提交：待提交；不自动 commit/push。
- 遗留事项的任务 ID：本任务内记录。

## 范围

- 要改：iTunes CN/US 标签与600px封面，三源独立容错、稳定去重、LRCLIB反哺，括号后缀空结果重试，来源标记，手动目标草稿与无候选应用。
- 不要改：音频引擎、schema、主配置、权限、AI工具范围、用户现用数据、既有快照、产品版本和依赖版本。
- 涉及 IPC/文件：扩展候选来源/genre/Apple cover_url与确认写入payload；复用既有副本/事务/回滚。下载URL严格校验，Apple CDN和CAA不同允许列表，不接受任意站点/重定向。
- 网络：仅Rust阻塞工作线程，MusicMine/0.1.0，iTunes请求8秒、固定/search参数、CN为空或失败回退US；JSON≤2MB，封面≤5MB且真实JPEG/PNG。独立来源错误不吞掉有效候选；仅全部标签源网络失败且无任何候选时提示标签来源失败。
- 检索先保留原始版本；所有原始方向无结果后，才移除末尾成对英文/中文括号或方括号，重复后缀可剥离，不清洗艺术家或原始音频标签。
- 工作台草稿独立于原候选；修改标题/艺术家/专辑/年份后仅确认应用才写入；切曲保留草稿，切候选显式替换目标；手动目标可在无检索结果时使用。

## 验收

- [x] 自动测试：PASS；pnpm format、pwsh -File scripts/check.ps1 退出0，Svelte 0 errors/0 warnings；前端106/106，Rust fmt/clippy通过，单元143通过/1 ignored、音频3/3、歌词6/6，Null输出。
- [x] 浏览器验证：PASS；pnpm test:ui 78/78退出0，maxDiffPixels=0 / threshold=0。9张既有快照前后SHA-256完全相同，TASK-28没有更新快照；TASK-27原有8张改动保留。
- [ ] Windows Tauri 实机：NOT RUN
- [x] 未打开用户正在使用的曲库和设置

## 来源

- 任务 ID：TASK-28-ENRICHMENT-MULTI-SOURCE-FALLBACK
- 时间：2026-11-17
- HEAD：afaed378a30cc09dc7da4a686df9089c59bf2b5b
- 工作区指纹：`36e2d219b05964687c72267d36ba305f793e6106fe7d43fb9a59efbbc4f4a462`。命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-28-ENRICHMENT-MULTI-SOURCE-FALLBACK.md,docs/tasks/done/TASK-28-ENRICHMENT-MULTI-SOURCE-FALLBACK.md,docs/current-state.md`。
- 命令、工具版本、构建配置：pnpm 12.21.0、Node v24.14.0、cargo 1.99.0 (5f94df478 2026-08-27)、Playwright 1.65.1；pnpm format、pnpm test:ui、pwsh -File scripts/check.ps1 全部退出0。Rust tests/debug，MUSIC_MINE_NULL_OUTPUT=1；UI沿用可正常清理Vite的沙箱外环境，Playwright配置未改。
- 产物哈希：无GUI exe
- 三层结果和未验证项：自动 PASS，浏览器 PASS，Windows Tauri 实机 NOT RUN；真实外网 NOT RUN。
- 官方参考：[Apple 搜索API](https://performance-partners.apple.com/search-api)、[结果字段](https://developer.apple.com/library/archive/documentation/AudioVideo/Conceptual/iTuneSearchAPI/UnderstandingSearchResults.html)。Apple公开文档约20次/分钟，增加独立3.1秒请求间隔，无新增服务密钥。

## 结束报告

- 改动文件：catalog/enrichment.rs、enrichment_tests.rs、platform/metadata_http.rs；music.ts、enrichment-workbench.ts、MetadataEnrichment.svelte；mock-ipc.js、调度与UI测试；IPC、架构、现状和本任务记录。没有新命令、schema、依赖版本、权限或CSP变更。
- 三条验收结果：自动 PASS，浏览器 PASS，Windows Tauri 实机 NOT RUN。
- 未验证项：实际iTunes CN/US/TLS/地域/限流与Apple CDN600px地址可用性，真实MB/LRCLIB/CAA服务，Windows WebView2/真实IPC/文件占用/硬件/DPI，以及原有断电/崩溃恢复缺口。所有数据写入测试只用临时库和复制的fixture，未打开用户现用曲库或设置。
- 新增回归：Rust 5项（iTunes字段与URL、括号后缀重试、多源容错/反哺/去重、Apple URL拒绝与genre事务、CN→US回退）；调度2项（无候选手填、原候选/目标隔离与切换），UI3项（手填四字段且默认不改音频、LRCLIB来源/反哺并精修、iTunes来源及下载URL确认）。保持既有批处理、取消、文件回滚等回归。
- 测试先行：新增Rust测试最初因解析/合并接口缺失失败，新增调度测试因草稿接口缺失失败；实现后通过。编译阶段补齐可空字段和预览缓存类型，不把编译失败当作成功验收。最终门禁在稳定指纹上重新运行。
- 浏览器：已查看1280×800深色、900×640浅色截图，编辑输入、来源标记与落盘选项可访问，整页不溢出，右栏内滚动。截图为 tests/perf/out/task28-workbench-*.png，不作为快照基准；harness不联网、不写真实文件，不能证明Rust真实播放或外网成功。
- 日志（忽略产物）：tests/perf/out/task28-format-final.log、task28-check-final.log、task28-ui-final.log、task28-snapshot-hashes-before.csv。既有Cargo PDB文件名冲突警告保留，不影响退出码。
- Git：未提交
- 已归档至 docs/tasks/done/，更新 current-state 后停止；未 commit/push。
