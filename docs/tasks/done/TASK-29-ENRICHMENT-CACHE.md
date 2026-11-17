# 任务：标签补全前端会话保留与 SQLite 候选缓存

## 基线

- 分支与 HEAD：main / afaed378a30cc09dc7da4a686df9089c59bf2b5b。
- 未提交改动：保留TASK-26～28全部未提交代码、文档、测试及TASK-27快照；不修改旧归档记录。
- 工作区指纹：`456fdbb4995ce5c0ad1209f1e1830177180f12b5beccd96e84d145916575bfe3`；最终门禁前后相同；排除本任务活动/归档记录及 docs/current-state.md。

## 状态

- 状态：完成
- 依赖与前置任务：当前多源标签补全工作台、既有rusqlite。
- 下一步：无；归档后停止，不自动提交。
- 提交：待提交；不自动 commit/push。
- 遗留事项的任务 ID：无新增功能遗留；外网/Windows隔离exe验收缺口仍在current-state记录。

## 范围

- 要改：增量 enrichment_cache 表，默认七天候选命中、force_refresh参数、查询键变化重取、应用成功/移除曲目清理；共享会话Store，保留筛选/列表/勾选/焦点/逐曲候选/草稿，按library.revision增量同步。
- 不要改：用户现用数据库/音乐/歌词/偏好、音频引擎、默认写入选项、服务范围、AI工具、依赖版本、CSP/权限和主配置、既有快照。
- 涉及数据库/IPC：使用临时库和旧schema fixture验证增量建表；查询参数新增forceRefresh（默认false），业务invoke仍仅在music.ts。
- 缓存：query_variants归一化首选title/artist的稳定JSON键；同ID/同键且0≤年龄<7天才命中。损坏/过大payload或未来时间不作为有效缓存。网络在Catalog锁外；写缓存时重验曲目仍存在，防止移除期间在途请求重建孤立缓存。成功应用在同一事务清理；移除文件夹时清理孤立缓存。
- 前端：会话仅内存，不是重启持久化。页面卸载仅取消后续调度，在途请求可完成到共享Store；返回不重置已有结果/勾选/草稿，不隐式批量检索。普通选曲直接展示缓存；右侧重新检索明确forceRefresh=true。曲库变化保留仍存在且路径相同的entry，只更新本地track/status并新增或删除条目。

## 验收

- [x] 自动测试：PASS。pnpm format、pwsh -File scripts/check.ps1退出0；Svelte 0 errors/0 warnings、前端109/109、Rust fmt/clippy、单元147通过/1 ignored、音频3/3、歌词6/6（Null输出）。Rust新增4项覆盖持久缓存命中免请求、查询键变化/强制更新、过期/未来时间/损坏JSON失效、旧schema增量建表、应用/目录移除清理和在途移除；前端新增3项覆盖增量同步保留草稿、会话命中/强刷参数及刷新失败保留旧结果。受控RED→GREEN已执行，不发真实外网请求。
- [x] 浏览器验证：PASS。pnpm test:ui退出0，80/80通过（其中工作台12项）；新增返回页面保留筛选/勾选/焦点/候选/草稿且不重复请求，重新检索forceRefresh=true，移除目录后清除会话条目。maxDiffPixels=0 / threshold=0，既有9张快照哈希相同，本任务未更新快照。1280×800深色和900×640浅色栏内滚动、按钮可达、整页无溢出已检查。
- [ ] Windows Tauri 实机：NOT RUN
- [x] 未打开用户正在使用的曲库和设置

## 来源

- 任务 ID：TASK-29-ENRICHMENT-CACHE
- 时间：2026-11-17
- HEAD：afaed378a30cc09dc7da4a686df9089c59bf2b5b
- 工作区指纹：`456fdbb4995ce5c0ad1209f1e1830177180f12b5beccd96e84d145916575bfe3`。命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-29-ENRICHMENT-CACHE.md,docs/tasks/done/TASK-29-ENRICHMENT-CACHE.md,docs/current-state.md`。
- 命令、工具版本、构建配置：pnpm 12.21.0、rustc/cargo 1.99.0；pnpm format、pnpm test:ui、pwsh -File scripts/check.ps1均0。Rust Null测试与浏览器harness，不打开用户数据库；UI沿用可正常清理Vite的沙箱外运行环境。保留既有Cargo PDB输出名碰撞警告。
- 产物哈希：无GUI exe
- 三层结果和未验证项：自动与浏览器PASS；Windows Tauri实机NOT RUN。真实SQLite缓存逻辑只在临时库验证；harness模拟缓存不证明真实IPC/网络/播放成功。真实服务、真实用户目录写权限、Windows文件占用和崩溃恢复NOT RUN。

## 结束报告

- 改动文件：catalog/db.rs、catalog/enrichment.rs及测试、commands/catalog.rs；music.ts、共享enrichment.svelte.ts、enrichment-workbench.ts、EnrichmentView.svelte；服务/调度/UI测试与mock；ipc-contract、architecture及current-state。
- 三条验收结果：自动PASS（check退出0）、浏览器PASS（80/80、零像素差异）、Windows Tauri实机NOT RUN。
- 未验证项：真实外网、隔离exe、断电/进程崩溃时的跨文件恢复。缓存不持久化前端草稿至应用重启；会话离开仅停止后续批量任务，在途操作仍可完成。
- 证据：tests/perf/out/task29-format.log、task29-check-final.log、task29-ui-final.log、task29-snapshot-hashes-before.csv；本轮截图task29-workbench-1280-top.png、task29-workbench-900.png。
- Git：未提交
- 完成后移到docs/tasks/done/，更新current-state后停止。
