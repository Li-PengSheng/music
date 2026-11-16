# 当前实现架构

## 核验基线

- 核验 Commit：HEAD `7a768e37ffd40c84deaed6a1d6a3f73dd81d1831`，分支 `main`；开工时工作区干净，本次改动未提交。
- 工作区指纹：`fa8806f1694d6756aa432e1054e22e9eedbb67506f69b05ab5b7bd9571f83cac`，验收前后相同；命令为 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/current-state.md,docs/architecture.md`。
- 工作区内容：ContextMenu 已用 Bits UI DropdownMenu 替换手写键盘循环、坐标算法与全局监听；Modal 以 Bits UI Dialog 管理焦点和 Escape，保留原生 dialog 顶层及最小遮罩命中适配，Dialog 业务接口保持原样。tooltip 使用 Floating UI 定位；Slider 保留浏览器原生 range，没有自建拖动或键盘算法。旧菜单坐标工具与源码函数提取器已删除，覆盖移至真实浏览器测试。
- CSS 责任：已完成本次指定的 Sidebar、TopBar、PlayerBar、Queue、NowPlaying、SettingsView、TrackDetails 七个区块搬迁。`src/app.css` 本次实际从 3147 行降到 1926 行（任务描述的旧基线为 3125 行）；共享 tokens、基础样式和跨组件响应式/减少动画规则继续留在全局。迁入组件的规则使用 `:global` 保持选择器权重与跨子组件语义，七个原始区块与迁入规则的选择器及声明逐块一致。
- 自动测试：`pnpm format`、`pnpm format:check`、`pnpm check`、`pnpm test`、`pwsh -File scripts/check.ps1` 退出码均为 0。前端单测 71/71（旧菜单导航/坐标及 Modal 键盘单测迁至浏览器）；Rust fmt、clippy 均通过，Rust 单元测试 78 通过、1 ignored，音频/歌词集成测试 3/6 通过；设置 `MUSIC_MINE_NULL_OUTPUT=1`，不证明真实设备出声。既有 Cargo PDB 文件名冲突警告仍在。CSS 搬迁后 Svelte CSS 检查会对原有 `-webkit-line-clamp` 提示缺少标准 `line-clamp`（1 warning）；为保持原视觉声明，未添加新属性。CSS 源码测试扫描全部 CSS 与 Svelte style 块，主题调色板单独验证。
- 浏览器验证：`pnpm test:ui` 16/16 通过，退出码 0；原有快照全部零像素差异，`tests/ui/snapshots/` 无改动。新增 4 项原语浏览器用例覆盖菜单循环/搜索/IME/焦点、点/矩形锚点与视口碰撞、动作失败、Modal 初始选区/焦点约束/拖出/忙碌保护、原生 range 键盘与 tooltip。覆盖深浅主题、1280×800、960×700、900×640。harness 不证明 Rust 播放。
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
| `src/app.css` | 1926 行：基础/共享样式、响应式与减少动画规则；主题变量在 `src/lib/styles/tokens.css`。七个组件独有区块已移入对应 Svelte 的 style |
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
