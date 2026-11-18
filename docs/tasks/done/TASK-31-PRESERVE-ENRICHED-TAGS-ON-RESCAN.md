# 任务：重扫不覆盖已补全的曲库标签（TASK-31）

## 基线

- 分支与 HEAD：`main` / `a8c9d2130f9b75a11aea1f2cef6333574ac3df83`（`feat(enrichment): add metadata workbench, caching and new-track focus`）。
- 未提交改动：仅未跟踪的 `repomix-output.md`（用户文件，本任务未修改、未删除）。
- 工作区指纹：
  - 任务开始基线（改动前）：`c2feef9628fbe39853f1ee1dc2524ee441900b21d7b5707e56b49f5e4197f6dd`。
  - 验收状态（门禁、便携构建、隔离实机完成后，写文档前）：`0ff68f46eaa4ae7742546b0a2197900f2728e95a30c82b1c416f2a196823d390`；文档写完后用相同排除再次核对为同值。
  - 排除：`docs/current-state.md,docs/architecture.md,docs/ipc-contract.md,docs/tasks/TASK-31-PRESERVE-ENRICHED-TAGS-ON-RESCAN.md,docs/tasks/done/TASK-31-PRESERVE-ENRICHED-TAGS-ON-RESCAN.md`。

## 状态

- 状态：完成（过程偏差见「验收」第 4 项）
- 依赖与前置任务：TASK-26～TASK-30（显式补全、工作台、候选缓存、新入库聚焦）。
- 下一步：无。
- 提交：待提交。本任务未授权 commit。
- 遗留事项的任务 ID：无新增。

## 范围

- 要改：
  - `src-tauri/src/catalog/db.rs`：`tracks` 增量列 `tags_enriched INTEGER NOT NULL DEFAULT 0`；`upsert_tracks` 的 `ON CONFLICT(path) DO UPDATE` 在已补全行保留文本标签与 `album_source`，只刷新 `duration_ms`/`disc_number`；`tags_enriched` 与原 `added_at` 不在 SET 列表。
  - `src-tauri/src/catalog/enrichment.rs`：`update_library_tags=true` 的 `tracks` UPDATE 同时写 `tags_enriched=1`。
  - `src-tauri/src/catalog/library.rs`：未改。扫描仍走 `db::upsert_tracks`，保留逻辑放在唯一 UPSERT 处，避免给 `Track` 增加会改变对外 DTO 的字段。
  - `src-tauri/src/catalog/enrichment_tests.rs`：新增重扫保留回归测试（临时 fixture、SHA-256、模拟重扫与重启）。
  - `docs/current-state.md`、`docs/architecture.md`、`docs/ipc-contract.md`。
- 不要改：播放引擎、解码器、AI 工具允许列表、依赖版本、能力权限、CSP、主配置；不改 `tracks` 对外 DTO/命令签名；不新增 IPC 命令/参数。
- 是否涉及音频、数据库、IPC 或用户数据目录：数据库（增量列与 UPSERT 语义）；不写音频字节（`write_audio_tags=false`）、不新增 IPC、验收只用 `%TEMP%` 隔离数据目录。

## 验收

- [x] 自动测试：PASS
- [x] 浏览器验证：PASS
- [x] Windows Tauri 实机：PASS
- [ ] 未打开用户正在使用的曲库和设置：**未满足（一次性过程偏差，已记录）**。首次自动化文件夹选择器尝试时，应用记住的上次目录是用户音乐库 `E:\LMY-Music\音乐库`，脚本在错误的“当前目录”上点了确认，导致应用把该目录 471 个文件的标签**只读**扫描进临时隔离库。没有改写任何用户文件、歌词、设置或用户曲库数据库；该临时目录已作为敏感副本删除。后续所有验收运行都先把选择器导航到临时目录、读地址栏确认后才接受，并让驱动断言所有入库路径必须位于临时目录内。

实机使用按 `docs/isolation.md` 构建的便携包；未修改 `src-tauri/tauri.conf.json`。

## 来源

