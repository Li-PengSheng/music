# 当前实现架构

## 核验基线

- 核验基线：2026-11-17，`main` / HEAD `d5f69abf2da382d58822b99e0e1293a152ad9669`，开工工作区干净；TASK-25 修改未提交。
- 本页描述当前源码；最终工作区指纹和三层检查证据见 [current-state](current-state.md) 与 TASK-25 审查报告。
- TASK-25 最终指纹 `eb1ed01dcd47151b5fa5cd342d12357d7c1fc5c95a57736d9ee5ca83a76d17a3`；format/check/UI均0，UI68/68且既有快照零差异；Windows实机 NOT RUN。细节与保留项见 [审查报告](tasks/done/TASK-25-FULL-REVIEW.md)。
- 覆盖 Console、ListenBrainz、AI 代理/工具、字体预设、已删除的旧库导入及公共播放/曲库边界。真实外网服务和 Windows 实机没有由源码审查替代。

本次核验覆盖交互原语与指定组件 CSS 搬迁后的工作区检查，不代表逐行重审所有实现。

## 数据流

```text
src/App.svelte、src/lib/views/、src/lib/layout/
        │
        ▼
src/lib/stores/*.svelte.ts
        │
        ▼
src/lib/services/music.ts          全部业务 invoke、主题/歌词事件 API
        │
        ▼
src-tauri/src/lib.rs               命令注册与应用状态
        ├── audio/player.rs              Symphonia 解码、cpal 输出
        ├── playback/controller.rs          播放控制、队列、会话
        ├── platform/media.rs               Windows 媒体集成
        ├── catalog/library.rs / catalog/mod.rs / commands/catalog.rs / catalog/collections.rs
        ├── catalog/db.rs                  SQLite
        ├── playback/lyrics.rs / catalog/metadata.rs
        ├── artwork/covers.rs              封面读取、尺寸限制和磁盘缓存
        ├── platform/desktop.rs             窗口、主题、托盘、桌面歌词窗口、desktop-v1.json
        ├── platform/thumbbar.rs            Windows 任务栏缩略图按钮（ITaskbarList3）
        ├── platform/scrobble.rs            Rust 快照采集 → SQLite outbox → 独立 HTTP 调度
        ├── platform/ai_agent.rs            有界 Chat Completions 代理，不执行模型工具
        └── infrastructure/logging.rs

ConsoleModal → ai-conversation（最多 5 轮）→ music.ai_chat_turn → Rust HTTP → 配置的 AI 服务
                    └→ ai-tools（六类允许工具）→ music.ts / 本地外观偏好 → Rust 权威快照
HomeView → listenbrainz_recommendations → 远端候选缓存 → 当前本地库精确匹配
SettingsView → AiSettings / ListenBrainzSettings → 专用配置 DTO（不含凭证）
```

改调用链时从实际函数追，不要假设每一条 UI 操作都经过同一个包装。

## 代码地图

