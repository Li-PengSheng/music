# 当前事实与验收记录

进度以本页为准；目标与历史记录不能替代源码和当前验收。

## 核验基线

- 时间：2026-11-17。分支 `main`，验收基线 HEAD `a8c9d2130f9b75a11aea1f2cef6333574ac3df83`；下列指纹与测试为提交前验收记录。
- 工作区指纹：验收状态 `0ff68f46eaa4ae7742546b0a2197900f2728e95a30c82b1c416f2a196823d390`，命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/current-state.md,docs/architecture.md,docs/ipc-contract.md,docs/tasks/TASK-31-PRESERVE-ENRICHED-TAGS-ON-RESCAN.md,docs/tasks/done/TASK-31-PRESERVE-ENRICHED-TAGS-ON-RESCAN.md`；文档写完后用相同排除再核对为同值。改动前基线为 `c2feef9628fbe39853f1ee1dc2524ee441900b21d7b5707e56b49f5e4197f6dd`。
- 工作区内容：TASK-31 给 `tracks` 增加 `tags_enriched` 增量列；显式补全更新库标签时置 1；扫描 UPSERT 对已补全行保留 title/artist/album/album_artist/genre/year/track_number/album_source，只刷新 duration_ms/disc_number，tags_enriched 与原 added_at 不变。未改 `Track` DTO、IPC 命令/参数、前端、依赖、权限、CSP 和主配置；TASK-26～30 的补全、缓存、忽略标记与既有快照保持不变。
- 自动测试：pnpm format、pnpm test:ui（85/85）、`pwsh -File scripts/check.ps1` 均退出 0；svelte-check 0 errors/0 warnings；Rust fmt/clippy 通过；单元 151 通过/1 ignored、音频 3/3、歌词 6/6，Null 输出。新增 Rust 1 项覆盖重扫保留、SHA-256 与模拟重启，受控红灯后转绿；快照零像素差异且未更新。仅保留既有 Cargo PDB 文件名冲突警告。
- 浏览器验证：pnpm test:ui 85/85、退出 0，maxDiffPixels=0 / threshold=0，未改快照。harness 只证明前端，不证明 Rust IPC、SQLite 或外网；后端由临时库单测与隔离实机验证。
- Windows Tauri 实机：PASS（`dist-portable/Music-Mine-Portable-x64.zip`，exe SHA-256 `B4EE…`，隔离来源记录绑定，数据目录为 exe 旁 `app-data`）。在 `%TEMP%` 临时目录完成 扫描 → 只更新库标签并写 `.lrc` / `.cover.jpg`（真实 CAA 封面）→ 完整重扫 → 优雅退出后重启；补全值、added_at、tags_enriched 与 album_source 保持，音频 SHA-256 不变，sidecar 可读，系统 `%APPDATA%` 数据目录未被改动。过程偏差：首次自动化文件夹选择器误选了用户音乐目录，只读扫描 471 个文件到临时隔离库；未改写任何用户文件或设置，临时库已删除，后续运行先校验地址栏并用路径白名单拦截。

## 当前任务

TASK-31-PRESERVE-ENRICHED-TAGS-ON-RESCAN 已实现并归档：[任务记录](tasks/done/TASK-31-PRESERVE-ENRICHED-TAGS-ON-RESCAN.md)。三项指定门禁均 0，Release 便携包已构建并做隔离实机验证，未自动 commit。真实外部补全检索（MB/iTunes/LRCLIB）、真实音频出声、托盘/DPI/系统媒体键、真实用户库迁移仍未验证；原有实机缺口继续保留，不代表完整发布放行。首次选择器误选用户目录的过程偏差记录在任务单与上文，并以此加强了自动化验收的路径约束。

后续 Git 处理：用户明确要求“完成git处理”，将 TASK-31 的重扫保留补全标签改动整理为本地提交 `fix(catalog): preserve enriched library tags on rescan`。任务单保留验收时的未提交状态；此次未推送远端。源码未再修改，复用上述指纹一致的门禁结果。

## 已知缺陷与架构债务

- **Controller 锁内 I/O 尚存**：execute 包含标签读取、音频线程屏障和同步 playback-v1.json 保存，State 也可能处理 EOF。已移出的只是额外桌面偏好写盘，不能称为全局零锁内 I/O。拆分需保留 revision、EOF 与落盘顺序。
- Catalog 初始化/operation 锁刻意覆盖 schema 与写事务；网络不在这些锁内。主控制线程依 exiting 停止但未 join，AI 在途 spawn_blocking 不能强制取消；真实退出时序尚未验收。
- 显式补全成功会失效前后端封面缓存，主窗口按 metadata_revision 刷新；应用之外改写文件仍可能需要重新扫描/重启。SMTC 同路径封面缓存未在本任务实机验收。
- **补全标记不可清除**：某行一旦 `tags_enriched=1`，重扫会一直保留库内文本标签；用户在应用外修改该音频标签后不会被扫描覆盖，目前没有“恢复按文件标签重扫/清除标记”的入口。需要时作为单独任务设计。
- 超过 512KB 的 WebP/GIF，以及 BMP、TIFF、AVIF、HEIC 仍不展示封面。
- 凭证在 desktop-v1.json 明文保存，未接入系统凭据库；LogBuffer 文件出口若启用会保留原始路径，当前 Controller 的 Player::new(None) 未启用该出口。Console 返回绝对目录脱敏，专用配置不返回凭证。自选远端 HTTP AI 服务的链路不加密，宜使用 HTTPS 或本机。

## 验收缺口

NOT RUN 不是已知缺陷，也不代表发布放行。

- **真实外网 Scrobble**：ListenBrainz 真实账户、Token、TLS、上传确认/重启重复提交窗口、限流/退避、推荐元数据与网络故障恢复均为实机外部依赖项，NOT RUN。
- **真实 AI API**：真实厂商、Key、外网 TLS、计费/配额及非标准扩展兼容为实机外部依赖项，NOT RUN。Mock 的工具闭环、取消和脱敏测试不证明外网成功。
- **真实补全服务**：MusicBrainz / iTunes CN/US / LRCLIB / CAA / Apple CDN 的实际 TLS、地域、限流、数据覆盖、重定向及600px封面可用性 NOT RUN。Windows文件占用、真实IPC/写权限与音频标签格式差异仍需隔离exe。文件/SQLite错误有回滚测试，不提供断电或进程崩溃时的跨文件事务恢复。
- 当前修复的真实 IPC、偏好写盘回退、后台退出、WebView2 字体同步、ClearType、125%/150% DPI、硬件音频、原生选择/扫描、托盘、任务栏按钮与系统媒体键均需隔离 exe，NOT RUN。TASK-31 的隔离便携包已实机覆盖「扫描 → 显式补全 → 重扫 → 重启」与 sidecar 写入；其余项目（真实出声、托盘、DPI、系统媒体键、人工操作原生对话框）仍未验证。
- 库外 player_open_file 与 WAV 经界面打开仍无当前实机证据。本轮复现 Null 暂停时钟漂移，已定位并修复“在途回调晚提交帧”竞态，受控回归经历失败/通过；不能回填历史无指纹事件，也不能由此证明真实设备无其他暂停问题。
- 旧任务 09.5.3 / 09.5.5 没有指纹，仍为“未记录，无法重建”；较早隔离 exe 与模拟服务结果不能代替本轮。

## 发布决定

Review 和自动/浏览器门禁不构成完整发布。上述 Windows 与外网依赖验收缺口仍阻止把当前程序称为完整发布；TASK-31 的便携实机只证明「扫描/补全/重扫/重启」这一条链路，不改变缺口结论。本轮不操作真实凭证、曲库或设置。

## 源码事实

| 领域 | 当前事实 |
|---|---|
| 技术栈与版本 | Rust、Tauri v2、Svelte 5 Runes、TypeScript、Vite、Tailwind v4，产品版本 0.1.0 |
| UI 基础 | bits-ui 适配 Select / ContextMenu / Modal，原生 range Slider，Floating UI tooltip；无 shadcn-svelte、Vitest、WebdriverIO、TanStack Virtual、tauri-specta |
| 播放权威 | Rust 带 revision 快照，经 player.accept 接收；约 200ms 轮询，前端不插值推进进度；control 防并发播放命令 |
| IPC | 业务 invoke 集中 music.ts；45 个注册、43 个前端封装，例外 library_cover / library_favorite；WindowControls 直接使用官方窗口 API |
| 本地曲库 | 扫描、搜索、歌曲/专辑/艺术家、歌单管理与排序、移除文件夹库内记录；重扫对已补全行保留库内文本标签、只刷新时长/碟号，tags_enriched 与 added_at 不变；旧版数据库导入全链路删除，schema 增量升级保留 |
| 在线补全 | 独立双栏工作台，详情跳转、默认近7天新入库/最近添加优先/无新歌回退四类缺失、可恢复忽略、串行批处理/取消；MB+iTunes合并、LRCLIB反哺、后缀重试、来源标记与可编辑目标。共享Store保留会话/草稿，revision增量同步；SQLite候选缓存七天、显式强刷，忽略标记独立于候选有效期。默认曲库和缺失sidecar（.lrc / .cover.jpg），音频/覆盖独立opt-in；库标签写入置 tags_enriched，重扫不覆盖；无自动后台补全 |
| 解码能力 | Symphonia default-features=false，显式 mp3 / flac / wav / pcm / adpcm；MP3/FLAC/PCM WAV fixture 已有，ADPCM 没有专用 fixture，不承诺所有 WAV 编码 |
| 外观 | palette 与 theme 独立：celadon / cinnabar / azure / amber × light/dark/system；默认浅色 celadon。font 独立：harmony / puhuiti / serif / kaiti / system，仅使用本机字体及回退 |
| 样式 | 颜色位于 tokens.css；组件独有 CSS 留在组件，共享样式在 app.css，不以行数作为稳定契约 |
| 队列与歌词 | retainQueue 保留未变化队列；宽屏队列覆盖；桌面歌词标签 lyrics-overlay，独立 capability，只轮询状态和歌词 |
| 桌面 | 无边框窗口、输出设备选择、会话恢复、托盘与 SMTC；任务栏缩略图按钮 ID 1001–1004。初始音量20%，已有会话保留 |
| Console | 最多1000行内存日志、清空仅内存、复制脱敏摘要、2秒诊断、本地短命令与AI分视图 |
| ListenBrainz | 默认关闭，observer/network 独立；scrobble_outbox 绑定账户，成功确认后删除；推荐按艺术家+曲名匹配当前本地曲库；无独立 Last.fm API |
| AI | 默认关闭，Rust 有界单轮 Chat Completions；Console 最多5轮、六类允许工具、取消/配置过期保护；不向工具投影路径、封面、歌词或日志 |
| 凭证与依赖 | 配置查询不返回 Key/Token，日志凭证先脱敏；ureq 2.12.1 禁默认 feature，仅 json/tls（Rustls）；rusqlite 仅 bundled，无 backup |

## 文档边界

- architecture / ipc-contract / rust-guidelines / README 已按当前源码刷新，详细分级发现与验收证据见 TASK-25 报告。
- AGENTS 中过时的“无 AI / 未安装 Bits UI”已校正，保留默认关闭、工具允许列表、数据与权限边界。
- RFC-v2.0 是目标，其 Vitest 与前端时钟插值描述不能覆盖当前实现。
- docs/tasks/done 和 docs/history 是历史基线，不改写旧验收；本轮没有用其结果勾掉当前缺口。

## 后续优先级

| 优先级 | 事项 | 完成证据 |
|---|---|---|
| P1 | 隔离 exe 验证剩余 Rust/IPC 与退出路径（扫描/补全/重扫/重启已由 TASK-31 覆盖） | 哈希绑定来源、数据目录、命令结果及退出记录 |
| P1 | Controller 会话/标签 I/O 解耦 | 保留时序语义的设计、并发/错误 fixture、Windows 播放验收 |
| P1 | 真实外部服务与硬件/扫描 | 用户专门授权凭证及隔离数据后的实际记录 |
| P2 | 系统凭据库、结构化错误与日志隐私边界 | 独立设计和迁移/回归证据，不覆盖用户设置 |
