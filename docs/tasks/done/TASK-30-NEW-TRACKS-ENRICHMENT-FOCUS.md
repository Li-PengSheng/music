# 任务：新入库歌曲补全聚焦与可恢复忽略

## 基线

- 分支与 HEAD：main / afaed378a30cc09dc7da4a686df9089c59bf2b5b。
- 未提交改动：保留TASK-26～29全部未提交实现、文档、测试及既有快照；不修改历史归档。
- 工作区指纹：`bd73403daf2f2938a4c614626d6510de50ba81b28097a1c928eb80acc3d09555`；最终门禁前后相同。排除本任务活动/归档记录和docs/current-state.md。

## 状态

- 状态：完成
- 依赖与前置任务：既有added排序、工作台会话Store、SQLite候选缓存。
- 下一步：无；归档后停止，不自动提交。
- 提交：待提交；不自动commit/push。
- 遗留事项的任务 ID：无新增。

## 范围

- 要改：工作台按added_at DESC/id DESC读取；默认近7天，无可见新歌自动显示四类缺失待办；另有新入库且缺信息、缺歌手/专辑、缺封面、缺歌词、全部。新歌徽标使用既有accent tokens。会话保留筛选偏好，新增扫描保留草稿。
- 忽略：ensure_column增量增加enrichment_cache.ignored。明确IPC设置/恢复，状态DTO返回标记，列表查询提供可选ignored条件；默认其他曲库视图仍显示全部。待办隐藏忽略，全部可恢复；忽略后清除其勾选，批量不处理忽略曲目。
- 缓存兼容：搜索UPSERT只替换候选字段，应用清理候选但保留ignored=1的标记行；目录移除仍删除孤立行。不自动取消用户忽略。
- 不要改：音频引擎、用户现用库/媒体/设置、默认安全写入、依赖版本、权限/CSP/主配置、既有快照。
- 数据库/IPC只在临时库和mock验证，不打开用户数据。

## 验收

- [x] 自动测试：PASS。pnpm format、pwsh -File scripts/check.ps1均退出0；Svelte 0 errors/0 warnings、前端112/112、Rust fmt/clippy、单元150通过/1 ignored、音频3/3、歌词6/6（Null输出）。新增Rust3项验证旧cache schema增量列/内容保留、设置恢复和非法ID、标记跨刷新/应用保留、查询过滤与added排序/分页；新增前端3项验证日期边界/缺失维度/回退、忽略成功与失败/批量跳过、初始焦点跳过持久忽略。核心行为经历受控RED→GREEN。
- [x] 浏览器验证：PASS。pnpm test:ui退出0，85/85；新增5项覆盖新歌默认/倒序/徽标/缺信息细分、无新歌回退、忽略移出/全部恢复，以及1280×800深色、900×640浅色的徽标和按钮键盘可达。原工作台12项全部保留通过；不自动联网/应用。既有9张快照SHA-256一致，maxDiffPixels=0 / threshold=0，本任务没有更新快照，保留TASK-27原有快照改动。
- [ ] Windows Tauri 实机：NOT RUN
- [x] 未打开用户正在使用的曲库和设置

## 来源

- 任务 ID：TASK-30-NEW-TRACKS-ENRICHMENT-FOCUS
- 时间：2026-11-17
- HEAD：afaed378a30cc09dc7da4a686df9089c59bf2b5b
- 工作区指纹：`bd73403daf2f2938a4c614626d6510de50ba81b28097a1c928eb80acc3d09555`。命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-30-NEW-TRACKS-ENRICHMENT-FOCUS.md,docs/tasks/done/TASK-30-NEW-TRACKS-ENRICHMENT-FOCUS.md,docs/current-state.md`。
- 命令、工具版本、构建配置：pnpm 12.21.0、rustc/cargo 1.99.0；pnpm format、pnpm test:ui、pwsh -File scripts/check.ps1均0。临时SQLite、Rust Null音频与浏览器harness；UI沿用可正常清理Vite的沙箱外环境。既有Cargo PDB输出名碰撞警告保留。
- 产物哈希：无GUI exe
- 三层结果和未验证项：自动与浏览器PASS，Windows Tauri实机NOT RUN。没有构建或启动GUI exe；harness不证明真实Rust IPC、SQLite、出声或外网，后端只用临时库/复制fixture测试。

## 结束报告

- 改动文件：catalog/db.rs、catalog/mod.rs、catalog/enrichment.rs及测试、commands/catalog.rs、lib.rs；enrichment.svelte.ts、enrichment-workbench.ts、EnrichmentView.svelte、music.ts；mock-ipc、调度/服务单测与新增enrichment-focus.spec.ts；IPC/architecture/current-state及本任务单。
- 三条验收结果：自动PASS（format/check退出0）、浏览器PASS（85/85、既有快照零像素差异）、Windows Tauri实机NOT RUN。
- 未验证项：Windows Tauri隔离exe及真实外网NOT RUN。
- Git：未提交
- 证据：tests/perf/out/task30-format.log、task30-check-final.log、task30-ui-final.log、task30-snapshot-hashes-before.csv；已查看task30-new-tracks-1280.png与task30-new-tracks-900.png，整页无溢出，徽标与忽略按钮清晰可达。
- 完成后移至docs/tasks/done/，更新current-state后停止。