- 任务 ID：TASK-31-PRESERVE-ENRICHED-TAGS-ON-RESCAN
- 时间：2026-11-17（Australia/Sydney）
- HEAD：`a8c9d2130f9b75a11aea1f2cef6333574ac3df83`
- 工作区指纹：基线 `c2feef96…`；验收状态 `0ff68f46…`（口径见「基线」）
- 命令、工具版本、构建配置：pnpm 12.21.0、Node 24.21.0（DSH 运行时，PATH 需包含其 `node/bin`）、rustc/cargo 1.99.0、PowerShell 7.6.10、WebView2 157.0.4341.46。自动测试：`pnpm format` 退出 0；`pnpm test:ui` 85/85 退出 0（首次运行因 Vite 冷启动前 4 项超时，重跑全通过，未更新任何快照）；`pwsh -File scripts/check.ps1` 退出 0（svelte-check 0 errors/0 warnings、Node 测试、cargo fmt/clippy、Rust 单元 151 通过/1 ignored、音频 3/3、歌词 6/6），仅保留既有 Cargo PDB 文件名冲突警告。便携构建：`powershell -File scripts/build-portable.ps1` 退出 0。
- 产物哈希：`dist-portable/Music-Mine-Portable-x64.zip` = `942899C703DFFC65C994B566098DE87686C4B8644BB92A9B9B0573C019B1E707`；包内 `music-mine-desktop.exe` = `B4EE7685B79CB69CC601CBA5B833152B6872D4C51D46B63882FDADE4BF0504BA`（隔离来源记录 `tests/perf/out/isolation-sources/B4EE…json`，`portable-relative`，配置哈希 `573DA293…`）。
- 结果和未验证项：
  - Rust 回归测试：新增 `rescan_preserves_enriched_library_tags_without_touching_the_audio_file`。受控红灯：临时关掉保留条件时重扫把标题改回文件名 `fixture`，断言失败；恢复后通过。测试同时校验迁移列默认 0、重扫后 `tags_enriched=1`、`added_at` 不变、物理字段仍刷新、音频 SHA-256 与字节不变。
  - 隔离实机（便携包解压到 `%TEMP%\mm-task31-9964218cc728\app`，数据目录为 exe 旁 `app-data`，WebView2 用户目录隔离，`MUSIC_MINE_NULL_OUTPUT=1`）：首次扫描只入库临时 `music\fixture.flac`；只更新曲库标签（`write_audio_tags=false`）并写 `.lrc` 与同名 `.cover.jpg`（真实 CAA `front` 下载，192312 字节真 JPEG）；再次完整重扫同一目录；优雅关闭后重新启动 exe。重扫与重启后 `id/title/artist/album/album_artist/genre/year/track_number/added_at` 保持补全值，`album_source=tag`、`tags_enriched=1`，`duration_ms` 仍为 3000；`.lrc` 54 字节且内容一致；`.cover.jpg` 192312 字节、JPEG 头有效，重扫与重启后仍可读；音频 SHA-256 全程为 `6A04EA578BE1751513CE4ED20AEEDB5212A2FE2E179C3C7353463E89A8D077DA`。两次关闭均 `graceful`。系统真实数据目录 `%APPDATA%\local.musicmine.desktop` 的存在性与 LastWriteTime 前后一致。
  - 证据文件（已忽略，不入库）：`tests/perf/out/task31-evidence/{scan,enrich,rescan,restart,pick-scan,pick-rescan,summary}.json`、`tests/perf/out/task31-{format,ui-rerun,check,build,accept}.log`。
  - NOT RUN：真实 MusicBrainz/iTunes/LRCLIB 检索链路与 600px Apple 封面（本轮只验证 CAA 封面下载）；真实音频出声、托盘、DPI、系统媒体键；真实用户曲库迁移（本任务只用临时库与旧 schema fixture）。

## 结束报告

- 改动文件：`src-tauri/src/catalog/db.rs`、`src-tauri/src/catalog/enrichment.rs`、`src-tauri/src/catalog/enrichment_tests.rs`；文档 `docs/current-state.md`、`docs/architecture.md`、`docs/ipc-contract.md`；本任务单移入 `docs/tasks/done/`。
- 三条验收结果：自动测试 PASS；浏览器验证 PASS；Windows Tauri 实机 PASS（含封面/歌词副作用与重启保持）。第 4 项隔离约束有一次已记录的过程偏差。
- 未验证项：见上「NOT RUN」。真实外部服务与硬件验收缺口仍按 `docs/current-state.md` 保留，不因本轮通过而放行。
- Git：未提交（未授权）。
- 完成后把本记录移到 `docs/tasks/done/`。
