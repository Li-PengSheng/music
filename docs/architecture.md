# 当前实现架构

## 核验基线

- 核验 Commit：HEAD `b048cecb669042c6b8e9e95b31d7ead444573071`，分支 `main`；开工时工作区干净，本次改动未提交。
- 工作区指纹：`9fa7f88abf51132580fd9ad42e797af3d50714d751ee249c642bbc1a4f88bc38`，验收前后相同；命令为 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/current-state.md,docs/architecture.md`。
- 工作区内容：TrackList（包括 container 规则）、HomeView、EmptyState 的 CSS 已移入对应组件 `<style>` 的 `:global` 块，选择器与声明逐块对比保持一致；共享 `.eq` 动画原样留在全局。NowPlaying 的两行截断规则补充 `line-clamp: 2`，消除兼容性警告。
- CSS 责任：`src/app.css` 从 1926 行降至 1455 行，P2 全局 CSS 责任已完成本轮十个指定组件范围；基础/共享样式、`.eq`、跨组件响应式和减少动画仍在全局。
- 工具与音量：旧 `menu-position.ts`、`source-ast.mjs` 已不存在且无引用；`modal.ts` 仍被 Modal 和真实遮罩单测引用，保留原生 dialog 遮罩命中、拖出及嵌套浮层保护。`amplitude_gain` 与 `fill_output` 已按三次方曲线映射：20% → 0.008，50% → 0.125，100% → 1.0；本次不改 Rust、IPC 或会话数据。
- 自动测试：`pnpm format`、`pnpm format:check`、`pnpm check`、`pnpm test`、`pwsh -File scripts/check.ps1` 退出码均为 0；Svelte 检查 0 error / 0 warning，前端 71/71，Rust fmt / clippy 通过，Rust 单元测试 78 通过、1 ignored，音频/歌词集成 3/6 通过。`amplitude_gain_maps_perceived_volume_and_clamps_bounds` 与 `volume_scales_the_actual_callback_samples` 均在完整 cargo test 中通过。测试设置 `MUSIC_MINE_NULL_OUTPUT=1`，不证明真实设备出声；既有 Cargo PDB 文件名冲突警告仍在。CSS 源码检查已扫描所有 CSS 与 Svelte style 块，无需改变断言范围。
- 浏览器验证：`pnpm test:ui` 16/16 通过，退出码 0；既有快照全部零像素差异，`tests/ui/snapshots/` 无改动。覆盖深浅主题、首页、歌曲/虚拟列表与 sticky 表头、队列、弹窗、菜单、Select、减少动画及 1280×800 / 960×700 / 900×640 视口。harness 不证明 Rust 播放。
- Windows Tauri 实机：NOT RUN。

本次核验覆盖交互原语与指定组件 CSS 搬迁后的工作区检查，不代表逐行重审所有实现。

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
| `src/lib/components/` | Bits UI Select / ContextMenu / Modal 适配层，Dialog 业务包装，原生 range Slider，以及 TrackList / EmptyState |
| `src/lib/components/Select.svelte` | Bits UI Select，保留项目 Props 和 tokens |
| `src/lib/layout/Queue.svelte`、`src/lib/views/NowPlaying.svelte`、`src/lib/components/QueueCover.svelte` | 队列抽屉、正在播放和队列封面。宽屏队列覆盖页面，不把正在播放挤窄 |
| `src/lib/components/Artwork.svelte`、`src/lib/components/Cover.svelte` | 正在播放和列表里的封面 |
| `src/lib/utils/cover-loader.ts`、`row-visibility.ts` | 封面并发、取消和可见行 |
| `src/lib/stores/player.svelte.ts` | `accept()` 丢掉更旧的 `revision`；`control()` 串行播放命令 |
| `src/lib/services/music.ts` | 全部前端 IPC 类型和调用 |
| `src/lib/utils/queue.ts` | `retainQueue`：曲目未变时保留原数组 |
| `src/app.css` | 1455 行：基础/共享样式、响应式与减少动画规则；主题变量在 `src/lib/styles/tokens.css`。十个指定组件独有区块已移入对应 Svelte 的 style |
| `src-tauri/capabilities/default.json` | 只有 `core:default` 和 `core:window:allow-set-theme` |
| `tests/harness/` | 浏览器内存曲库。它设置 `window.isTauri`，并让 `bridge_probe` 返回 `backend: 'rust'` |
| `tests/ui/` | Playwright。视口 1280×800，`deviceScaleFactor: 1` |
| `tests/perf/queue-profile.mjs` | 诊断脚本，不是常规测试命令 |

普通 `pnpm dev` 页面里 `isTauri()` 为假，连接文案是「浏览器预览」。harness 里的在线指示是夹具，不是 Rust 引擎。

## 关键约定

- `PlayerSnapshot` 在 `music.ts`：`revision`、`track_id`、`current_track`、`playback_state`、`position_ms`、`duration_ms`、`volume`、`playback_mode`、`lyrics_offset_ms`、`output_device_id`、`error`、`queue`。改字段时对照 Rust 的 serde 定义。
- 进度显示快照里的 `position_ms`。拖动进度条可以有松开前的本地草稿。
- 数据库用 `CREATE TABLE IF NOT EXISTS` 和 `ensure_column`。没有独立迁移框架。每个 `Catalog` 用实例级 `Mutex<bool>` 只迁移一次；`db::open` 仍执行 `init`，热路径随后走 `open_light`。`import_copy` 对副本仍直接调用 `db::open`。
- `Action::Open` 把 `prepare` 后的 `synchronize` 状态交给同一次 `execute` 的尾部。`Play` 仍单独做一次屏障。成功的 `Start` 仍是两次 `execute` 和两次 `persist` 调用。
- `audio/player.rs` 里音频路径上的 `Mutex` 毒化用 `into_inner` 取出，避免二次 panic。这不证明锁内数据完整。
- 启用的解码 feature：`mp3`、`flac`、`wav`、`pcm`。fixture 在 `src-tauri/tests/fixtures/`：`tone.mp3`、`tone.flac`、`tone.wav`。
- 窗口默认 1280×850，最小 900×640。

## 改动分界

- UI：主要在 `src/`。不改 Rust 和数据库。
- 契约：`music.ts` 与 Rust command 一起改，并更新 harness。
- 音频：单独安排 fixture 和 Windows 出声验收。
- 数据：只用临时数据库。
- 桌面：托盘、标题栏、DPI、窗口行为用构建出的 exe。

模块细节见 `docs/rust-guidelines.md`、`docs/ipc-contract.md`、`docs/frontend-guidelines.md`。
