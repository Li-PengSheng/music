# 任务：移除设置页旧版迁移与无调用导入链路

## 基线

- 分支与 HEAD：main / b2c35e73d6814677639ac413cdb4370ccac26716。
- 未提交改动：保留 TASK-22 的全部未提交前后端/测试/文档改动及归档记录，以及此前 npm.cmd 删除差异。仅在涉及的共享文件上增量删除旧版导入。
- 工作区指纹：`1a0d956ed51b1b130b9966ddbd5688272f7f5e8d122ff9ea14f9dea7029ce3c7`，最终验收前后相同。排除本单 active/done 和 docs/current-state.md。

## 状态

- 状态：完成
- 依赖与前置任务：现有设置页、旧版 library_import IPC、Catalog.import_copy。
- 下一步：归档并停止；实机检查因已有进程占用 debug exe 未运行，保留验收缺口。
- 提交：待提交；不自动 commit。
- 遗留事项的任务 ID：沿用此前实机/外网服务未验证项，不在本轮扩展。

## 范围

- 删除 SettingsView 迁移卡片、导入状态/回调/Database 图标，actions/importLegacyDatabase、music/importLegacy、mock/library_import 与 Rust 注册/命令。
- 删除 ImportReport、legacy_snapshot、Catalog.import_copy 及其专用 WAL 备份测试；移除仅该路径使用的 rusqlite backup feature。
- 当前库 schema 升级及其数据保留测试不删除；settings 旧字段继续保留，仅读取/写入测试夹具的辅助函数标记 cfg(test)。不做删表或任何真实用户库/设置/音乐操作。
- IPC 清单直接删除 library_import，不增加 ipc-unused 例外；既有两个例外不变。更新 Rust 文档中失效的导入描述。
- 不修改音频引擎、现有新服务、主配置、能力权限、任何旧播放器项目和快照。

## 验收

- [x] 自动测试：PASS。pnpm format 退出0；最终 pwsh -File scripts/check.ps1 退出0。Svelte 0 errors/0 warnings，前端95/95，IPC surface 3/3，Rust fmt、cargo clippy --all-targets -- -D warnings 无诊断，单元114 passed/1 ignored（移除1项专用导入备份测试），音频集成3/3、歌词集成6/6。Cargo.lock无变化。测试使用Null音频及临时fixture库，不访问真实用户数据。
- [x] 浏览器验证：PASS。pnpm test:ui 59/59，退出0，既有快照零像素差异、未更新。新增设置清理回归先确认旧卡片存在导致失败，再删除后通过；覆盖1280×800/900×640、旧迁移标题/按钮消失、最后卡片为键盘快捷键及无横向溢出。其他设置/AI/Console/曲库用例全部通过。
- [ ] Windows Tauri 实机：NOT RUN。普通沙箱和沙箱外两次隔离构建都因不能替换 src-tauri/target/debug/music-mine-desktop.exe 而失败（Windows os error5）。只读进程查询确认已有PID4696占用该路径；未停止既有进程，未启动该现有exe，未以未知来源产物冒充隔离验收。新命令移除的实机调用确认仍未覆盖。
- [x] 未打开用户正在使用的曲库和设置

## 来源

- 任务 ID：TASK-23-REMOVE-LEGACY-IMPORT
- 时间：2026-11-17
- HEAD：b2c35e73d6814677639ac413cdb4370ccac26716
- 工作区指纹：同基线；命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-23-REMOVE-LEGACY-IMPORT.md,docs/tasks/done/TASK-23-REMOVE-LEGACY-IMPORT.md,docs/current-state.md`。最终格式/UI/check前、check复跑前后及归档前后均一致，保留此前未提交改动和npm.cmd删除。
- 命令、工具版本、构建配置：pnpm12.21.0、Node v24.14.0、Playwright1.65.1、Cargo1.99.0、PowerShell7.6.10。最终三项指定门禁退出0。首轮check退出101：原有 encoded_formats_decode_and_play_with_real_device_controls 在 tone.mp3 暂停位置断言得到155ms/150ms；检查失败位置后单独复跑通过，完整check复跑也通过。未修改或放宽音频实现/测试；未稳定复现或定位根因，不称为已修复。既有Cargo PDB名称冲突警告保留。
- 隔离构建尝试：`pwsh -File scripts/build-isolated.ps1 -Config <测试根>/isolation.json -TestRoot <测试根>`，测试根 `%TEMP%/music-mine-task23-474bc5c831de402db40eea8d5abdc87f`，CLI appDirectoriesOverride 指向其data，未改主配置；两次均因已有exe占用失败，未生成新的哈希绑定来源记录。
- 产物哈希：无新验证exe（构建失败）；不引用上一任务exe的哈希作为本轮来源。
- 证据：忽略目录 tests/perf/out/task23-format-final.log、task23-ui-final.log、task23-check-final.log（首轮失败）、task23-check-retry.log（完整复跑通过）、task23-build-final.log、task23-build-retry.log。源代码/harness全局查询无 importLegacy/importLegacyDatabase/library_import/import_copy/ImportReport/legacy_snapshot/backup API 残留；契约说明中的移除记录保留。
- 三层结果和未验证项：自动PASS、浏览器PASS、Windows Tauri NOT RUN。首轮Null暂停时钟单次失败根因未定位；真实设备、DPI和此前外网服务仍未验证。

## 结束报告

- 改动文件：SettingsView.svelte、actions.ts、music.ts、mock-ipc.js；Rust lib.rs、commands/catalog.rs、catalog/{mod,db,catalog_tests}.rs、Cargo.toml；settings-cleanup UI回归；IPC/Rust约定、本单和current-state。上一任务AI增量及npm.cmd删除未撤销。
- 三条验收结果：见上；三项指定门禁全部0，既有快照不变。实机阻挡如实记NOT RUN。
- 未验证项：硬件音频/DPI、此前真实外网服务验收不在本轮范围。
- Git：不提交，完成后归档并更新 current-state 后停止。
