# 当前实现架构

## 核验基线

- 核验 Commit：HEAD `ed191d48c7af4c9bf66c4f8accee308897860af6`。一阶段目录分层已在该提交。二阶段改动仍在工作区，尚未提交。
- 工作区指纹：`c6c401ff55e8cfe6656d000ce6f96cf7ce0291c972b4158320524aa8a01c8b0c`。命令是 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/current-state.md,docs/architecture.md`。取指纹前已删除根目录 `repomix-output.xml`，并把 `docs/STAGE2-PLAN.md` 移到 `docs/history/STAGE2-PLAN.md`。这是写入本页与 `docs/current-state.md` 本次基线之前的工作区。脚本的 Exclude 只跳过未跟踪路径；这两份文档当时已是受跟踪修改，它们的 diff 仍计入该指纹。写入后的指纹不同。
- 工作区内容：一阶段已提交的 Rust 与前端目录分层。二阶段每个 `Catalog` 实例只 `init` 一次，热路径走 `open_light`；`catalog/mod.rs` 的模块级 `dead_code` 豁免已去掉；`Action::Open` 复用 `prepare` 之后那一次 `synchronize` 的状态，`Start` 仍是两次 `execute`；`audio/player.rs` 的互斥锁毒化改为 `into_inner`；前端 17 个白名单文件已用 Prettier 展开。施工图在 `docs/history/STAGE2-PLAN.md`。
- 自动测试：`pnpm test` 75/75；`pnpm check` 0 error、0 warning；`cargo fmt --all -- --check` PASS；`cargo clippy --all-targets -- -D warnings` PASS；`cargo test` 77 通过、1 ignored（`cover_corpus`），`tests/audio.rs` 3 通过，`tests/lyrics.rs` 6 通过。`pwsh -File scripts/check.ps1` 退出码 0。`pnpm build` 通过。Rust 测试设置 `MUSIC_MINE_NULL_OUTPUT=1`，不证明真实设备出声。Cargo 的 bin/lib PDB 文件名冲突警告仍在，未导致失败。
- 浏览器验证：`pnpm test:ui` 8/8 通过，未更新 `tests/ui/snapshots/`。这是 harness，不是 Rust 播放。
- Windows Tauri 实机：NOT RUN。

本次核验覆盖目录分层之后的工作区检查，不代表逐行重审所有实现。`src/app.css` 的行数仍是 `da5fe14` 的历史记录。

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
