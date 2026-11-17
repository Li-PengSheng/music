# 当前事实与验收记录

进度以本页为准；目标与历史记录不能替代源码和当前验收。

## 核验基线

- 时间：2026-11-17。分支 `main`，HEAD `d5f69abf2da382d58822b99e0e1293a152ad9669`，TASK-25 开工时工作区干净；本轮修改未提交。
- 工作区指纹：`eb1ed01dcd47151b5fa5cd342d12357d7c1fc5c95a57736d9ee5ca83a76d17a3`。命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-25-FULL-REVIEW.md,docs/tasks/done/TASK-25-FULL-REVIEW.md,docs/current-state.md,docs/architecture.md`，验收前后及归档前后核对。
- 工作区内容：全量边界与文档 Review；PCM 回调改为 try_lock，竞争时静音/underrun，诊断采样只用预分配容量，错误回调仅置原子标志并由工作线程记录；暂停/时钟重置增加工作线程屏障，等待已进入回调发布最终帧，修复本轮复现的暂停时钟晚增问题。输出设备偏好写盘移出 Controller 锁、只读 desktop_preferences 不写盘、主要 IPC 错误路径脱敏、扫描结束日志移出进度锁、ListenBrainz 线程退出收拢与推荐匹配减少重复归一化；首页/设置/歌词与服务设置页异步过期保护、收起动画期间歌词错误抑制。显式收敛 Symphonia features 并保留既有 ADPCM WAV 能力；删除未使用 Library 容器。解码算法、数据库 schema、权限及主 Tauri 配置未改，未打开用户数据。
- 自动测试：pnpm format、pwsh -File scripts/check.ps1 退出0；Svelte 0 errors/0 warnings，前端95/95、Rust fmt/clippy、单元118通过/1 ignored、音频3/3、歌词6/6，Null输出。首轮暂停时钟失败未被复跑覆盖：已增加受控红/绿回归并修复在途回调屏障；既有Cargo PDB警告保留。
- 浏览器验证：pnpm test:ui 68/68、退出0；既有快照 maxDiffPixels=0 / threshold=0 全通过且基准文件无 diff，未更新快照。新增6项覆盖旧请求/关闭响应与4页面×2尺寸×40主题字体组合（320状态）；已查看4张900×640截图。harness 只证明前端，不证明 Rust 播放。
- Windows Tauri 实机：本轮 NOT RUN；没有构建、启动 exe，不使用旧产物作为本轮证明。

## 当前任务

TASK-25-FULL-REVIEW 审查与本轮修复完成并归档：[审查报告](tasks/done/TASK-25-FULL-REVIEW.md)。三项指定门禁均0，未自动 commit。R11–R13 和实机缺口保留，不代表全项架构或完整发布放行。

## 已知缺陷与架构债务

- **Controller 锁内 I/O 尚存**：execute 包含标签读取、音频线程屏障和同步 playback-v1.json 保存，State 也可能处理 EOF。已移出的只是额外桌面偏好写盘，不能称为全局零锁内 I/O。拆分需保留 revision、EOF 与落盘顺序。
- Catalog 初始化/operation 锁刻意覆盖 schema 与写事务；网络不在这些锁内。主控制线程依 exiting 停止但未 join，AI 在途 spawn_blocking 不能强制取消；真实退出时序尚未验收。
- 会话内替换封面而不重新扫描时，页面可继续显示旧图，直到重新扫描或重启；缓存按 mtime 和长度失效。
- 超过 512KB 的 WebP/GIF，以及 BMP、TIFF、AVIF、HEIC 仍不展示封面。
- 凭证在 desktop-v1.json 明文保存，未接入系统凭据库；LogBuffer 文件出口若启用会保留原始路径，当前 Controller 的 Player::new(None) 未启用该出口。Console 返回绝对目录脱敏，专用配置不返回凭证。自选远端 HTTP AI 服务的链路不加密，宜使用 HTTPS 或本机。

## 验收缺口

NOT RUN 不是已知缺陷，也不代表发布放行。

- **真实外网 Scrobble**：ListenBrainz 真实账户、Token、TLS、上传确认/重启重复提交窗口、限流/退避、推荐元数据与网络故障恢复均为实机外部依赖项，NOT RUN。
- **真实 AI API**：真实厂商、Key、外网 TLS、计费/配额及非标准扩展兼容为实机外部依赖项，NOT RUN。Mock 的工具闭环、取消和脱敏测试不证明外网成功。
- 当前修复的真实 IPC、偏好写盘回退、后台退出、WebView2 字体同步、ClearType、125%/150% DPI、硬件音频、原生选择/扫描、托盘、任务栏按钮与系统媒体键均需隔离 exe，NOT RUN。
- 库外 player_open_file 与 WAV 经界面打开仍无当前实机证据。本轮复现 Null 暂停时钟漂移，已定位并修复“在途回调晚提交帧”竞态，受控回归经历失败/通过；不能回填历史无指纹事件，也不能由此证明真实设备无其他暂停问题。
- 旧任务 09.5.3 / 09.5.5 没有指纹，仍为“未记录，无法重建”；较早隔离 exe 与模拟服务结果不能代替本轮。

## 发布决定

Review 和自动/浏览器门禁不构成完整发布。上述 Windows 与外网依赖验收缺口仍阻止把当前程序称为完整发布；本轮不操作真实凭证、曲库或设置。

## 源码事实

| 领域 | 当前事实 |
|---|---|
| 技术栈与版本 | Rust、Tauri v2、Svelte 5 Runes、TypeScript、Vite、Tailwind v4，产品版本 0.1.0 |
| UI 基础 | bits-ui 适配 Select / ContextMenu / Modal，原生 range Slider，Floating UI tooltip；无 shadcn-svelte、Vitest、WebdriverIO、TanStack Virtual、tauri-specta |
| 播放权威 | Rust 带 revision 快照，经 player.accept 接收；约 200ms 轮询，前端不插值推进进度；control 防并发播放命令 |
| IPC | 业务 invoke 集中 music.ts；41 个注册、39 个前端封装，例外 library_cover / library_favorite；WindowControls 直接使用官方窗口 API |
| 本地曲库 | 扫描、搜索、歌曲/专辑/艺术家、歌单管理与排序、移除文件夹库内记录；旧版数据库导入全链路删除，schema 增量升级保留 |
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
| P1 | 隔离 exe 验证本轮 Rust/IPC 与退出路径 | 哈希绑定来源、数据目录、命令结果及退出记录 |
| P1 | Controller 会话/标签 I/O 解耦 | 保留时序语义的设计、并发/错误 fixture、Windows 播放验收 |
| P1 | 真实外部服务与硬件/扫描 | 用户专门授权凭证及隔离数据后的实际记录 |
| P2 | 系统凭据库、结构化错误与日志隐私边界 | 独立设计和迁移/回归证据，不覆盖用户设置 |
