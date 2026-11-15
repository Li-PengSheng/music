# 当前实现架构

代码地图核对到 `a4451df`。这次补上封面模块，没有逐文件重审其余条目。`src/app.css` 的行数仍是 `da5fe14` 的记录。

## 数据流

```text
src/App.svelte、src/lib/views/、src/lib/layout/
        │
        ▼
src/lib/stores/*.svelte.ts
        │
        ▼
src/lib/services/music.ts          invoke 与窗口主题 API
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
        ├── platform/desktop.rs             窗口、主题、托盘、desktop-v1.json
        └── infrastructure/logging.rs
```

改调用链时从实际函数追，不要假设每一条 UI 操作都经过同一个包装。

## 代码地图

| 位置 | 当前角色 |
|---|---|
| `src/App.svelte` | 壳、连接、约 200ms 的 `setTimeout` 轮询、快捷键、队列开关 |
| `src/lib/layout/` | `Sidebar`、`TopBar`、`PlayerBar` |
| `src/lib/views/` | `HomeView`、`SongsView`、`GridView`、`SettingsView` |
| `src/lib/components/` | 手写 `Modal`、`Dialog`、`ContextMenu`、`Slider`、`TrackList`、`EmptyState` |
| `src/lib/components/Select.svelte` | 手写选择器 |
| `src/lib/layout/Queue.svelte`、`src/lib/views/NowPlaying.svelte`、`src/lib/components/QueueCover.svelte` | 队列抽屉、正在播放和队列封面。宽屏队列覆盖页面，不把正在播放挤窄 |
| `src/lib/components/Artwork.svelte`、`src/lib/components/Cover.svelte` | 正在播放和列表里的封面 |
| `src/lib/utils/cover-loader.ts`、`row-visibility.ts` | 封面并发、取消和可见行 |
| `src/lib/stores/player.svelte.ts` | `accept()` 丢掉更旧的 `revision`；`control()` 串行播放命令 |
| `src/lib/services/music.ts` | 全部前端 IPC 类型和调用 |
| `src/lib/utils/queue.ts` | `retainQueue`：曲目未变时保留原数组 |
| `src/app.css` | 全局样式。`da5fe14` 时 3072 行。主题变量在 `src/lib/styles/tokens.css` |
| `src-tauri/capabilities/default.json` | 只有 `core:default` 和 `core:window:allow-set-theme` |
| `tests/harness/` | 浏览器内存曲库。它设置 `window.isTauri`，并让 `bridge_probe` 返回 `backend: 'rust'` |
| `tests/ui/` | Playwright。视口 1280×800，`deviceScaleFactor: 1` |
| `tests/perf/queue-profile.mjs` | 诊断脚本，不是常规测试命令 |

普通 `pnpm dev` 页面里 `isTauri()` 为假，连接文案是「浏览器预览」。harness 里的在线指示是夹具，不是 Rust 引擎。

## 关键约定

- `PlayerSnapshot` 在 `music.ts`：`revision`、`track_id`、`current_track`、`playback_state`、`position_ms`、`duration_ms`、`volume`、`playback_mode`、`lyrics_offset_ms`、`output_device_id`、`error`、`queue`。改字段时对照 Rust 的 serde 定义。
- 进度显示快照里的 `position_ms`。拖动进度条可以有松开前的本地草稿。
- 数据库用 `CREATE TABLE IF NOT EXISTS` 和 `ensure_column`。没有独立迁移框架。
- 启用的解码 feature：`mp3`、`flac`、`wav`、`pcm`。fixture 在 `src-tauri/tests/fixtures/`：`tone.mp3`、`tone.flac`、`tone.wav`。
- 窗口默认 1280×850，最小 900×640。

## 改动分界

- UI：主要在 `src/`。不改 Rust 和数据库。
- 契约：`music.ts` 与 Rust command 一起改，并更新 harness。
- 音频：单独安排 fixture 和 Windows 出声验收。
- 数据：只用临时数据库。
- 桌面：托盘、标题栏、DPI、窗口行为用构建出的 exe。

模块细节见 `docs/rust-guidelines.md`、`docs/ipc-contract.md`、`docs/frontend-guidelines.md`。