| 位置 | 当前角色 |
|---|---|
| `src/App.svelte` | 主窗口壳、连接、约 200ms 轮询、快捷键。`?window=lyrics` 时只挂载桌面歌词 |
| `src/lib/layout/` | `Sidebar`、`TopBar`、`PlayerBar`。播放栏可开关桌面歌词，锁定时改为解锁 |
| `src/lib/views/` | `HomeView`、`SongsView`、`GridView`、`SettingsView`、`DesktopLyrics.svelte` |
| `src/lib/views/DesktopLyrics.svelte` | 置顶悬浮歌词。复用 `groupLines` / `activeLine`，不挂曲库、队列或封面；歌词读取独立于状态轮询，失败间隔重试，切歌丢弃旧结果 |
| `src/lib/components/` | Bits UI Select / ContextMenu / Modal 适配层，Dialog 业务包装，原生 range Slider，以及 TrackList / EmptyState |
| `src/lib/components/Select.svelte` | Bits UI Select，保留项目 Props 和 tokens |
| `src/lib/layout/Queue.svelte`、`src/lib/views/NowPlaying.svelte`、`src/lib/components/QueueCover.svelte` | 队列抽屉、正在播放和队列封面。宽屏队列覆盖页面，不把正在播放挤窄 |
| `src/lib/components/Artwork.svelte`、`src/lib/components/Cover.svelte` | 正在播放和列表里的封面 |
| `src/lib/utils/cover-loader.ts`、`row-visibility.ts` | 封面并发、取消和可见行 |
| `src/lib/stores/player.svelte.ts` | `accept()` 丢掉更旧的 `revision`；`control()` 串行播放命令 |
| `src/lib/services/music.ts` | 全部前端业务 IPC 类型和调用；窗口/事件 API 封装。WindowControls 直接使用官方窗口 API |
| `src/lib/components/ConsoleModal.svelte` | 脱敏会话日志、2 秒诊断轮询、本地短命令、AI 对话。销毁时清理定时器并中止后续 AI 轮次 |
| `src/lib/components/AiSettings.svelte`、`ListenBrainzSettings.svelte` | 默认关闭的服务配置；输入凭证只用于保存请求，清理弹窗状态，不读取已保存密钥 |
| `src/lib/utils/ai-conversation.ts` | 配置版本检查、最多 5 轮、工具结果配对、取消/失败检查点与完整轮次裁剪 |
| `src/lib/utils/ai-tools.ts` | 六类工具参数验证、最小元数据投影、路径/底层错误隔离、部分写入报告、AbortSignal 检查；没有任意 IPC/shell |
| `src/lib/styles/tokens.css`、`public/theme-init.js`、`src/lib/stores/preferences.ts` | 四色系 × 深浅主题、五套本机字体，首屏设置与 storage/系统主题监听及清理 |
| `src-tauri/src/platform/scrobble.rs` | 200ms observer 独立于 network；账户绑定持久 outbox、限流/退避、Token 验证、推荐缓存；退出后 join 两个线程 |
| `src-tauri/src/platform/ai_agent.rs` | ureq/Rustls 代理、URL/消息/工具边界、单请求限流、固定错误码、凭证脱敏、过期配置和退出响应拒绝 |
| `src-tauri/src/platform/desktop.rs` | 桌面偏好、窗口/托盘/歌词、输出偏好串行保存。只读查询不写盘，返回副本固定清除两种密钥 |
| `src-tauri/src/infrastructure/logging.rs` | stderr/文件/最多 1000 行内存日志；凭证先脱敏，Console IPC 再隐藏绝对目录前缀 |
| `src/lib/utils/queue.ts` | `retainQueue`：曲目未变时保留原数组 |
| `src/app.css` | 基础/共享样式、响应式与减少动画规则；组件独有样式位于对应 Svelte 的 style，颜色来自 tokens |
| `src-tauri/capabilities/default.json` | 主窗口 `core:default`、set-theme 与 minimize / toggle-maximize / close / start-dragging；不影响歌词窗口权限 |
| `src-tauri/capabilities/lyrics-overlay.json` | 桌面歌词窗口只允许 `core:window:allow-start-dragging` |
| `tests/harness/` | 浏览器内存曲库。它设置 `window.isTauri`，并让 `bridge_probe` 返回 `backend: 'rust'` |
| `tests/ui/` | Playwright。视口 1280×800，`deviceScaleFactor: 1` |
| `tests/perf/queue-profile.mjs` | 诊断脚本，不是常规测试命令 |

普通 `pnpm dev` 页面里 `isTauri()` 为假，连接文案是「浏览器预览」。harness 里的在线指示是夹具，不是 Rust 引擎。

## 关键约定

