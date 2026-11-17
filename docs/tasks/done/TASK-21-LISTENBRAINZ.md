# 任务：可选 ListenBrainz 听歌记录与本地推荐

## 基线

- 分支与 HEAD：main / 2af3b3c66d6449edf6d48e482f042a9b3d31cc67。
- 未提交改动：开工时工作区干净。current-state 的上轮未提交说明已落后于当前 HEAD。
- 工作区指纹：`84ba08ac6188cb4e2c5f56b443babd2fbeb82bfb5cb9cc2b1dd324065870b295`，最终门禁前后及实机后相同。排除本单 active/done 路径和 docs/current-state.md。

## 状态

- 状态：完成
- 依赖与前置任务：现有 Controller 快照、Desktop 偏好、Catalog SQLite。
- 下一步：归档后停止。真实账户与外网端到端留作未验证项。
- 提交：待提交；用户禁止自动 commit。
- 遗留事项的任务 ID：TASK-21 实际 ListenBrainz 账户与外网端到端验收。

## 范围

- 默认关闭；显式开启后才采集/上传，关闭保留待传记录。只接 ListenBrainz；Last.fm 可由用户在 ListenBrainz 网站关联，本应用不直接接入 Last.fm。
- 后台采集和网络调度分开，网络不占 Controller 锁。实际进度增量受单调时间约束，暂停与 Seek 不计；同曲重播使用内部会话序号区分。
- scrobble_outbox 持久化，按用户名隔离队列；成功后删除，失败保留并退避；统一网络限流至少 1.1 秒，遵守 Reset-In/Retry-After。
- Token 只存 desktop-v1.json，不经查询 IPC 返回，不进日志；设置使用密码框。空 Token 清除，null 保留。保存凭证时验证 Token 并核对用户名，后续开关无需网络。请求取消版本与成功提交的采集版本分开，同一会话按用户名保留去重记录；失败/重复保存不重置计时或造成重复上传。PlayingNow 在限流结束、发送前再次校验会话。
- 推荐 API 返回 MBID，通过官方元数据解析歌曲/艺术家名后严格匹配本地曲库；无推荐时回退公开近期收听。缓存远端候选而非本地结果，重新匹配当前 SQLite，避免已移除曲目残留。
- 不改音频回调/解码器/真实用户数据。涉及数据库新增表、Desktop 配置、IPC 与 UI。
- 依赖：现有直接依赖没有 HTTP 客户端；锁文件的 reqwest 仅为其他目标的间接依赖，本机没有其源码。选择本机已有、较轻的 ureq =2.12.1，关闭默认 features、只开 json/tls（Rustls + ring），无 OpenSSL；同步客户端只运行在 network 线程，增加 Rustls/ring 构建成本。Cargo.lock 已锁定。

## 验收

- [x] 自动测试：PASS。pnpm format、pwsh -File scripts/check.ps1 退出 0；Svelte 0 error / 0 warning，前端单元 85/85，Rust fmt/clippy 通过，单元 109 passed / 1 ignored，音频集成 3/3，歌词集成 6/6。新增 10 项 Scrobble 测试和 1 项日志脱敏测试，覆盖阈值、暂停/前后 Seek/停滞/重播、队列失败保留/确认删除/账户隔离/重启、配置默认关闭/Token 返回边界、响应体限额、限流和同会话去重。测试不发真实外网请求，音频使用 Null 输出。
- [x] 浏览器验证：PASS。pnpm test:ui 最终 53/53，退出 0。新增 4 项默认关闭/凭证失败和成功/键盘开关/窄窗字段、推荐卡片播放和右键菜单、空结果/网络失败用例。1280×800 与 900×640 均检查，新卡片和推荐架截图已查看。首轮 51/52，仅 settings-dark 的副标题文字产生 316 像素差异；查看后只更新该一张基线，最终普通复跑零差异。
- [x] Windows Tauri 实机：限定范围 PASS。哈希校验通过的隔离 exe，1280×850 / 100%，旧偏好缺字段默认关闭；真实 listenbrainz_config 查询/禁用/清除凭证持久化、缺凭证启用被拒、非法 Token 在 HTTP 前被拒，禁用推荐返回空，desktop_preferences 不回传 Token，Console 无测试凭证。真实 WebView2 密码框和关闭状态已截图；临时旧 schema 新增 outbox 八列，原 3 首 fixture 保留。实例已关闭；无外网上传。
- [x] 未打开用户正在使用的曲库和设置