- `PlayerSnapshot` 在 `music.ts`：`revision`、`track_id`、`current_track`、`playback_state`、`position_ms`、`duration_ms`、`volume`、`playback_mode`、`lyrics_offset_ms`、`output_device_id`、`error`、`queue`。改字段时对照 Rust 的 serde 定义。
- 进度显示快照里的 `position_ms`。拖动进度条可以有松开前的本地草稿。
- 数据库用 `CREATE TABLE IF NOT EXISTS` 和 `ensure_column`。没有独立迁移框架。每个 `Catalog` 用实例级 `Mutex<bool>` 只迁移一次；`db::open` 执行 `init`，热路径随后走 `open_light`。旧版数据库导入命令、入口、副本/backup 路径均已删除；当前 schema 升级仍保留。
- `Action::Open` 把 `prepare` 后的 `synchronize` 状态交给同一次 `execute` 的尾部。`Play` 仍单独做一次屏障。成功的 `Start` 仍是两次 `execute` 和两次 `persist` 调用。
- `audio/player.rs` 里音频路径上的 `Mutex` 毒化用 `into_inner` 取出，避免二次 panic。这不证明锁内数据完整。
- Symphonia 禁用默认 feature，显式启用 `mp3`、`flac`、`wav`、`pcm`、`adpcm`；ADPCM 保留此前默认集合带来的 WAV 能力，仅删除未提供入口的 Vorbis/MKV/Ogg。fixture 在 `src-tauri/tests/fixtures/`：`tone.mp3`、`tone.flac`、`tone.wav`。ADPCM 没有专用 fixture，不承诺所有 WAV 编码变体。
- 主窗口默认 1280×850，最小 900×640。桌面歌词窗口默认约 640×108，最小 360×92，标签 `lyrics-overlay`。
- 主窗口无系统装饰，保留 resizable / shadow。TopBar 和 NowPlaying 使用 WindowControls 调用官方窗口 API，只有顶部空白背景可拖动。主窗口关闭默认请求应用退出（包括隐藏歌词窗口），close-to-tray 开启时仍隐藏到托盘。
- 托盘右键菜单展示当前曲目、显示主窗口、播放或暂停、停止、上一首、下一首、桌面歌词、播放模式和退出。歌词锁定时才插入「解锁桌面歌词穿透」。左键在显示与唤起之间切换，隐藏时不退出进程。Windows 任务栏缩略图有上一首、播放/暂停、停止、下一首四个按钮，命令进入同一个控制器。

## 改动分界

- UI：主要在 `src/`。不改 Rust 和数据库。
- 契约：`music.ts` 与 Rust command 一起改，并更新 harness。
- 音频：单独安排 fixture 和 Windows 出声验收。
- 数据：只用临时数据库。
- 桌面：托盘、标题栏、DPI、窗口行为用构建出的 exe。

模块细节见 `docs/rust-guidelines.md`、`docs/ipc-contract.md`、`docs/frontend-guidelines.md`。

## 锁、网络与保留限制

- AI 在配置锁内复制偏好/版本，HTTP 与读取响应体期间无 Controller、Catalog 或偏好锁。ListenBrainz observer 释放 Controller guard 后才入库、记录日志或投递网络；网络请求不持有 Catalog 操作/进度锁。outbox 先完成读取再请求，确认成功后才短事务删除。
- Catalog 的初始化锁和 operation 锁刻意串行化 DDL、扫描、歌单/目录事务，不能称为“所有 Catalog 锁内零磁盘 I/O”；进度锁不覆盖扫描结束日志写盘。
- 输出设备使用独立操作锁；执行设备动作后释放 Controller，再获取偏好锁保存桌面文件，失败重新取得 Controller 回退。配置版本与专用锁避免旧网络验证重新启用服务。
- PCM 回调仅 try_lock ring/capture；竞争时静音并计 underrun，不等待生产者。诊断采样写入预分配容量，回调不扩容；cpal 错误回调只设置 stream_dead，工作线程统一记录中断。Null/锁竞争单测不证明硬件竞争条件下无断音。
- Pause/时钟重置/输出切换与释放通过工作线程的 pause_callbacks 屏障，先置 paused，再等原子回调计数归零，防止已越过暂停检查的回调在同步确认后追加帧。回调只登记原子计数，不等待工作线程；IPC 和 revision 契约不变。
- Controller 现有 execute 仍在锁内读取标签、同步音频工作线程及定期写 playback-v1.json（State 也可能处理 EOF）。这是保留的架构债务，当前锁内 I/O 要求未完全满足；本轮未改播放/会话时序。音频回调仍不执行这些工作。
- ExitRequested 标记退出、释放媒体注册/音频输出；Exit 收拢 ListenBrainz 两线程（在途 HTTP 最多等既有超时）。AI 请求阻止退出后新请求并拒绝退出后的响应；在途 spawn_blocking 不能强制取消。player-controller 仍依退出标志停止，未 join；完整 Windows 退出时序仍须隔离 exe 验证。
- 凭证仍明文保存在 desktop-v1.json，尚无系统凭据库；LogBuffer 可选文件出口保留原诊断路径，当前 Controller::new 的 Player::new(None) 未启用该出口。Console 与主要业务错误边界脱敏，不等同静态加密或磁盘日志全路径脱敏。