## 来源

- 任务 ID：TASK-21-LISTENBRAINZ
- 时间：2026-11-17
- HEAD：2af3b3c66d6449edf6d48e482f042a9b3d31cc67
- 工作区指纹：同基线；命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-21-LISTENBRAINZ.md,docs/tasks/done/TASK-21-LISTENBRAINZ.md,docs/current-state.md`。
- 命令、工具版本、构建配置：pnpm 12.21.0、Node v24.14.0、Cargo 1.99.0、PowerShell 7.6.10、Playwright 1.65.1。三项指定门禁均退出 0。阈值/outbox 测试先确认缺失失败；503 Retry-After 回归先确认 1.1s ≠ 60s 失败，再修复通过。代码只读审阅发现的计时重置、过期 PlayingNow 和 503 退避问题均修复，复审无 Critical/Important。既有 Cargo PDB 名称冲突警告仍存在。
- 构建：`pwsh -File scripts/build-isolated.ps1 -Config <测试根>/isolation.json -TestRoot <测试根>`；底层 `pnpm exec tauri build --debug --no-bundle --config <测试根>/isolation.json`。测试根 `%TEMP%/music-mine-task21-0ff7810f38304937ae78b5d03e1f8a17`，appDirectoriesOverride 仅 CLI 合并到该根的 data，未修改主配置。来源类型 absolute-override；构建发生在 Cargo 测试之后，避免测试重写 exe。
- 产物哈希：`src-tauri/target/debug/music-mine-desktop.exe` SHA-256 `C259D4FDC8FD7451435EAE0E180B75CBD947A3DE2D07A587EE51C1B2188C8C30`；配置 SHA-256 `FE0CE149AF74C8A700B743BA3F4E164857CFD7921939DA29E4B802FCFCE885B1`。哈希绑定来源 `tests/perf/out/isolation-sources/C259D4FDC8FD7451435EAE0E180B75CBD947A3DE2D07A587EE51C1B2188C8C30.json`（忽略产物）。
- 证据：忽略目录 tests/perf/out/task21-{format,ui,check,build}.log；测试根 desktop-result.json、listenbrainz-settings-real.png、process.json、stderr.log。启动前使用 Test-IsolationSource，关闭前核对 PID/启动时间/exe，实例已关闭。测试根只包含新建库、旧 schema 与仓库 fixture 副本，未访问真实用户库、设置、音乐或 Token。
- 三层结果和未验证项：自动测试 PASS；浏览器 PASS；Windows 限定范围 PASS。真实 ListenBrainz 账户验证/外网 TLS 和上传/真实服务限流/在线推荐命中、硬件音频、DPI 125%/150% 均 NOT RUN；模拟的成功/失败队列测试不能替代真实服务验收。
- API 依据：https://listenbrainz.readthedocs.io/en/latest/users/api/core.html 、recommendation.html、metadata.html；依赖依据 https://docs.rs/crate/reqwest/0.13.5/features 。

## 结束报告

- 改动文件：Rust Cargo.toml/lock、catalog/db.rs、infrastructure/logging.rs、lib.rs、platform/{mod,desktop,scrobble}.rs、playback/controller.rs；前端 music.ts、ListenBrainzSettings.svelte、SettingsView/HomeView；harness、ListenBrainz UI 测试及 settings-dark 快照；IPC 契约、本单和 current-state。
- 三条验收结果：见验收；三项指定门禁全部退出 0，当前工作区指纹一致。
- 未验证项：真实账户、真实外网上传、硬件出声及 DPI。
- Git：不提交。
- 完成后移到 docs/tasks/done/。
