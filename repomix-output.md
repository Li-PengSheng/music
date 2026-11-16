This file is a merged representation of the entire codebase, combined into a single document by Repomix.

# File Summary

## Purpose
This file contains a packed representation of the entire repository's contents.
It is designed to be easily consumable by AI systems for analysis, code review,
or other automated processes.

## File Format
The content is organized as follows:
1. This summary section
2. Repository information
3. Directory structure
4. Repository files (if enabled)
5. Multiple file entries, each consisting of:
  a. A header with the file path (## File: path/to/file)
  b. The full contents of the file in a code block

## Usage Guidelines
- This file should be treated as read-only. Any changes should be made to the
  original repository files, not this packed version.
- When processing this file, use the file path to distinguish
  between different files in the repository.
- Be aware that this file may contain sensitive information. Handle it with
  the same level of security as you would the original repository.

## Notes
- Some files may have been excluded based on .gitignore rules and Repomix's configuration
- Binary files are not included in this packed representation. Please refer to the Repository Structure section for a complete list of file paths, including binary files
- Files matching patterns in .gitignore are excluded
- Files matching default ignore patterns are excluded
- Files are sorted by Git change count (files with more changes are at the bottom)

# Directory Structure
````
.github/
  workflows/
    check.yml
docs/
  tasks/
    TEMPLATE.md
  architecture.md
  current-state.md
  decisions.md
  frontend-guidelines.md
  ipc-contract.md
  isolation.md
  RFC-v2.0.md
  rust-guidelines.md
  testing.md
portable/
  README.txt
public/
  theme-init.js
scripts/
  build-isolated.ps1
  build-portable.ps1
  check.ps1
  isolation-cli.ps1
  isolation-lib.ps1
  worktree-fingerprint.ps1
src/
  lib/
    components/
      Artwork.svelte
      ContextMenu.svelte
      Cover.svelte
      Dialog.svelte
      EmptyState.svelte
      IconButton.svelte
      Modal.svelte
      QueueCover.svelte
      Select.svelte
      Slider.svelte
      tooltip.ts
      TrackDetails.svelte
      TrackList.svelte
    layout/
      PlayerBar.svelte
      Queue.svelte
      Sidebar.svelte
      TopBar.svelte
    services/
      music.ts
    stores/
      library.svelte.ts
      nav.svelte.ts
      player.svelte.ts
      preferences.ts
      ui.svelte.ts
    styles/
      tokens.css
    utils/
      collections.ts
      color.ts
      cover-diagnostics.ts
      cover-loader.ts
      format.ts
      history.ts
      layers.ts
      lyrics-view.ts
      modal.ts
      native-menu.ts
      playlist-name.ts
      queue.ts
      row-visibility.ts
      shortcuts.ts
      theme.ts
      virtual-range.ts
      volume.ts
    views/
      GridView.svelte
      HomeView.svelte
      NowPlaying.svelte
      SettingsView.svelte
      SongsView.svelte
    actions.ts
  app.css
  App.svelte
  main.ts
src-tauri/
  capabilities/
    default.json
  src/
    artwork/
      covers.rs
      mod.rs
    audio/
      mod.rs
      player.rs
    catalog/
      catalog_tests.rs
      collections.rs
      db.rs
      library.rs
      metadata.rs
      mod.rs
    commands/
      catalog.rs
      mod.rs
    infrastructure/
      logging.rs
      mod.rs
    platform/
      desktop.rs
      media.rs
      mod.rs
    playback/
      controller.rs
      lyrics.rs
      mod.rs
    test_support/
      cover_corpus.rs
      mod.rs
    lib.rs
    main.rs
  tests/
    audio.rs
    lyrics.rs
  build.rs
  Cargo.toml
  tauri.conf.json
  tauri.portable.conf.json
tests/
  harness/
    index.html
    mock-ipc.js
    primitives.html
    primitives.ts
    PrimitivesFixture.svelte
    select.html
    select.ts
    SelectFixture.svelte
  helpers/
    source-styles.mjs
  perf/
    cdp-eval.mjs
    compare-recover.mjs
    queue-cover-profile.mjs
    queue-profile.mjs
    run-restart-recover.ps1
    scan-sampler.mjs
    stage-cover-run.ps1
    transient-cover.mjs
  ui/
    primitives.spec.ts
    reduce-motion.spec.ts
    select.spec.ts
    shell.spec.ts
  ipc-surface.test.mjs
  isolation-guard.test.mjs
  lyrics-view.test.mjs
  player-stability.test.mjs
  reduce-motion.test.mjs
  shortcuts.test.mjs
  ui-refinements.test.mjs
  v09-utils.test.mjs
  v091-collections.test.mjs
  v091-layers.test.mjs
  v091-theme.test.mjs
  v092-queue-motion.test.mjs
  v092-visual.test.mjs
  v092-volume.test.mjs
  v0953-cover-loader.test.mjs
  v0955-cover-generation.test.mjs
  worktree-fingerprint.test.mjs
.gitattributes
.gitignore
.node-version
.prettierignore
.prettierrc
.repomixignore
AGENTS.md
CLAUDE.md
index.html
npm.cmd
package.json
playwright.config.ts
pnpm-workspace.yaml
README.md
svelte.config.js
tsconfig.json
vite.config.ts
````

# Files

## File: .github/workflows/check.yml
````yaml
name: check

on:
  push:
  pull_request:

permissions:
  contents: read

jobs:
  check:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
        with:
          version: 12.21.0
      - uses: actions/setup-node@v4
        with:
          node-version: 24.14.0
          cache: pnpm
      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: 1.99.0
          components: rustfmt, clippy
      - run: pnpm install --frozen-lockfile
      - run: pwsh -File scripts/check.ps1
````

## File: docs/tasks/TEMPLATE.md
````markdown
# 任务：<标题>

## 基线

- 分支与 HEAD：
- 未提交改动：
- 工作区指纹：验收前运行 `powershell -File scripts/worktree-fingerprint.ps1`。没有保存过的历史任务写「未记录，无法重建」，不要补算。

## 状态

- 状态：进行中 / 受阻 / 待审阅 / 完成
- 依赖与前置任务：
- 下一步：
- 提交：待提交 / <提交哈希>
- 遗留事项的任务 ID：

## 范围

- 要改：
- 不要改：
- 是否涉及音频、数据库、IPC 或用户数据目录：

## 验收

- [ ] 自动测试：PASS / FAIL / NOT RUN
- [ ] 浏览器验证：PASS / FAIL / NOT RUN
- [ ] Windows Tauri 实机：PASS / FAIL / NOT RUN
- [ ] 未打开用户正在使用的曲库和设置

实机必须使用构建出的 exe。会写数据时按 `docs/isolation.md`。不要修改 `src-tauri/tauri.conf.json`。

## 来源

新任务填写。算法见 `docs/testing.md`。

- 任务 ID：
- 时间：
- HEAD：
- 工作区指纹：
- 命令、工具版本、构建配置：
- 产物哈希：
- 三层结果和未验证项：

## 结束报告

- 改动文件：
- 三条验收结果：
- 未验证项：
- Git：只有当前任务明确要求时才提交
- 完成后把本记录移到 `docs/tasks/done/`。进行中的任务不要和历史记录放在一起
````

## File: docs/architecture.md
````markdown
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
````

## File: docs/current-state.md
````markdown
# 当前事实与验收记录

进度只看本页。四个入口：

- [当前任务](#当前任务)
- [已知缺陷](#已知缺陷)
- [验收缺口](#验收缺口)
- [发布决定](#发布决定)

## 核验基线

- 核验 Commit：HEAD `b048cecb669042c6b8e9e95b31d7ead444573071`，分支 `main`；开工时工作区干净，本次改动未提交。
- 工作区指纹：`9fa7f88abf51132580fd9ad42e797af3d50714d751ee249c642bbc1a4f88bc38`，验收前后相同；命令为 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/current-state.md,docs/architecture.md`。
- 工作区内容：TrackList（包括 container 规则）、HomeView、EmptyState 的 CSS 已移入对应组件 `<style>` 的 `:global` 块，选择器与声明逐块对比保持一致；共享 `.eq` 动画原样留在全局。NowPlaying 的两行截断规则补充 `line-clamp: 2`，消除兼容性警告。
- CSS 责任：`src/app.css` 从 1926 行降至 1455 行，P2 全局 CSS 责任已完成本轮十个指定组件范围；基础/共享样式、`.eq`、跨组件响应式和减少动画仍在全局。
- 工具与音量：旧 `menu-position.ts`、`source-ast.mjs` 已不存在且无引用；`modal.ts` 仍被 Modal 和真实遮罩单测引用，保留原生 dialog 遮罩命中、拖出及嵌套浮层保护。`amplitude_gain` 与 `fill_output` 已按三次方曲线映射：20% → 0.008，50% → 0.125，100% → 1.0；本次不改 Rust、IPC 或会话数据。
- 自动测试：`pnpm format`、`pnpm format:check`、`pnpm check`、`pnpm test`、`pwsh -File scripts/check.ps1` 退出码均为 0；Svelte 检查 0 error / 0 warning，前端 71/71，Rust fmt / clippy 通过，Rust 单元测试 78 通过、1 ignored，音频/歌词集成 3/6 通过。`amplitude_gain_maps_perceived_volume_and_clamps_bounds` 与 `volume_scales_the_actual_callback_samples` 均在完整 cargo test 中通过。测试设置 `MUSIC_MINE_NULL_OUTPUT=1`，不证明真实设备出声；既有 Cargo PDB 文件名冲突警告仍在。CSS 源码检查已扫描所有 CSS 与 Svelte style 块，无需改变断言范围。
- 浏览器验证：`pnpm test:ui` 16/16 通过，退出码 0；既有快照全部零像素差异，`tests/ui/snapshots/` 无改动。覆盖深浅主题、首页、歌曲/虚拟列表与 sticky 表头、队列、弹窗、菜单、Select、减少动画及 1280×800 / 960×700 / 900×640 视口。harness 不证明 Rust 播放。
- Windows Tauri 实机：NOT RUN。

历史任务 09.5.3、09.5.5 的指纹仍为“未记录，无法重建”；本次结果不回填历史验收。

## 当前任务

没有进行中的任务。新任务从 `docs/tasks/TEMPLATE.md` 写起，完成后把记录放到 `docs/tasks/done/`。

封面加载单元已经提交：

- 状态：完成
- 提交：`4352fc6`
- 记录：[09.5.5](tasks/done/TASK-09.5.5-RC-FREEZE.md)，依赖 [09.5.3](tasks/done/TASK-09.5.3-QUEUE-COVER.md)
- 下一步：没有指定的下一任务
- 遗留事项的任务 ID：库外打开、WAV 经界面打开、托盘三态还没有单独的任务 ID

## 已知缺陷

这些是任务记录里已经写明的行为，不是 NOT RUN。

- 会话中替换文件里的封面、又没有重新扫描时，页面会继续显示旧图，直到重新扫描或重启。Rust 按 mtime 和长度失效。见 09.5.3。
- 超过 512 KB 的 WebP/GIF，以及 BMP、TIFF、AVIF、HEIC，仍显示为没有封面。见 09.5.3。

## 验收缺口

NOT RUN 放在这里。它不是已知缺陷，也不表示可以放行。

- 库外 `player_open_file`、WAV 经界面打开、托盘三态：09.5.5 写明未做。保持 NOT RUN。
- 当前工作区的 Rust 自动检查和浏览器 harness 已记在核验基线。真实音频、扫描、DPI、托盘和系统媒体键的实机仍是 NOT RUN。历史无指纹记录不作为当前验收。
- 浏览器 harness 不是 Rust 播放。

## 发布决定

这是 2026-11-15 这次文档整理做出的决定，不是 09.5.3 或 09.5.5 当时已经写完的验收结论。

库外打开、WAV 经界面打开、托盘三态这三项 NOT RUN，不阻塞 `4352fc6` 里的封面加载。它们仍阻塞把当前程序称为完整发布。

## 源码事实

| 状态 | 事实 |
|---|---|
| 源码可见 | Svelte 5、TypeScript、Vite、Tailwind v4、Tauri v2、Rust |
| 源码可见 | 产品版本 `0.1.0`。`docs/tasks/done/` 里的 V0.9.x 是任务编号，不是安装包版本 |
| 源码可见 | IPC 在 `src/lib/services/music.ts`。播放快照有 `revision` |
| 源码可见 | `src/app.css` 1455 行。Select / ContextMenu / Modal 使用 Bits UI，Slider 使用原生 range |
| 源码可见 | `package.json` 有 `@playwright/test` 和 `bits-ui`。没有 shadcn-svelte、Vitest、WebdriverIO、tauri-specta、TanStack Virtual |
| 源码可见 | Symphonia feature 为 mp3、flac、wav、pcm |
| 源码可见 | 数据库演进是 `db.rs` 的建表和 `ensure_column` |
| 源码可见 | 队列用 `retainQueue`。宽屏队列覆盖页面 |

`src/app.css` 的行数已在本次组件 CSS 搬迁后重数。

## 已记录的验证

这些是 `da5fe14` 提交前的结果。指纹：未记录，无法重建。不能用来勾掉 `decbefb` 及更晚提交。

| 层次 | 结果 |
|---|---|
| 自动测试 | `pnpm test`：50 通过，0 失败。`pnpm check`：0 error，0 warning |
| 浏览器验证 | `pnpm test:ui`：7 通过。覆盖侧栏页面、播放栏、队列、弹窗、主题和 960×700 窄窗。截图与 `tests/ui/snapshots/` 一致 |
| Windows Tauri 实机 | 部分通过。临时 `appDirectoriesOverride` 指向空目录后构建 debug exe。空曲库中确认：连接指示在线、主题切换、新建歌单取消与创建、播放栏显示「未在播放」、队列可 Escape 关闭、980×720 时音量百分比隐藏并在 1280×850 恢复。随后覆盖项已删除并重新构建 |
| 未跑 | `cargo fmt`、`cargo clippy`、`cargo test`。实机没有播放真实音频，没有扫描文件夹，没有测 DPI、托盘或系统媒体键 |

浏览器 harness 使用内存夹具。实机用的是隔离空库。两者都没有打开用户正在使用的 `library-tauri-v1.sqlite` 做实验。曾有一次未隔离启动把桌面主题偏好写成深色，随后已按原文件恢复。以后的实机检查按 `docs/isolation.md`。

上面的实机做法是当时的记录：改主配置再删掉重编。这个做法已经停止，见 ADR-008。

## 文档与代码不一致

- `docs/RFC-v2.0.md` 写自动化测试是 cargo test + Vitest。当前命令是 `pnpm test` 和 `pnpm test:ui`。RFC 仍是产品目标，测试一行过时。
- RFC 写前端用单调时钟对播放进度做平滑插值。当前界面只显示快照里的 `position_ms`，拖动时另有松开前的草稿。约定禁止用定时器推进进度。
- `docs/tasks/done/TASK-V0.9.2-VISUAL-POLISH.md` 写宽屏正在播放会为队列让出宽度。当前 CSS 是队列覆盖，`min-width: 1200px` 时非沉浸页隐藏遮罩。
- `docs/history/` 和 `docs/tasks/done/` 是历史交接，可能落后于后续提交。其中没有进行中的任务。
- 规范包 `music-mine-ai-guidelines-v3-project-wide/` 基于 `2d8d8d2`，其中「测试仍因格式化失败」「尚未安装 Playwright」「README 仍是 Task 02」已过时。以本仓库文档为准。该目录未纳入版本管理。

## 后续优先级

| 优先级 | 事项 | 完成标志 |
|---|---|---|
| P1 | 实机播放与扫描 | 隔离目录里播放一条 fixture 或扫描一个临时文件夹，并记录 IPC 结果 |
| P1 | 实机窗口外观 | DPI 125% 或 150%，以及托盘行为有记录 |
| P2 | 全局 CSS 责任 | 本轮十个指定组件区块已搬迁，规则逐块对比一致；共享样式仍由全局管理 |
| P2 | Select 组件库试点 | Select、ContextMenu 和 Modal 已接入 `bits-ui 2.21.0`；本次范围已完成，Windows 隔离窗口验收仍为 NOT RUN |

P2 全局 CSS 责任已完成本轮十组件范围；实机相关缺口仍未完成。自动检查结果在核验基线。
````

## File: docs/decisions.md
````markdown
# 技术决策

只记录会影响后续多个任务的选择。

## ADR-001 — Rust + Tauri v2 + Svelte 5

- 状态：已采用。代码可见。
- 决定：Rust 负责音频和曲库，Svelte 5 负责界面，Tauri 负责窗口和 IPC。
- 后果：接受 WebView2 的成本。播放事实仍在 Rust。

## ADR-002 — 先修测试，再谈组件迁移

- 状态：测试部分已做；Select 的 Bits UI 试点已实施，Windows 隔离实机验收待完成。
- 决定：先用行为测试和截图基线稳住界面，再单独考虑通用组件。
- 证据：`5a7c9b3` 替换了格式化即失败的字符串断言。`9f141df` 加入 Playwright 和基准截图。

## ADR-003 — 用行为而不是源码拼写做测试

- 状态：已采用。
- 决定：纯函数做单测，交互做浏览器操作，Windows 专有能力用构建出的 exe。
- 后果：仍有少量检查会读取源码。不再新增只匹配空格或类名的断言。

## ADR-004 — 不自动提交

- 状态：已采用。替代旧规则「after any modification, finish git commit」。
- 决定：任务结束时报告 diff 和三种验收结果。只有当前任务明确要求时才 `git commit` 或 `git push`。
- 原因：失败和误改会被自动提交带进历史。用户数据隔离也需要先看 diff。

## ADR-005 — 通用组件后续用 shadcn-svelte 与 Bits UI

- 状态：已安装 `bits-ui 2.21.0`（Svelte 5），`Select.svelte` 试点已迁移；shadcn-svelte 未安装。
- 决定：将来的通用交互优先 shadcn-svelte；焦点、键盘和浮层原语用 Bits UI。外观仍用本项目 tokens。播放器业务组件保持自定义。
- 约束：没有单独的迁移任务时，继续用手写组件。初始化命令不得覆盖 `src/app.css`。
- 试点进展：Select 已迁移；本次明确授权继续迁移 ContextMenu 和 Modal。它们使用 Bits UI，Modal 保留原生 dialog 顶层以保持外观；Slider 采用浏览器原生 range，tooltip 复用 Floating UI。Windows 隔离实机验收仍为 NOT RUN，不能用浏览器结果替代。
- 试点实现：使用 Bits UI Select 的 Root、Trigger、Portal、Content、Item，保留原 Props、样式类、选项按钮与 tokens。定位和键盘导航由库处理；触发器采用库的 `combobox` / `aria-activedescendant` 语义，测试按真实 DOM 行为验证。仅保留 IME 保护及原生 dialog / 外部菜单的兼容接线；不运行样式初始化 CLI。

## ADR-006 — 一份根约定，按模块展开

- 状态：已采用。文件在本仓库，尚未推送。
- 决定：根 `AGENTS.md` 放安全边界和完成标准。Rust、IPC、前端、测试分文件。`CLAUDE.md` 只引用 `AGENTS.md`。

## ADR-007 — 保留现有 IPC 错误类型

- 状态：已记录，代码未改。
- 当前事实：命令返回 `Result<T, String>`，TypeScript 类型手写在 `music.ts`。
- 决定：新的复杂接口可以另用结构化错误。现有命令不批量改签名。

## ADR-008 — Windows 数据目录隔离

- 状态：已采用。2026-11-15 修订。替代「临时写入 `tauri.conf.json` 的 `appDirectoriesOverride`，验收后删除并重新构建」。
- 事实：Tauri `app_data_dir()` 使用系统已知文件夹。进程 `APPDATA` 不会重定向它。`src-tauri/target` 和 `dist-portable` 里都可能有普通构建或旧产物。
- 决定：不修改 `tauri.conf.json`。隔离操作只维护 `docs/isolation.md`。验收脚本只启动带有 exe 哈希来源记录的构建。绝对路径覆盖和便携相对路径分开核对。
- 原因：多份说明不一致时，实机可能打开正在使用的曲库。exe 的位置和二进制里的路径字符串都不能证明这次配置已经生效。

## 新决策模板

```markdown
## ADR-NNN — 标题
- 状态：提议 / 已采用 / 已弃用
- 决定：
- 替代：
- 后果：
- 验证：
```
````

## File: docs/frontend-guidelines.md
````markdown
# 前端开发约定

现代、稳定的 Windows 本地播放器。歌词可读、列表可操作、播放不中断，优先于视觉噱头。

## Svelte 5

- 新组件用 Runes：`$state`、`$derived`、`$props`。`$effect` 只用于要清理的副作用。
- 共享状态放在 `src/lib/stores/*.svelte.ts`。播放状态来自 Rust 快照。
- 页面负责组合。IPC 留在 `src/lib/services/music.ts`。
- 定时器、观察器和订阅在组件销毁时清理。
- 队列轮询用 `retainQueue` 保留未变化的数组。不要为了少渲染再引入一套互相冲突的缓存。

## 布局与样式

- 壳层是侧栏、主区域、底部播放栏。每个纵向滚动区域只有一个滚动所有者。
- 队列是覆盖抽屉。关闭时不能被 Tab 聚焦。宽屏不为队列压缩正在播放页。
- 颜色、圆角、层级和公共尺寸用 `src/lib/styles/tokens.css`。
- 改 `src/app.css` 的公共选择器前先搜索使用处。组件独有样式放进该组件的 `<style>`。
- 不用 `!important` 或很大的 `z-index` 掩盖 Grid、Flex 或 overflow 问题。减少动效里已有的 `!important` 保持原样，除非正在修那一条。
- 不按行数拆 `app.css`。碰到哪一块，只在验证过的范围内改。
- 长中文、英文和数字混排、超长路径不能把操作挤出窗口。唯一关键信息不要只靠截断。
- 深色、浅色、跟随系统和减少动画都是主路径。

## 现有组件

`Select.svelte`、`ContextMenu.svelte`、`Modal.svelte` 已使用 Bits UI；Dialog 保留业务表单包装，Slider 保留原生 range。tooltip 通过 Floating UI 定位，保持 action 接口。播放栏、队列、歌词、专辑卡片和歌曲表保持业务组件。

歌曲表使用 `src/lib/utils/virtual-range.ts`。TanStack Virtual 没有接入。

图标继续用 `lucide-svelte`。

## 组件库选型

`docs/decisions.md` 的 ADR-005 已启动 Select 试点：`bits-ui 2.21.0` 已安装，`Select.svelte` 的 Props 与调用点保持原样，保留 `.select-trigger` / `.select-panel` 和项目 tokens。库负责定位、列表键盘导航与关闭；触发器使用原生 button 和库提供的 `combobox` 角色，以 `aria-activedescendant` 表达高亮选项。shadcn-svelte 未安装。

没有单独写出「迁移 Select」的任务时：

- 继续改现有手写组件。
- 不运行 shadcn-svelte 或其他会重写 `src/app.css` 的初始化命令。
- 不把页面改成组件库的默认外观。

Select 试点只通过包管理器安装，无样式初始化。保留现有 `[data-theme]` 和 tokens；原生 dialog 内的列表 portal 到所属 dialog，Modal 识别库的打开状态，确保 Escape 和遮罩先关闭列表。手写键盘处理的源码抽取测试已迁移到 `tests/ui/select.spec.ts`，覆盖绑定、回调、键盘、IME、主题、定位和弹窗。浏览器和 Windows 隔离窗口都验收之后再考虑下一个组件；本次 Windows Tauri 实机为 `NOT RUN`。

## 改界面时要看的状态

改到哪一块，就检查那一块的正常、空、失败、窄窗口和键盘路径。截图只说明外观。点击、键盘和关闭行为要另外确认。

| 区域 | 改到它时检查 |
|---|---|
| 首页、歌曲、专辑 | 有数据、空数据、窄窗口 |
| 队列 | 打开、关闭、Escape、遮罩、窄窗口 |
| 弹窗、菜单、选择器 | 确认、取消、Escape、焦点回到触发按钮 |
| 正在播放、歌词 | 有歌词、无歌词、无封面 |
| 设置、主题 | 深色、浅色、跟随系统，音量不被主题改掉 |

窗口尺寸至少覆盖最小窗附近（900×640）和默认附近（1280×800 或 1280×850）。125%/150% DPI 属于 Windows 实机项，浏览器 1 倍缩放不能代替。

本轮十个指定组件的独有 CSS 已从 `app.css` 移入各自 `<style>` 的 `:global` 块，先保留等价选择器与权重。不要直接去掉 `:global`，否则会改变子组件样式和级联；共享响应式与减少动画规则仍在全局。
````

## File: docs/ipc-contract.md
````markdown
# IPC 与播放快照

涉及 `src-tauri/src/` 的 command、`src/lib/services/music.ts`、`src/lib/stores/*.svelte.ts` 和调用它们的组件。

## 当前事实

- 前端只在 `music.ts` 里调用 `invoke` 和窗口主题 API。没有 tauri-specta，类型是手写的。
- `PlayerSnapshot` 字段见 `docs/architecture.md`。`accept()` 拒绝更小的 `revision`。`control()` 同时只跑一个播放命令。
- `volume` 是 `0.0..=1.0` 的用户感知音量比例；`PlayerStatus`、`PlayerSnapshot`、前端和 `playback-v1.json` 保留原始比例，音频回调按三次方曲线（`x^3`）映射为 PCM 振幅增益。例如 20% 仍传输并保存 `0.2`，输出增益为 `0.008`；50% 的增益为 `0.125`。未加载会话时的 Rust 初始音量保持 `1.0`。
- `App.svelte` 用 `setTimeout(refresh, 200)` 轮询 `player_get_state`。
- 命令返回 `Result<T, String>`。错误文本由 Rust 产生，界面用现有通知显示。
- `verifyConnection()` 调用 `bridge_probe`，并核对 `nonce` 与 `backend === 'rust'`。
- 普通浏览器里 `isTauri()` 为假，不会显示已连接。`tests/harness/mock-ipc.js` 会设置 `isTauri` 并回答 `backend: 'rust'`，那是测试夹具。

## 修改一个 command 时

1. 核对 Rust 命令名、参数和 serde。注意 `Option`、camelCase 和枚举写法。
2. 不扩大 capability。路径和数值在 Rust 校验。
3. 更新 `music.ts` 的类型和封装，并搜索全部调用点与 `tests/harness/mock-ipc.js`。
4. 成功、失败和取消都要有界面反馈。前端不编造成功快照。
5. 返回的播放快照交给 `accept()`。调用点不直接覆盖 `player.snapshot`。
6. 浏览器 Mock 只证明前端契约。真实 `invoke` 要在构建出的 exe 里跑一次；不能跑就写 `NOT RUN`。
7. 改已有协议时列出全部调用点和兼容方式。

## 进度

Rust 是播放位置的权威。界面显示 `position_ms`。拖动进度条只保留松开前的草稿，结果以命令返回的快照为准。

不要用定时器在两次轮询之间插值进度。不要把约 200ms 的轮询改成事件推送，除非先有测量说明它是瓶颈。

列表、专辑网格和歌曲详情调用 `library_thumb`。队列封面调用 `library_queue_thumb`，只匹配曲库中的完整路径，不按标题查找。Rust 在后台线程把内嵌 JPEG/PNG 限制尺寸后编码成 JPEG，缓存在应用缓存目录的 `covers`；不超过 512 KB 的 WebP/GIF 原样以对应 MIME 的 data URL 返回。`player_artwork` 只读当前播放文件，返回最长边不超过 960 的图片。进度轮询不请求封面。

这三个封面命令的返回值：`null` 表示确定没有可用封面（无图片、格式不支持、超出尺寸限制、图片损坏），会缓存到文件变化为止；错误字符串是不含路径的代码，表示这次拿不到、稍后可能成功，不缓存：`cover-file-missing`、`cover-unreadable`，`library_queue_thumb` 另有 `cover-not-in-library`。前端经 `src/lib/utils/cover-loader.ts` 限并发、按优先级（当前曲目、可见行、预取）排队，可取消，暂时性错误最多重试两次。`library_cover_stats` 返回封面缓存计数和耗时（只有数字），用于诊断。

## 错误协议

现有 `Result<T, String>` 保持不变。新的复杂命令可以在单独任务里使用可序列化的 `code`、`message`、可选 `details`。不在无关的 UI 任务里批量替换错误类型。

日志可以保留调试上下文。界面和记录里不额外散播用户绝对路径、歌词正文或整库内容。

## 命令清单

检查比较四个名字集合：Rust `generate_handler!`、`src/lib/services/music.ts` 的 `invoke`、`tests/harness/mock-ipc.js` 的 handlers、下面两份清单。提取时忽略注释，允许多行、泛型和同一命令多次调用。参数、可空性和返回类型不在这次检查里。

已注册但前端没有封装的命令必须写进例外清单。例外清单必须等于「已注册命令减去前端封装」。命令被删除、后来补了前端封装，或新注册了一个前端未使用的命令时，过时或缺漏的名字都会让 `tests/ipc-surface.test.mjs` 失败。当前例外只有 `library_cover` 和 `library_favorite`。

```ipc-commands
bridge_probe
desktop_preferences
library_audio_info
library_cancel
library_cover
library_cover_stats
library_favorite
library_home
library_import
library_play
library_playlist
library_playlist_edit
library_progress
library_query
library_queue
library_queue_thumb
library_queue_tracks
library_scan
library_summary
library_thumb
library_track
media_status
player_artwork
player_configure
player_get_state
player_lyrics
player_open_file
player_pause
player_play
player_seek
player_set_volume
player_stop
```

```ipc-unused
library_cover
library_favorite
```
````

## File: docs/isolation.md
````markdown
# 数据隔离

这是隔离操作的唯一说明。其他文档只保留禁止项并链接到这里。

Tauri 的 `app_data_dir()` 使用 Windows 已知文件夹。进程环境变量 `APPDATA` 不会换掉它。普通 `pnpm tauri build --debug --no-bundle` 和 `pnpm tauri dev` 使用系统数据目录，会打开正在使用的曲库和设置。

不要修改 `src-tauri/tauri.conf.json` 来写入 `appDirectoriesOverride`。exe 位于 `src-tauri/target` 或 `dist-portable`，以及二进制里出现某段路径，都不能证明这次构建使用了隔离配置。

## 来源记录

构建脚本在成功后把记录写到 `tests/perf/out/isolation-sources/<exe 的 SHA-256>.json`。这个目录已被忽略。记录至少包含：

- exe 的 SHA-256，以及当时位于允许目录中的构建产物路径
- 配置文件路径和该文件的 SHA-256
- 完整构建命令
- `dataDirKind`：`portable-relative` 或 `absolute-override`
- 预期数据目录

允许目录只限仓库里的 `src-tauri/target` 和 `dist-portable`。没有哈希匹配的记录，验收脚本拒绝启动。配置文件在构建后被改过，哈希对不上，同样拒绝。

两种数据目录分开核对：

- `portable-relative`：配置文件必须是 `src-tauri/tauri.portable.conf.json`，其中的覆盖值必须是相对路径 `./app-data`。数据目录是进程工作目录旁的 `app-data`。
- `absolute-override`：覆盖值必须是绝对路径。规范化之后等于记录里的预期目录，并且位于记录里的测试根目录中。测试根不能是盘符根，并且必须位于 `%TEMP%` 下面。

## 便携构建

```powershell
powershell -File scripts/build-portable.ps1
powershell -File scripts/build-portable.ps1 -Debug
```

`pnpm portable` 和 `pnpm portable:debug` 使用同一份配置，但不会写来源记录。需要记录时运行上面的脚本。

## 一次空库

准备一份未入库的 JSON，只包含绝对路径覆盖，例如 `{ "app": { "appDirectoriesOverride": "C:\\Users\\<you>\\AppData\\Local\\Temp\\music-mine-isolated\\data" } }`。测试根使用已存在的专用空目录，例如 `%TEMP%\music-mine-isolated`。

```powershell
powershell -File scripts/build-isolated.ps1 -Config <json 路径> -TestRoot <测试根>
```

脚本拒绝 `src-tauri/tauri.conf.json`，也拒绝已经入库的配置。

## 语料验收进程

`tests/perf/stage-cover-run.ps1` 只接受带 `portable-relative` 记录的 exe。它把 exe 复制到 `%TEMP%\mm-cover-corpus\run-<label>`，工作目录就是这个运行目录。

启动时写下 `process.json`：PID、启动时间、exe 完整路径。结束时先关闭主窗口，再只结束身份仍然匹配的播放器及其后代进程。PID 被复用，或启动时间、exe 路径对不上时，不结束进程，也不按进程名结束。

删除运行目录前，路径必须落在语料根目录里。遇到指向根目录之外的 junction，直接拒绝删除。
````

## File: docs/RFC-v2.0.md
````markdown
# Windows 现代本地音乐播放器

## 完整技术方案与开发规范 · RFC v2.0

**目标平台：** Windows 10 / Windows 11  
**项目类型：** 本地桌面音乐播放器  
**主要用途：** 个人长期使用  
**核心技术：** Rust + Tauri v2 + Svelte 5  
**开发方式：** Codex / Claude Code 辅助开发  
**方案状态：** 正式技术基线

---

# 1. 项目愿景

## 1.1 产品定位

打造一款现代、强大、易用的 Windows 本地音乐播放器，兼顾：

- **foobar2000 的可靠播放能力**
    
- **MusicBee 的丰富曲库管理**
    
- **Apple Music / Spotify 的现代交互体验**
    

不以全面复刻这些软件为目标，而是吸收其中适合个人本地音乐管理的功能与设计。

播放器应具有以下特点：

1. 音乐播放稳定、音质可靠。
    
2. 能够高效管理大型本地音乐库。
    
3. 界面美观、简洁、操作流畅。
    
4. 支持歌词、封面、歌单等丰富功能。
    
5. 支持 Windows 系统集成。
    
6. 在普通电脑上保持合理的 CPU、GPU 与内存占用。
    
7. 方便持续增加功能和改进界面。
    
8. 默认离线运行，不依赖服务器或账号。
    

## 1.2 开发原则

### 稳定性优先

播放音乐是软件最重要的功能。界面动画、数据库扫描、文件操作都不能干扰正常播放。

### 体验优先

优先考虑日常使用的便利性，而不是技术参数的极限表现。

### 本地优先

用户的音乐、歌单、数据库和设置都保存在本地。

### 适度模块化

确保主要模块职责清晰，但不强制建立复杂的分层框架或大量 Trait。

### 渐进式增强

先构建完整、稳定、好用的播放器，再增加高级音频处理与复杂视觉效果。

### AI 开发友好

项目结构和接口应易于理解、测试、修改，并通过文档约束 AI Agent 的修改范围。

---

# 2. 最终技术栈

|架构模块|技术选择|说明|
|---|---|---|
|核心语言|Rust|播放器业务、音频和数据处理|
|桌面容器|Tauri v2|窗口、IPC、系统集成|
|前端框架|Svelte 5|现代 UI|
|前端语言|TypeScript|类型安全|
|前端构建|Vite|构建、热更新|
|包管理器|pnpm|前端依赖管理|
|样式系统|Tailwind CSS|组件布局与样式|
|主题系统|CSS Variables|颜色、尺寸、主题|
|图标|lucide-svelte|统一图标风格|
|UI 动画|Svelte Transitions + CSS|动画与交互|
|复杂可视化|Canvas / WebGL2|按需使用|
|虚拟滚动|@tanstack/svelte-virtual|大型歌曲列表|
|音频解码|Symphonia|常见格式解码|
|音频输出|cpal|默认设备播放|
|高级 Windows 音频|wasapi / windows-rs|后期独占模式等功能|
|音频重采样|rubato|必要时进行采样率转换|
|音乐标签|lofty|元数据及封面|
|数据库|SQLite|本地持久化|
|数据库驱动|rusqlite|Rust 数据库访问|
|全文检索|SQLite FTS5|快速音乐搜索|
|文件监控|notify|增量扫描|
|后台任务|Tokio + 专用线程|IO、扫描、工作任务|
|Rust 日志|tracing|调试与故障排查|
|类型绑定|tauri-specta|自动生成 TS 接口|
|UI 类型检查|svelte-check|Svelte 类型和组件检查|
|前端规范|ESLint + Prettier|格式与静态检查|
|Rust 规范|rustfmt + Clippy|代码质量|
|自动化测试|cargo test + Vitest|业务与 UI 测试|
|桌面发布|Tauri NSIS|Windows 安装包|

## 2.1 选型说明

采用 Tauri + Svelte，而不是纯 Rust UI，主要是因为：

- 更容易制作现代、精致的播放器界面。
    
- 组件化开发自然，适合歌词、曲库和播放控制等界面。
    
- Svelte 5 响应式能力足够优秀。
    
- UI 可以与 Rust 音频引擎独立开发。
    
- AI 能比较方便地修改样式、布局和交互。
    
- 复杂动画可以按需使用浏览器渲染能力。
    

接受的技术成本包括 WebView2 运行开销、IPC 设计和跨语言调试。

对于个人长期使用，这些成本是合理的。

## 2.2 依赖管理

- 优先选择稳定、活跃维护的依赖。
    
- 兼容性优先于追逐最新版本。
    
- 提交 Cargo.lock 和 pnpm-lock.yaml。
    
- 涉及 RC 版本的 tauri-specta / Specta 必须固定兼容版本。
    
- 新依赖必须有明确用途。
    
- 定期升级，但避免一次性升级整个技术栈。
    

---

# 3. 系统整体架构

## 3.1 架构模式

采用轻量模块化分层架构。

主要分为四层。

### 第一层：Svelte UI

负责：

- 界面展示。
    
- 用户交互。
    
- 视觉动画。
    
- 页面导航。
    
- 临时 UI 状态。
    
- 歌曲列表与歌词渲染。
    

### 第二层：Tauri 通信层

负责：

- 命令调用。
    
- 数据类型转换。
    
- 事件通知。
    
- 扫描进度传输。
    
- Rust 与 TypeScript 的类型契约。
    

### 第三层：Rust 应用服务层

负责：

- 播放器状态。
    
- 播放队列。
    
- 曲库管理。
    
- 歌单管理。
    
- 歌词加载。
    
- 设置与任务调度。
    

### 第四层：基础设施

负责：

- 音频解码。
    
- 音频输出。
    
- 数据库。
    
- 文件系统。
    
- 音乐元数据。
    
- Windows 系统 API。
    

## 3.2 基本规则

1. 音频引擎不能依赖 Svelte、WebView 或具体 UI 组件。
    
2. Svelte 不能直接访问音频设备。
    
3. Rust 是真实播放状态的唯一权威来源。
    
4. 数据库与文件扫描不能阻塞音频输出。
    
5. 所有跨端通信使用明确的数据类型。
    
6. 前端业务组件通过统一的 Service 层调用 Rust。
    
7. 具体基础设施实现尽量隐藏在应用服务后面。
    

不要求每一层都通过复杂 Trait 或抽象工厂连接。

---

# 4. 项目目录结构

初期只使用一个主要 Rust crate。

```
music-player/
│
├── src/                         # Svelte 5 前端
│   ├── lib/
│   │   ├── components/          # 通用 UI 组件
│   │   │   ├── Button.svelte
│   │   │   ├── Modal.svelte
│   │   │   ├── ContextMenu.svelte
│   │   │   ├── Slider.svelte
│   │   │   └── Tooltip.svelte
│   │   │
│   │   ├── features/
│   │   │   ├── player/          # 播放控制
│   │   │   ├── library/         # 曲库
│   │   │   ├── albums/          # 专辑
│   │   │   ├── artists/         # 歌手
│   │   │   ├── playlists/       # 歌单
│   │   │   ├── lyrics/          # 歌词
│   │   │   ├── search/          # 搜索
│   │   │   └── settings/        # 设置
│   │   │
│   │   ├── stores/              # Svelte 5 共享状态
│   │   ├── services/            # Tauri API 封装
│   │   ├── types/               # 前端辅助类型
│   │   ├── utils/               # 通用工具
│   │   └── styles/
│   │       ├── app.css
│   │       ├── tokens.css
│   │       └── themes.css
│   │
│   ├── App.svelte
│   └── main.ts
│
├── src-tauri/
│   ├── src/
│   │   ├── audio/
│   │   │   ├── decoder.rs
│   │   │   ├── output.rs
│   │   │   ├── buffer.rs
│   │   │   └── resampler.rs
│   │   │
│   │   ├── player/
│   │   │   ├── controller.rs
│   │   │   ├── queue.rs
│   │   │   └── state.rs
│   │   │
│   │   ├── library/
│   │   │   ├── scanner.rs
│   │   │   ├── metadata.rs
│   │   │   └── search.rs
│   │   │
│   │   ├── database/
│   │   │   ├── connection.rs
│   │   │   ├── tracks.rs
│   │   │   ├── playlists.rs
│   │   │   └── migrations.rs
│   │   │
│   │   ├── lyrics/
│   │   ├── commands/
│   │   ├── windows/
│   │   ├── artwork/
│   │   ├── settings/
│   │   ├── app_state.rs
│   │   ├── lib.rs
│   │   └── main.rs
│   │
│   ├── migrations/
│   ├── capabilities/
│   └── Cargo.toml
│
├── tests/
│   ├── fixtures/
│   └── integration/
│
├── docs/
│   ├── architecture.md
│   ├── features.md
│   ├── decisions.md
│   └── testing.md
│
├── AGENTS.md
├── package.json
├── pnpm-lock.yaml
├── svelte.config.js
├── vite.config.ts
└── README.md
```

此结构用于指导代码组织，不要求初始化项目时建立所有空文件。

随着真实功能开发逐步增加对应目录。

---

# 5. Svelte 5 前端架构

这是相对于 SolidJS 版本最重要的调整。

## 5.1 统一使用 Runes

新代码使用 Svelte 5 的响应式模型：

- `$state`：可变响应式状态。
    
- `$derived`：派生状态。
    
- `$effect`：必要的副作用。
    
- `$props`：组件参数。
    
- `$bindable`：确实需要双向绑定时使用。
    

不在新组件中使用旧版的 `$:` 响应语法或 `on:click` 写法。

避免将 `$effect` 当作通用状态更新工具。

能够使用 `$derived` 表达的派生关系，不通过 `$effect` 手动同步。

## 5.2 状态分层

前端状态分为三类。

### 组件本地状态

例如：

- 菜单是否打开。
    
- 弹窗状态。
    
- 当前悬停项目。
    
- 输入框内容。
    
- 临时拖拽状态。
    

使用组件内 `$state`。

### 共享 UI 状态

例如：

- 当前主题。
    
- 当前页面。
    
- 侧边栏是否展开。
    
- 播放队列面板状态。
    
- 歌词展示模式。
    
- 曲库筛选条件。
    

使用独立的 `.svelte.ts` 文件管理。

### 后端业务状态

例如：

- 当前播放曲目。
    
- 实际播放状态。
    
- 当前播放队列。
    
- 音频输出设备。
    
- 曲库歌曲数据。
    

由 Rust 维护，前端保存只读状态快照或用于展示的响应式副本。

## 5.3 前端状态示例

可以设计：

```
stores/
├── player.svelte.ts
├── navigation.svelte.ts
├── theme.svelte.ts
├── library.svelte.ts
└── lyrics.svelte.ts
```

其中 `player.svelte.ts` 负责：

- 保存当前 PlayerSnapshot。
    
- 处理后端推送的播放状态。
    
- 提供 UI 可以读取的响应式字段。
    
- 调用统一的播放器 Service。
    

但不直接解码音频，也不自行维护第二套真实播放队列。

## 5.4 数据与展示分离

推荐数据流：

```
Svelte Component
       ↓
Frontend Service
       ↓
Tauri Command
       ↓
Rust Player Controller
       ↓
Audio Engine / Database
       ↓
State Snapshot / Result
       ↓
Svelte Reactive State
       ↓
UI Update
```

Svelte 组件尽量只处理展示和操作。

复杂数据处理放进独立的 TypeScript 工具或 Rust 服务中。

## 5.5 页面组织

不默认引入 SvelteKit。

使用 Vite 构建单页桌面应用。

初期采用简单的页面状态和组件切换。

页面数量增加、确实需要深层导航和历史记录后，再考虑引入专门的路由方案。

## 5.6 组件规范

建议：

- 每个组件具有清晰职责。
    
- 通用组件不包含具体业务逻辑。
    
- 页面组件负责组合功能组件。
    
- 不把整个播放器写在一个 App.svelte 中。
    
- 公共颜色、尺寸与动画时长通过 Design Tokens 管理。
    
- 不强制每个文件少于固定行数。
    
- 尽量避免过多组件之间的双向状态依赖。
    

## 5.7 Svelte 生命周期管理

- 通过 `onMount` 建立需要浏览器环境的监听。
    
- 组件销毁时释放事件订阅和观察器。
    
- 播放页面切换不能导致重复订阅。
    
- 长时间运行的动画需要支持暂停和销毁。
    
- 后台隐藏时降低不必要的渲染频率。
    

---

# 6. 现代 UI 设计系统

## 6.1 整体设计

采用现代桌面播放器风格：

- 深色主题优先。
    
- 支持浅色主题。
    
- 中性色为基础。
    
- 以专辑封面作为视觉焦点。
    
- 少量强调色。
    
- 适度圆角。
    
- 克制的阴影。
    
- 清晰的字体层级。
    
- 自然、轻量的动画。
    

## 6.2 主窗口布局

推荐三部分结构。

### 左侧导航栏

包含：

- 首页。
    
- 歌曲。
    
- 专辑。
    
- 艺术家。
    
- 我喜欢的。
    
- 播放列表。
    
- 设置。
    

支持折叠。

### 中间主内容区

根据页面显示：

- 歌曲列表。
    
- 专辑网格。
    
- 艺术家列表。
    
- 搜索结果。
    
- 专辑详情。
    
- 歌单详情。
    

### 底部播放控制栏

始终显示：

- 当前封面。
    
- 歌曲标题与歌手。
    
- 上一曲、播放暂停、下一曲。
    
- 播放进度。
    
- 音量。
    
- 循环与随机模式。
    
- 歌词和播放队列入口。
    

右侧队列面板可以按需展开。

## 6.3 主要页面

|页面|主要内容|
|---|---|
|首页|最近播放、最近添加、常用歌单|
|歌曲|全部歌曲、排序、筛选|
|专辑|专辑封面网格|
|艺术家|歌手与相关专辑|
|专辑详情|专辑信息和歌曲|
|歌单详情|歌单歌曲与编辑|
|搜索|综合搜索结果|
|正在播放|大封面、歌词、音频信息|
|设置|外观、播放、曲库与系统|

## 6.4 视觉规范

建议从以下 Design Tokens 开始：

- 间距：4px 基础网格。
    
- 常规圆角：8–12px。
    
- 大型容器圆角：12–16px。
    
- 控件动画：约 120–220ms。
    
- 页面过渡：约 180–300ms。
    
- 字体：系统中文字体与 Segoe UI 等系统字体合理回退。
    
- 图标：统一 Lucide 风格。
    
- 控件尺寸：优先适合鼠标操作。
    

上述尺寸是初始设计参考，可根据实际视觉效果调整。

## 6.5 主题系统

支持：

1. 深色。
    
2. 浅色。
    
3. 跟随系统。
    
4. 自定义强调色。
    
5. 后续增加根据封面提取强调色。
    

颜色使用 CSS Variables。

避免在每个 Svelte 组件中硬编码完整颜色体系。

## 6.6 动画策略

优先使用：

- CSS Transition。
    
- CSS Transform。
    
- Svelte Transition。
    
- Svelte Motion。
    
- Web Animations API。
    

复杂背景才使用 Canvas 或 WebGL2。

不默认添加大型动画库。

支持“减少动画”选项并尊重系统相关偏好。

---

# 7. 音频播放引擎

## 7.1 播放架构

核心链路：

```
Audio File
    ↓
Symphonia Demux / Decode
    ↓
PCM Samples
    ↓
Buffer
    ↓
Format Conversion
    ↓
Optional Resampler / DSP
    ↓
cpal Output Stream
    ↓
Windows Audio Device
```

音频引擎由 Rust 实现。

前端不使用 HTML Audio 元素承担主要音乐播放工作。

## 7.2 音频格式

第一阶段目标：

- MP3
    
- FLAC
    
- WAV
    
- OGG Vorbis
    
- AAC / M4A
    
- ALAC
    
- AIFF（按需）
    

实际支持需要验证解码器的对应 feature、容器和编码变体。

不以“支持所有格式”作为初期目标。

后续可评估 Opus、WavPack 等格式的扩展解码实现。

## 7.3 音频输出

初期使用 cpal。

默认：

- Windows 默认输出设备。
    
- 共享模式。
    
- 合理缓冲大小。
    
- 自动匹配或转换采样率。
    
- 支持基础音量控制。
    

后续扩展：

- 输出设备选择。
    
- 默认设备变化检测。
    
- WASAPI 独占模式。
    
- 更详细的采样格式和缓冲设置。
    

## 7.4 线程模型

至少区分：

**解码工作线程**

负责文件读取、解码、Seek 和 PCM 数据准备。

**音频输出线程 / 回调**

负责从准备好的缓冲区取数据并提交给音频设备。

**应用服务任务**

负责播放队列、状态控制、曲库访问。

**UI 渲染线程**

由 WebView2 / Svelte 负责。

音频回调中不执行数据库查询、文件读取、阻塞等待或复杂日志操作。

采用有界 PCM 缓冲区，必要时使用成熟的 SPSC 环形缓冲库。

## 7.5 播放状态

建议状态：

- Idle
    
- Loading
    
- Playing
    
- Paused
    
- Stopped
    
- Error
    

需要时增加 Buffering、Recovering。

Rust 维护真实状态。

在播放设备成功启动之前，不向前端报告已成功播放。

## 7.6 播放队列

支持：

- 添加歌曲。
    
- 添加整个专辑。
    
- 添加歌单。
    
- 立即播放。
    
- 添加到下一首。
    
- 添加到队列末尾。
    
- 删除队列项目。
    
- 手动排序。
    
- 清空队列。
    
- 顺序、随机、单曲循环、列表循环。
    

随机播放应保留必要的播放历史，避免“上一曲”功能行为混乱。

## 7.7 Seek 与切歌

Seek 需要：

1. 接受指定目标时间。
    
2. 使旧解码和缓冲数据失效。
    
3. 解码器移动到目标位置。
    
4. 重新填充缓冲区。
    
5. 更新播放器状态。
    
6. 向 UI 发送实际结果。
    

使用曲目版本或请求序号避免旧操作结果覆盖新状态。

## 7.8 播放进度

播放器内部记录实际播放位置。

前端根据状态快照和单调时钟平滑插值。

不需要每帧从 Rust 请求播放进度。

播放暂停、Seek、切歌后立即校正前端位置。

在窗口隐藏时降低或暂停视觉插值更新。

## 7.9 后期音频增强

根据使用需求增加：

- Gapless。
    
- ReplayGain。
    
- Rubato 重采样。
    
- 均衡器。
    
- Crossfade。
    
- WASAPI 独占模式。
    
- 音频可视化。
    

无缝播放必须处理编码延迟、尾部填充和采样边界，不能只靠提前缓冲。

Bit-perfect 如果未来要实现，应作为独立的高级播放模式考虑，并明确与 EQ、软件音量和重采样的冲突。

---

# 8. 曲库管理

## 8.1 曲库导入

支持：

- 添加多个音乐目录。
    
- 递归扫描。
    
- 自动发现支持的音频格式。
    
- 读取音乐元数据。
    
- 关联专辑和歌手。
    
- 提取封面。
    
- 保存到 SQLite。
    

扫描必须在后台进行。

界面显示扫描进度，允许取消。

## 8.2 曲库数据模型

建议基础实体：

- Track。
    
- Album。
    
- Artist。
    
- Playlist。
    
- PlaylistItem。
    
- LibraryFolder。
    
- PlayHistory。
    
- Favorite。
    
- Setting。
    

歌曲与歌手采用可支持多歌手的关系设计。

使用稳定的内部歌曲 ID，不直接把完整文件路径作为永久主键。

## 8.3 基础数据库表

建议：

```
tracks
albums
artists
track_artists
library_folders
playlists
playlist_tracks
favorites
play_history
settings
schema_migrations
```

其中 `tracks` 可包含：

- id
    
- file_path
    
- title
    
- album_id
    
- duration_ms
    
- track_number
    
- disc_number
    
- format
    
- sample_rate
    
- channels
    
- bit_depth
    
- bitrate
    
- file_size
    
- modified_at
    
- added_at
    
- availability_status
    

根据实际解析能力保存字段，不需要一次填满。

## 8.4 数据库策略

采用 SQLite + rusqlite。

开启适当的 WAL 模式与索引。

使用事务进行批量写入。

提供可测试的数据库迁移脚本。

数据库升级前考虑备份。

不在音频回调线程中进行数据库操作。

## 8.5 搜索

初期支持：

- 歌曲名称。
    
- 歌手。
    
- 专辑。
    
- 歌单。
    

采用 SQLite FTS5。

中文搜索使用合适的字符处理和索引策略。

可以结合 trigram 与普通查询，并针对一两个字符的短关键词提供回退查询。

后续再增加拼音搜索和更高级的模糊匹配。

## 8.6 大型列表

Svelte 前端使用 @tanstack/svelte-virtual。

曲库查询支持：

- 分页。
    
- 稳定排序。
    
- 筛选。
    
- 搜索。
    
- 可取消的异步请求。
    

普通列表不会一次性把十万首歌曲全部渲染到 DOM 中。

深度翻页时可评估游标分页或其他更合适的查询机制。

## 8.7 增量扫描

使用 notify 监听目录变化。

处理：

- 新增文件。
    
- 删除文件。
    
- 文件重命名。
    
- 文件内容变化。
    
- 文件夹移动。
    

监听事件需要去抖动与合并。

应用启动时进行必要的轻量校验，防止休眠或退出期间的文件变化被遗漏。

文件暂时找不到时先标记为不可用，尽量保留歌单关联。

---

# 9. 音乐标签与封面

## 9.1 音乐标签

使用 lofty 读取：

- Title。
    
- Artist。
    
- Album。
    
- Album Artist。
    
- Genre。
    
- Year / Date。
    
- Track Number。
    
- Disc Number。
    
- Embedded Artwork。
    

标签读取失败不能阻断整个曲库扫描。

## 9.2 标签编辑

后期支持：

- 单首编辑。
    
- 批量编辑。
    
- 修改封面。
    
- 补充缺失信息。
    

默认不自动修改原始音频文件。

写入前检查权限及格式支持。

批量操作提供预览和错误反馈。

重要修改考虑备份或可恢复策略。

## 9.3 封面查找

优先级：

1. 音频内嵌封面。
    
2. 音乐目录中的 cover / folder 等图片。
    
3. 默认占位封面。
    

## 9.4 缓存设计

建立磁盘缩略图缓存。

可保存：

- 小尺寸列表封面。
    
- 中尺寸网格封面。
    
- 播放页所需的大尺寸封面。
    

内存缓存设置容量限制。

通过受限的 Tauri 资源访问方式向前端提供图片，避免大量 Base64 数据通过 IPC 传输。

不将所有原始封面长期加载进内存。

---

# 10. 歌词系统

## 10.1 基础歌词

支持：

- .lrc 文件。
    
- 普通时间戳。
    
- 歌词自动滚动。
    
- 当前行高亮。
    
- 点击歌词跳转。
    
- 歌词时间偏移设置。
    
- 字体大小和对齐。
    

## 10.2 歌词查找

优先搜索：

1. 同名 LRC 文件。
    
2. 音乐目录中的匹配歌词。
    
3. 后续增加用户指定歌词文件。
    

不默认引入在线歌词搜索依赖。

## 10.3 逐字歌词

后续支持逐字时间戳。

前端通过本地播放时间进行高亮动画。

避免每个字的高亮进度都依赖 Rust IPC。

## 10.4 歌词页面

设计：

- 大尺寸专辑封面。
    
- 歌曲标题和歌手。
    
- 逐行歌词。
    
- 当前歌词居中。
    
- 柔和的背景渐变。
    
- 可切换封面 / 歌词布局。
    

后续增加独立桌面歌词窗口。

---

# 11. Tauri 前后端通信

## 11.1 通信原则

按数据类型选择最简单的通信方式。

|数据|通信方式|
|---|---|
|播放控制|Command|
|歌曲详情查询|Command|
|曲库分页|Command|
|歌单编辑|Command|
|状态变更|Event|
|扫描进度|Event / Channel|
|大量持续数据|按需 Channel|
|封面资源|缓存资源 URL|

## 11.2 主要 Commands

播放器：

```
player_play
player_pause
player_stop
player_seek
player_next
player_previous
player_set_volume
player_set_mode
player_get_state
player_get_devices
player_set_device
```

曲库：

```
library_add_folder
library_remove_folder
library_scan
library_cancel_scan
library_search
library_get_tracks
library_get_albums
library_get_artists
library_get_track
```

歌单：

```
playlist_create
playlist_rename
playlist_delete
playlist_add_tracks
playlist_remove_tracks
playlist_reorder
playlist_get
```

设置：

```
settings_get
settings_update
```

具体参数及返回值以 Rust 类型为准。

## 11.3 状态同步

推荐核心数据结构：

```
PlayerSnapshot
├── revision
├── track_id
├── playback_state
├── position_ms
├── duration_ms
├── volume
├── playback_mode
├── output_device_id
└── error
```

其中 `revision` 用于区分状态新旧。

对于播放、Seek 等快速连续操作，还可以使用命令序号或曲目代次。

前端不得用旧事件覆盖更新的播放状态。

## 11.4 错误处理

Rust 命令返回统一、可序列化的错误类型。

前端将错误转换成用户可以理解的提示。

例如：

- 文件不存在。
    
- 不支持该格式。
    
- 设备无法打开。
    
- 文件正在被占用。
    
- 数据库暂时不可用。
    

避免直接把底层调试字符串原样显示给用户。

## 11.5 自动类型绑定

使用 tauri-specta 生成前端接口。

生成文件禁止手动编辑。

项目初始化时需要验证所采用版本与 Tauri v2 的兼容性。

如果该依赖在当前版本组合中存在问题，可以先使用手工定义的集中式类型与 API 封装，等版本稳定后再接入自动生成。

---

# 12. Windows 系统集成

## 12.1 基础功能

优先支持：

- 窗口位置和大小记忆。
    
- 最小化。
    
- 最大化。
    
- 关闭到托盘选项。
    
- 系统托盘菜单。
    
- 文件拖放。
    
- 系统主题适配。
    

## 12.2 Windows 媒体控制

后续集成 SMTC：

- 媒体键播放暂停。
    
- 上一曲、下一曲。
    
- 系统媒体浮窗。
    
- 歌曲标题和歌手。
    
- 专辑封面。
    
- 播放状态。
    

## 12.3 系统托盘

托盘菜单包含：

- 显示主窗口。
    
- 播放 / 暂停。
    
- 上一曲。
    
- 下一曲。
    
- 退出。
    

关闭主窗口时可选择：

- 完全退出。
    
- 最小化到托盘并继续播放。
    

这两种行为必须清楚区分。

## 12.4 Windows 11 外观

第一版优先使用正常、可靠的窗口机制。

后期可加入：

- Mica。
    
- Acrylic。
    
- 自定义标题栏。
    
- Windows 11 风格窗口按钮。
    

不能因为自定义标题栏而破坏窗口缩放、拖动、最大化及 Snap Layouts 等系统体验。

---

# 13. 设置系统

## 13.1 设置页面结构

### 常规

- 语言。
    
- 开机启动。
    
- 关闭行为。
    
- 启动时恢复。
    

### 播放

- 默认播放模式。
    
- 是否恢复进度。
    
- 音量记忆。
    
- 音频设备。
    
- 后期 DSP 设置。
    

### 曲库

- 音乐目录。
    
- 自动扫描。
    
- 文件监控。
    
- 清理缓存。
    
- 重建索引。
    

### 外观

- 深色 / 浅色。
    
- 跟随系统。
    
- 强调色。
    
- 动画设置。
    
- 布局密度。
    

### 歌词

- 字体。
    
- 大小。
    
- 对齐。
    
- 歌词偏移。
    
- 逐字显示。
    

### 快捷键

- 播放暂停。
    
- 上一曲 / 下一曲。
    
- 搜索。
    
- 音量。
    
- 显示歌词。
    

## 13.2 设置存储

优先采用 SQLite 或本地配置文件中的一套统一持久化机制。

不同时引入多种功能重叠的设置存储系统。

窗口状态可以使用 Tauri 的专用插件独立管理。

---

# 14. 性能优化方案

## 14.1 主要原则

性能优化以实际体验为准。

避免过早针对某个框架的理论性能进行复杂设计。

## 14.2 UI 性能

- 使用 Svelte 5 细粒度响应式更新。
    
- 大型歌曲列表虚拟化。
    
- 避免不必要的全局状态更新。
    
- 滚动期间避免加载完整大封面。
    
- 使用图片尺寸缓存。
    
- 减少大面积 CSS Blur。
    
- 尽量使用 transform 和 opacity 动画。
    
- 动态背景支持关闭。
    
- 后台窗口降低视觉计算。
    

## 14.3 音频性能

- 解码与音频输出分离。
    
- 使用有界 PCM 缓冲。
    
- 避免音频回调中的锁等待与分配。
    
- 对可能产生断音的情况记录统计。
    
- 按实际设备情况调整缓冲策略。
    

## 14.4 数据库性能

- 建立必要索引。
    
- 使用批量事务。
    
- 搜索索引。
    
- 异步分页查询。
    
- 文件变化增量更新。
    
- 避免无意义的全曲库重新扫描。
    

## 14.5 初步性能目标

这些是优化方向，不是未经测试的保证。

|项目|初步目标|
|---|---|
|启动速度|常用 SSD 电脑上约 2 秒内可交互|
|常规操作|操作反馈无明显迟滞|
|曲库搜索|常见查询体感即时|
|曲库规模|优先保障数万首，逐步测试十万首|
|列表滚动|60 FPS 为基本目标|
|高刷体验|硬件允许时进一步优化|
|长时间播放|不出现软件导致的异常中断|
|内存使用|稳定、可控、不持续增长|
|后台使用|不产生无意义的高 CPU/GPU 占用|

测试时记录机器配置、测试数据规模和冷 / 热缓存状态。

---

# 15. 稳定性与异常处理

必须覆盖以下问题：

1. 音乐文件损坏或格式不支持。
    
2. 音乐文件被移动、删除。
    
3. 输出设备突然断开。
    
4. Windows 默认输出设备切换。
    
5. 系统休眠及唤醒。
    
6. 解码器异常。
    
7. 数据库损坏或迁移失败。
    
8. 曲库扫描途中取消。
    
9. 大量文件扫描导致资源压力。
    
10. 软件异常退出。
    

处理原则：

- 不崩溃优先。
    
- 能恢复则自动恢复。
    
- 无法恢复时明确提示。
    
- 不丢失歌单和设置。
    
- 不擅自删除或修改原始音乐。
    
- 关键错误记录到日志。
    

使用 tracing 进行日志管理。

日志容量需要限制，不允许无限增长。

---

# 16. 数据安全与本地隐私

## 16.1 文件安全

- 默认只读取用户音乐文件。
    
- 标签编辑需要用户明确操作。
    
- 删除曲库记录不等于删除磁盘文件。
    
- 不允许界面无权限访问任意文件路径。
    
- 处理路径规范化、目录边界和文件访问错误。
    

## 16.2 本地数据

数据库、日志、设置和封面缓存放在 Windows 标准应用数据目录。

分离用户数据与可重建缓存。

清除缓存不能删除歌单、收藏或播放历史。

## 16.3 Tauri 安全

- 使用最小必要的 Capabilities。
    
- 不随意暴露文件系统能力。
    
- 不加载不受信任的远程页面。
    
- 自定义资源协议限制访问范围。
    
- Rust 命令验证输入参数。
    

---

# 17. 测试与质量控制

## 17.1 Rust 测试

主要测试：

- 状态机。
    
- 播放队列。
    
- 随机播放。
    
- 循环模式。
    
- Seek 控制。
    
- 曲库扫描。
    
- 元数据解析。
    
- 数据库迁移。
    
- 歌单操作。
    
- LRC 解析。
    
- 错误恢复逻辑。
    

## 17.2 Svelte 测试

主要测试：

- 组件交互。
    
- 状态展示。
    
- 列表选择。
    
- 搜索界面。
    
- 歌单操作。
    
- 设置管理。
    
- 重要组件的事件清理。
    

可使用 Vitest。

实际桌面集成测试根据必要性逐步增加。

## 17.3 实机测试

重点验证：

- 不同音频格式。
    
- 快速连续切歌。
    
- 长时间播放。
    
- 休眠恢复。
    
- 设备拔插。
    
- 托盘播放。
    
- 文件扫描期间播放。
    
- 大曲库滚动。
    
- Windows 安装和卸载。
    

音频设备兼容性不能完全依赖模拟测试。

## 17.4 基础质量检查

```
# 前端
pnpm exec svelte-check
pnpm exec eslint .
pnpm exec prettier --check .
pnpm build

# Rust（在 src-tauri 中）
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

以上命令以最终配置的工具版本为准。

可以将其封装成项目统一检查脚本。

---

# 18. AI Agent 开发规范

## 18.1 核心要求

Codex / Claude Code 进行开发时应：

1. 先阅读 AGENTS.md。
    
2. 理解相关模块的现有逻辑。
    
3. 不随意替换技术框架。
    
4. 优先修改现有模块。
    
5. 不创建没有必要的抽象层。
    
6. 不擅自调整数据库结构。
    
7. 不随意修改音频实时线程。
    
8. 不使用 Mock 假装功能已经实现。
    
9. 新增功能时补充必要测试。
    
10. 完成任务后运行对应检查。
    

## 18.2 Svelte 专项规则

- 新组件统一使用 Svelte 5 Runes。
    
- 不混用旧版 Svelte 状态语法。
    
- 共享状态放到明确的 `.svelte.ts` 模块。
    
- 不在视图组件内直接散落 Tauri invoke 调用。
    
- 后端接口统一通过 Services 层。
    
- 事件订阅必须正确释放。
    
- 不通过 `$effect` 构造复杂、循环的状态同步。
    
- 派生状态优先使用 `$derived`。
    
- 大型列表必须考虑虚拟化。
    
- 禁止无理由增加全局状态。
    

## 18.3 Rust 专项规则

- 不在音频实时回调中进行阻塞 IO。
    
- 不在音频回调中进行数据库访问。
    
- 不允许 UI 状态反向控制底层实现细节。
    
- 数据库变更必须提供迁移。
    
- 不能忽略重要错误。
    
- 不使用 `.unwrap()` 掩盖正常运行时可能发生的失败。
    
- 优先复用可靠的成熟 crate。
    
- 避免无意义的 Trait 和多层抽象。
    

## 18.4 分任务开发

每个任务应说明：

- 目标。
    
- 涉及文件或模块。
    
- 接口要求。
    
- 视觉要求（如有）。
    
- 验收标准。
    
- 需要运行的测试。
    

建议每次完成一个可以独立验证的功能。

## 18.5 文档更新

项目保留：

- README.md。
    
- AGENTS.md。
    
- architecture.md。
    
- features.md。
    
- decisions.md。
    
- testing.md。
    

重大架构调整需要更新文档。

普通 CSS 微调不需要额外编写复杂设计记录。

---

# 19. 开发阶段

## Phase 0：项目初始化

内容：

- 创建 Tauri v2 + Svelte 5 + TypeScript 项目。
    
- 配置 Vite。
    
- 配置 Tailwind CSS。
    
- 配置主题变量。
    
- 配置 ESLint、Prettier、svelte-check。
    
- 初始化 Rust 目录。
    
- 设置基本 Tauri 权限。
    
- 建立 AGENTS.md。
    
- 建立 Git 仓库与基础检查流程。
    

**验收：** 桌面窗口正常启动，Svelte 热更新正常，Rust 与 TypeScript 可以成功通信，项目能够完成构建。

## Phase 1：基础播放器

内容：

- Symphonia 解码。
    
- cpal 输出。
    
- 文件选择。
    
- 播放、暂停、停止。
    
- Seek。
    
- 音量控制。
    
- 上下曲。
    
- 播放队列。
    
- 基础播放状态同步。
    
- 简单的播放器界面。
    

**验收：** 能够稳定播放常见本地音频，并正确处理暂停、Seek 和切歌。

## Phase 2：曲库系统

内容：

- SQLite。
    
- 添加音乐文件夹。
    
- 文件扫描。
    
- 标签解析。
    
- 专辑和歌手关联。
    
- 封面读取与缓存。
    
- 歌曲、专辑、歌手页面。
    
- 搜索。
    
- 收藏。
    
- 歌单。
    

**验收：** 可以导入真实音乐收藏，快速找到并播放歌曲，重启后数据仍然存在。

## Phase 3：完整日常体验

内容：

- 完善现代 UI。
    
- LRC 歌词。
    
- 播放队列面板。
    
- 深浅主题。
    
- 托盘。
    
- 快捷键。
    
- 播放历史。
    
- 上次播放恢复。
    
- 增量扫描。
    
- 错误提示。
    
- 性能和交互优化。
    

**验收：** 可以作为主要的 Windows 本地音乐播放器长期使用。

## Phase 4：增强功能

内容：

- Gapless。
    
- ReplayGain。
    
- 音频设备管理。
    
- Windows SMTC。
    
- 标签编辑。
    
- CUE 支持。
    
- M3U / M3U8。
    
- 智能歌单。
    
- 迷你播放器。
    

**验收：** 增强功能可靠，不影响已有播放能力。

## Phase 5：视觉与高级能力

按实际需要选择：

- 逐字歌词。
    
- 桌面歌词。
    
- 封面动态取色。
    
- 流体背景。
    
- 频谱显示。
    
- 参数均衡器。
    
- WASAPI 独占。
    
- 高级重采样。
    

此阶段允许只实现个人真正需要的功能。

## Phase 6：长期维护

- 修复 Bug。
    
- 优化交互。
    
- 升级依赖。
    
- 增强兼容性。
    
- 优化大型曲库。
    
- 完善错误恢复。
    
- 维护测试与文档。
    

---

# 20. 发布方案

## 20.1 发布形式

初期采用：

- Windows x64。
    
- NSIS 安装包。
    
- 普通用户安装。
    
- 无需注册账号。
    
- 默认离线运行。
    

便携版和自动更新后续再考虑。

## 20.2 数据与升级

- 升级应用时保留用户数据库。
    
- 数据库迁移需要测试。
    
- 日志、缓存、用户数据分别管理。
    
- 卸载时明确区分程序文件与个人数据。
    
- 在发布前验证全新安装和覆盖升级。
    

## 20.3 更新策略

个人自用可以采用手动下载新版本。

没有必要第一版就建设完整自动更新服务。

---

# 21. 暂不开发的功能

为控制复杂度，第一阶段不考虑：

- 在线音乐服务。
    
- 云同步。
    
- 用户账号。
    
- 多设备同步。
    
- 手机客户端。
    
- 插件商店。
    
- 自研通用音频解码框架。
    
- 完整专业 DSP 插件系统。
    
- 内置 Web 服务器。
    
- 复杂自动更新后台。
    
- 不必要的微服务和分布式架构。
    
- 全面复刻其他播放器的全部功能。
    

真正出现需求后再评估。

---

# 22. 最终架构决策

## 正式采用

**Rust + Tauri v2 + Svelte 5 + TypeScript + Vite + Tailwind CSS**

Rust 负责：

- 音频。
    
- 曲库。
    
- 歌单。
    
- 元数据。
    
- 数据库。
    
- Windows 集成。
    

Svelte 5 负责：

- 界面。
    
- 动画。
    
- 页面。
    
- 视觉状态。
    
- 歌词展示。
    
- 用户交互。
    

Tauri 负责：

- 桌面窗口。
    
- 前后端通信。
    
- 系统能力接入。
    
- 应用打包。
    

## 最终原则

**播放必须可靠。**

**界面必须舒服。**

**功能必须实用。**

**性能必须合理。**

**架构必须容易理解和维护。**

**开发复杂度必须与实际需求匹配。**

第一阶段到第三阶段完成后，项目就应该成为一款真正能长期使用的本地音乐播放器。

之后的专业音频和高级视觉功能，以实际使用体验决定是否开发，而不是为了追求规格而增加复杂度。
````

## File: docs/rust-guidelines.md
````markdown
# Rust、音频与数据库约定

适用于 `src-tauri/`。全项目安全规则在根目录 `AGENTS.md`。本页规则来自代码边界，这次文档整理没有重审音频回调的每一行。

## 模块

- `lib.rs`：Tauri 启动、命令、应用数据目录、播放控制线程。
- `audio/player.rs`：Symphonia 解码、cpal 输出、音频线程。
- `playback/controller.rs`：播放控制、队列、会话文件。
- `platform/media.rs`：Windows 媒体集成。`platform/desktop.rs`：窗口、主题、托盘、`desktop-v1.json`。
- `catalog/library.rs`、`catalog/mod.rs`、`commands/catalog.rs`、`catalog/collections.rs`：曲库、扫描、歌单。
- `catalog/db.rs`：SQLite 连接、`CREATE TABLE IF NOT EXISTS`、`ensure_column`。
- `playback/lyrics.rs`、`catalog/metadata.rs`：歌词和标签。`artwork/covers.rs`：封面缩略图、内存和磁盘缓存。`infrastructure/logging.rs`：日志。

没有按 RFC 假设尚未存在的模块或 crate。

Catalog 每实例首次连接完成 schema 初始化；后续轻量连接不执行 DDL，每个连接仍设置 `foreign_keys=ON` 与 2 秒 `busy_timeout`，导入备份继续显式迁移。

## 用户数据

应用数据目录来自 Tauri 的 `app.path().app_data_dir()`：

- `library-tauri-v1.sqlite`
- `playback-v1.json`
- `desktop-v1.json`

Windows 上这个路径来自系统已知文件夹。把进程的 `APPDATA` 指到临时目录，不会换掉它。

普通构建不改 `tauri.conf.json`。隔离构建和启动只按 `docs/isolation.md`。便携配置是 `src-tauri/tauri.portable.conf.json`，数据目录是 exe 旁的 `app-data`。不要把覆盖路径写进主配置。exe 所在目录不能用来证明它是隔离构建。

便携版在打开数据库之前检查 `app-data`、缓存和日志目录可写。写失败时提示错误并退出，不改用 Windows 用户目录。封面缓存写失败只跳过缓存文件，不回滚 SQLite。

结构变更在新建的临时库、空库和旧 schema fixture 上验证。多表写入使用事务。禁止用用户正在使用的库做迁移、去重、删除或改标签。缺文件和用户移除曲库要区分，避免删掉歌单关系。

## 音频

- 音频模块的指定 Mutex 锁在毒化后恢复 guard，避免二次 panic；这不保证受保护数据完整，也不恢复已退出的线程。
- Controller 的 Open 复用 prepare 后已确认的状态生成快照；复合切歌仍执行 Open、Play 两次，保持 revision +2 与两次 persist 调用及原失败顺序。

- cpal 输出回调中不做文件 IO、数据库 IO、阻塞等待、可阻塞 Mutex、线程 join、无界分配、网络和高频日志格式化。
- 解码、读文件、Seek 和格式转换放在工作线程。回调只读取已经准备好的 PCM。
- UI 轮询不得改变音频输出节奏。
- 断音、暂停、切歌、Seek、EOF、播放模式、设备拔插、休眠恢复，有对应测试；真实设备不可用时写 `NOT RUN`。
- 格式只认已启用的 Symphonia feature：`mp3`、`flac`、`wav`、`pcm`。现有 fixture 是 `tone.mp3`、`tone.flac`、`tone.wav`。
- UI 任务不重写播放控制器。

## 错误、权限与依赖

- 可预期的文件、设备和数据库错误不用 `unwrap` 或 `expect` 吞掉。
- 现有命令返回 `Result<T, String>`。不为了统一风格改全部签名。
- 不扩大 `src-tauri/capabilities/default.json`。现在只有 `core:default` 和 `core:window:allow-set-theme`。路径和数值在 Rust 侧校验。
- 新增 crate 前说明现有依赖为何不够、Windows 目标、feature、构建成本和替代方案。依赖变更带上 `Cargo.lock`。
- Rust 改动按范围运行 `cargo fmt --check`、`cargo clippy --all-targets -- -D warnings` 和相关 `cargo test`。没跑就写 `NOT RUN`。
````

## File: docs/testing.md
````markdown
# 测试与验收

三种结果分开写。上一级通过不能写成下一级通过。没跑写 `NOT RUN`。

| 层次 | 命令 | 能证明 | 不能证明 |
|---|---|---|---|
| 自动测试 | `pnpm format:check`、`pnpm check`、`pnpm test`；Rust 侧 `cargo fmt --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test` | 格式、类型、纯函数、从源码抽出的交互函数、Rust fixture | 布局、真实 IPC、Windows DPI |
| 浏览器验证 | `pnpm test:ui`，必要时 `pnpm test:ui:update` | harness 里的点击、键盘、主题、视口和截图 | Rust 出声、真实数据库、WebView2、DPI |
| Windows Tauri 实机 | 按 `docs/isolation.md` 构建并启动。普通 debug exe 使用系统数据目录 | WebView2、真实 command、窗口尺寸、隔离目录里的持久化 | 未操作的硬件、托盘、DPI 或曲库规模 |

`pnpm test` 是 `node --experimental-strip-types --test tests/*.test.mjs`。它不包含 `tests/ui` 和 `tests/perf`。`scripts/check.ps1` 跑 `cargo test` 时设置 `MUSIC_MINE_NULL_OUTPUT`，测试进程不打开系统音频设备。本机直接跑 `cargo test` 仍会打开真实设备。真实设备自测在该变量存在时记为跳过，不能当成扬声器已经出声。

Playwright 已安装：`@playwright/test`，配置在 `playwright.config.ts`。浏览器是本机 Playwright Chromium，视口 1280×800，`deviceScaleFactor` 为 1，时区 `Asia/Shanghai`。截图在 `tests/ui/snapshots/`。WebdriverIO 和 Tauri WebDriver 没有安装。

harness 设置 `window.isTauri`，因此壳层会显示连接成功。这只说明夹具回答了 `bridge_probe`。播放位置停在夹具给出的时间，不会自己前进。

## 桌面启动

使用构建出的 exe。Craft Anything 的持久开发服务器和 `pnpm dev` 只能预览界面。

隔离用户数据的步骤只维护在 `docs/isolation.md`。只改 `APPDATA` 会打开正在使用的曲库和设置。

便携包：`powershell -File scripts/build-portable.ps1`。调试用的同一配置加 `-Debug`，或 `pnpm portable:debug`。产物是 `dist-portable/Music-Mine-Portable-x64.zip`。这份配置只在传入 `--config src-tauri/tauri.portable.conf.json` 时合并，普通 `pnpm tauri dev` 和 `pnpm tauri build` 仍使用系统数据目录。

## 按改动选择证据

- 纯 CSS：`pnpm check` 或 `pnpm build`，加上受影响页面的浏览器验证。
- TypeScript / Svelte：相关 `pnpm test`、`pnpm check`，界面行为用 `pnpm test:ui` 或同等浏览器操作。
- Rust：相关 `cargo test`、fmt、clippy。
- IPC：类型和 harness 一起改，并在 exe 里调用一次。
- 音频设备或数据库迁移：fixture 或临时库，加上对应实机或迁移结果。真实用户库不参与。

## 测试写法

优先断言输入输出、DOM 行为和计算样式。可以从源码抽出真实函数再执行。

不要新增只匹配空格、类名拼写或临时变量名的断言。这类断言已在 `5a7c9b3` 里删过一批。仍会读源码的检查包括：主题色允许列表、Modal 接线、层级 token、删除歌单的确认顺序、从 `App.svelte` 编译封面效果。源码检查允许空白、换行及 CSS 等价的小数和属性选择器引号写法，保留运行时行为断言。

禁止把实现改回旧代码，只为了让过时字符串测试通过。

## 源码格式

Prettier `3.10.1` 与 `prettier-plugin-svelte` `4.1.1` 在 devDependencies。配置是根目录 `.prettierrc`：插件启用 Svelte parser，2 空格，单引号，`printWidth` 100。`.prettierignore` 仅排除 `node_modules/`、`dist*/`、`src-tauri/`、`docs/`、`pnpm-lock.yaml`。

`src/`、`public/`、`tests/` 全量受 Prettier 统一管理，没有源码白名单或豁免。运行 `pnpm format` 统一格式化，运行 `pnpm format:check` 检查。`scripts/check.ps1` 和调用它的 GitHub Actions CI 在 `pnpm check` 之前执行 `pnpm format:check`，失败立即退出。

Playwright 截图检查使用 `maxDiffPixels: 0`、`threshold: 0`，要求零像素差异。本次格式化验收禁止更新快照。

菜单的旧 `key()` 源码提取器和坐标工具已删除，对应键盘、碰撞与焦点行为在 `tests/ui/primitives.spec.ts` 中通过真实组件验证。`tests/helpers/source-styles.mjs` 扫描 `src/` 的所有 CSS 和 Svelte `<style>`；颜色检查单独排除已由主题测试验证的 `tokens.css` 调色板，避免 CSS 搬迁漏检。播放栏效果和删除歌单等剩余源码检查仍允许空白与换行。

## 完成报告

```text
基线：branch / HEAD
目标：
修改文件：
自动测试：PASS / FAIL / NOT RUN
浏览器验证：PASS / FAIL / NOT RUN（视口、主题、操作）
Windows Tauri 实机：PASS / FAIL / NOT RUN（exe 路径、数据目录、操作）
未验证：
Git：未提交 / 已提交的 SHA
```

## 验收来源

新任务在验收开始前运行 `powershell -File scripts/worktree-fingerprint.ps1`。验收命令结束后、写入记录之前再运行一次。两次不同则这次验收无效。

指纹包含已暂存差异、未暂存的受跟踪差异，以及 `git ls-files --others --exclude-standard` 列出的未跟踪文件的相对路径和内容。已忽略的临时产物不计入。即将写入的验收记录可以用 `-Exclude` 排除，避免记录自己把指纹改掉。

09.5.3 和 09.5.5 没有保存指纹。写「未记录，无法重建」。不要用后来的工作区补算后填进这两份记录。
````

## File: portable/README.txt
````
Music Mine 便携版（Windows x64）

把整个文件夹解压到可写入的位置，然后运行 music-mine-desktop.exe。
不需要安装程序。

系统要求
- 64 位 Windows 10（1803 或更新）或 Windows 11。
- 已安装 Microsoft Edge WebView2 Runtime（常青版）。Windows 11 通常已经自带。
  Windows 10 如果启动时提示缺少 WebView2，请安装微软提供的 Evergreen Runtime 后再运行。
- 本程序不附带安装向导，也不会自动下载运行库。

数据位置
- 首次运行会在 EXE 旁边创建 app-data。
- 曲库数据库、播放记录、窗口和主题设置、封面缓存、日志都在这个目录里。
- 不会读取或迁移 Windows 用户目录里已有的 Music Mine 数据。
- 不会在首次启动时扫描电脑上的音乐文件夹。

备份
- 退出程序后，复制整个文件夹。app-data 要和 EXE 一起复制。

更新
- 退出程序。
- 用新版本的 EXE 替换旧的 EXE，保留旁边的 app-data。
- 不要删除 app-data，除非你打算清空便携版的曲库和设置。

音乐文件路径
- 移动或复制这个文件夹时，会一起带走设置、曲库数据库、歌单、播放记录和封面缓存。
- 数据库保存的是扫描当时的完整文件路径。带走数据库不等于歌曲文件也会自动换到新位置。
- 歌曲在外部文件夹、另一台电脑，或原来的路径已经改变时，这些记录不会自己改写。打不开时需要重新扫描或重新指定文件夹。
- 把音乐放进便携文件夹，数据库里仍然是扫描时的完整路径，不会自动变成相对路径。

移除
- 退出程序后删除整个便携文件夹，包括 app-data。
- 这不会删除标准版在 Windows 用户目录中的数据，也不会删除你的音乐文件。
````

## File: public/theme-init.js
````javascript
// Runs before the app bundle so the first paint already uses the saved theme (no light/dark flash).
// Mirrors parseTheme/resolveTheme in src/lib/utils/theme.ts.
(function () {
  var choice = null;
  try {
    choice = localStorage.getItem('music-mine-theme');
  } catch (e) {
    /* storage unavailable: dark */
  }
  var theme =
    choice === 'light'
      ? 'light'
      : choice === 'system' &&
          window.matchMedia &&
          matchMedia('(prefers-color-scheme: light)').matches
        ? 'light'
        : 'dark';
  var root = document.documentElement;
  root.dataset.theme = theme;
  root.style.colorScheme = theme;
})();
````

## File: scripts/build-isolated.ps1
````powershell
param(
    [Parameter(Mandatory)][string]$Config,
    [Parameter(Mandatory)][string]$TestRoot
)
$ErrorActionPreference = 'Stop'
$repo = Resolve-Path (Join-Path $PSScriptRoot '..')
Set-Location $repo
. (Join-Path $PSScriptRoot 'isolation-lib.ps1')

$configFull = [System.IO.Path]::GetFullPath($Config)
if (-not (Test-Path -LiteralPath $configFull)) { throw "config not found: $configFull" }
$mainConfig = Get-FinalPath (Join-Path $repo 'src-tauri\tauri.conf.json')
if ($configFull.Equals($mainConfig, [System.StringComparison]::OrdinalIgnoreCase)) {
    throw "refusing to build with src-tauri/tauri.conf.json"
}
$repoFull = $repo.Path.TrimEnd('\')
$repoPrefix = $repoFull + '\'
if ($configFull.StartsWith($repoPrefix, [System.StringComparison]::OrdinalIgnoreCase)) {
    $relative = $configFull.Substring($repoPrefix.Length)
    & git -C $repo ls-files --error-unmatch -- $relative 1>$null 2>$null
    if ($LASTEXITCODE -eq 0) { throw "isolation config must be untracked: $configFull" }
}

$override = Read-IsolationConfig $configFull
if (-not [System.IO.Path]::IsPathRooted($override)) { throw "appDirectoriesOverride must be absolute" }
$rootFinal = Get-FinalPath $TestRoot
$tempFinal = Get-FinalPath $env:TEMP
if (-not (Test-PathUnderRoot $rootFinal $tempFinal)) { throw "test root must stay under TEMP" }
$dataFinal = Get-FinalPath $override
if (-not (Test-PathUnderRoot $dataFinal $rootFinal)) { throw "override escapes the test root" }

$command = "pnpm exec tauri build --debug --no-bundle --config `"$configFull`""
& pnpm exec tauri build --debug --no-bundle --config $configFull
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$exe = Join-Path $repo 'src-tauri\target\debug\music-mine-desktop.exe'
if (-not (Test-Path -LiteralPath $exe)) { throw "debug exe was not produced: $exe" }
$dest = Write-IsolationSource -Repo $repo -BuiltExe $exe -ConfigPath $configFull -Command $command -DataDirKind 'absolute-override' -ExpectedDataDir $dataFinal -TestRoot $rootFinal
Write-Output "ISOLATED_EXE=$exe"
Write-Output "ISOLATION_SOURCE=$dest"
````

## File: scripts/build-portable.ps1
````powershell
param([switch]$Debug)

$ErrorActionPreference = "Stop"
$repo = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $repo
. (Join-Path $PSScriptRoot "isolation-lib.ps1")

$profileName = "release"
$tauriArgs = @("exec", "tauri", "build", "--no-bundle", "--config", "src-tauri/tauri.portable.conf.json")
$command = "pnpm exec tauri build --no-bundle --config src-tauri/tauri.portable.conf.json"
if ($Debug) {
    $profileName = "debug"
    $tauriArgs += "--debug"
    $command += " --debug"
}

pnpm @tauriArgs
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$candidates = @(
    (Join-Path $repo "src-tauri\target\$profileName\music-mine-desktop.exe")
)
if ($env:CARGO_TARGET_DIR) {
    $candidates = @(
        (Join-Path $env:CARGO_TARGET_DIR "$profileName\music-mine-desktop.exe")
    ) + $candidates
}

$exe = $candidates | Where-Object { Test-Path $_ } | Select-Object -First 1
if (-not $exe) {
    throw "找不到便携版 EXE。已查找：$($candidates -join ', ')"
}

$stage = Join-Path $repo "dist-portable\Music Mine"
if (Test-Path $stage) { Remove-Item $stage -Recurse -Force }
New-Item -ItemType Directory -Path $stage | Out-Null
Copy-Item $exe (Join-Path $stage "music-mine-desktop.exe")
Copy-Item (Join-Path $repo "portable\README.txt") (Join-Path $stage "README.txt")

$zip = Join-Path $repo "dist-portable\Music-Mine-Portable-x64.zip"
if (Test-Path $zip) { Remove-Item $zip -Force }
Compress-Archive -Path (Join-Path $stage "*") -DestinationPath $zip
$recorded = $exe
if (-not (Test-AllowlistedBuildPath $exe $repo)) {
    $recorded = Join-Path $stage "music-mine-desktop.exe"
}
$source = Write-IsolationSource -Repo $repo -BuiltExe $recorded -ConfigPath "src-tauri/tauri.portable.conf.json" -Command $command -DataDirKind "portable-relative" -ExpectedDataDir "./app-data"
Write-Output "PORTABLE_EXE=$exe"
Write-Output "PORTABLE_ZIP=$zip"
Write-Output "ISOLATION_SOURCE=$source"
````

## File: scripts/check.ps1
````powershell
$ErrorActionPreference = "Stop"
$root = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$origin = (Get-Location).Path
$code = 0
try {
    Set-Location $root
    & pnpm format:check
    if ($LASTEXITCODE -ne 0) { $code = $LASTEXITCODE; throw "pnpm format:check exited $code" }
    & pnpm check
    if ($LASTEXITCODE -ne 0) { $code = $LASTEXITCODE; throw "pnpm check exited $code" }
    & pnpm test
    if ($LASTEXITCODE -ne 0) { $code = $LASTEXITCODE; throw "pnpm test exited $code" }
    Set-Location (Join-Path $root "src-tauri")
    & cargo fmt --all -- --check
    if ($LASTEXITCODE -ne 0) { $code = $LASTEXITCODE; throw "cargo fmt exited $code" }
    & cargo clippy --all-targets -- -D warnings
    if ($LASTEXITCODE -ne 0) { $code = $LASTEXITCODE; throw "cargo clippy exited $code" }
    # GitHub's Windows runner access-violates inside WASAPI when tests open real devices.
    $env:MUSIC_MINE_NULL_OUTPUT = "1"
    & cargo test
    if ($LASTEXITCODE -ne 0) { $code = $LASTEXITCODE; throw "cargo test exited $code" }
} catch {
    if ($code -eq 0) { $code = 1 }
    Write-Host $_.Exception.Message
} finally {
    Set-Location $origin
}
exit $code
````

## File: scripts/isolation-cli.ps1
````powershell
param(
    [Parameter(Mandatory)][ValidateSet('test-path', 'test-delete', 'check-source', 'write-source', 'descendants')][string]$Action,
    [string]$Candidate = '',
    [string]$Root = '',
    [string]$Repo = '',
    [string]$Exe = '',
    [string]$BuiltExe = '',
    [string]$Config = '',
    [string]$Command = '',
    [string]$Kind = '',
    [string]$Expected = '',
    [string]$TestRoot = '',
    [string]$Table = '',
    [int]$ProcessId = 0
)
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'isolation-lib.ps1')
try {
    switch ($Action) {
        'test-path' {
            if (Test-PathUnderRoot $Candidate $Root) { Write-Output 'OK'; exit 0 }
            Write-Output 'REJECT'
            exit 1
        }
        'test-delete' {
            Assert-DeletableUnderRoot $Candidate $Root
            Write-Output 'OK'
            exit 0
        }
        'write-source' {
            $dest = Write-IsolationSource -Repo $Repo -BuiltExe $BuiltExe -ConfigPath $Config -Command $Command -DataDirKind $Kind -ExpectedDataDir $Expected -TestRoot $TestRoot
            Write-Output $dest
            exit 0
        }
        'check-source' {
            [void](Test-IsolationSource -Repo $Repo -ExePath $Exe)
            Write-Output 'OK'
            exit 0
        }
        'descendants' {
            $items = Get-Content -LiteralPath $Table -Raw | ConvertFrom-Json
            $ids = @(Get-DescendantIds $items $ProcessId)
            if ($ids.Count -eq 0) { Write-Output '' } else { $ids | ForEach-Object { Write-Output $_ } }
            exit 0
        }
    }
} catch {
    Write-Output $_.Exception.Message
    exit 1
}
````

## File: scripts/isolation-lib.ps1
````powershell
# Dot-source this file. It does not take parameters.
# Build provenance and path checks for isolated desktop runs.

$script:IsolationNativeReady = $false

function Initialize-IsolationNativePath {
    if ($script:IsolationNativeReady) { return }
    if (-not ('MusicMineNativePath' -as [type])) {
        Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
using System.Text;
public static class MusicMineNativePath {
    [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
    public static extern IntPtr CreateFile(string name, uint access, uint share, IntPtr security, uint disposition, uint flags, IntPtr templateFile);
    [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
    public static extern uint GetFinalPathNameByHandle(IntPtr handle, StringBuilder path, uint capacity, uint flags);
    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern bool CloseHandle(IntPtr handle);
}
"@
    }
    $script:IsolationNativeReady = $true
}

function ConvertFrom-ExtendedPath([string]$Path) {
    if ($Path.StartsWith('\\?\UNC\')) { return '\' + $Path.Substring(7) }
    if ($Path.StartsWith('\\?\')) { return $Path.Substring(4) }
    return $Path
}

function Get-FinalExistingPath([string]$Path) {
    Initialize-IsolationNativePath
    $full = [System.IO.Path]::GetFullPath($Path)
    $invalid = New-Object IntPtr -1
    $handle = [MusicMineNativePath]::CreateFile($full, 0, 7, [IntPtr]::Zero, 3, 0x02000000, [IntPtr]::Zero)
    if ($handle -eq $invalid -or $handle -eq [IntPtr]::Zero) {
        throw "cannot open path: $full"
    }
    try {
        $buffer = New-Object System.Text.StringBuilder 2048
        $count = [MusicMineNativePath]::GetFinalPathNameByHandle($handle, $buffer, [uint32]$buffer.Capacity, 0)
        if ($count -eq 0 -or $count -ge $buffer.Capacity) { throw "cannot resolve final path: $full" }
        return (ConvertFrom-ExtendedPath $buffer.ToString()).TrimEnd('\')
    } finally {
        [void][MusicMineNativePath]::CloseHandle($handle)
    }
}

function Get-FinalPath([string]$Path) {
    if ([string]::IsNullOrWhiteSpace($Path)) { throw "empty path" }
    $full = [System.IO.Path]::GetFullPath($Path)
    if (Test-Path -LiteralPath $full) { return (Get-FinalExistingPath $full) }
    $missing = New-Object System.Collections.Generic.List[string]
    $current = $full
    while ($current -and -not (Test-Path -LiteralPath $current)) {
        $name = [System.IO.Path]::GetFileName($current)
        if ($name -eq '.' -or $name -eq '..') { throw "unresolved segment in $Path" }
        $missing.Insert(0, $name)
        $parent = [System.IO.Path]::GetDirectoryName($current)
        if ([string]::IsNullOrEmpty($parent) -or $parent -eq $current) { throw "path does not exist: $Path" }
        $current = $parent
    }
    $base = Get-FinalExistingPath $current
    foreach ($segment in $missing) {
        if ($segment -eq '.' -or $segment -eq '..' -or $segment.IndexOfAny([char[]]@('\', '/')) -ge 0) {
            throw "unsafe path segment: $segment"
        }
        $base = [System.IO.Path]::GetFullPath((Join-Path $base $segment))
    }
    return $base.TrimEnd('\')
}

function Test-PathUnderRoot([string]$Candidate, [string]$Root) {
    $rootFinal = (Get-FinalPath $Root).TrimEnd('\')
    $candidateFinal = (Get-FinalPath $Candidate).TrimEnd('\')
    if ($candidateFinal.Equals($rootFinal, [System.StringComparison]::OrdinalIgnoreCase)) { return $true }
    $prefix = $rootFinal + '\'
    return $candidateFinal.StartsWith($prefix, [System.StringComparison]::OrdinalIgnoreCase)
}

function Assert-DeletableUnderRoot([string]$Directory, [string]$Root) {
    if (-not (Test-PathUnderRoot $Directory $Root)) { throw "path escapes test root: $Directory" }
    $pending = New-Object System.Collections.Generic.Queue[string]
    $pending.Enqueue((Get-FinalPath $Directory))
    while ($pending.Count -gt 0) {
        $dir = $pending.Dequeue()
        foreach ($child in @(Get-ChildItem -LiteralPath $dir -Force -ErrorAction Stop)) {
            $reparse = [IO.FileAttributes]::ReparsePoint
            if (($child.Attributes -band $reparse) -eq $reparse) { throw "refusing to delete through a reparse point: $($child.FullName)" }
            if (-not (Test-PathUnderRoot $child.FullName $Root)) { throw "path escapes test root: $($child.FullName)" }
            if ($child.PSIsContainer) { $pending.Enqueue($child.FullName) }
        }
    }
}

function Get-Sha256Hex([string]$Path) {
    $sha = [System.Security.Cryptography.SHA256]::Create()
    $stream = [System.IO.File]::Open($Path, [System.IO.FileMode]::Open, [System.IO.FileAccess]::Read, [System.IO.FileShare]::ReadWrite)
    try {
        $hash = $sha.ComputeHash($stream)
        return ([System.BitConverter]::ToString($hash)).Replace('-', '')
    } finally {
        $stream.Dispose()
        $sha.Dispose()
    }
}

function Test-PortableRelativeOverride([string]$Value) {
    if ([string]::IsNullOrWhiteSpace($Value)) { return $false }
    if ([System.IO.Path]::IsPathRooted($Value)) { return $false }
    if ($Value -match '^[A-Za-z]:' -or $Value.StartsWith('\\') -or $Value.StartsWith('/')) { return $false }
    $normal = ($Value -replace '\\', '/').Trim()
    while ($normal.StartsWith('./')) { $normal = $normal.Substring(2) }
    return $normal -eq 'app-data'
}

function Test-AllowlistedBuildPath([string]$Path, [string]$Repo) {
    $repoFull = [System.IO.Path]::GetFullPath($Repo)
    foreach ($relative in @('src-tauri\target', 'dist-portable')) {
        $root = Join-Path $repoFull $relative
        if (Test-Path -LiteralPath $root) {
            if (Test-PathUnderRoot $Path $root) { return $true }
        }
    }
    return $false
}

function Get-IsolationSourceDirectory([string]$Repo) {
    return (Join-Path ([System.IO.Path]::GetFullPath($Repo)) 'tests\perf\out\isolation-sources')
}

function Resolve-RecordedConfigPath([string]$Repo, [string]$ConfigPath) {
    if ([System.IO.Path]::IsPathRooted($ConfigPath)) { return [System.IO.Path]::GetFullPath($ConfigPath) }
    return [System.IO.Path]::GetFullPath((Join-Path $Repo ($ConfigPath -replace '/', '\')))
}

function Read-IsolationConfig([string]$Path) {
    $parsed = Get-Content -LiteralPath $Path -Raw | ConvertFrom-Json
    $top = @($parsed.PSObject.Properties.Name)
    if ($top.Count -ne 1 -or $top[0] -ne 'app') { throw "config may only contain app.appDirectoriesOverride" }
    $app = @($parsed.app.PSObject.Properties.Name)
    if ($app.Count -ne 1 -or $app[0] -ne 'appDirectoriesOverride') { throw "config may only contain app.appDirectoriesOverride" }
    return [string]$parsed.app.appDirectoriesOverride
}

function Write-IsolationSource {
    param(
        [Parameter(Mandatory)][string]$Repo,
        [Parameter(Mandatory)][string]$BuiltExe,
        [Parameter(Mandatory)][string]$ConfigPath,
        [Parameter(Mandatory)][string]$Command,
        [Parameter(Mandatory)][ValidateSet('portable-relative', 'absolute-override')][string]$DataDirKind,
        [Parameter(Mandatory)][string]$ExpectedDataDir,
        [string]$TestRoot = ''
    )
    if (-not (Test-AllowlistedBuildPath $BuiltExe $Repo)) {
        throw "build output is not under src-tauri\target or dist-portable: $BuiltExe"
    }
    $builtFull = Get-FinalPath $BuiltExe
    $configFull = Resolve-RecordedConfigPath $Repo $ConfigPath
    if (-not (Test-Path -LiteralPath $configFull)) { throw "config not found: $configFull" }
    $configFull = Get-FinalPath $configFull
    $override = Read-IsolationConfig $configFull
    if ($DataDirKind -eq 'portable-relative') {
        $portable = Get-FinalPath (Join-Path $Repo 'src-tauri\tauri.portable.conf.json')
        if (-not $configFull.Equals($portable, [System.StringComparison]::OrdinalIgnoreCase)) {
            throw "portable-relative records must use src-tauri/tauri.portable.conf.json"
        }
        if (-not (Test-PortableRelativeOverride $override)) { throw "portable override must be a relative app-data path" }
        if ($ExpectedDataDir -ne './app-data') { throw "portable expectedDataDir must be ./app-data" }
    } else {
        if (-not [System.IO.Path]::IsPathRooted($override)) { throw "absolute-override config must use an absolute path" }
        if ([string]::IsNullOrWhiteSpace($TestRoot)) { throw "absolute-override requires a test root" }
        $rootFinal = Get-FinalPath $TestRoot
        if ($rootFinal.Length -le 3) { throw "test root must be a dedicated directory, not a drive root" }
        $overrideFinal = Get-FinalPath $override
        $expectedFinal = Get-FinalPath $ExpectedDataDir
        if (-not $overrideFinal.Equals($expectedFinal, [System.StringComparison]::OrdinalIgnoreCase)) {
            throw "config override does not match expectedDataDir"
        }
        if (-not (Test-PathUnderRoot $expectedFinal $rootFinal)) { throw "expected data directory escapes the test root" }
    }
    $hash = Get-Sha256Hex $builtFull
    $record = [ordered]@{
        exeSha256 = $hash
        builtExePath = $builtFull
        configPath = $ConfigPath
        configSha256 = (Get-Sha256Hex $configFull)
        command = $Command
        dataDirKind = $DataDirKind
        expectedDataDir = $(if ($DataDirKind -eq 'portable-relative') { './app-data' } else { (Get-FinalPath $ExpectedDataDir) })
        testRoot = $(if ($DataDirKind -eq 'absolute-override') { (Get-FinalPath $TestRoot) } else { '' })
    }
    $dir = Get-IsolationSourceDirectory $Repo
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    $dest = Join-Path $dir ($hash + '.json')
    $json = $record | ConvertTo-Json -Compress
    [System.IO.File]::WriteAllText($dest, $json)
    return $dest
}

function Read-IsolationSource([string]$Repo, [string]$ExePath) {
    $hash = Get-Sha256Hex $ExePath
    $path = Join-Path (Get-IsolationSourceDirectory $Repo) ($hash + '.json')
    if (-not (Test-Path -LiteralPath $path)) { throw "MISSING_RECORD $hash" }
    $record = Get-Content -LiteralPath $path -Raw | ConvertFrom-Json
    return $record
}

function Test-IsolationSource {
    param(
        [Parameter(Mandatory)][string]$Repo,
        [Parameter(Mandatory)][string]$ExePath
    )
    $record = Read-IsolationSource $Repo $ExePath
    foreach ($field in @('exeSha256', 'builtExePath', 'configPath', 'configSha256', 'command', 'dataDirKind', 'expectedDataDir')) {
        if ([string]::IsNullOrWhiteSpace([string]$record.$field)) { throw "record missing $field" }
    }
    $launchedHash = Get-Sha256Hex $ExePath
    if ($launchedHash -ne ([string]$record.exeSha256).ToUpperInvariant()) { throw "HASH_MISMATCH" }
    if (-not (Test-AllowlistedBuildPath ([string]$record.builtExePath) $Repo)) { throw "BUILT_PATH_NOT_ALLOWLISTED" }
    if ((Get-Sha256Hex ([string]$record.builtExePath)) -ne $launchedHash) { throw "BUILT_FILE_HASH_MISMATCH" }
    $configFull = Resolve-RecordedConfigPath $Repo ([string]$record.configPath)
    if (-not (Test-Path -LiteralPath $configFull)) { throw "config not found: $configFull" }
    $configFull = Get-FinalPath $configFull
    if ((Get-Sha256Hex $configFull) -ne ([string]$record.configSha256).ToUpperInvariant()) { throw "CONFIG_HASH_MISMATCH" }
    $mainConfig = Get-FinalPath (Join-Path $Repo 'src-tauri\tauri.conf.json')
    if ($configFull.Equals($mainConfig, [System.StringComparison]::OrdinalIgnoreCase)) { throw "REFUSING_MAIN_CONFIG" }
    $override = Read-IsolationConfig $configFull
    if ([string]$record.dataDirKind -eq 'portable-relative') {
        $portable = Get-FinalPath (Join-Path $Repo 'src-tauri\tauri.portable.conf.json')
        if (-not $configFull.Equals($portable, [System.StringComparison]::OrdinalIgnoreCase)) { throw "PORTABLE_CONFIG_PATH" }
        if (-not (Test-PortableRelativeOverride $override)) { throw "PORTABLE_OVERRIDE_NOT_RELATIVE" }
        if ([string]$record.expectedDataDir -ne './app-data') { throw "PORTABLE_EXPECTED_DIR" }
    } elseif ([string]$record.dataDirKind -eq 'absolute-override') {
        if (-not [System.IO.Path]::IsPathRooted($override)) { throw "ABSOLUTE_OVERRIDE_REQUIRED" }
        if ([string]::IsNullOrWhiteSpace([string]$record.testRoot)) { throw "TEST_ROOT_MISSING" }
        $overrideFinal = Get-FinalPath $override
        $expectedFinal = Get-FinalPath ([string]$record.expectedDataDir)
        $rootFinal = Get-FinalPath ([string]$record.testRoot)
        if (-not $overrideFinal.Equals($expectedFinal, [System.StringComparison]::OrdinalIgnoreCase)) { throw "OVERRIDE_EXPECTED_MISMATCH" }
        if (-not (Test-PathUnderRoot $expectedFinal $rootFinal)) { throw "DATA_DIR_ESCAPES_TEST_ROOT" }
    } else {
        throw "UNKNOWN_DATA_DIR_KIND"
    }
    return $record
}

function Assert-PortableLaunchSource {
    param([Parameter(Mandatory)][string]$Repo, [Parameter(Mandatory)][string]$ExePath)
    $record = Test-IsolationSource -Repo $Repo -ExePath $ExePath
    if ([string]$record.dataDirKind -ne 'portable-relative') { throw "stage-cover-run only launches portable-relative builds" }
    return $record
}

function ConvertTo-StartToken($CreationDate) {
    $value = $CreationDate
    if ($value -isnot [datetime]) {
        $value = [System.Management.ManagementDateTimeConverter]::ToDateTime([string]$CreationDate)
    }
    return $value.ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ')
}

function Get-DescendantIds($Items, [int]$RootPid) {
    $children = @{}
    foreach ($item in @($Items)) {
        $parent = [int]$item.ParentProcessId
        if (-not $children.ContainsKey($parent)) { $children[$parent] = New-Object System.Collections.Generic.List[int] }
        $children[$parent].Add([int]$item.ProcessId)
    }
    $found = New-Object System.Collections.Generic.List[int]
    $queue = New-Object System.Collections.Generic.Queue[int]
    $seen = @{}
    $queue.Enqueue($RootPid)
    $seen[$RootPid] = $true
    while ($queue.Count -gt 0) {
        $id = $queue.Dequeue()
        if (-not $children.ContainsKey($id)) { continue }
        foreach ($child in @($children[$id])) {
            if ($seen.ContainsKey($child)) { continue }
            $seen[$child] = $true
            $found.Add($child)
            $queue.Enqueue($child)
        }
    }
    return @($found)
}

function Test-AncestorOfProcess($Items, [int]$ProcessId, [int]$RootPid) {
    $byId = @{}
    foreach ($item in @($Items)) { $byId[[int]$item.ProcessId] = $item }
    $current = $ProcessId
    for ($guard = 0; $guard -lt 50; $guard++) {
        if (-not $byId.ContainsKey($current)) { return $false }
        $parent = [int]$byId[$current].ParentProcessId
        if ($parent -eq $RootPid) { return $true }
        if ($parent -eq 0 -or $parent -eq $current) { return $false }
        $current = $parent
    }
    return $false
}

function Get-LiveProcessTable {
    return @(Get-CimInstance Win32_Process | Select-Object ProcessId, ParentProcessId, ExecutablePath, CreationDate)
}

function Test-RecordedProcessStillMatches($Record, $Process) {
    if (-not $Process -or -not $Process.ExecutablePath -or -not $Process.CreationDate) { return $false }
    $start = ConvertTo-StartToken $Process.CreationDate
    if ($start -ne [string]$Record.startTime) { return $false }
    $actual = Get-FinalPath ([string]$Process.ExecutablePath)
    $expected = Get-FinalPath ([string]$Record.exePath)
    return $actual.Equals($expected, [System.StringComparison]::OrdinalIgnoreCase)
}

function Stop-RecordedPlayer {
    param([Parameter(Mandatory)][string]$RecordPath)
    if (-not (Test-Path -LiteralPath $RecordPath)) { return }
    $record = Get-Content -LiteralPath $RecordPath -Raw | ConvertFrom-Json
    $processId = [int]$record.pid
    if ($processId -le 0) { throw "process record has no pid" }
    $live = Get-CimInstance Win32_Process -Filter "ProcessId=$processId"
    if (-not $live) { return }
    if (-not (Test-RecordedProcessStillMatches $record $live)) {
        Write-Output "SKIP_IDENTITY"
        return
    }
    $main = Get-Process -Id $processId -ErrorAction SilentlyContinue
    if ($main) { [void]$main.CloseMainWindow() }
    $deadline = (Get-Date).AddSeconds(5)
    do {
        Start-Sleep -Milliseconds 200
        $live = Get-CimInstance Win32_Process -Filter "ProcessId=$processId"
        if (-not $live) { return }
        if (-not (Test-RecordedProcessStillMatches $record $live)) {
            Write-Output "SKIP_IDENTITY"
            return
        }
    } while ((Get-Date) -lt $deadline -and $live)

    $live = Get-CimInstance Win32_Process -Filter "ProcessId=$processId"
    if (-not $live -or -not (Test-RecordedProcessStillMatches $record $live)) {
        Write-Output "SKIP_IDENTITY"
        return
    }
    $table = Get-LiveProcessTable
    $descendants = @(Get-DescendantIds $table $processId)
    [array]::Reverse($descendants)
    foreach ($childId in $descendants) {
        $table = Get-LiveProcessTable
        $root = $null
        foreach ($item in $table) { if ([int]$item.ProcessId -eq $processId) { $root = $item } }
        if (-not (Test-RecordedProcessStillMatches $record $root)) {
            Write-Output "SKIP_IDENTITY"
            return
        }
        if (Test-AncestorOfProcess $table $childId $processId) {
            Stop-Process -Id $childId -Force -ErrorAction SilentlyContinue
        }
    }
    $table = Get-LiveProcessTable
    $root = $null
    foreach ($item in $table) { if ([int]$item.ProcessId -eq $processId) { $root = $item } }
    if ($root -and (Test-RecordedProcessStillMatches $record $root)) {
        Stop-Process -Id $processId -Force -ErrorAction SilentlyContinue
    }
}

function Write-RunProcessRecord {
    param(
        [Parameter(Mandatory)][string]$RecordPath,
        [Parameter(Mandatory)][int]$ProcessId,
        [Parameter(Mandatory)][string]$ExePath
    )
    $live = Get-CimInstance Win32_Process -Filter "ProcessId=$ProcessId"
    if (-not $live -or -not $live.ExecutablePath) {
        $started = Get-Process -Id $ProcessId -ErrorAction SilentlyContinue
        if ($started -and $started.Path -and (Get-FinalPath $started.Path).Equals((Get-FinalPath $ExePath), [System.StringComparison]::OrdinalIgnoreCase)) {
            Stop-Process -Id $ProcessId -Force -ErrorAction SilentlyContinue
        }
        throw "cannot record process identity for pid $ProcessId"
    }
    $actual = Get-FinalPath ([string]$live.ExecutablePath)
    $expected = Get-FinalPath $ExePath
    if (-not $actual.Equals($expected, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "refusing to record pid $ProcessId; executable does not match $expected"
    }
    $payload = [ordered]@{
        pid = $ProcessId
        startTime = (ConvertTo-StartToken $live.CreationDate)
        exePath = $actual
    }
    $json = $payload | ConvertTo-Json -Compress
    [System.IO.File]::WriteAllText($RecordPath, $json)
}
````

## File: scripts/worktree-fingerprint.ps1
````powershell
param(
    [string]$Repo = "",
    [string[]]$Exclude = @()
)
$ErrorActionPreference = "Stop"
if (-not $Repo) { $Repo = (Get-Location).Path }
$Repo = (Resolve-Path -LiteralPath $Repo).Path

function Get-GitText([string[]]$GitArgs) {
    $lines = & git -C $Repo -c color.ui=false -c color.diff=false -c core.quotepath=false -c i18n.logOutputEncoding=utf-8 @GitArgs
    if ($LASTEXITCODE -ne 0) { throw "git failed: $($GitArgs -join ' ')" }
    if ($null -eq $lines) { return "" }
    if ($lines -is [string]) { return $lines + "`n" }
    return (($lines | ForEach-Object { $_ }) -join "`n") + "`n"
}

$buffer = New-Object System.IO.MemoryStream
function Add-Bytes([byte[]]$Bytes) {
    if ($null -eq $Bytes -or $Bytes.Length -eq 0) { return }
    $buffer.Write($Bytes, 0, $Bytes.Length)
}
function Add-Text([string]$Text) {
    Add-Bytes ([System.Text.Encoding]::UTF8.GetBytes($Text))
}

$skip = @{}
foreach ($item in @($Exclude | ForEach-Object { $_ -split ',' })) {
    if ([string]::IsNullOrWhiteSpace($item)) { continue }
    $key = ($item -replace '\\', '/').Trim()
    while ($key.StartsWith('./')) { $key = $key.Substring(2) }
    $key = $key.TrimStart('/')
    $skip[$key] = $true
}

Add-Text "music-mine-worktree-v1`n"
$diffPaths = @("--", ".") + @($skip.Keys | Sort-Object | ForEach-Object { ":(top,literal,exclude)$_" })
Add-Text "staged`n"
Add-Text (Get-GitText (@("diff", "--cached") + $diffPaths))
Add-Text "unstaged`n"
Add-Text (Get-GitText (@("diff") + $diffPaths))
Add-Text "untracked`n"

$listed = & git -C $Repo -c core.quotepath=false ls-files --others --exclude-standard
if ($LASTEXITCODE -ne 0) { throw "git ls-files failed" }
$paths = @()
if ($null -ne $listed) {
    if ($listed -is [string]) { $paths = @($listed) }
    else { $paths = @($listed) }
}
$paths = @($paths | ForEach-Object { ($_ -replace '\\', '/') } | Where-Object { $_ -and -not $skip.ContainsKey($_) } | Sort-Object { $_ })
foreach ($relative in $paths) {
    $full = Join-Path $Repo ($relative -replace '/', '\')
    if (-not (Test-Path -LiteralPath $full -PathType Leaf)) { continue }
    $bytes = [System.IO.File]::ReadAllBytes($full)
    Add-Text ($relative + "`n" + $bytes.Length + "`n")
    Add-Bytes $bytes
    Add-Text "`n"
}

$buffer.Position = 0
$sha = [System.Security.Cryptography.SHA256]::Create()
try {
    $hash = $sha.ComputeHash($buffer)
} finally {
    $sha.Dispose()
    $buffer.Dispose()
}
Write-Output (([System.BitConverter]::ToString($hash)).Replace('-', '').ToLowerInvariant())
````

## File: src/lib/components/Artwork.svelte
````svelte
<script lang="ts">
  import { diagnostics } from '../utils/cover-diagnostics';
  import { Disc3, MicVocal, ListMusic } from 'lucide-svelte';
  let {
    src = null,
    alt = '专辑封面',
    kind = 'album',
  }: { src?: string | null; alt?: string; kind?: 'album' | 'artist' | 'playlist' } = $props();
  let failed = $state(false);
  let loaded = $state(false);
  let previous: string | null = null;
  $effect(() => {
    const current = src;
    if (current !== previous) {
      previous = current;
      failed = false;
      loaded = false;
      if (diagnostics) diagnostics.urlChanges++;
    }
  });
  function mounted(_node: HTMLImageElement) {
    if (diagnostics) diagnostics.mounts++;
  }
</script>

<div
  class="artwork"
  class:artwork-artist={kind === 'artist'}
  data-image={!src ? 'empty' : failed ? 'error' : loaded ? 'loaded' : 'loading'}
>
  {#if src && !failed}<img
      use:mounted
      class:loaded
      onload={() => {
        loaded = true;
        if (diagnostics) diagnostics.loads++;
      }}
      {src}
      {alt}
      decoding="async"
      draggable="false"
      onerror={() => {
        failed = true;
        if (diagnostics) diagnostics.errors++;
      }}
    />
  {:else}<span
      class="artwork-placeholder"
      role="img"
      aria-label={kind === 'artist'
        ? '暂无艺术家图片'
        : kind === 'playlist'
          ? '歌单'
          : '暂无专辑封面'}
      >{#if kind === 'artist'}<MicVocal />{:else if kind === 'playlist'}<ListMusic />{:else}<Disc3
        />{/if}</span
    >{/if}
</div>
````

## File: src/lib/components/ContextMenu.svelte
````svelte
<script lang="ts">
  import { DropdownMenu } from 'bits-ui';
  import { Check } from 'lucide-svelte';
  import { ui, closeMenu, notifyError, type MenuItem } from '../stores/ui.svelte';
  let menu = $derived(ui.menu);
  let anchor = $state<HTMLSpanElement | null>(null);
  let panel = $state<HTMLDivElement | null>(null);
  // Preserve the imperative point/rectangle API; Floating UI owns positioning and collisions.
  const rect = $derived(
    menu && ('width' in menu.at ? menu.at : { ...menu.at, width: 0, height: 0 }),
  );
  const portal = $derived(menu?.returnFocus?.closest('dialog') ?? undefined);
  async function run(item: Extract<MenuItem, { label: string; action: () => unknown }>) {
    closeMenu(true);
    try {
      await item.action();
    } catch (e) {
      notifyError(e);
    }
  }
</script>

{#if menu && rect}
  {@const current = menu}
  {#key menu.id}
    <DropdownMenu.Root
      open
      onOpenChange={(open) => {
        if (!open && ui.menu === current) closeMenu();
      }}
    >
      <DropdownMenu.Portal to={portal}>
        <span
          bind:this={anchor}
          aria-hidden="true"
          style:position="fixed"
          style:left={rect.x + 'px'}
          style:top={rect.y + 'px'}
          style:width={rect.width + 'px'}
          style:height={rect.height + 'px'}
          style:pointer-events="none"
        ></span>
        <DropdownMenu.Content
          bind:ref={panel}
          class="menu"
          aria-label={menu.label}
          customAnchor={anchor}
          align={'width' in menu.at ? 'end' : 'start'}
          sideOffset={'width' in menu.at ? 4 : 0}
          collisionPadding={8}
          preventScroll={false}
          onOpenAutoFocus={(e) => {
            e.preventDefault();
            panel?.querySelector<HTMLElement>('.menu-item:not(:disabled)')?.focus();
          }}
          onCloseAutoFocus={(e) => {
            e.preventDefault();
          }}
          onEscapeKeydown={() => closeMenu(true)}
          onkeydowncapture={(e) => {
            if (e.isComposing) e.stopImmediatePropagation();
            else if (e.key === 'Tab') {
              e.preventDefault();
              closeMenu(true);
            }
          }}
          oncontextmenu={(e) => e.preventDefault()}
        >
          {#each menu.items as item, i (i)}
            {#if item.kind === 'separator'}<DropdownMenu.Separator class="menu-separator" />
            {:else if item.kind === 'heading'}<div class="menu-heading" role="presentation">
                {item.label}
              </div>
            {:else}
              <DropdownMenu.Item
                disabled={item.disabled}
                textValue={item.label}
                onSelect={() => run(item)}
              >
                {#snippet child({ props })}
                  <button
                    {...props}
                    type="button"
                    class={['menu-item', item.danger && 'danger']}
                    disabled={item.disabled}
                  >
                    <span class="menu-icon" aria-hidden="true"
                      >{#if item.icon}<item.icon size={16} />{/if}</span
                    >
                    <span class="menu-label">{item.label}</span>
                    {#if item.checked}<Check
                        size={15}
                        class="menu-check"
                        aria-label="已选"
                      />{:else if item.hint}<kbd>{item.hint}</kbd>{/if}
                  </button>
                {/snippet}
              </DropdownMenu.Item>
            {/if}
          {/each}
        </DropdownMenu.Content>
      </DropdownMenu.Portal>
    </DropdownMenu.Root>
  {/key}
{/if}
````

## File: src/lib/components/Cover.svelte
````svelte
<script lang="ts">
  import { untrack } from 'svelte';
  import Artwork from './Artwork.svelte';
  import {
    desktopAvailable,
    peekTrackThumb,
    requestTrackThumb,
    type CoverStatus,
    type CoverTicket,
  } from '../services/music';
  import { library } from '../stores/library.svelte';
  let {
    id,
    alt,
    kind = 'album',
    size = 112,
  }: { id: number; alt: string; kind?: 'album' | 'artist'; size?: number } = $props();
  let host = $state<HTMLDivElement>();
  let shown = $state<{ key: string; src: string | null; status: CoverStatus | 'loading' } | null>(
    null,
  );
  $effect(() => {
    const requested = id,
      edge = size,
      node = host,
      key = requested + '@' + edge;
    void library.coverEpoch;
    if (!desktopAvailable || !node) return;
    // Another cover in this slot drops the old picture; a rescan keeps it until the new result.
    if (untrack(() => shown?.key !== key)) shown = { key, src: null, status: 'loading' };
    const hit = peekTrackThumb(requested, edge);
    if (hit && hit.status !== 'failed') {
      shown = { key, src: hit.src, status: hit.status };
      return;
    }
    let ticket: CoverTicket | null = null,
      finished = false;
    const done = (o: { status: CoverStatus; src: string | null; code?: string }) => {
      ticket = null;
      if (o.code === 'dropped') return;
      finished = true;
      shown = { key, src: o.src, status: o.status };
    };
    const observer = new IntersectionObserver(
      (entries) => {
        const visible = entries.some((e) => e.isIntersecting);
        if (finished) return;
        if (!visible) {
          ticket?.cancel();
          ticket = null;
        } else if (!ticket) {
          const next = requestTrackThumb(requested, edge, 1, done);
          if (!finished) ticket = next;
        }
      },
      { rootMargin: '240px' },
    );
    observer.observe(node);
    return () => {
      observer.disconnect();
      ticket?.cancel();
    };
  });
</script>

<div class="cover-fill" bind:this={host} data-cover={shown?.status ?? 'loading'}>
  <Artwork src={shown?.src ?? null} {alt} {kind} />
</div>
````

## File: src/lib/components/Dialog.svelte
````svelte
<script lang="ts">
  import { tick } from 'svelte';
  import { TriangleAlert } from 'lucide-svelte';
  import Modal from './Modal.svelte';
  import { ui, type DialogState } from '../stores/ui.svelte';
  import { errorText } from '../utils/format';

  let input = $state<HTMLInputElement>();
  let value = $state('');
  let error = $state('');
  let busy = $state(false);
  let touched = $state(false);
  let active = $derived(ui.dialog);

  $effect.pre(() => {
    const current = active;
    if (current) {
      value = current.kind === 'prompt' ? current.value : '';
      error = '';
      busy = false;
      touched = false;
    }
  });

  // Cancelling (button, backdrop or Escape) only resolves false; it never runs the submit action.
  function finish(current: DialogState, ok: boolean) {
    if (ui.dialog !== current || busy) return;
    ui.dialog = null;
    current.resolve(ok);
  }

  let validation = $derived(active?.kind === 'prompt' ? active.validate(value) : null);

  async function submit(e: SubmitEvent, current: DialogState) {
    e.preventDefault();
    if (busy) return;
    if (current.kind === 'confirm') {
      finish(current, true);
      return;
    }
    touched = true;
    if (validation) {
      error = validation;
      return;
    }
    busy = true;
    error = '';
    try {
      await current.submit(value);
      busy = false;
      finish(current, true);
    } catch (err) {
      busy = false;
      error = errorText(err);
      void tick().then(() => input?.focus());
    }
  }
</script>

{#if active}
  {@const current = active}
  {@const danger = current.kind === 'confirm' && current.danger}
  {#key current}
    <Modal
      onclose={() => finish(current, false)}
      dismissible={!busy}
      initialFocus={current.kind === 'prompt' ? '#app-dialog-input' : '[data-autofocus]'}
      aria-labelledby="app-dialog-title"
      aria-describedby={current.kind === 'confirm' ? 'app-dialog-message' : undefined}
    >
      <form onsubmit={(e) => submit(e, current)} novalidate>
        <header class="dialog-header">
          {#if danger}<span class="dialog-badge danger" aria-hidden="true"
              ><TriangleAlert size={18} /></span
            >{/if}
          <h2 id="app-dialog-title">{current.title}</h2>
        </header>
        <div class="dialog-body">
          {#if current.kind === 'confirm'}
            <p id="app-dialog-message">{current.message}</p>
            {#if current.detail}<p class="dialog-detail">{current.detail}</p>{/if}
          {:else}
            <label class="field-label" for="app-dialog-input">{current.label}</label>
            <input
              id="app-dialog-input"
              class="input"
              bind:this={input}
              bind:value
              maxlength="100"
              autocomplete="off"
              spellcheck="false"
              aria-invalid={!!(error || (touched && validation))}
              aria-describedby="app-dialog-error"
              oninput={() => {
                touched = true;
                error = '';
              }}
            />
            <p id="app-dialog-error" class="field-error" role="alert">
              {error || (touched ? (validation ?? '') : '')}
            </p>
          {/if}
        </div>
        <footer class="dialog-actions">
          <button
            type="button"
            class="btn btn-ghost"
            onclick={() => finish(current, false)}
            disabled={busy}
            data-autofocus={danger ? '' : undefined}>取消</button
          >
          <button
            type="submit"
            class="btn"
            class:btn-danger={danger}
            class:btn-primary={!danger}
            data-autofocus={current.kind === 'confirm' && !danger ? '' : undefined}
            disabled={busy || (current.kind === 'prompt' && touched && !!validation)}
          >
            {busy ? '正在保存…' : current.confirmLabel}
          </button>
        </footer>
      </form>
    </Modal>
  {/key}
{/if}
````

## File: src/lib/components/EmptyState.svelte
````svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { IconComponent } from '../stores/ui.svelte';
  let {
    icon: Icon,
    title,
    message = '',
    children,
  }: { icon?: IconComponent; title: string; message?: string; children?: Snippet } = $props();
</script>

<div class="empty-state">
  {#if Icon}<span class="empty-icon" aria-hidden="true"><Icon size={26} /></span>{/if}
  <h2>{title}</h2>
  {#if message}<p>{message}</p>{/if}
  {#if children}<div class="empty-actions">{@render children()}</div>{/if}
</div>

<style>
  :global {
    /* ============================================================ empty state */
    .empty-state {
      display: flex;
      flex-direction: column;
      align-items: center;
      text-align: center;
      gap: 8px;
      padding: 56px 24px;
      color: var(--text-2);
    }

    .empty-icon {
      display: grid;
      place-items: center;
      width: 56px;
      height: 56px;
      margin-bottom: 6px;
      border-radius: 50%;
      background: var(--surface);
      color: var(--text-3);
    }

    .empty-state h2 {
      color: var(--text);
      font-size: var(--fs-section);
      font-weight: var(--fw-semibold);
    }

    .empty-state p {
      max-width: 420px;
      font-size: var(--fs-meta);
      line-height: 1.6;
    }

    .empty-actions {
      display: flex;
      gap: 8px;
      margin-top: 8px;
    }
  }
</style>
````

## File: src/lib/components/IconButton.svelte
````svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';
  import { tooltip } from './tooltip';
  let {
    label,
    active = false,
    size = 'md',
    children,
    element = $bindable(),
    class: className = '',
    ...rest
  }: HTMLButtonAttributes & {
    label: string;
    active?: boolean;
    size?: 'sm' | 'md' | 'lg';
    children: Snippet;
    element?: HTMLButtonElement;
  } = $props();
</script>

<button
  bind:this={element}
  type="button"
  class={['icon-btn', 'icon-btn-' + size, className]}
  class:active
  aria-label={label}
  use:tooltip={label}
  {...rest}>{@render children()}</button
>
````

## File: src/lib/components/Modal.svelte
````svelte
<script lang="ts">
  import { Dialog } from 'bits-ui';
  import type { Snippet } from 'svelte';
  import type { HTMLDialogAttributes } from 'svelte/elements';
  import { createDismissal } from '../utils/modal';
  let {
    onclose,
    dismissible = true,
    initialFocus,
    class: className = '',
    children,
    ...rest
  }: Omit<HTMLDialogAttributes, 'class' | 'onclose' | 'children'> & {
    onclose: () => void;
    dismissible?: boolean;
    initialFocus?: string;
    class?: string;
    children: Snippet;
  } = $props();

  let node = $state<HTMLDialogElement | null>(null);
  const previous = document.activeElement;
  const popoverOpen = () =>
    !!document.querySelector('.menu[data-state="open"],.select-panel[data-state="open"]');
  const dismissal = $derived(
    node
      ? createDismissal(node, {
          canDismiss: () => dismissible,
          popoverOpen,
          dismiss: onclose,
        })
      : undefined,
  );
  function nativeDialog(el: HTMLDialogElement) {
    el.showModal();
    return {
      destroy() {
        if (el.open) el.close();
      },
    };
  }
</script>

<Dialog.Root
  open
  onOpenChange={(open) => {
    if (!open && dismissible) onclose();
  }}
>
  <Dialog.Content
    bind:ref={node}
    preventScroll={false}
    preventBodyPointerEvents={false}
    onOpenAutoFocus={(e) => {
      const target = initialFocus ? node?.querySelector<HTMLElement>(initialFocus) : null;
      if (target) {
        e.preventDefault();
        target.focus();
        if (target instanceof HTMLInputElement) target.select();
      }
    }}
    onCloseAutoFocus={(e) => {
      e.preventDefault();
      if (previous instanceof HTMLElement && previous.isConnected)
        previous.focus({ preventScroll: true });
    }}
    onEscapeKeydown={(e) => {
      if (!dismissible || e.isComposing || popoverOpen()) e.preventDefault();
    }}
    onInteractOutside={(e) => e.preventDefault()}
  >
    {#snippet child({ props })}
      <dialog
        {...props}
        {...rest}
        class={['dialog', className]}
        style:contain="none"
        use:nativeDialog
        onpointerdown={(e) => dismissal?.pointerdown(e)}
        onclick={(e) => dismissal?.click(e)}
        oncancel={(e) => e.preventDefault()}
      >
        {@render children()}
      </dialog>
    {/snippet}
  </Dialog.Content>
</Dialog.Root>
````

## File: src/lib/components/QueueCover.svelte
````svelte
<script lang="ts">
  import { untrack } from 'svelte';
  import Artwork from './Artwork.svelte';
  import {
    peekQueueCover,
    requestQueueCover,
    type CoverPriority,
    type CoverStatus,
    type CoverTicket,
    type Track,
  } from '../services/music';
  import { library } from '../stores/library.svelte';
  import { watchRow, type RowState } from '../utils/row-visibility';
  let { track, current = false }: { track: Track; current?: boolean } = $props();
  let host = $state<HTMLSpanElement>();
  // Depends on the path string, not the track object: snapshots rebuild the queue array.
  const path = $derived(track.path);
  let shown = $state<{
    path: string;
    src: string | null;
    status: CoverStatus | 'loading';
    code?: string;
  } | null>(null);
  let ticket: CoverTicket | null = null;
  let row: RowState = { near: false, seen: false };
  const priority = (): CoverPriority => (current ? 0 : row.seen ? 1 : 2);
  $effect(() => {
    const requested = path;
    void library.coverEpoch;
    const node = host;
    if (!node) return;
    // A different track in this row drops the old picture; a rescan keeps it until the new result.
    if (untrack(() => shown?.path !== requested))
      shown = { path: requested, src: null, status: 'loading' };
    const hit = peekQueueCover(requested);
    if (hit && hit.status !== 'failed') {
      shown = { path: requested, src: hit.src, status: hit.status };
      return;
    }
    let finished = false;
    const done = (o: { status: CoverStatus; src: string | null; code?: string }) => {
      ticket = null;
      // Dropped preloads ask again once the row is visible.
      if (o.code === 'dropped') return;
      finished = true;
      shown = { path: requested, src: o.src, status: o.status, code: o.code };
    };
    const ask = () => {
      if (finished) return;
      if (!row.near && !row.seen && !current) {
        ticket?.cancel();
        ticket = null;
      } else if (ticket) ticket.setPriority(priority());
      else {
        const next = requestQueueCover(requested, priority(), done);
        if (!finished) ticket = next;
      }
    };
    const list = node.closest('.queue-list');
    // The row box, not the span: row contents are skipped by content-visibility while off-screen.
    const target = node.closest('.queue-item') ?? node;
    row = { near: false, seen: false };
    const stop = list
      ? watchRow(target, list, '360px 0px', (state) => {
          row = state;
          ask();
        })
      : ((row = { near: true, seen: true }), untrack(ask), () => {});
    return () => {
      stop();
      ticket?.cancel();
      ticket = null;
    };
  });
  $effect(() => {
    void current;
    ticket?.setPriority(priority());
  });
</script>

<span
  bind:this={host}
  class="cover-fill"
  data-cover={shown?.status ?? 'loading'}
  data-cover-code={shown?.code}
  ><Artwork src={shown?.src ?? null} alt={track.album || track.title} /></span
>
````

## File: src/lib/components/Select.svelte
````svelte
<script lang="ts">
  import { Select } from 'bits-ui';
  import { ChevronDown, Check } from 'lucide-svelte';
  let {
    label,
    value = $bindable(''),
    options,
    disabled = false,
    onchange,
  }: {
    label: string;
    value?: string;
    options: { value: string; label: string }[];
    disabled?: boolean;
    onchange?: (value: string) => void;
  } = $props();
  let trigger = $state<HTMLButtonElement | null>(null);
  let opened = $state(false);
  const dialog = $derived(trigger?.closest('dialog') ?? undefined);
</script>

<Select.Root
  type="single"
  bind:value
  bind:open={opened}
  {disabled}
  loop
  allowDeselect={false}
  onValueChange={(next) => onchange?.(next)}
>
  <Select.Trigger
    class="select-trigger"
    aria-label={label}
    bind:ref={trigger}
    onkeydowncapture={(e) => {
      // Let the IME own composition keys before Bits UI handles list navigation.
      if (e.isComposing) e.stopImmediatePropagation();
    }}
  >
    <span>{options.find((o) => o.value === value)?.label ?? label}</span><ChevronDown size={14} />
  </Select.Trigger>
  <Select.Portal to={dialog}>
    <Select.Content
      class="select-panel"
      aria-label={label}
      align="start"
      sideOffset={4}
      collisionPadding={8}
      collisionBoundary={dialog}
      onEscapeKeydown={(e) => e.stopPropagation()}
      onFocusOutside={(e) => {
        if (e.target !== trigger) opened = false;
      }}
    >
      <Select.Viewport class="select-viewport">
        {#each options as option (option.value)}
          <Select.Item value={option.value} label={option.label}>
            {#snippet child({ props, selected })}
              <button {...props} type="button" tabindex="-1">
                <span>{option.label}</span>{#if selected}<Check size={15} />{/if}
              </button>
            {/snippet}
          </Select.Item>
        {/each}
      </Select.Viewport>
    </Select.Content>
  </Select.Portal>
</Select.Root>
````

## File: src/lib/components/Slider.svelte
````svelte
<script lang="ts">
  // Native range input (keyboard + screen reader support) with a shared visual style.
  let {
    value,
    min = 0,
    max = 100,
    step = 1,
    label,
    valuetext,
    disabled = false,
    class: className = '',
    oninput,
    onchange,
  }: {
    value: number;
    min?: number;
    max?: number;
    step?: number;
    label: string;
    valuetext?: string;
    disabled?: boolean;
    class?: string;
    oninput?: (value: number) => void;
    onchange?: (value: number) => void;
  } = $props();
  let fill = $derived(
    max > min ? Math.min(100, Math.max(0, ((value - min) / (max - min)) * 100)) : 0,
  );
</script>

<input
  class={['slider', className]}
  type="range"
  {min}
  {max}
  {step}
  {value}
  {disabled}
  aria-label={label}
  aria-valuetext={valuetext}
  style:--fill={fill + '%'}
  oninput={(e) => oninput?.(Number(e.currentTarget.value))}
  onchange={(e) => onchange?.(Number(e.currentTarget.value))}
/>
````

## File: src/lib/components/tooltip.ts
````typescript
import { autoUpdate, computePosition, flip, offset, shift } from '@floating-ui/dom';

// One shared tooltip element in the top layer; shown on hover after a short delay and on keyboard focus.
let tip: HTMLDivElement | null = null;
let timer: ReturnType<typeof setTimeout> | undefined;
let owner: HTMLElement | null = null;
let stopPositioning: (() => void) | undefined;

function element() {
  if (!tip) {
    tip = document.createElement('div');
    tip.className = 'tooltip';
    tip.setAttribute('role', 'tooltip');
    tip.id = 'app-tooltip';
    tip.setAttribute('popover', 'manual');
    document.body.append(tip);
  }
  return tip;
}

function show(node: HTMLElement, text: string) {
  const el = element();
  owner = node;
  el.textContent = text;
  try {
    el.showPopover();
  } catch {
    /* already open */
  }
  stopPositioning?.();
  stopPositioning = autoUpdate(node, el, () => {
    void computePosition(node, el, {
      strategy: 'fixed',
      placement: 'top',
      middleware: [offset(8), flip({ padding: 6 }), shift({ padding: 6 })],
    }).then(({ x, y }) => {
      if (owner !== node) return;
      el.style.left = x + 'px';
      el.style.top = y + 'px';
    });
  });
  el.dataset.visible = 'true';
}

function hide(node?: HTMLElement) {
  clearTimeout(timer);
  if (node && owner !== node) return;
  owner = null;
  stopPositioning?.();
  stopPositioning = undefined;
  if (tip) {
    tip.dataset.visible = 'false';
    try {
      tip.hidePopover();
    } catch {
      /* closed */
    }
  }
}

/** Usage: use:tooltip={'播放队列'}; only overflowing text when {text, overflow:true}. */
export function tooltip(
  node: HTMLElement,
  value: string | { text: string; overflow?: boolean } | null,
) {
  let options = typeof value === 'string' ? { text: value } : value;
  const enter = (e: Event) => {
    if (!options?.text) return;
    if (options.overflow && node.scrollWidth <= node.clientWidth) return;
    const focus = e.type === 'focusin';
    if (focus && !node.matches(':focus-visible')) return;
    clearTimeout(timer);
    timer = setTimeout(() => show(node, options!.text), focus ? 150 : 500);
  };
  const leave = () => hide(node);
  node.addEventListener('pointerenter', enter);
  node.addEventListener('focusin', enter);
  node.addEventListener('pointerleave', leave);
  node.addEventListener('focusout', leave);
  node.addEventListener('pointerdown', leave);
  return {
    update(next: typeof value) {
      options = typeof next === 'string' ? { text: next } : next;
      if (owner === node && options?.text && tip) tip.textContent = options.text;
    },
    destroy() {
      hide(node);
      node.removeEventListener('pointerenter', enter);
      node.removeEventListener('focusin', enter);
      node.removeEventListener('pointerleave', leave);
      node.removeEventListener('focusout', leave);
      node.removeEventListener('pointerdown', leave);
    },
  };
}
````

## File: src/lib/components/TrackDetails.svelte
````svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { X, Copy, Play, ListEnd, ListPlus } from 'lucide-svelte';
  import Artwork from './Artwork.svelte';
  import Select from './Select.svelte';
  import IconButton from './IconButton.svelte';
  import Modal from './Modal.svelte';
  import {
    trackCover,
    audioInfo,
    playTrack,
    queueAction,
    addToPlaylist,
    type LibraryTrack,
    type AudioInfo,
  } from '../services/music';
  import { perform } from '../stores/player.svelte';
  import { library, refreshSummary } from '../stores/library.svelte';
  import { errorText, duration } from '../utils/format';
  let { track, onclose }: { track: LibraryTrack; onclose: () => void } = $props();
  let cover = $state<string | null>(null);
  let audio = $state<AudioInfo | null>(null);
  let audioError = $state(false);
  let busy = $state(false);
  let feedback = $state('');
  let playlist = $state('');
  onMount(() => {
    let disposed = false;
    void trackCover(track.id)
      .then((v) => {
        if (!disposed) cover = v;
      })
      .catch(() => {});
    void audioInfo(track.id)
      .then((v) => {
        if (!disposed) audio = v;
      })
      .catch(() => {
        if (!disposed) audioError = true;
      });
    return () => {
      disposed = true;
    };
  });
  async function run(action: () => Promise<void>) {
    if (busy) return;
    busy = true;
    feedback = '';
    try {
      await action();
    } catch (e) {
      feedback = errorText(e);
    } finally {
      busy = false;
    }
  }
  async function copy() {
    await navigator.clipboard.writeText(track.path);
    feedback = '已复制文件路径';
  }
  const channels = (n: number) => (n === 1 ? '单声道' : n === 2 ? '立体声' : n + ' 声道');
</script>

<Modal class="dialog-wide" aria-labelledby="song-detail-title" {onclose}>
  <header class="dialog-header">
    <h2 class="dialog-heading-sm">歌曲详情</h2>
    <IconButton label="关闭歌曲详情" onclick={onclose}><X size={18} /></IconButton>
  </header>
  <div class="dialog-body details-body">
    <div class="details-hero">
      <div class="details-cover"><Artwork src={cover} /></div>
      <div class="details-head">
        <h3 id="song-detail-title">{track.title}</h3>
        <p>{track.artist || '未知艺术家'}</p>
        <p class="muted">{track.album || '未知专辑'}</p>
      </div>
    </div>
    <div class="details-actions">
      <button
        type="button"
        class="btn btn-primary"
        disabled={busy}
        onclick={() =>
          run(async () => {
            await perform(() => playTrack(track.id));
          })}><Play size={16} fill="currentColor" />播放</button
      >
      <button
        type="button"
        class="btn btn-secondary"
        disabled={busy}
        onclick={() =>
          run(async () => {
            if (await perform(() => queueAction('add', track.id))) feedback = '已加入播放队列';
          })}><ListEnd size={16} />加入队列</button
      >
    </div>
    <section class="details-section">
      <h4>歌曲信息</h4>
      <dl class="info-grid">
        <dt>专辑</dt>
        <dd>{track.album || '未知专辑'}</dd>
        <dt>艺术家</dt>
        <dd>{track.artist || '未知艺术家'}</dd>
        {#if track.album_artist && track.album_artist !== track.artist}<dt>专辑艺术家</dt>
          <dd>{track.album_artist}</dd>{/if}
        <dt>时长</dt>
        <dd class="num">{duration(track.duration_ms)}</dd>
        {#if track.track_number}<dt>曲目</dt>
          <dd class="num">
            {track.disc_number ? track.disc_number + '-' : ''}{track.track_number}
          </dd>{/if}
        {#if track.year}<dt>年份</dt>
          <dd class="num">{track.year}</dd>{/if}
        {#if track.genre}<dt>流派</dt>
          <dd>{track.genre}</dd>{/if}
      </dl>
    </section>
    <section class="details-section">
      <h4>音频文件</h4>
      <dl class="info-grid">
        {#if audio}
          <dt>格式</dt>
          <dd>{audio.format}</dd>
          {#if audio.sample_rate}<dt>采样率</dt>
            <dd class="num">
              {(audio.sample_rate / 1000).toLocaleString(undefined, { maximumFractionDigits: 1 })} kHz
            </dd>{/if}
          {#if audio.channels}<dt>声道</dt>
            <dd>{channels(audio.channels)}</dd>{/if}
          {#if audio.bitrate}<dt>码率</dt>
            <dd class="num">{audio.bitrate} kbps</dd>{/if}
        {:else}<dt>音频参数</dt>
          <dd>{audioError ? '暂时无法读取' : '正在读取…'}</dd>{/if}
      </dl>
      <label class="field-label" for="detail-path">文件位置</label>
      <div class="path-copy">
        <input
          id="detail-path"
          class="input mono"
          readonly
          value={track.path}
          title={track.path}
        /><IconButton label="复制文件路径" onclick={() => run(copy)}><Copy size={16} /></IconButton>
      </div>
    </section>
    <section class="details-section">
      <h4>添加到歌单</h4>
      <div class="playlist-add">
        <Select
          label="选择歌单"
          bind:value={playlist}
          options={[
            {
              value: '',
              label: library.summary.playlists.length ? '选择歌单' : '请先在侧边栏创建歌单',
            },
            ...library.summary.playlists.map((p) => ({ value: String(p.id), label: p.name })),
          ]}
        />
        <button
          type="button"
          class="btn btn-secondary"
          disabled={!playlist || busy}
          onclick={() =>
            run(async () => {
              await addToPlaylist(Number(playlist), track.id);
              await refreshSummary();
              feedback = '已加入歌单';
            })}><ListPlus size={16} />加入歌单</button
        >
      </div>
    </section>
    <p class="detail-feedback" role="status">{feedback}</p>
  </div>
</Modal>

<style>
  :global {
    /* ============================================================ song details dialog */
    .details-body {
      color: var(--text);
    }

    .details-hero {
      display: flex;
      align-items: center;
      gap: var(--space-4);
    }

    .details-cover {
      position: relative;
      flex-shrink: 0;
      width: 96px;
      height: 96px;
      border-radius: var(--radius);
      overflow: hidden;
    }

    .details-head {
      min-width: 0;
    }

    .details-head h3 {
      font-size: var(--fs-section);
      font-weight: var(--fw-semibold);
      line-height: 1.35;
      overflow-wrap: anywhere;
    }

    .details-head p {
      margin-top: 2px;
      color: var(--text-2);
      font-size: var(--fs-meta);
      overflow-wrap: anywhere;
    }

    .details-head p.muted {
      color: var(--text-3);
    }

    .details-actions {
      display: flex;
      gap: 8px;
      flex-wrap: wrap;
      margin: var(--space-4) 0 var(--space-2);
    }

    .details-section {
      padding: var(--space-4) 0 0;
      margin-top: var(--space-4);
      border-top: 1px solid var(--line);
    }

    .details-section h4 {
      margin-bottom: var(--space-3);
      color: var(--text-2);
      font-size: var(--fs-small);
      font-weight: var(--fw-semibold);
    }

    .info-grid {
      display: grid;
      grid-template-columns: 96px minmax(0, 1fr);
      gap: 8px 16px;
      font-size: var(--fs-meta);
      line-height: 1.5;
    }

    .info-grid dt {
      color: var(--text-3);
    }

    .info-grid dd {
      color: var(--text);
      overflow-wrap: anywhere;
    }

    .details-section .field-label {
      margin-top: var(--space-4);
    }

    .path-copy {
      display: flex;
      align-items: center;
      gap: 6px;
    }

    .path-copy .input {
      flex: 1;
      min-width: 0;
      text-overflow: ellipsis;
    }

    .playlist-add {
      display: flex;
      align-items: center;
      gap: 8px;
      flex-wrap: wrap;
    }

    .playlist-add .select-trigger {
      flex: 1;
      min-width: 180px;
    }

    .detail-feedback {
      min-height: 20px;
      margin-top: var(--space-3);
      color: var(--accent);
      font-size: var(--fs-meta);
    }
  }
</style>
````

## File: src/lib/components/TrackList.svelte
````svelte
<script lang="ts">
  import { getContext, onMount, tick, untrack } from 'svelte';
  import { Ellipsis, Play, Clock3 } from 'lucide-svelte';
  import Cover from './Cover.svelte';
  import { tooltip } from './tooltip';
  import { queryLibrary, playTrack, type LibraryQuery, type LibraryTrack } from '../services/music';
  import { player, perform } from '../stores/player.svelte';
  import { library } from '../stores/library.svelte';
  import { openMenu, notifyError } from '../stores/ui.svelte';
  import { memory } from '../stores/nav.svelte';
  import { songMenu, openDetails } from '../actions';
  import { visibleRange, pagesFor, scrollToRow } from '../utils/virtual-range';
  import { duration } from '../utils/format';

  let {
    query,
    memoryKey,
    playlistId = null,
    numbering = 'index',
    sortable = true,
    onsort,
    ontotal,
  }: {
    query: LibraryQuery;
    memoryKey: string;
    playlistId?: number | null;
    numbering?: 'index' | 'track';
    sortable?: boolean;
    onsort?: (sort: string) => void;
    ontotal?: (total: number, loaded: boolean) => void;
  } = $props();

  const ROW = 56,
    PAGE = 100;
  const scroller = getContext<() => HTMLElement | undefined>('scroller');

  let total = $state(0);
  let loaded = $state(false);
  let failed = $state(false);
  let pages = $state<Record<number, LibraryTrack[]>>({});
  let range = $state({ start: 0, end: 0 });
  let selected = $state(-1);
  let host = $state<HTMLDivElement>();
  let generation = 0;
  const inflight = new Map<number, number>();

  let queryKey = $derived(JSON.stringify(query));

  async function fetchPage(page: number, gen: number) {
    if (inflight.get(page) === gen) return;
    inflight.set(page, gen);
    try {
      const result = await queryLibrary({ ...query, offset: page * PAGE, limit: PAGE });
      if (gen !== generation) return;
      pages[page] = result.tracks;
      total = result.total;
      if (!loaded) {
        loaded = true;
        ontotal?.(total, true);
        void tick().then(restore);
      } else ontotal?.(total, true);
    } catch (e) {
      if (gen === generation) {
        failed = true;
        notifyError(e);
      }
    } finally {
      if (inflight.get(page) === gen) inflight.delete(page);
    }
  }

  function measure() {
    const el = scroller(),
      list = host;
    if (!el || !list) return;
    const listTop =
      list.getBoundingClientRect().top - el.getBoundingClientRect().top + el.scrollTop;
    const next = visibleRange(el.scrollTop, el.clientHeight, listTop, ROW, total);
    if (next.start !== range.start || next.end !== range.end) range = next;
    const gen = generation;
    for (const p of pagesFor(next.start, Math.max(next.end, 1), PAGE))
      if (!pages[p]) void fetchPage(p, gen);
  }

  // A new query starts over; a library revision refreshes loaded pages in place without flashing.
  $effect(() => {
    queryKey;
    untrack(() => {
      generation++;
      pages = {};
      loaded = false;
      failed = false;
      total = 0;
      selected = -1;
      range = { start: 0, end: 0 };
      ontotal?.(0, false);
      void fetchPage(0, generation);
    });
  });
  let lastRevision = library.revision;
  $effect(() => {
    const rev = library.revision;
    if (rev === lastRevision) return;
    lastRevision = rev;
    untrack(() => {
      generation++;
      const gen = generation;
      const keep = Object.keys(pages).map(Number);
      if (!keep.length) keep.push(0);
      for (const p of keep) void fetchPage(p, gen);
    });
  });
  $effect(() => {
    total;
    untrack(measure);
  });

  onMount(() => {
    const el = scroller();
    if (!el) return;
    let frame = 0;
    const onScroll = () => {
      if (!frame)
        frame = requestAnimationFrame(() => {
          frame = 0;
          measure();
        });
    };
    el.addEventListener('scroll', onScroll, { passive: true });
    const ro = new ResizeObserver(onScroll);
    ro.observe(el);
    return () => {
      el.removeEventListener('scroll', onScroll);
      ro.disconnect();
      cancelAnimationFrame(frame);
      memory.set(memoryKey, { ...memory.get(memoryKey), scroll: el.scrollTop });
    };
  });
  function restore() {
    const el = scroller();
    const saved = memory.get(memoryKey)?.scroll;
    if (el && saved) el.scrollTop = saved;
    measure();
  }

  let rows = $derived.by(() => {
    const out: { index: number; track: LibraryTrack | undefined }[] = [];
    for (let i = range.start; i < range.end; i++)
      out.push({ index: i, track: pages[Math.floor(i / PAGE)]?.[i % PAGE] });
    return out;
  });

  let currentPath = $derived(player.snapshot?.track_id ?? null);
  let playing = $derived(player.snapshot?.playback_state === 'playing');

  async function start(t: LibraryTrack) {
    await perform(() => playTrack(t.id));
  }
  function menu(e: MouseEvent | HTMLElement, t: LibraryTrack, i: number) {
    selected = i;
    openMenu(e, songMenu(t, { playlistId, onDetails: () => openDetails(t) }), t.title + ' 操作');
  }

  async function focusRow(i: number) {
    const el = scroller(),
      list = host;
    if (!el || !list || i < 0 || i >= total) return;
    selected = i;
    const listTop =
      list.getBoundingClientRect().top - el.getBoundingClientRect().top + el.scrollTop;
    const target = scrollToRow(i, el.scrollTop, el.clientHeight, listTop, ROW, ROW);
    if (target != null) {
      el.scrollTop = target;
      measure();
    }
    await tick();
    host?.querySelector<HTMLElement>(`[data-index="${i}"]`)?.focus({ preventScroll: true });
  }
  function key(e: KeyboardEvent, i: number, t: LibraryTrack | undefined) {
    if (e.isComposing || e.target !== e.currentTarget) return;
    const page = Math.max(1, Math.floor((scroller()?.clientHeight ?? ROW * 8) / ROW) - 1);
    let next = -1;
    if (e.key === 'ArrowDown') next = Math.min(total - 1, i + 1);
    else if (e.key === 'ArrowUp') next = Math.max(0, i - 1);
    else if (e.key === 'PageDown') next = Math.min(total - 1, i + page);
    else if (e.key === 'PageUp') next = Math.max(0, i - page);
    else if (e.key === 'Home' && e.ctrlKey) next = 0;
    else if (e.key === 'End' && e.ctrlKey) next = total - 1;
    else if (e.key === 'Enter' && t) {
      e.preventDefault();
      void start(t);
      return;
    }
    if (next >= 0) {
      e.preventDefault();
      void focusRow(next);
    }
  }

  let sort = $derived(query.sort ?? 'added');
  const columns = [
    { key: 'title', label: '标题' },
    { key: 'artist', label: '艺术家' },
    { key: 'album', label: '专辑' },
  ];
</script>

<div
  class="track-table"
  role="grid"
  aria-label="歌曲列表"
  aria-rowcount={total + 1}
  aria-busy={!loaded}
>
  <div class="track-head" role="row" aria-rowindex={1}>
    <span role="columnheader" class="col-index">#</span>
    {#each columns as c (c.key)}
      <span
        role="columnheader"
        class={'col-' + c.key}
        aria-sort={sortable && sort === c.key ? 'ascending' : undefined}
      >
        {#if sortable && onsort}<button
            type="button"
            class:sorted={sort === c.key}
            onclick={() => onsort(c.key)}>{c.label}</button
          >{:else}{c.label}{/if}
      </span>
    {/each}
    <span
      role="columnheader"
      class="col-duration"
      aria-label="时长"
      aria-sort={sortable && sort === 'duration' ? 'ascending' : undefined}
    >
      {#if sortable && onsort}<button
          type="button"
          class:sorted={sort === 'duration'}
          aria-label="按时长排序"
          onclick={() => onsort('duration')}><Clock3 size={15} /></button
        >{:else}<Clock3 size={15} />{/if}
    </span>
    <span role="columnheader" class="col-actions"><span class="sr-only">操作</span></span>
  </div>

  <div class="track-body" bind:this={host} style:height={total * ROW + 'px'}>
    <div class="track-window" style:transform={`translateY(${range.start * ROW}px)`}>
      {#each rows as { index, track: t } (t ? t.id : 'loading-' + index)}
        {@const current = !!t && t.path === currentPath}
        <div
          class="track-row"
          role="row"
          aria-rowindex={index + 2}
          aria-selected={selected === index}
          data-index={index}
          tabindex={selected === index || (selected < 0 && index === range.start) ? 0 : -1}
          class:selected={selected === index}
          class:current
          class:loading={!t}
          onclick={() => (selected = index)}
          ondblclick={() => {
            if (t) void start(t);
          }}
          onkeydown={(e) => key(e, index, t)}
          oncontextmenu={(e) => {
            if (t) menu(e, t, index);
            else e.preventDefault();
          }}
        >
          {#if t}
            <span role="gridcell" class="col-index">
              {#if current}<span
                  class="eq"
                  class:paused={!playing}
                  aria-label={playing ? '正在播放' : '已暂停'}><i></i><i></i><i></i></span
                >
              {:else}<span class="row-number"
                  >{numbering === 'track' && t.track_number ? t.track_number : index + 1}</span
                >{/if}
              <button
                type="button"
                class="row-play"
                tabindex="-1"
                aria-label={'播放 ' + t.title}
                onclick={(e) => {
                  e.stopPropagation();
                  void start(t);
                }}><Play size={15} fill="currentColor" /></button
              >
            </span>
            <span role="gridcell" class="col-title">
              <span class="row-cover"><Cover id={t.id} alt={t.album || t.title} size={96} /></span>
              <span class="row-title" use:tooltip={{ text: t.title, overflow: true }}
                >{t.title}</span
              >
            </span>
            <span role="gridcell" class="col-artist"
              ><span class="row-text" use:tooltip={{ text: t.artist, overflow: true }}
                >{t.artist || '未知艺术家'}</span
              ></span
            >
            <span role="gridcell" class="col-album"
              ><span class="row-text" use:tooltip={{ text: t.album, overflow: true }}
                >{t.album || '—'}</span
              ></span
            >
            <span role="gridcell" class="col-duration">{duration(t.duration_ms)}</span>
            <span role="gridcell" class="col-actions">
              <button
                type="button"
                class="row-action"
                tabindex="-1"
                aria-haspopup="menu"
                aria-label={'更多操作：' + t.title}
                use:tooltip={'更多操作'}
                onclick={(e) => {
                  e.stopPropagation();
                  menu(e.currentTarget as HTMLElement, t, index);
                }}><Ellipsis size={16} /></button
              >
            </span>
          {:else}
            <span role="gridcell" class="col-index"></span>
            <span role="gridcell" class="col-title"
              ><span class="row-cover skeleton"></span><span class="skeleton skeleton-text"
              ></span></span
            >
            <span role="gridcell" class="col-artist"
              ><span class="skeleton skeleton-text short"></span></span
            >
            <span role="gridcell" class="col-album"
              ><span class="skeleton skeleton-text short"></span></span
            >
            <span role="gridcell" class="col-duration"></span><span
              role="gridcell"
              class="col-actions"
            ></span>
          {/if}
        </div>
      {/each}
    </div>
  </div>
  {#if failed}<p class="list-error" role="alert">
      读取歌曲失败。<button
        type="button"
        class="btn btn-ghost btn-sm"
        onclick={() => {
          failed = false;
          generation++;
          pages = {};
          void fetchPage(0, generation);
        }}>重试</button
      >
    </p>{/if}
</div>

<style>
  :global {
    /* ============================================================ track table (virtualised) */
    .track-table {
      container-type: inline-size;
      --cols: 44px minmax(0, 4fr) minmax(0, 2.4fr) minmax(0, 2.6fr) 56px 40px;
    }

    .track-head,
    .track-row {
      display: grid;
      grid-template-columns: var(--cols);
      align-items: center;
      column-gap: var(--space-3);
      padding: 0 var(--space-2);
    }

    /* Sticky offsets are measured inside the scroller's padding; cancel .main's top padding so rows cannot show above the header. */
    .track-head {
      position: sticky;
      top: calc(var(--page-top) * -1);
      z-index: var(--z-sticky);
      height: 36px;
      margin: 0 calc(var(--space-2) * -1) var(--space-1);
      padding-inline: calc(var(--space-2) * 2);
      background: var(--bg);
      border-bottom: 1px solid var(--line);
      color: var(--text-3);
      font-size: var(--fs-small);
    }

    .track-head button {
      display: inline-flex;
      align-items: center;
      gap: 4px;
      color: inherit;
      border-radius: var(--radius-xs);
      font-size: inherit;
    }

    .track-head button:hover,
    .track-head button.sorted {
      color: var(--text);
    }

    .track-head .col-index,
    .track-row .col-index {
      justify-self: center;
    }

    .track-head .col-duration,
    .track-row .col-duration {
      justify-self: end;
    }

    .track-body {
      position: relative;
      contain: strict;
      width: 100%;
    }

    .track-window {
      position: absolute;
      inset: 0 0 auto;
      will-change: transform;
    }

    .track-row {
      height: var(--row-h);
      border-radius: var(--radius-sm);
      color: var(--text-2);
      font-size: var(--fs-meta);
      cursor: default;
      user-select: none;
      outline: none;
    }

    .track-row:hover {
      background: var(--hover);
    }

    .track-row.selected {
      background: var(--selected);
    }

    .track-row:focus-visible {
      box-shadow: inset 0 0 0 2px var(--focus);
    }

    .track-row.current .row-title {
      color: var(--accent);
    }

    .col-index {
      position: relative;
      display: grid;
      place-items: center;
      width: 32px;
      height: 32px;
      color: var(--text-3);
    }

    .row-play {
      position: absolute;
      inset: 0;
      display: none;
      place-items: center;
      border-radius: 50%;
      color: var(--text);
    }

    .track-row:hover .row-play,
    .track-row.selected .row-play {
      display: grid;
    }

    .track-row:hover .row-number,
    .track-row.selected .row-number,
    .track-row:hover .eq,
    .track-row.selected .eq {
      visibility: hidden;
    }

    .col-title {
      display: flex;
      align-items: center;
      gap: var(--space-3);
      min-width: 0;
    }

    .row-cover {
      position: relative;
      flex-shrink: 0;
      width: 40px;
      height: 40px;
      border-radius: var(--radius-xs);
      overflow: hidden;
      background: var(--surface-hover);
    }

    .row-title {
      min-width: 0;
      color: var(--text);
      font-size: var(--fs-body);
      font-weight: var(--fw-semibold);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .col-artist,
    .col-album {
      min-width: 0;
    }

    .row-text {
      display: block;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .col-duration {
      color: var(--text-3);
    }

    .col-actions {
      display: flex;
      justify-content: flex-end;
      gap: 2px;
    }

    .row-action {
      display: grid;
      place-items: center;
      width: 30px;
      height: 30px;
      border-radius: 50%;
      color: var(--text-3);
      opacity: 0;
      transition:
        opacity var(--dur-fast),
        color var(--dur-fast),
        background-color var(--dur-fast);
    }

    .row-action:hover {
      color: var(--text);
      background: var(--surface-active);
    }

    .track-row:hover .row-action,
    .track-row.selected .row-action,
    .track-row:focus-within .row-action {
      opacity: 1;
    }

    .track-row.loading .col-title .skeleton-text {
      flex: 1;
    }

    .list-error {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 16px;
      color: var(--danger);
      font-size: var(--fs-meta);
    }

    @container (max-width:760px) {
      .track-table {
        --cols: 40px minmax(0, 3fr) minmax(0, 2fr) 52px 40px;
      }

      .col-album {
        display: none;
      }
    }

    @container (max-width:520px) {
      .track-table {
        --cols: 36px minmax(0, 1fr) 48px 40px;
      }

      .col-artist {
        display: none;
      }
    }
  }
</style>
````

## File: src/lib/layout/PlayerBar.svelte
````svelte
<script lang="ts">
  import {
    Play,
    Pause,
    SkipBack,
    SkipForward,
    Shuffle,
    Repeat,
    Repeat1,
    Volume2,
    Volume1,
    VolumeX,
    ListMusic,
    Maximize2,
  } from 'lucide-svelte';
  import Artwork from '../components/Artwork.svelte';
  import Slider from '../components/Slider.svelte';
  import IconButton from '../components/IconButton.svelte';
  import { tooltip } from '../components/tooltip';
  import { player, control, accept } from '../stores/player.svelte';
  import { ui } from '../stores/ui.svelte';
  import {
    play,
    pause,
    seek,
    setVolume,
    setMode,
    queueAction,
    type PlaybackMode,
  } from '../services/music';
  import { duration } from '../utils/format';
  import { DEFAULT_PERCENT, toPercent, toGain, muteTarget, latest } from '../utils/volume';
  import { notifyError } from '../stores/ui.svelte';

  const UNMUTE_KEY = 'music-mine-unmute-volume';

  let {
    artwork,
    ontogglequeue,
    queueTrigger = $bindable(),
  }: {
    artwork: string | null;
    ontogglequeue: () => void;
    queueTrigger?: HTMLButtonElement;
  } = $props();

  let s = $derived(player.snapshot);
  let track = $derived(s?.current_track ?? null);
  let playing = $derived(s?.playback_state === 'playing');
  let mode = $derived<PlaybackMode>(s?.playback_mode ?? 'sequential');
  let hasQueue = $derived(!!s?.queue.length);

  let dragging = $state<number | null>(null);
  let trackId = $derived(s?.track_id);
  $effect(() => {
    trackId;
    dragging = null;
  });
  let position = $derived(dragging ?? s?.position_ms ?? 0);
  let total = $derived(s?.duration_ms ?? 0);
  let canSeek = $derived(!!total && s?.playback_state !== 'idle');

  // The draft only covers the gap until Rust's snapshot confirms the last value sent.
  let volumeDraft = $state<number | null>(null);
  let volume = $derived(volumeDraft ?? (s ? toPercent(s.volume) : DEFAULT_PERCENT));
  let lastAudible = $state(loadUnmuteLevel());
  $effect(() => {
    if (s && s.volume > 0) lastAudible = toPercent(s.volume);
  });
  const volumeSync = latest((percent: number) => setVolume(toGain(percent)), {
    result: accept,
    error: notifyError,
    settled: () => {
      volumeDraft = null;
    },
  });

  function changeVolume(value: number) {
    volumeDraft = value;
    volumeSync.push(value);
  }
  function toggleMute() {
    const target = muteTarget(volume, lastAudible);
    if (target === 0) saveUnmuteLevel(volume);
    changeVolume(target);
  }
  function loadUnmuteLevel() {
    try {
      return Number(localStorage.getItem(UNMUTE_KEY)) || DEFAULT_PERCENT;
    } catch {
      return DEFAULT_PERCENT;
    }
  }
  function saveUnmuteLevel(value: number) {
    try {
      localStorage.setItem(UNMUTE_KEY, String(value));
    } catch {
      /* in-memory only */
    }
  }
  async function commitSeek(value: number) {
    await control(() => seek(value));
    dragging = null;
  }

  const repeatNext: Record<PlaybackMode, PlaybackMode> = {
    sequential: 'repeat_all',
    shuffle: 'repeat_all',
    repeat_all: 'repeat_one',
    repeat_one: 'sequential',
  };
  let repeatLabel = $derived(
    mode === 'repeat_all' ? '列表循环' : mode === 'repeat_one' ? '单曲循环' : '循环播放：关',
  );
</script>

<footer class="player-bar" aria-label="播放控制">
  <div class="pb-track">
    <button
      type="button"
      class="pb-cover"
      aria-label="打开正在播放"
      disabled={!track}
      onclick={() => (ui.immersive = true)}
      ><Artwork src={artwork} alt={track?.album || '专辑封面'} /></button
    >
    <div class="pb-meta">
      {#if track}
        <button
          type="button"
          class="pb-title"
          onclick={() => (ui.immersive = true)}
          use:tooltip={{ text: track.title, overflow: true }}>{track.title}</button
        >
        <span class="pb-artist" use:tooltip={{ text: track.artist, overflow: true }}
          >{track.artist || '未知艺术家'}</span
        >
      {:else}
        <span class="pb-title pb-idle">未在播放</span>
        <span class="pb-artist">从音乐库选择一首歌曲</span>
      {/if}
    </div>
  </div>

  <div class="pb-center">
    <div class="pb-controls">
      <IconButton
        label={mode === 'shuffle' ? '随机播放：开' : '随机播放：关'}
        active={mode === 'shuffle'}
        aria-pressed={mode === 'shuffle'}
        disabled={!s || player.busy}
        onclick={() => control(() => setMode(mode === 'shuffle' ? 'sequential' : 'shuffle'))}
        ><Shuffle size={17} /></IconButton
      >
      <IconButton
        label="上一首"
        size="lg"
        disabled={!hasQueue || player.busy}
        onclick={() => control(() => queueAction('previous'))}
        ><SkipBack size={19} fill="currentColor" /></IconButton
      >
      <button
        type="button"
        class="pb-play"
        aria-label={playing ? '暂停' : '播放'}
        use:tooltip={playing ? '暂停 (Space)' : '播放 (Space)'}
        disabled={!track || player.busy}
        onclick={() => control(playing ? pause : play)}
        >{#if playing}<Pause size={18} fill="currentColor" />{:else}<Play
            size={18}
            fill="currentColor"
            class="pb-play-icon"
          />{/if}</button
      >
      <IconButton
        label="下一首"
        size="lg"
        disabled={!hasQueue || player.busy}
        onclick={() => control(() => queueAction('next'))}
        ><SkipForward size={19} fill="currentColor" /></IconButton
      >
      <IconButton
        label={repeatLabel}
        active={mode === 'repeat_all' || mode === 'repeat_one'}
        aria-pressed={mode === 'repeat_all' || mode === 'repeat_one'}
        disabled={!s || player.busy}
        onclick={() => control(() => setMode(repeatNext[mode]))}
        >{#if mode === 'repeat_one'}<Repeat1 size={17} />{:else}<Repeat
            size={17}
          />{/if}</IconButton
      >
    </div>
    <div class="pb-progress">
      <span class="pb-time">{duration(position)}</span>
      <Slider
        class="pb-seek"
        label="播放进度"
        value={position}
        max={total || 1}
        valuetext={duration(position) + ' / ' + duration(total)}
        disabled={!canSeek || player.busy}
        oninput={(v) => (dragging = v)}
        onchange={commitSeek}
      />
      <span class="pb-time pb-time-end">{duration(total)}</span>
    </div>
  </div>

  <div class="pb-options">
    <IconButton
      label="正在播放与歌词 (Ctrl+L)"
      active={ui.immersive}
      disabled={!track}
      onclick={() => (ui.immersive = !ui.immersive)}><Maximize2 size={17} /></IconButton
    >
    <IconButton
      label="播放队列"
      active={ui.queueOpen}
      aria-expanded={ui.queueOpen}
      aria-controls="playback-queue"
      bind:element={queueTrigger}
      onclick={ontogglequeue}><ListMusic size={18} /></IconButton
    >
    <div class="pb-volume">
      <IconButton
        label={volume > 0 ? `静音（当前音量 ${volume}%）` : `取消静音（恢复到 ${lastAudible}%）`}
        disabled={!s}
        onclick={toggleMute}
      >
        {#if volume === 0}<VolumeX size={18} />{:else if volume < 50}<Volume1
            size={18}
          />{:else}<Volume2 size={18} />{/if}
      </IconButton>
      <Slider
        class="pb-volume-slider"
        label="音量"
        value={volume}
        valuetext={volume === 0 ? '0%，已静音' : volume + '%'}
        disabled={!s}
        oninput={changeVolume}
        onchange={changeVolume}
      />
      <span class="pb-volume-value" aria-hidden="true">{volume}%</span>
    </div>
  </div>
</footer>

<style>
  :global {
    /* ============================================================ player bar */
    .player-bar {
      grid-column: 1/-1;
      grid-row: 2;
      z-index: var(--z-player);
      display: grid;
      grid-template-columns: minmax(180px, 1fr) minmax(320px, 2fr) minmax(180px, 1fr);
      align-items: center;
      gap: var(--space-4);
      padding: 0 var(--space-4);
      background: var(--bg-player);
      border-top: 1px solid var(--line);
    }

    .pb-track {
      display: flex;
      align-items: center;
      gap: var(--space-3);
      min-width: 0;
    }

    .pb-cover {
      position: relative;
      flex-shrink: 0;
      width: 56px;
      height: 56px;
      border-radius: var(--radius-sm);
      overflow: hidden;
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.4);
    }

    .pb-cover:disabled {
      cursor: default;
    }

    .pb-meta {
      min-width: 0;
      display: flex;
      flex-direction: column;
      align-items: flex-start;
      gap: 2px;
    }

    .pb-title {
      max-width: 100%;
      color: var(--text);
      font-size: var(--fs-body);
      font-weight: var(--fw-semibold);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      text-align: left;
      border-radius: var(--radius-xs);
    }

    button.pb-title:hover {
      text-decoration: underline;
    }

    .pb-idle {
      color: var(--text-2);
    }

    .pb-artist {
      max-width: 100%;
      color: var(--text-3);
      font-size: var(--fs-small);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .pb-center {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 4px;
      min-width: 0;
      max-width: 640px;
      width: 100%;
      justify-self: center;
    }

    .pb-controls {
      display: flex;
      align-items: center;
      gap: var(--space-3);
    }

    .pb-play {
      display: grid;
      place-items: center;
      flex-shrink: 0;
      width: 38px;
      height: 38px;
      border-radius: 50%;
      background: var(--text);
      color: var(--bg);
      transition:
        transform var(--dur-fast) var(--ease),
        background-color var(--dur-fast);
    }

    .pb-play:hover:not(:disabled) {
      transform: scale(1.06);
      background: var(--play-hover);
    }

    .pb-play:active:not(:disabled) {
      transform: scale(0.96);
    }

    .pb-play:disabled {
      opacity: 0.4;
    }

    .pb-play-icon {
      margin-left: 2px;
    }

    .pb-progress {
      display: grid;
      grid-template-columns: 44px minmax(0, 1fr) 44px;
      align-items: center;
      gap: var(--space-2);
      width: 100%;
    }

    .pb-time {
      color: var(--text-3);
      font-size: 11px;
      text-align: right;
    }

    .pb-time-end {
      text-align: left;
    }

    .pb-options {
      display: flex;
      align-items: center;
      justify-content: flex-end;
      gap: var(--space-1);
      min-width: 0;
    }

    .pb-volume {
      display: flex;
      align-items: center;
      gap: 4px;
      min-width: 0;
    }

    .pb-volume .slider {
      width: clamp(64px, 8vw, 112px);
    }

    .pb-volume-value {
      min-width: 4ch;
      color: var(--text-2);
      font-size: var(--fs-small);
      font-variant-numeric: tabular-nums;
      text-align: right;
      user-select: none;
    }
  }
</style>
````

## File: src/lib/layout/Queue.svelte
````svelte
<script lang="ts">
  import {
    X,
    ListMusic,
    GripVertical,
    Ellipsis,
    Play,
    ListStart,
    ListEnd,
    ListX,
    Trash2,
  } from 'lucide-svelte';
  import QueueCover from '../components/QueueCover.svelte';
  import IconButton from '../components/IconButton.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import { tooltip } from '../components/tooltip';
  import { reorderQueue, playQueued, queueAction, type Track } from '../services/music';
  import { player, control } from '../stores/player.svelte';
  import { openMenu, confirmAction, notify, type MenuItem } from '../stores/ui.svelte';
  import { duration } from '../utils/format';
  import { retainQueue } from '../utils/queue';

  let { open, onclose }: { open: boolean; onclose: () => void } = $props();
  let closeButton = $state<HTMLButtonElement>();
  let list = $state<HTMLDivElement>();
  $effect(() => {
    if (open) closeButton?.focus({ preventScroll: true });
  });

  // Every player poll delivers a fresh queue array. Keep the rendered one until the tracks really change,
  // so polling, opening and closing never re-diff every row.
  let rendered: readonly Track[] = [];
  let queue = $derived.by(() => {
    rendered = retainQueue(rendered, player.snapshot?.queue ?? []);
    return rendered;
  });
  let currentIndex = $derived(queue.findIndex((t) => t.path === player.snapshot?.track_id));
  let upcoming = $derived(currentIndex < 0 ? queue.length : queue.length - currentIndex - 1);

  // Queue items are identified by file path: Rust deduplicates the queue by path.
  let drag = $state<{
    path: string;
    from: number;
    to: number;
    offset: number;
    startY: number;
    height: number;
  } | null>(null);
  let announce = $state('');

  function rows() {
    return Array.from(list?.querySelectorAll<HTMLElement>('[data-queue-row]') ?? []);
  }
  function startDrag(e: PointerEvent, t: Track, i: number) {
    if (e.button !== 0 || player.busy) return;
    const row = (e.currentTarget as HTMLElement).closest<HTMLElement>('[data-queue-row]');
    if (!row) return;
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    drag = {
      path: t.path,
      from: i,
      to: i,
      offset: 0,
      startY: e.clientY,
      height: row.offsetHeight,
    };
    e.preventDefault();
  }
  function moveDrag(e: PointerEvent) {
    if (!drag || !list) return;
    const box = list.getBoundingClientRect();
    if (e.clientY < box.top + 32) list.scrollTop -= 10;
    else if (e.clientY > box.bottom - 32) list.scrollTop += 10;
    drag.offset = e.clientY - drag.startY;
    let to = 0;
    rows().forEach((row, index) => {
      if (index === drag!.from) return;
      const r = row.getBoundingClientRect();
      if (e.clientY > r.top + r.height / 2) to++;
    });
    drag.to = to;
  }
  async function endDrag() {
    const current = drag;
    drag = null;
    if (current && current.to !== current.from) {
      await control(() => reorderQueue(current.path, current.to));
      announce = `已移动到第 ${current.to + 1} 位`;
    }
  }
  async function moveBy(t: Track, i: number, delta: number) {
    const to = Math.max(0, Math.min(queue.length - 1, i + delta));
    if (to === i) return;
    if (await control(() => reorderQueue(t.path, to))) {
      announce = `${t.title} 已移动到第 ${to + 1} 位`;
      requestAnimationFrame(() => rows()[to]?.querySelector<HTMLElement>('.queue-handle')?.focus());
    }
  }
  function handleKey(e: KeyboardEvent, t: Track, i: number) {
    if (e.key === 'ArrowUp' || e.key === 'ArrowDown') {
      e.preventDefault();
      void moveBy(t, i, e.key === 'ArrowUp' ? -1 : 1);
    } else if (e.key === 'Home' || e.key === 'End') {
      e.preventDefault();
      void moveBy(t, i, e.key === 'Home' ? -i : queue.length);
    }
  }
  function shift(i: number) {
    if (!drag) return 0;
    if (i === drag.from) return drag.offset;
    if (drag.from < i && i <= drag.to) return -drag.height;
    if (drag.to <= i && i < drag.from) return drag.height;
    return 0;
  }

  function itemMenu(t: Track, i: number): MenuItem[] {
    const next = currentIndex < 0 ? 0 : i > currentIndex ? currentIndex + 1 : currentIndex;
    return [
      {
        label: '播放',
        icon: Play,
        disabled: i === currentIndex,
        action: () => control(() => playQueued(t.path)),
      },
      {
        label: '移到下一首播放',
        icon: ListStart,
        disabled: i === currentIndex || i === currentIndex + 1,
        action: () => control(() => reorderQueue(t.path, next)),
      },
      {
        label: '移到队列末尾',
        icon: ListEnd,
        disabled: i === queue.length - 1,
        action: () => control(() => reorderQueue(t.path, queue.length - 1)),
      },
      { kind: 'separator' },
      {
        label: '从队列移除',
        icon: ListX,
        danger: true,
        action: () => control(() => queueAction('remove', null, t.path)),
      },
    ];
  }

  async function clear() {
    const removing = currentIndex < 0 ? queue.length : queue.length - 1;
    const ok = await confirmAction({
      title: '清空播放队列？',
      message:
        currentIndex < 0
          ? `将从播放队列移除全部 ${removing} 首歌曲。`
          : `将从播放队列移除其他 ${removing} 首歌曲，正在播放的歌曲会保留并继续播放。`,
      detail: '只影响播放队列，不会修改歌单或任何音乐文件。',
      confirmLabel: '清空队列',
      danger: true,
    });
    if (ok && (await control(() => queueAction('clear')))) notify('播放队列已清空');
  }
</script>

<aside id="playback-queue" class="queue" aria-label="播放队列" inert={!open}>
  <header class="queue-header">
    <div class="queue-heading">
      <h2>播放队列</h2>
      <p>
        {queue.length ? `${queue.length} 首 · 待播 ${upcoming} 首` : '空'}
      </p>
    </div>
    <IconButton
      label="清空队列"
      disabled={queue.length <= (currentIndex < 0 ? 0 : 1) || player.busy}
      onclick={clear}><Trash2 size={17} /></IconButton
    >
    <IconButton label="关闭队列 (Esc)" bind:element={closeButton} onclick={onclose}
      ><X size={18} /></IconButton
    >
  </header>
  <div
    class="queue-list"
    bind:this={list}
    role="list"
    aria-label="队列歌曲"
    class:dragging={!!drag}
  >
    {#each queue as t, i (t.path)}
      <div
        class="queue-item"
        role="listitem"
        data-queue-row
        class:current={i === currentIndex}
        class:played={currentIndex >= 0 && i < currentIndex}
        class:lifted={drag?.from === i}
        style:transform={drag ? `translateY(${shift(i)}px)` : undefined}
        oncontextmenu={(e) => openMenu(e, itemMenu(t, i), t.title + ' 队列操作')}
      >
        <button
          type="button"
          class="queue-handle"
          aria-label={`调整顺序：${t.title}，按上下方向键移动`}
          use:tooltip={'拖动调整顺序'}
          onpointerdown={(e) => startDrag(e, t, i)}
          onpointermove={moveDrag}
          onpointerup={endDrag}
          onpointercancel={() => (drag = null)}
          onkeydown={(e) => handleKey(e, t, i)}><GripVertical size={16} /></button
        >
        <button
          type="button"
          class="queue-main"
          aria-current={i === currentIndex ? 'true' : undefined}
          aria-label={(i === currentIndex ? '正在播放：' : '播放 ') + t.title}
          onclick={() => {
            if (i !== currentIndex) void control(() => playQueued(t.path));
          }}
        >
          <span class="queue-cover"
            ><QueueCover track={t} current={i === currentIndex} />{#if i === currentIndex}<span
                class="eq"
                class:paused={player.snapshot?.playback_state !== 'playing'}
                aria-hidden="true"><i></i><i></i><i></i></span
              >{/if}</span
          >
          <span class="queue-text"
            ><strong>{t.title}</strong><small>{t.artist || '未知艺术家'}</small></span
          >
          <span class="queue-duration">{duration(t.duration_ms)}</span>
        </button>
        <IconButton
          label={'更多操作：' + t.title}
          size="sm"
          class="queue-more"
          onclick={(e) =>
            openMenu(e.currentTarget as HTMLElement, itemMenu(t, i), t.title + ' 队列操作')}
          ><Ellipsis size={16} /></IconButton
        >
      </div>
    {:else}
      <EmptyState
        icon={ListMusic}
        title="播放队列是空的"
        message="在音乐库中双击歌曲，或使用右键菜单“加入播放队列”。"
      />
    {/each}
  </div>
  <p class="sr-only" aria-live="polite">{announce}</p>
</aside>

<style>
  :global {
    /* ============================================================ queue */
    /* The queue is an app-level overlay drawer between the top bar and the player bar; pages keep their width.
   Only transform and opacity animate. Closing delays visibility:hidden until the exit transition ends,
   and an interrupted toggle simply reverses from the current position. */
    .queue {
      position: fixed;
      top: var(--topbar-h);
      right: 0;
      bottom: var(--player-h);
      z-index: var(--z-queue);
      width: var(--queue-w);
      min-height: 0;
      display: flex;
      flex-direction: column;
      background: var(--bg-drawer);
      border-left: 1px solid var(--line);
      box-shadow: var(--shadow-drawer);
      transform: translateX(var(--drawer-shift));
      opacity: 0;
      visibility: hidden;
      transition:
        transform var(--dur-drawer-out) var(--ease-exit),
        opacity var(--dur-drawer-out) var(--ease-exit),
        visibility 0s linear var(--dur-drawer-out);
    }

    .app.queue-open .queue {
      transform: none;
      opacity: 1;
      visibility: visible;
      transition:
        transform var(--dur-drawer-in) var(--ease-enter),
        opacity var(--dur-drawer-in) var(--ease-enter),
        visibility 0s;
    }

    @starting-style {
      .app.queue-open .queue {
        transform: translateX(var(--drawer-shift));
        opacity: 0;
      }
    }

    .app.immersive-open .queue {
      top: 0;
    }

    /* The drawer covers Now Playing instead of narrowing it, so lyrics never re-wrap and the cover keeps its size.
   A light scrim marks the drawer as the active layer wherever it covers content: Now Playing at any width
   and every page on narrow windows. Wide library pages stay usable beside it. */
    .queue-scrim {
      position: fixed;
      top: var(--topbar-h);
      left: var(--sidebar-w);
      right: 0;
      bottom: var(--player-h);
      z-index: var(--z-queue-scrim);
      background: var(--scrim-soft);
      cursor: default;
      opacity: 0;
      pointer-events: none;
      transition: opacity var(--dur-drawer-out) var(--ease-exit);
    }

    .app.queue-open .queue-scrim {
      opacity: 1;
      pointer-events: auto;
      transition: opacity var(--dur-drawer-in) var(--ease-enter);
    }

    .app.immersive-open .queue-scrim {
      top: 0;
      left: 0;
    }

    @media (min-width: 1200px) {
      .app:not(.immersive-open) .queue-scrim {
        display: none;
      }
    }

    .queue-header {
      display: flex;
      align-items: center;
      gap: 4px;
      height: 64px;
      padding: 0 var(--space-2) 0 var(--space-4);
      flex-shrink: 0;
      border-bottom: 1px solid var(--line);
    }

    .queue-heading {
      flex: 1;
      min-width: 0;
    }

    .queue-heading h2 {
      font-size: 15px;
      font-weight: var(--fw-semibold);
    }

    .queue-heading p {
      color: var(--text-3);
      font-size: var(--fs-small);
      font-variant-numeric: tabular-nums;
    }

    .queue-list {
      flex: 1;
      min-height: 0;
      overflow-y: auto;
      overflow-x: hidden;
      overscroll-behavior: contain;
      padding: var(--space-2);
    }

    .queue-list.dragging {
      cursor: grabbing;
      user-select: none;
    }

    /* Off-screen rows skip style, layout and paint, so toggling the drawer restyles only the visible rows
   (inherited visibility/inert changes otherwise recompute every element of a long queue). */
    .queue-item {
      position: relative;
      display: flex;
      align-items: center;
      gap: 2px;
      height: 56px;
      padding-right: 4px;
      border-radius: var(--radius-sm);
      transition: background-color var(--dur-fast);
      content-visibility: auto;
      contain-intrinsic-size: auto 56px;
    }

    .queue-list:not(.dragging) .queue-item {
      transition:
        background-color var(--dur-fast),
        transform var(--dur) var(--ease);
    }

    .queue-item:hover {
      background: var(--hover);
    }

    .queue-item.current {
      background: var(--selected);
    }

    .queue-item.current .queue-text strong {
      color: var(--accent);
    }

    .queue-item.played {
      opacity: 0.55;
    }

    .queue-item.played:hover {
      opacity: 1;
    }

    .queue-item.lifted {
      z-index: 3;
      background: var(--surface-active);
      box-shadow: var(--shadow-card);
      opacity: 1;
      transition: none;
    }

    .queue-handle {
      display: grid;
      place-items: center;
      width: 24px;
      height: 40px;
      flex-shrink: 0;
      border-radius: var(--radius-xs);
      color: var(--text-3);
      cursor: grab;
      opacity: 0;
      touch-action: none;
    }

    .queue-item:hover .queue-handle,
    .queue-handle:focus-visible,
    .queue-item.lifted .queue-handle {
      opacity: 1;
    }

    .queue-handle:hover {
      color: var(--text);
    }

    .queue-main {
      flex: 1;
      min-width: 0;
      display: flex;
      align-items: center;
      gap: 10px;
      height: 100%;
      text-align: left;
      border-radius: var(--radius-sm);
    }

    .queue-cover {
      position: relative;
      flex-shrink: 0;
      width: 40px;
      height: 40px;
      border-radius: var(--radius-xs);
      overflow: hidden;
    }

    .queue-cover .eq {
      position: absolute;
      inset: 0;
      z-index: 1;
      justify-content: center;
      align-items: center;
      height: auto;
      padding: 12px 0;
      background: rgba(0, 0, 0, 0.5);
    }

    .queue-cover .eq i {
      height: 14px;
    }

    .queue-text {
      flex: 1;
      min-width: 0;
      display: flex;
      flex-direction: column;
    }

    .queue-text strong {
      font-size: var(--fs-meta);
      font-weight: var(--fw-semibold);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .queue-text small {
      color: var(--text-3);
      font-size: var(--fs-small);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .queue-duration {
      flex-shrink: 0;
      color: var(--text-3);
      font-size: var(--fs-small);
    }

    .queue-item .queue-more {
      opacity: 0;
    }

    .queue-item:hover .queue-more,
    .queue-item:focus-within .queue-more {
      opacity: 1;
    }
  }
</style>
````

## File: src/lib/layout/Sidebar.svelte
````svelte
<script lang="ts">
  import {
    House,
    Clock3,
    Music2,
    Disc3,
    MicVocal,
    ListMusic,
    Plus,
    Settings2,
    HardDrive,
  } from 'lucide-svelte';
  import { nav, go } from '../stores/nav.svelte';
  import { library } from '../stores/library.svelte';
  import { openMenu } from '../stores/ui.svelte';
  import { playlistMenu, localMenu, newPlaylist } from '../actions';
  import { tooltip } from '../components/tooltip';
  import { LOCAL_COLLECTION, shadowsBuiltIn } from '../utils/collections';

  let active = $derived(nav.route.view);
  let libraryActive = $derived(
    active === 'songs' ||
      active === 'albums' ||
      active === 'artists' ||
      active === 'album' ||
      active === 'artist',
  );
  const top = [
    { view: 'home', label: '音乐空间', icon: House },
    { view: 'recent', label: '最近添加', icon: Clock3 },
  ] as const;
  let sections = $derived([
    { view: 'songs', label: '歌曲', icon: Music2, count: library.summary.tracks, match: ['songs'] },
    {
      view: 'albums',
      label: '专辑',
      icon: Disc3,
      count: library.summary.albums.length,
      match: ['albums', 'album'],
    },
    {
      view: 'artists',
      label: '艺术家',
      icon: MicVocal,
      count: library.summary.artists.length,
      match: ['artists', 'artist'],
    },
  ] as const);
</script>

<aside class="sidebar" aria-label="侧边栏">
  <button
    type="button"
    class="brand"
    onclick={() => go({ view: 'home' })}
    aria-label="Music Mine 音乐空间"
  >
    <span class="brand-mark" aria-hidden="true"><Music2 size={18} strokeWidth={2.4} /></span>
    <span class="brand-name">Music Mine</span>
  </button>

  <nav class="nav" aria-label="主导航">
    <ul class="nav-group">
      {#each top as item (item.view)}
        <li>
          <button
            type="button"
            class="nav-item"
            class:active={active === item.view}
            aria-current={active === item.view ? 'page' : undefined}
            onclick={() => go({ view: item.view })}
          >
            <item.icon size={18} /><span class="nav-label">{item.label}</span>
          </button>
        </li>
      {/each}
    </ul>

    <div class="nav-heading" class:active-group={libraryActive}>音乐库</div>
    <ul class="nav-group">
      {#each sections as item (item.view)}
        {@const selected = (item.match as readonly string[]).includes(active)}
        <li>
          <button
            type="button"
            class="nav-item"
            class:active={selected}
            aria-current={selected ? 'page' : undefined}
            onclick={() => go({ view: item.view })}
          >
            <item.icon size={18} /><span class="nav-label">{item.label}</span
            >{#if library.loaded}<span class="nav-count">{item.count}</span>{/if}
          </button>
        </li>
      {/each}
    </ul>

    <div class="nav-heading nav-heading-row">
      <span>我的歌单</span>
      <button
        type="button"
        class="icon-btn icon-btn-sm"
        aria-label="新建歌单"
        use:tooltip={'新建歌单'}
        onclick={() => newPlaylist()}><Plus size={16} /></button
      >
    </div>
    <ul class="nav-group nav-playlists" aria-label="我的歌单">
      <li>
        <button
          type="button"
          class="nav-item"
          class:active={active === 'local'}
          aria-current={active === 'local' ? 'page' : undefined}
          onclick={() => go({ view: 'local' })}
          oncontextmenu={(e) =>
            openMenu(e, localMenu(), '系统歌单「' + LOCAL_COLLECTION.name + '」操作')}
          use:tooltip={'系统歌单：自动包含音乐库中的全部歌曲'}
        >
          <HardDrive size={18} /><span class="nav-label">{LOCAL_COLLECTION.name}</span
          >{#if library.loaded}<span class="nav-count">{library.summary.tracks}</span>{/if}
        </button>
      </li>
      {#each library.summary.playlists as list (list.id)}
        {@const selected = nav.route.view === 'playlist' && nav.route.id === list.id}
        <li>
          <button
            type="button"
            class="nav-item"
            class:active={selected}
            aria-current={selected ? 'page' : undefined}
            onclick={() => go({ view: 'playlist', id: list.id })}
            oncontextmenu={(e) => openMenu(e, playlistMenu(list), '歌单「' + list.name + '」操作')}
          >
            <ListMusic size={18} /><span
              class="nav-label"
              use:tooltip={{ text: list.name, overflow: true }}>{list.name}</span
            >
            {#if shadowsBuiltIn(list.name)}<span
                class="nav-tag"
                use:tooltip={'你创建的同名普通歌单，内容与系统歌单无关'}>自建</span
              >{/if}
            <span class="nav-count">{list.count}</span>
          </button>
        </li>
      {/each}
      {#if library.loaded && !library.summary.playlists.length}
        <li>
          <button type="button" class="nav-item nav-item-muted" onclick={() => newPlaylist()}
            ><Plus size={18} /><span class="nav-label">新建歌单</span></button
          >
        </li>
      {/if}
    </ul>
  </nav>

  <div class="sidebar-footer">
    <button
      type="button"
      class="nav-item"
      class:active={active === 'settings'}
      aria-current={active === 'settings' ? 'page' : undefined}
      onclick={() => go({ view: 'settings' })}
    >
      <Settings2 size={18} /><span class="nav-label">设置</span>
    </button>
  </div>
</aside>

<style>
  :global {
    /* ============================================================ sidebar */
    .sidebar {
      grid-column: 1;
      grid-row: 1;
      min-height: 0;
      display: flex;
      flex-direction: column;
      background: var(--bg-sidebar);
      padding: var(--space-3) var(--space-2) var(--space-2);
      overflow: hidden;
    }

    .brand {
      display: flex;
      align-items: center;
      gap: 10px;
      height: 40px;
      padding: 0 10px;
      margin-bottom: var(--space-3);
      border-radius: var(--radius);
      font-family: var(--font-display);
      font-size: 17px;
      font-weight: 700;
      letter-spacing: -0.2px;
      flex-shrink: 0;
    }

    .brand-mark {
      display: grid;
      place-items: center;
      width: 28px;
      height: 28px;
      border-radius: var(--radius);
      background: var(--accent);
      color: var(--accent-ink);
    }

    .nav {
      flex: 1;
      min-height: 0;
      display: flex;
      flex-direction: column;
    }

    .nav-group {
      display: flex;
      flex-direction: column;
      gap: 2px;
      flex-shrink: 0;
    }

    .nav-heading {
      display: flex;
      align-items: center;
      justify-content: space-between;
      min-height: 30px;
      margin-top: var(--space-4);
      padding: 0 6px 0 12px;
      color: var(--text-3);
      font-size: var(--fs-small);
      font-weight: var(--fw-semibold);
      flex-shrink: 0;
    }

    .nav-heading.active-group {
      color: var(--text-2);
    }

    .nav-item {
      position: relative;
      display: flex;
      align-items: center;
      gap: 12px;
      width: 100%;
      height: 36px;
      padding: 0 10px 0 12px;
      border-radius: var(--radius);
      color: var(--text-2);
      font-size: var(--fs-body);
      text-align: left;
      transition:
        background-color var(--dur-fast) var(--ease),
        color var(--dur-fast) var(--ease);
    }

    .nav-item > svg {
      flex-shrink: 0;
    }

    .nav-item:hover {
      background: var(--hover);
      color: var(--text);
    }

    .nav-item.active {
      background: var(--selected);
      color: var(--text);
      font-weight: var(--fw-semibold);
    }

    .nav-item.active::before {
      content: '';
      position: absolute;
      left: 0;
      top: 10px;
      bottom: 10px;
      width: 3px;
      border-radius: 0 3px 3px 0;
      background: var(--accent);
    }

    .nav-item.active > svg {
      color: var(--accent);
    }

    .nav-item-muted {
      color: var(--text-3);
    }

    .nav-label {
      flex: 1;
      min-width: 0;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .nav-tag {
      flex-shrink: 0;
      padding: 0 6px;
      border-radius: 999px;
      background: var(--surface-hover);
      color: var(--text-3);
      font-size: 11px;
      line-height: 18px;
    }

    .nav-count {
      flex-shrink: 0;
      color: var(--text-3);
      font-size: var(--fs-small);
      font-weight: 400;
      font-variant-numeric: tabular-nums;
    }

    /* Only the playlist list scrolls; brand, library groups and settings stay put. */
    .nav-playlists {
      flex: 1 1 auto;
      min-height: 72px;
      overflow-y: auto;
      overflow-x: hidden;
      margin-right: -4px;
      padding-right: 4px;
      padding-bottom: var(--space-2);
      flex-shrink: 1;
      mask-image: linear-gradient(to bottom, #000 calc(100% - 16px), transparent);
    }

    .sidebar-footer {
      flex-shrink: 0;
      padding-top: var(--space-2);
      border-top: 1px solid var(--line);
    }
  }
</style>
````

## File: src/lib/layout/TopBar.svelte
````svelte
<script lang="ts">
  import { ChevronLeft, ChevronRight, Search, X } from 'lucide-svelte';
  import { nav, go, back, forward } from '../stores/nav.svelte';
  import { library } from '../stores/library.svelte';
  import { player } from '../stores/player.svelte';
  import { ui } from '../stores/ui.svelte';
  import IconButton from '../components/IconButton.svelte';
  import { tooltip } from '../components/tooltip';
  import { LOCAL_COLLECTION } from '../utils/collections';

  let { searchInput = $bindable() }: { searchInput?: HTMLInputElement } = $props();

  let crumbs = $derived.by(() => {
    const r = nav.route;
    switch (r.view) {
      case 'home':
        return { parent: null, title: '音乐空间' };
      case 'recent':
        return { parent: null, title: '最近添加' };
      case 'songs':
        return { parent: '音乐库', title: '歌曲' };
      case 'albums':
        return { parent: '音乐库', title: '专辑' };
      case 'artists':
        return { parent: '音乐库', title: '艺术家' };
      case 'album':
        return {
          parent: '专辑',
          title: library.summary.albums.find((a) => a.key === r.key)?.name ?? '专辑',
        };
      case 'artist':
        return { parent: '艺术家', title: r.name || '未知艺术家' };
      case 'playlist':
        return {
          parent: '我的歌单',
          title: library.summary.playlists.find((p) => p.id === r.id)?.name ?? '歌单',
        };
      case 'local':
        return { parent: '我的歌单', title: LOCAL_COLLECTION.name };
      case 'settings':
        return { parent: null, title: '设置' };
    }
  });
  const parentRoute = () => {
    const v = nav.route.view;
    if (v === 'album') go({ view: 'albums' });
    else if (v === 'artist') go({ view: 'artists' });
    else go({ view: 'songs' });
  };

  function input() {
    const v = nav.route.view;
    if (ui.query.trim() && (v === 'home' || v === 'settings')) go({ view: 'songs' });
  }
  function key(e: KeyboardEvent) {
    if (e.isComposing) return;
    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      if (ui.query) ui.query = '';
      else searchInput?.blur();
    }
  }
</script>

<header class="topbar">
  <div class="history-buttons">
    <IconButton label="后退" disabled={!nav.canBack} onclick={back}
      ><ChevronLeft size={18} /></IconButton
    >
    <IconButton label="前进" disabled={!nav.canForward} onclick={forward}
      ><ChevronRight size={18} /></IconButton
    >
  </div>
  <div class="crumbs">
    {#if crumbs.parent}<button type="button" class="crumb-parent" onclick={parentRoute}
        >{crumbs.parent}</button
      ><span class="crumb-sep" aria-hidden="true">/</span>{/if}
    <h1 class="crumb-title" use:tooltip={{ text: crumbs.title, overflow: true }}>{crumbs.title}</h1>
  </div>
  <div class="search" role="search">
    <Search size={16} aria-hidden="true" />
    <input
      bind:this={searchInput}
      bind:value={ui.query}
      type="search"
      placeholder="搜索歌曲、艺术家或专辑"
      aria-label="搜索歌曲、艺术家或专辑"
      autocomplete="off"
      spellcheck="false"
      oninput={input}
      onkeydown={key}
    />
    {#if ui.query}<button
        type="button"
        class="search-clear"
        aria-label="清除搜索"
        onclick={() => {
          ui.query = '';
          searchInput?.focus();
        }}><X size={14} /></button
      >
    {:else}<kbd aria-hidden="true">Ctrl F</kbd>{/if}
  </div>
  <span class="local-status" class:offline={!player.connected} use:tooltip={player.connection}>
    <span class="status-dot" aria-hidden="true"></span><span class="local-label">本地模式</span>
  </span>
</header>

<style>
  :global {
    /* ============================================================ topbar */
    .topbar {
      display: flex;
      align-items: center;
      gap: var(--space-3);
      padding: 0 var(--page-x) 0 calc(var(--page-x) - 8px);
      min-width: 0;
    }

    .history-buttons {
      display: flex;
      gap: 2px;
      flex-shrink: 0;
    }

    .crumbs {
      display: flex;
      align-items: baseline;
      gap: 8px;
      min-width: 0;
      flex: 1;
    }

    .crumb-parent {
      color: var(--text-3);
      font-size: var(--fs-meta);
      white-space: nowrap;
      border-radius: var(--radius-xs);
    }

    .crumb-parent:hover {
      color: var(--text);
    }

    .crumb-sep {
      color: var(--text-3);
      font-size: var(--fs-meta);
    }

    .crumb-title {
      font-size: 15px;
      font-weight: var(--fw-semibold);
      min-width: 0;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .search {
      display: flex;
      align-items: center;
      gap: 8px;
      width: clamp(200px, 28vw, 340px);
      height: 34px;
      padding: 0 6px 0 12px;
      border-radius: 999px;
      background: var(--surface);
      color: var(--text-3);
      border: 1px solid var(--field-line);
      transition:
        background-color var(--dur-fast),
        border-color var(--dur-fast);
      flex-shrink: 1;
      min-width: 150px;
    }

    .search:hover {
      background: var(--surface-hover);
    }

    .search:focus-within {
      background: var(--surface-hover);
      border-color: var(--accent-line);
      color: var(--text-2);
    }

    .search input {
      flex: 1;
      min-width: 0;
      height: 100%;
      background: none;
      border: 0;
      outline: none;
      font-size: var(--fs-meta);
      color: var(--text);
    }

    .search input::placeholder {
      color: var(--text-3);
    }

    .search input::-webkit-search-cancel-button {
      display: none;
    }

    .search kbd {
      margin-right: 4px;
    }

    .search-clear {
      display: grid;
      place-items: center;
      width: 24px;
      height: 24px;
      border-radius: 50%;
      color: var(--text-2);
    }

    .search-clear:hover {
      background: var(--surface-active);
      color: var(--text);
    }

    .local-status {
      display: flex;
      align-items: center;
      gap: 6px;
      flex-shrink: 0;
      color: var(--text-3);
      font-size: var(--fs-small);
    }

    .status-dot {
      width: 7px;
      height: 7px;
      border-radius: 50%;
      background: var(--accent);
      box-shadow: 0 0 0 3px var(--accent-soft);
    }

    .local-status.offline .status-dot {
      background: var(--text-3);
      box-shadow: none;
    }
  }
</style>
````

## File: src/lib/services/music.ts
````typescript
import { invoke, isTauri } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import type { Theme, ThemeChoice } from '../utils/theme';
import {
  createCoverLoader,
  type CoverOutcome,
  type CoverPriority,
  type CoverStatus,
  type CoverTicket,
} from '../utils/cover-loader';
export type { CoverOutcome, CoverPriority, CoverStatus, CoverTicket };
export interface Track {
  path: string;
  title: string;
  artist: string;
  album: string;
  duration_ms: number;
}
export interface PlayerSnapshot {
  revision: number;
  track_id: string | null;
  current_track: Track | null;
  playback_state: 'idle' | 'playing' | 'paused';
  position_ms: number;
  duration_ms: number;
  volume: number;
  playback_mode: PlaybackMode;
  lyrics_offset_ms: number;
  output_device_id: string | null;
  error: string | null;
  queue: Track[];
}
interface BridgeReply {
  nonce: string;
  backend: string;
  snapshot: PlayerSnapshot;
}
export const desktopAvailable = isTauri();
export async function verifyConnection() {
  if (!desktopAvailable) throw new Error('浏览器预览 · 请在桌面应用中使用本地播放器');
  const nonce = crypto.randomUUID();
  const reply = await invoke<BridgeReply>('bridge_probe', { nonce });
  if (reply.nonce !== nonce || reply.backend !== 'rust') throw new Error('连接响应校验失败');
  return reply;
}
export const getPlayerState = () => invoke<PlayerSnapshot>('player_get_state');
export const openFile = () => invoke<PlayerSnapshot>('player_open_file');
export const play = () => invoke<PlayerSnapshot>('player_play');
export const pause = () => invoke<PlayerSnapshot>('player_pause');
export const stop = () => invoke<PlayerSnapshot>('player_stop');
export const seek = (positionMs: number) => invoke<PlayerSnapshot>('player_seek', { positionMs });
export const setVolume = (volume: number) =>
  invoke<PlayerSnapshot>('player_set_volume', { volume });
export interface LibraryTrack extends Track {
  id: number;
  album_artist: string;
  genre: string;
  year: number | null;
  track_number: number | null;
  disc_number: number | null;
  added_at: number;
  album_source: string;
}
export interface Group {
  key: string;
  name: string;
  artist: string;
  count: number;
  cover_id: number;
}
export interface Playlist {
  id: number;
  name: string;
  count: number;
}
export interface LibrarySummary {
  tracks: number;
  albums: Group[];
  artists: Group[];
  folders: string[];
  playlists: Playlist[];
}
export interface LibraryQuery {
  search?: string;
  sort?: string;
  offset?: number;
  limit?: number;
  playlist?: number | null;
  album?: string | null;
  artist?: string | null;
}
// Rust still returns stored favourite IDs for database compatibility; the UI no longer reads them.
export interface LibraryPage {
  tracks: LibraryTrack[];
  total: number;
}
export interface ScanProgress {
  running: boolean;
  visited: number;
  imported: number;
  current: string;
  cancelled: boolean;
  errors: string[];
}
export const queryLibrary = (query: LibraryQuery) =>
  invoke<LibraryPage>('library_query', { query });
export const librarySummary = () => invoke<LibrarySummary>('library_summary');
export const scanFolder = () => {
  clearCoverCaches();
  return invoke<void>('library_scan');
};
export const scanProgress = () => invoke<ScanProgress>('library_progress');
export const cancelScan = () => invoke<void>('library_cancel');
export const trackDetails = (id: number) => invoke<LibraryTrack>('library_track', { id });
export interface AudioInfo {
  format: string;
  sample_rate: number | null;
  channels: number | null;
  bitrate: number | null;
}
export const audioInfo = (id: number) => invoke<AudioInfo>('library_audio_info', { id });
// Every library cover request goes through one bounded loader: null results mean "no cover" and are
// kept; error codes (file missing or unreadable, IPC failure) get limited retries and are never kept as null.
const covers = createCoverLoader({ terminal: (code) => code === 'cover-not-in-library' });
// Counts and durations only; no paths or image data.
Object.assign(globalThis, { __musicMineCoverStats: covers.stats });
const noTicket: CoverTicket = { setPriority() {}, cancel() {} };
const thumbEdge = (px: number) => Math.max(32, Math.min(960, Math.round(px)));
const thumbKey = (id: number, px: number) => 't:' + id + '@' + thumbEdge(px);
export const peekTrackThumb = (id: number, px: number) => covers.peek(thumbKey(id, px));
/** Small and medium covers. Rust resizes; the page does not receive the original image. */
export function requestTrackThumb(
  id: number,
  px: number,
  priority: CoverPriority,
  done: (outcome: CoverOutcome) => void,
) {
  const edge = thumbEdge(px);
  return covers.request(
    thumbKey(id, px),
    priority,
    () => invoke<string | null>('library_thumb', { id, edge }),
    done,
  );
}
export function trackThumb(id: number, px: number) {
  return new Promise<string | null>((resolve, reject) => {
    requestTrackThumb(id, px, 1, (o) =>
      o.status === 'failed' ? reject(new Error(o.code)) : resolve(o.src),
    );
  });
}
/** Song details use a larger JPEG, still bounded, instead of the embedded original. */
export function trackCover(id: number) {
  return trackThumb(id, 720);
}
export const coverStats = () => invoke<Record<string, number>>('library_cover_stats');
export const homeData = () =>
  invoke<{ recent: LibraryTrack[]; artists: [string, number][] }>('library_home');
export const playTrack = (id: number) => invoke<PlayerSnapshot>('library_play', { id });
export const createPlaylist = (name: string) =>
  invoke<number | null>('library_playlist', { name, playlistId: null, trackId: null }).then(
    (id) => {
      if (id == null) throw new Error('创建歌单失败');
      return id;
    },
  );
export const addToPlaylist = (playlistId: number, trackId: number) =>
  invoke<number | null>('library_playlist', { name: null, playlistId, trackId }).then(() => {});
// Playlist edits address rows by ID only; library tracks and audio files are never touched.
export const renamePlaylist = (playlistId: number, name: string) =>
  invoke<void>('library_playlist_edit', { action: 'rename', playlistId, name, trackId: null });
export const deletePlaylist = (playlistId: number) =>
  invoke<void>('library_playlist_edit', {
    action: 'delete',
    playlistId,
    name: null,
    trackId: null,
  });
export const removeFromPlaylist = (playlistId: number, trackId: number) =>
  invoke<void>('library_playlist_edit', {
    action: 'remove_track',
    playlistId,
    name: null,
    trackId,
  });
export const importLegacy = () =>
  invoke<{ tracks: number; playlists: number; entries: number; warnings: string[] } | null>(
    'library_import',
  );
export const queueAction = (
  action: 'add' | 'play_next' | 'remove' | 'next' | 'previous' | 'clear',
  id: number | null = null,
  path: string | null = null,
) => invoke<PlayerSnapshot>('library_queue', { action, id, path });
export const queueTracks = (ids: number[], replace: boolean) =>
  invoke<PlayerSnapshot>('library_queue_tracks', { ids, replace });
export async function collectTrackIds(query: LibraryQuery): Promise<number[]> {
  const ids: number[] = [];
  let offset = 0;
  while (true) {
    const page = await queryLibrary({ ...query, offset, limit: 200 });
    ids.push(...page.tracks.map((t) => t.id));
    offset += page.tracks.length;
    if (!page.tracks.length || offset >= page.total || ids.length >= 20000) return ids;
  }
}

export type PlaybackMode = 'sequential' | 'shuffle' | 'repeat_one' | 'repeat_all';
export interface Lyrics {
  lines: { time_ms: number; text: string }[];
  offset_ms: number;
  synced: boolean;
  source: string;
}
export const getLyrics = () => invoke<Lyrics | null>('player_lyrics');
/** `null` is "no cover"; a transient error (file busy or unreadable) is retried once. */
export async function getArtwork() {
  try {
    return await invoke<string | null>('player_artwork');
  } catch {
    covers.stats.retries++;
    await new Promise((r) => setTimeout(r, 800));
    return invoke<string | null>('player_artwork');
  }
}
const configure = (
  mode: PlaybackMode | null,
  offset: number | null,
  path: string | null,
  target: number | null,
) => invoke<PlayerSnapshot>('player_configure', { mode, offset, path, target });
export const setMode = (mode: PlaybackMode) => configure(mode, null, null, null);
export const setLyricsOffset = (offset: number) => configure(null, offset, null, null);
export const reorderQueue = (path: string, target: number) => configure(null, null, path, target);

export const playQueued = (path: string) => configure(null, null, path, null);

export interface DesktopPreferences {
  close_to_tray: boolean;
  queue_open: boolean;
  theme: string;
}
export const desktopPreferences = (
  closeToTray: boolean | null = null,
  queueOpen: boolean | null = null,
  theme: ThemeChoice | null = null,
) => invoke<DesktopPreferences>('desktop_preferences', { closeToTray, queueOpen, theme });
/** Native title bar colour; `system` hands control back to Windows. */
export const setWindowTheme = (choice: ThemeChoice) =>
  getCurrentWindow().setTheme(choice === 'system' ? null : choice);
export const windowTheme = () => getCurrentWindow().theme();
export const onWindowThemeChanged = (handler: (theme: Theme) => void) =>
  getCurrentWindow().onThemeChanged(({ payload }) => handler(payload));

export const mediaStatus = () => invoke<string>('media_status');

// Exact library path only. A path that is not a library row fails with `cover-not-in-library`.
const queueKey = (path: string) => 'q:' + path + '@112';
export const peekQueueCover = (path: string): CoverOutcome | undefined =>
  desktopAvailable ? covers.peek(queueKey(path)) : { status: 'none', src: null };
export function requestQueueCover(
  path: string,
  priority: CoverPriority,
  done: (outcome: CoverOutcome) => void,
): CoverTicket {
  if (!desktopAvailable) {
    done({ status: 'none', src: null });
    return noTicket;
  }
  return covers.request(
    queueKey(path),
    priority,
    () => invoke<string | null>('library_queue_thumb', { path, edge: 112 }),
    done,
  );
}
export function clearCoverCaches() {
  covers.clear();
}
````

## File: src/lib/stores/library.svelte.ts
````typescript
import {
  desktopAvailable,
  librarySummary,
  scanProgress,
  clearCoverCaches,
  type LibrarySummary,
  type ScanProgress,
} from '../services/music';
import { notifyError } from './ui.svelte';

export const library = $state({
  summary: { tracks: 0, albums: [], artists: [], folders: [], playlists: [] } as LibrarySummary,
  loaded: false,
  /** Bumped after any change to tracks or playlists so open lists (including the built-in “本地”) reload. */
  revision: 0,
  progress: null as ScanProgress | null,
  /** Bumped only when a scan finishes, so visible covers reload without a theme or progress refresh. */
  coverEpoch: 0,
});

export async function refreshSummary() {
  if (!desktopAvailable) return;
  try {
    library.summary = await librarySummary();
    library.loaded = true;
    library.revision++;
  } catch (e) {
    notifyError(e);
  }
}

/** Polls scan progress and refreshes the library when a scan finishes. */
export function watchScans() {
  if (!desktopAvailable) return () => {};
  let disposed = false;
  let timer: ReturnType<typeof setTimeout>;
  async function poll() {
    try {
      const next = await scanProgress();
      const was = library.progress?.running;
      library.progress = next;
      if (was && !next.running) {
        clearCoverCaches();
        library.coverEpoch++;
        await refreshSummary();
      }
    } catch (e) {
      if (!disposed) notifyError(e);
    }
    if (!disposed) timer = setTimeout(poll, library.progress?.running ? 500 : 2000);
  }
  void poll();
  return () => {
    disposed = true;
    clearTimeout(timer);
  };
}
````

## File: src/lib/stores/nav.svelte.ts
````typescript
import { createHistory } from '../utils/history';

export type Route =
  | { view: 'home' }
  | { view: 'songs' }
  | { view: 'recent' }
  | { view: 'albums' }
  | { view: 'artists' }
  | { view: 'album'; key: string }
  | { view: 'artist'; name: string }
  | { view: 'playlist'; id: number }
  /** Built-in “本地” collection: the whole library. */
  | { view: 'local' }
  | { view: 'settings' };

const history = createHistory<Route>({ view: 'home' });

export const nav = $state({ route: history.current as Route, canBack: false, canForward: false });

function sync() {
  nav.route = history.current;
  nav.canBack = history.canBack;
  nav.canForward = history.canForward;
}

export function go(route: Route) {
  history.push(route);
  sync();
}
export function back() {
  history.back();
  sync();
}
export function forward() {
  history.forward();
  sync();
}
export function forgetPlaylist(id: number) {
  history.forget((r) => r.view === 'playlist' && r.id === id, { view: 'songs' });
  sync();
}

/** Stable key for per-page UI memory such as scroll position. */
export function routeKey(route: Route) {
  switch (route.view) {
    case 'album':
      return 'album:' + route.key;
    case 'artist':
      return 'artist:' + route.name;
    case 'playlist':
      return 'playlist:' + route.id;
    default:
      return route.view;
  }
}

/** Remembered per-route list state (scroll offset and sort) for the session. */
export const memory = new Map<string, { scroll: number; sort?: string }>();
````

## File: src/lib/stores/player.svelte.ts
````typescript
import type { PlayerSnapshot } from '../services/music';
import { notify, notifyError } from './ui.svelte';

/** Read-only copy of Rust's player state; Rust remains the only authority. */
export const player = $state({
  snapshot: null as PlayerSnapshot | null,
  busy: false,
  connection: '尚未验证',
  connected: false,
});

export function accept(next: PlayerSnapshot) {
  const current = player.snapshot;
  if (current && next.revision < current.revision) return;
  player.snapshot = next;
  if (next.error && next.error !== current?.error) notify(next.error, 'error');
}

/** Runs one player command at a time and applies its snapshot. */
export async function control(action: () => Promise<PlayerSnapshot>) {
  if (player.busy) return false;
  player.busy = true;
  try {
    accept(await action());
    return true;
  } catch (e) {
    notifyError(e);
    return false;
  } finally {
    player.busy = false;
  }
}

/** Like control, but not blocked by an in-flight command (library actions). */
export async function perform(action: () => Promise<PlayerSnapshot>, success?: string) {
  try {
    accept(await action());
    if (success) notify(success);
    return true;
  } catch (e) {
    notifyError(e);
    return false;
  }
}
````

## File: src/lib/stores/preferences.ts
````typescript
import { ui, notifyError } from './ui.svelte';
import {
  desktopAvailable,
  desktopPreferences,
  setWindowTheme,
  windowTheme,
  onWindowThemeChanged,
} from '../services/music';
import {
  THEME_KEY,
  THEME_COLOR,
  parseTheme,
  resolveTheme,
  type Theme,
  type ThemeChoice,
} from '../utils/theme';

const KEY = 'music-mine-reduce-motion';

const systemDark =
  typeof matchMedia === 'function' ? matchMedia('(prefers-color-scheme: dark)') : null;

/** Paint the resolved theme; `system` is the theme Windows reported for this window, when known. */
function paint(system?: Theme) {
  const theme = resolveTheme(ui.theme, system ? system === 'dark' : (systemDark?.matches ?? true));
  const root = document.documentElement;
  root.dataset.theme = theme;
  root.style.colorScheme = theme;
  document.querySelector('meta[name="theme-color"]')?.setAttribute('content', THEME_COLOR[theme]);
}

/** Bring the native title bar in line with the page; in `system` mode Windows then reports the real theme. */
async function syncWindow(choice: ThemeChoice) {
  if (!desktopAvailable) return;
  await setWindowTheme(choice);
  if (choice === 'system' && ui.theme === 'system') paint((await windowTheme()) ?? undefined);
}

export function loadPreferences() {
  try {
    ui.reduceMotion = localStorage.getItem(KEY) === '1';
    ui.theme = parseTheme(localStorage.getItem(THEME_KEY));
  } catch {
    /* storage unavailable: keep defaults */
  }
  paint();
  syncWindow(ui.theme).catch(notifyError);
}

export function setTheme(choice: ThemeChoice) {
  ui.theme = choice;
  try {
    localStorage.setItem(THEME_KEY, choice);
  } catch {
    /* keep for this session */
  }
  paint();
  syncWindow(choice).catch(notifyError);
  // Rust applies the saved theme to the window before the page loads on the next start.
  if (desktopAvailable) desktopPreferences(null, null, choice).catch(notifyError);
}

/** Follow Windows light/dark changes while the user has chosen `system`. Returns a cleanup function. */
export function watchSystemTheme() {
  const onMedia = () => {
    if (ui.theme === 'system') paint();
  };
  systemDark?.addEventListener('change', onMedia);
  let unlisten: (() => void) | undefined;
  let disposed = false;
  if (desktopAvailable)
    onWindowThemeChanged((t) => {
      if (ui.theme === 'system') paint(t);
    })
      .then((u) => {
        if (disposed) u();
        else unlisten = u;
      })
      .catch(notifyError);
  return () => {
    disposed = true;
    systemDark?.removeEventListener('change', onMedia);
    unlisten?.();
  };
}

export function setReduceMotion(on: boolean) {
  ui.reduceMotion = on;
  try {
    localStorage.setItem(KEY, on ? '1' : '0');
  } catch {
    /* keep for this session */
  }
}

const systemReduced =
  typeof matchMedia === 'function' ? matchMedia('(prefers-reduced-motion: reduce)') : null;

/** Transition duration honouring both the in-app setting and the Windows animation preference. */
export function motionMs(ms: number) {
  return ui.reduceMotion || systemReduced?.matches ? 0 : ms;
}
````

## File: src/lib/stores/ui.svelte.ts
````typescript
import type { Play } from 'lucide-svelte';
import { errorText } from '../utils/format';
import type { LibraryTrack } from '../services/music';
import type { ThemeChoice } from '../utils/theme';

/** Any Lucide icon component (lucide-svelte ships class-style component types). */
export type IconComponent = typeof Play;

export type MenuItem =
  | {
      kind?: 'item';
      label: string;
      icon?: IconComponent;
      action: () => unknown;
      danger?: boolean;
      disabled?: boolean;
      checked?: boolean;
      hint?: string;
    }
  | { kind: 'separator' }
  | { kind: 'heading'; label: string };

export interface MenuState {
  id: number;
  label: string;
  items: MenuItem[];
  /** Pointer position, or an anchor rectangle for keyboard/button invocations. */
  at: { x: number; y: number } | DOMRect;
  returnFocus: HTMLElement | null;
}

interface ConfirmState {
  kind: 'confirm';
  title: string;
  message: string;
  detail?: string;
  confirmLabel: string;
  danger: boolean;
  resolve: (ok: boolean) => void;
}
interface PromptState {
  kind: 'prompt';
  title: string;
  label: string;
  value: string;
  confirmLabel: string;
  validate: (value: string) => string | null;
  submit: (value: string) => Promise<void>;
  resolve: (ok: boolean) => void;
}
export type DialogState = ConfirmState | PromptState;

export const ui = $state({
  menu: null as MenuState | null,
  dialog: null as DialogState | null,
  toast: null as { id: number; message: string; tone: 'info' | 'error' } | null,
  query: '',
  queueOpen: false,
  immersive: false,
  reduceMotion: false,
  theme: 'dark' as ThemeChoice,
  /** Track whose details dialog is open. */
  details: null as LibraryTrack | null,
});

let menuId = 0;
export function openMenu(trigger: MouseEvent | HTMLElement, items: MenuItem[], label = '操作菜单') {
  let at: MenuState['at'];
  let returnFocus: HTMLElement | null;
  if (trigger instanceof MouseEvent) {
    trigger.preventDefault();
    const target = trigger.currentTarget instanceof HTMLElement ? trigger.currentTarget : null;
    // Keyboard-generated contextmenu events (Shift+F10, Menu key) report 0,0.
    at =
      trigger.clientX || trigger.clientY
        ? { x: trigger.clientX, y: trigger.clientY }
        : (target?.getBoundingClientRect() ?? { x: 0, y: 0 });
    returnFocus = target?.matches(':focus-within')
      ? (document.activeElement as HTMLElement)
      : target;
  } else {
    at = trigger.getBoundingClientRect();
    returnFocus = trigger;
  }
  ui.menu = { id: ++menuId, label, items, at, returnFocus };
}
export function closeMenu(restoreFocus = false) {
  const menu = ui.menu;
  ui.menu = null;
  if (restoreFocus && menu?.returnFocus?.isConnected) menu.returnFocus.focus();
}

export function confirmAction(options: {
  title: string;
  message: string;
  detail?: string;
  confirmLabel?: string;
  danger?: boolean;
}) {
  return new Promise<boolean>((resolve) => {
    ui.dialog?.resolve(false);
    ui.dialog = { kind: 'confirm', confirmLabel: '确定', danger: false, ...options, resolve };
  });
}
export function promptText(options: {
  title: string;
  label: string;
  value?: string;
  confirmLabel?: string;
  validate?: (value: string) => string | null;
  submit: (value: string) => Promise<void>;
}) {
  return new Promise<boolean>((resolve) => {
    ui.dialog?.resolve(false);
    ui.dialog = {
      kind: 'prompt',
      value: '',
      confirmLabel: '保存',
      validate: () => null,
      ...options,
      resolve,
    };
  });
}

let toastId = 0;
let toastTimer: ReturnType<typeof setTimeout> | undefined;
export function notify(message: string, tone: 'info' | 'error' = 'info') {
  if (ui.toast?.message === message && ui.toast.tone === tone) return;
  ui.toast = { id: ++toastId, message, tone };
  clearTimeout(toastTimer);
  if (tone === 'info')
    toastTimer = setTimeout(() => {
      ui.toast = null;
    }, 4000);
}
export function notifyError(e: unknown) {
  notify(errorText(e), 'error');
}
export function dismissToast() {
  clearTimeout(toastTimer);
  ui.toast = null;
}
````

## File: src/lib/styles/tokens.css
````css
/* Music Mine design tokens. Components read only these variables; no colour is hard-coded elsewhere.
   :root is the dark theme (default); [data-theme=light] overrides the palette, set before first paint by public/theme-init.js. */
:root {
  color-scheme: dark;

  /* Neutral surfaces: hierarchy comes from lightness, not borders. */
  --bg: #121414;
  --bg-sidebar: #090b0c;
  --bg-player: #0d0f0f;
  --surface: #1c2220;
  --surface-hover: #272f2a;
  --surface-active: #2f3933;
  --surface-overlay: #1a1f1d;
  --scrim: rgba(4, 6, 6, 0.62);
  --line: rgba(232, 236, 231, 0.08);
  --line-strong: rgba(232, 236, 231, 0.14);

  /* Interaction fills and layers (semantic; each theme maps them to its own surfaces). */
  --hover: #1c2220;
  --selected: #272f2a;
  --field-line: transparent;
  --bg-drawer: #0f1211;
  --scrim-soft: rgba(4, 6, 6, 0.34);

  /* Text (contrast on --bg: 15:1 / 8:1 / 5.6:1; --text-3 stays ≥4.5:1 on --hover). */
  --text: #e8ece7;
  --text-2: #a9b3aa;
  --text-3: #8d988f;
  --lyric-dim: 0.42;
  --lyric-past: 0.3;

  /* One brand accent, used for playback, progress, selection and the primary action. */
  --accent: #b9d69b;
  --accent-hover: #cde5b4;
  --accent-ink: #142011;
  --accent-soft: rgba(185, 214, 155, 0.13);
  --accent-line: rgba(185, 214, 155, 0.42);
  --focus: #cfe7b5;

  --danger: #f0998e;
  --danger-bg: #c4473a;
  --danger-bg-hover: #d4574a;
  --danger-soft: rgba(240, 153, 142, 0.12);

  /* Typography: Windows system fonts only; CJK fallbacks for zh/ja/ko. */
  --font-sans:
    'Segoe UI Variable Text', 'Segoe UI', 'Microsoft YaHei UI', 'Microsoft YaHei', 'Malgun Gothic',
    'Yu Gothic UI', 'Meiryo UI', system-ui, sans-serif;
  --font-display:
    'Segoe UI Variable Display', 'Segoe UI', 'Microsoft YaHei UI', 'Microsoft YaHei',
    'Malgun Gothic', 'Yu Gothic UI', system-ui, sans-serif;
  --font-mono: 'Cascadia Mono', Consolas, 'Microsoft YaHei UI', monospace;
  --fs-page: 26px;
  --fs-section: 18px;
  --fs-body: 14px;
  --fs-meta: 13px;
  --fs-small: 12px;
  --fw-semibold: 600;

  /* Shape and spacing (4px grid). */
  --radius-xs: 4px;
  --radius-sm: 6px;
  --radius: 8px;
  --radius-lg: 12px;
  --radius-xl: 16px;
  --space-1: 4px;
  --space-2: 8px;
  --space-3: 12px;
  --space-4: 16px;
  --space-5: 20px;
  --space-6: 24px;
  --space-8: 32px;

  /* Layout. */
  --sidebar-w: 220px;
  --player-h: 88px;
  --topbar-h: 56px;
  --queue-w: 340px;
  --row-h: 56px;
  --control-h: 32px;
  --page-x: clamp(16px, 2.4vw, 32px);
  --page-top: 16px;

  /* Stacking order of fixed app layers. Dialogs, menus and tooltips use the browser top layer above all of these. */
  --z-sticky: 2;
  --z-player: 30;
  --z-immersive: 40;
  --z-queue-scrim: 44;
  --z-queue: 45;
  --z-toast: 60;

  /* Motion. */
  --dur-fast: 140ms;
  --dur: 180ms;
  --ease: cubic-bezier(0.2, 0, 0, 1);
  /* Drawer: quick start with a long natural deceleration in; accelerate out. No overshoot. */
  --dur-drawer-in: 240ms;
  --dur-drawer-out: 180ms;
  --ease-enter: cubic-bezier(0.16, 1, 0.3, 1);
  --ease-exit: cubic-bezier(0.4, 0, 1, 1);
  --drawer-shift: 40px;

  /* Foreground ink as an RGB triplet, for translucent text, tracks and scrollbars: rgb(var(--ink) / .2). */
  --ink: 232 236 231;
  --on-danger: #fff;
  --play-hover: #fff;
  --tooltip-bg: #2c3430;
  --tooltip-text: var(--text);
  --toast-bg: #2a322e;
  --toast-error-bg: #3a2522;
  --toast-error-text: #ffd9d3;
  --art-from: #2a332e;
  --art-to: #1d2421;
  --art-icon: #6f7d72;
  --glow: rgba(185, 214, 155, 0.06);
  --ambient-a: 0.32;
  --ambient-b: 0.12;

  --shadow-pop: 0 12px 32px rgba(0, 0, 0, 0.5), 0 0 0 1px var(--line-strong);
  --shadow-card: 0 6px 20px rgba(0, 0, 0, 0.35);
  --shadow-dialog: 0 24px 64px rgba(0, 0, 0, 0.6), 0 0 0 1px var(--line-strong);
  --shadow-art: 0 24px 60px rgba(0, 0, 0, 0.5);
  --shadow-drawer: -12px 0 32px rgba(0, 0, 0, 0.45);
  --shadow-control: 0 6px 16px rgba(0, 0, 0, 0.45);
  --shadow-tooltip: 0 4px 14px rgba(0, 0, 0, 0.45);
  --shadow-thumb: 0 1px 4px rgba(0, 0, 0, 0.5);
}

/* Light theme: soft warm grey-green, no large pure-white areas; only the palette changes.
   Contrast (WCAG): --text 11.8:1, --text-2 6.6:1, --text-3 5.6:1 on --bg; --text-3 stays ≥4.5:1 on
   --bg-sidebar and --hover; --accent 5.0:1 on --bg and 5.5:1 against --accent-ink. */
:root[data-theme='light'] {
  color-scheme: light;
  --bg: #f3f5f1;
  --bg-sidebar: #e9ede7;
  --bg-player: #e9ede7;
  --surface: #fafbf8;
  --surface-hover: #e2e8e0;
  --surface-active: #d7dfd4;
  --surface-overlay: #fafbf8;
  --scrim: rgba(41, 52, 44, 0.26);
  --line: rgba(41, 52, 44, 0.08);
  --line-strong: rgba(41, 52, 44, 0.14);

  --hover: rgba(41, 52, 44, 0.05);
  --selected: rgba(41, 52, 44, 0.085);
  --field-line: rgba(41, 52, 44, 0.12);
  --bg-drawer: #f7f8f5;
  --scrim-soft: rgba(41, 52, 44, 0.1);

  --text: #29342c;
  --text-2: #4e5a51;
  --text-3: #57635a;
  --lyric-dim: 0.5;
  --lyric-past: 0.38;

  --accent: #52705a;
  --accent-hover: #46624e;
  --accent-ink: #ffffff;
  --accent-soft: rgba(82, 112, 90, 0.12);
  --accent-line: rgba(82, 112, 90, 0.42);
  --focus: #3e5c46;

  --danger: #a93d31;
  --danger-bg: #b4473b;
  --danger-bg-hover: #a23f34;
  --danger-soft: rgba(169, 61, 49, 0.09);

  --ink: 41 52 44;
  --play-hover: #1d2620;
  --tooltip-bg: #29342c;
  --tooltip-text: #f3f5f1;
  --toast-bg: #fafbf8;
  --toast-error-bg: #f7e6e2;
  --toast-error-text: #8a3027;
  --art-from: #e1e7de;
  --art-to: #d3dbd0;
  --art-icon: #76837a;
  --glow: rgba(82, 112, 90, 0.06);
  --ambient-a: 0.2;
  --ambient-b: 0.08;

  --shadow-pop: 0 8px 24px rgba(41, 52, 44, 0.12), 0 0 0 1px var(--line);
  --shadow-card: 0 1px 2px rgba(41, 52, 44, 0.06), 0 4px 12px rgba(41, 52, 44, 0.06);
  --shadow-dialog: 0 16px 48px rgba(41, 52, 44, 0.16), 0 0 0 1px var(--line);
  --shadow-art: 0 16px 40px rgba(41, 52, 44, 0.18);
  --shadow-drawer: -8px 0 24px rgba(41, 52, 44, 0.08);
  --shadow-control: 0 4px 12px rgba(41, 52, 44, 0.16);
  --shadow-tooltip: 0 4px 12px rgba(41, 52, 44, 0.16);
  --shadow-thumb: 0 0 0 1px rgba(41, 52, 44, 0.08), 0 1px 3px rgba(41, 52, 44, 0.3);
}
````

## File: src/lib/utils/collections.ts
````typescript
/**
 * Built-in collections are addressed by route kind, never by display name, so a user playlist that happens to be
 * called “本地” keeps its own ID, songs, rename and delete.
 */
export const LOCAL_COLLECTION = { view: 'local', name: '本地' } as const;

/** Every track in the library: no stored membership, so imports, rescans and removed files update it automatically. */
export function localQuery(sort: string, search: string) {
  return { sort, search };
}

const norm = (name: string) => name.trim().toLowerCase();

/** True for a user playlist whose name collides with a built-in collection; the UI labels it instead of changing it. */
export function shadowsBuiltIn(name: string) {
  return norm(name) === norm(LOCAL_COLLECTION.name);
}
````

## File: src/lib/utils/color.ts
````typescript
/**
 * Picks a calm ambient colour from RGBA pixels: favours saturated mid-tone pixels,
 * then caps saturation and lightness so lyrics keep their contrast.
 */
export function ambientColor(data: ArrayLike<number>): [number, number, number] | null {
  let r = 0,
    g = 0,
    b = 0,
    weight = 0;
  for (let i = 0; i + 3 < data.length; i += 4) {
    if (data[i + 3] < 128) continue;
    const pr = data[i],
      pg = data[i + 1],
      pb = data[i + 2];
    const max = Math.max(pr, pg, pb),
      min = Math.min(pr, pg, pb);
    const light = (max + min) / 510;
    if (light < 0.08 || light > 0.92) continue;
    const w = 0.15 + (max - min) / 255;
    r += pr * w;
    g += pg * w;
    b += pb * w;
    weight += w;
  }
  if (!weight) return null;
  const [h, s, l] = toHsl(r / weight, g / weight, b / weight);
  return toRgb(h, Math.min(s, 0.42), Math.min(Math.max(l, 0.28), 0.42));
}

function toHsl(r: number, g: number, b: number): [number, number, number] {
  r /= 255;
  g /= 255;
  b /= 255;
  const max = Math.max(r, g, b),
    min = Math.min(r, g, b),
    l = (max + min) / 2;
  if (max === min) return [0, 0, l];
  const d = max - min,
    s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
  const h =
    max === r ? (g - b) / d + (g < b ? 6 : 0) : max === g ? (b - r) / d + 2 : (r - g) / d + 4;
  return [h / 6, s, l];
}

function toRgb(h: number, s: number, l: number): [number, number, number] {
  if (!s) {
    const v = Math.round(l * 255);
    return [v, v, v];
  }
  const q = l < 0.5 ? l * (1 + s) : l + s - l * s,
    p = 2 * l - q;
  const hue = (t: number) => {
    if (t < 0) t += 1;
    if (t > 1) t -= 1;
    if (t < 1 / 6) return p + (q - p) * 6 * t;
    if (t < 1 / 2) return q;
    if (t < 2 / 3) return p + (q - p) * (2 / 3 - t) * 6;
    return p;
  };
  return [hue(h + 1 / 3), hue(h), hue(h - 1 / 3)].map((v) => Math.round(v * 255)) as [
    number,
    number,
    number,
  ];
}
````

## File: src/lib/utils/cover-diagnostics.ts
````typescript
/// <reference types="vite/client" />
// Development-only counters: never store song IDs, URLs, or local paths.
export const diagnostics = import.meta.env.DEV
  ? {
      snapshots: 0,
      trackChanges: 0,
      coverRequests: 0,
      urlChanges: 0,
      mounts: 0,
      loads: 0,
      errors: 0,
      startedAt: performance.now(),
    }
  : null;
if (diagnostics) Object.assign(globalThis, { __musicMineDiagnostics: diagnostics });
````

## File: src/lib/utils/cover-loader.ts
````typescript
// Bounded, prioritised cover requests. The loader owns no IPC; callers pass the load function.
// Only rows that ask for a cover are queued, and a row that leaves the screen cancels its request,
// so fast scrolling cannot build an unbounded backlog in front of the rows that are visible now.

/** 0 = current track, 1 = visible row, 2 = preload just outside the list. */
export type CoverPriority = 0 | 1 | 2;
export type CoverStatus = 'ready' | 'none' | 'failed';
export interface CoverOutcome {
  status: CoverStatus;
  src: string | null;
  code?: string;
}
export interface CoverTicket {
  setPriority(priority: CoverPriority): void;
  cancel(): void;
}
type Done = (outcome: CoverOutcome) => void;
interface Waiter {
  priority: CoverPriority;
  done: Done;
}

interface Job {
  key: string;
  load: () => Promise<string | null>;
  priority: CoverPriority;
  seq: number;
  /** Cache generation the result belongs to. A running job keeps the generation it started in. */
  generation: number;
  waiters: Map<number, Waiter>;
  state: 'queued' | 'running' | 'retry';
  attempts: number;
  queuedAt: number;
}

export interface CoverLoaderOptions {
  concurrency?: number;
  maxQueued?: number;
  maxResults?: number;
  /** Delays before each retry of a transient failure. Its length is the retry limit. */
  retryDelays?: number[];
  timeoutMs?: number;
  failedTtlMs?: number;
  /** Errors that will not change by retrying in this session. */
  terminal?: (code: string) => boolean;
  now?: () => number;
  schedule?: (fn: () => void, ms: number) => unknown;
}

export const emptyCoverStats = () => ({
  requested: 0,
  peekHits: 0,
  coalesced: 0,
  started: 0,
  ready: 0,
  none: 0,
  failed: 0,
  retries: 0,
  cancelled: 0,
  dropped: 0,
  timeouts: 0,
  maxQueued: 0,
  maxRunning: 0,
  /** Requests that finished after clear(): never cached; failures were handed to a current job. */
  stale: 0,
  staleHandedOver: 0,
  waitMsTotal: 0,
  waitMsMax: 0,
  ipcMsTotal: 0,
  ipcMsMax: 0,
  codes: {} as Record<string, number>,
});
export type CoverStats = ReturnType<typeof emptyCoverStats>;

/** Error code from a rejected cover request; never includes a path. */
export function coverErrorCode(error: unknown): string {
  const text = typeof error === 'string' ? error : error instanceof Error ? error.message : '';
  return /^cover-[a-z-]+$/.test(text) ? text : 'ipc-error';
}

export function createCoverLoader(options: CoverLoaderOptions = {}) {
  const concurrency = options.concurrency ?? 4;
  const maxQueued = options.maxQueued ?? 48;
  const maxResults = options.maxResults ?? 600;
  const retryDelays = options.retryDelays ?? [700, 2500];
  const timeoutMs = options.timeoutMs ?? 15000;
  const failedTtlMs = options.failedTtlMs ?? 30000;
  const terminal = options.terminal ?? (() => false);
  const now = options.now ?? (() => performance.now());
  const schedule = options.schedule ?? ((fn: () => void, ms: number) => setTimeout(fn, ms));
  const results = new Map<string, CoverOutcome & { at: number }>();
  /** Jobs of the current generation, one per key. Requests that outlived clear() are not in here. */
  const jobs = new Map<string, Job>();
  /** The job each waiting ticket currently belongs to; a hand-over moves tickets to another job. */
  const owners = new Map<number, Job>();
  const stats = emptyCoverStats();
  // `running` counts every load still in flight, including ones from before clear(): they still hold
  // an IPC call and a Rust thread, so the concurrency bound covers them too.
  let running = 0,
    seq = 0,
    waiterSeq = 0,
    generation = 0;

  function remember(key: string, outcome: CoverOutcome) {
    results.delete(key);
    results.set(key, { ...outcome, at: now() });
    if (results.size > maxResults) results.delete(results.keys().next().value!);
  }
  /** A finished result, or undefined when the cover still has to be requested. */
  function peek(key: string): CoverOutcome | undefined {
    const hit = results.get(key);
    if (!hit) return undefined;
    if (hit.status === 'failed' && now() - hit.at > failedTtlMs) {
      results.delete(key);
      return undefined;
    }
    results.delete(key);
    results.set(key, hit);
    return { status: hit.status, src: hit.src, code: hit.code };
  }
  const queued = () => [...jobs.values()].filter((j) => j.state === 'queued');
  function release(job: Job) {
    if (jobs.get(job.key) === job) jobs.delete(job.key);
    for (const id of job.waiters.keys()) owners.delete(id);
  }
  function notify(job: Job, outcome: CoverOutcome) {
    const waiters = [...job.waiters.values()];
    release(job);
    for (const { done } of waiters) done(outcome);
  }
  function finish(job: Job, outcome: CoverOutcome) {
    stats[outcome.status]++;
    if (outcome.code) stats.codes[outcome.code] = (stats.codes[outcome.code] ?? 0) + 1;
    remember(job.key, outcome);
    notify(job, outcome);
  }
  function settle(job: Job) {
    job.priority = Math.min(
      ...[...job.waiters.values()].map((w) => w.priority),
      2,
    ) as CoverPriority;
  }
  function createJob(
    key: string,
    load: () => Promise<string | null>,
    priority: CoverPriority,
  ): Job {
    const job: Job = {
      key,
      load,
      priority,
      seq: ++seq,
      generation,
      waiters: new Map(),
      state: 'queued',
      attempts: 0,
      queuedAt: now(),
    };
    jobs.set(key, job);
    return job;
  }
  /** Moves the waiters of a request that failed after clear() to the current job for the same key. */
  function handOver(stale: Job) {
    if (stale.waiters.size === 0) return;
    const fresh = jobs.get(stale.key) ?? createJob(stale.key, stale.load, stale.priority);
    for (const [id, waiter] of stale.waiters) {
      fresh.waiters.set(id, waiter);
      owners.set(id, fresh);
    }
    stale.waiters.clear();
    settle(fresh);
    stats.staleHandedOver++;
  }
  function pump() {
    while (running < concurrency) {
      const next = queued().sort((a, b) => a.priority - b.priority || a.seq - b.seq)[0];
      if (!next) return;
      start(next);
    }
  }
  function start(job: Job) {
    job.state = 'running';
    job.attempts++;
    running++;
    stats.started++;
    stats.maxRunning = Math.max(stats.maxRunning, running);
    const wait = now() - job.queuedAt;
    stats.waitMsTotal += wait;
    stats.waitMsMax = Math.max(stats.waitMsMax, wait);
    const began = now();
    let timer = false;
    const timeout = new Promise<never>((_, reject) =>
      schedule(() => {
        timer = true;
        reject('cover-timeout');
      }, timeoutMs),
    );
    Promise.race([job.load(), timeout])
      .then(
        (src) => ({ src, error: null as unknown }),
        (error) => ({ src: null, error }),
      )
      .then(({ src, error }) => {
        running--;
        const spent = now() - began;
        stats.ipcMsTotal += spent;
        stats.ipcMsMax = Math.max(stats.ipcMsMax, spent);
        if (job.generation !== generation) {
          // Started before clear(): the result may predate the library change, so it is never cached.
          // Its own rows still get a picture they asked for; a failure is retried by a current job.
          stats.stale++;
          if (error === null)
            notify(job, src ? { status: 'ready', src } : { status: 'none', src: null });
          else handOver(job);
          pump();
          return;
        }
        if (error === null)
          finish(job, src ? { status: 'ready', src } : { status: 'none', src: null });
        else {
          const code = timer && error === 'cover-timeout' ? 'cover-timeout' : coverErrorCode(error);
          if (code === 'cover-timeout') stats.timeouts++;
          const delay = retryDelays[job.attempts - 1];
          if (terminal(code) || delay === undefined || job.waiters.size === 0)
            finish(job, { status: 'failed', src: null, code });
          else {
            stats.retries++;
            job.state = 'retry';
            schedule(() => {
              if (jobs.get(job.key) !== job || job.state !== 'retry') return;
              if (job.waiters.size === 0) {
                release(job);
                return;
              }
              job.state = 'queued';
              job.queuedAt = now();
              pump();
            }, delay);
          }
        }
        pump();
      });
  }
  function request(
    key: string,
    priority: CoverPriority,
    load: () => Promise<string | null>,
    done: Done,
  ): CoverTicket {
    stats.requested++;
    const hit = peek(key);
    if (hit) {
      stats.peekHits++;
      done(hit);
      return { setPriority() {}, cancel() {} };
    }
    let job = jobs.get(key);
    if (job) stats.coalesced++;
    else job = createJob(key, load, priority);
    const id = ++waiterSeq;
    job.waiters.set(id, { priority, done });
    owners.set(id, job);
    settle(job);
    const waiting = queued();
    stats.maxQueued = Math.max(stats.maxQueued, waiting.length);
    if (waiting.length > maxQueued) {
      // Over the bound only preloads are dropped, newest first; their rows ask again when visible.
      const victim = waiting.filter((j) => j.priority === 2).sort((a, b) => b.seq - a.seq)[0];
      if (victim) {
        stats.dropped++;
        notify(victim, { status: 'failed', src: null, code: 'dropped' });
      }
    }
    pump();
    return {
      setPriority(next) {
        const owner = owners.get(id);
        const waiter = owner?.waiters.get(id);
        if (!owner || !waiter || waiter.priority === next) return;
        waiter.priority = next;
        settle(owner);
        pump();
      },
      cancel() {
        const owner = owners.get(id);
        if (!owner || !owner.waiters.delete(id)) return;
        owners.delete(id);
        stats.cancelled++;
        if (owner.waiters.size === 0 && owner.state === 'queued') release(owner);
        else settle(owner);
      },
    };
  }
  /** Library contents changed: forget results. Rows keep showing what they have until new results arrive.
   *  Loads already running finish in the background but cannot write the new cache; queued and retrying
   *  requests have not read anything yet, so they simply belong to the new generation. */
  function clear() {
    generation++;
    results.clear();
    for (const job of [...jobs.values()]) {
      if (job.state === 'running') jobs.delete(job.key);
      else job.generation = generation;
    }
  }
  return {
    request,
    peek,
    clear,
    stats,
    pending: () => ({ queued: queued().length, running, waiting: owners.size }),
  };
}
````

## File: src/lib/utils/format.ts
````typescript
export function duration(ms: number) {
  const total = Math.max(0, Math.floor(ms / 1000));
  const h = Math.floor(total / 3600),
    m = Math.floor(total / 60) % 60,
    s = String(total % 60).padStart(2, '0');
  return h ? `${h}:${String(m).padStart(2, '0')}:${s}` : `${m}:${s}`;
}

/** First visible character for artist placeholders (works for CJK and emoji). */
export function initial(name: string) {
  return Array.from(name.trim())[0]?.toUpperCase() ?? '?';
}

/** Stable hue per name so placeholder tints stay consistent between sessions. */
export function nameHue(name: string) {
  let h = 0;
  for (const c of name) h = (h * 31 + c.codePointAt(0)!) % 360;
  return h;
}

export function errorText(e: unknown) {
  if (e instanceof Error) return e.message;
  if (typeof e === 'string') return e;
  try {
    return JSON.stringify(e);
  } catch {
    return String(e);
  }
}
````

## File: src/lib/utils/history.ts
````typescript
/** Back/forward stacks for in-app navigation; equal consecutive entries are not duplicated. */
export function createHistory<T>(
  initial: T,
  same: (a: T, b: T) => boolean = (a, b) => JSON.stringify(a) === JSON.stringify(b),
  limit = 50,
) {
  const back: T[] = [];
  let forward: T[] = [];
  let current = initial;
  return {
    get current() {
      return current;
    },
    get canBack() {
      return back.length > 0;
    },
    get canForward() {
      return forward.length > 0;
    },
    push(next: T) {
      if (same(current, next)) return false;
      back.push(current);
      if (back.length > limit) back.shift();
      forward = [];
      current = next;
      return true;
    },
    replace(next: T) {
      current = next;
    },
    back() {
      const prev = back.pop();
      if (prev === undefined) return false;
      forward.push(current);
      current = prev;
      return true;
    },
    forward() {
      const next = forward.pop();
      if (next === undefined) return false;
      back.push(current);
      current = next;
      return true;
    },
    /** Drops entries matching the predicate, e.g. a deleted playlist. */
    forget(match: (entry: T) => boolean, fallback: T) {
      for (const stack of [back, forward])
        for (let i = stack.length - 1; i >= 0; i--) if (match(stack[i])) stack.splice(i, 1);
      if (match(current)) current = back.pop() ?? fallback;
    },
  };
}
````

## File: src/lib/utils/layers.ts
````typescript
/**
 * App surfaces stacked above the page, top first. The queue drawer is drawn above Now Playing,
 * so one Escape closes only the queue and leaves the lyrics page open.
 * Dialogs and popover menus live in the browser top layer and handle their own Escape.
 */
export type EscapeLayer = 'queue' | 'immersive' | 'toast';
export function escapeLayer(open: {
  queue: boolean;
  immersive: boolean;
  toast: boolean;
}): EscapeLayer | null {
  if (open.queue) return 'queue';
  if (open.immersive) return 'immersive';
  if (open.toast) return 'toast';
  return null;
}
````

## File: src/lib/utils/lyrics-view.ts
````typescript
// U+2009 is the bilingual separator verified in Ailee - No No No.lrc.
// Ordinary whitespace and language changes are never treated as translation markers.
export function displayParts(text: string): string[] {
  return text.split('\u2009');
}
export function groupLines(lines: { time_ms: number; text: string }[], synced: boolean) {
  const groups: { time_ms: number; parts: string[] }[] = [];
  for (const line of lines) {
    const last = groups.at(-1);
    const parts = displayParts(line.text);
    if (synced && last?.time_ms === line.time_ms) last.parts.push(...parts);
    else groups.push({ time_ms: line.time_ms, parts });
  }
  return groups;
}
export function activeLine(lines: { time_ms: number }[], position: number) {
  let lo = 0,
    hi = lines.length;
  while (lo < hi) {
    const mid = (lo + hi) >>> 1;
    if (lines[mid].time_ms <= position) lo = mid + 1;
    else hi = mid;
  }
  return lo - 1;
}
export function scrollTarget(top: number, height: number, viewport: number, total: number) {
  return Math.max(0, Math.min(total - viewport, top + height / 2 - viewport * 0.4));
}
````

## File: src/lib/utils/modal.ts
````typescript
type Box = { left: number; right: number; top: number; bottom: number };
type Target = { getBoundingClientRect(): Box };
type PointerLike = { target: unknown; clientX: number; clientY: number; button?: number };

/** A modal <dialog> fills the top layer; a press on its ::backdrop targets the dialog but lies outside its box. */
export function outsideBox(box: Box, x: number, y: number) {
  return x < box.left || x > box.right || y < box.top || y > box.bottom;
}

/**
 * Native backdrop hit-testing; keyboard dismissal and focus are owned by Bits UI Dialog.
 * Dismisses only when both press and release land on the backdrop, so a text selection dragged out of an
 * input never closes the dialog, and a press that only light-dismisses an open popover is not counted.
 */
export function createDismissal(
  node: Target,
  options: { canDismiss: () => boolean; popoverOpen: () => boolean; dismiss: () => void },
) {
  let armed = false;
  const onBackdrop = (e: PointerLike) =>
    e.target === node && outsideBox(node.getBoundingClientRect(), e.clientX, e.clientY);
  return {
    pointerdown(e: PointerLike) {
      armed = (e.button ?? 0) === 0 && onBackdrop(e) && !options.popoverOpen();
    },
    click(e: PointerLike) {
      const hit = armed && onBackdrop(e);
      armed = false;
      if (hit && options.canDismiss()) options.dismiss();
    },
  };
}
````

## File: src/lib/utils/native-menu.ts
````typescript
// WebView2 shows a browser menu (Back, Refresh, Print, Inspect…) on every right click.
// Text fields keep it for copy/paste; in development Shift+right click keeps Inspect.
const EDITABLE =
  'textarea,[contenteditable]:not([contenteditable="false"]),input:not([type=range]):not([type=checkbox]):not([type=radio]):not([type=button]):not([type=submit]):not([type=color]):not([type=file])';

export function allowsNativeMenu(
  target: { closest(selector: string): unknown } | null,
  dev: boolean,
  shiftKey: boolean,
) {
  if (dev && shiftKey) return true;
  return !!target?.closest(EDITABLE);
}

type Key = Pick<KeyboardEvent, 'key' | 'ctrlKey' | 'shiftKey' | 'altKey' | 'metaKey'>;
// Browser reload/print/find-in-page accelerators make no sense in a desktop player.
export function isBrowserAccelerator(e: Key) {
  const key = e.key.length === 1 ? e.key.toLowerCase() : e.key;
  if (key === 'F5' || key === 'BrowserRefresh' || key === 'BrowserBack' || key === 'BrowserForward')
    return true;
  if (
    e.ctrlKey &&
    !e.altKey &&
    (key === 'r' || key === 'p' || key === 's' || key === 'u' || key === 'g')
  )
    return true;
  return false;
}

export function installNativeMenuGuard(dev: boolean) {
  const menu = (e: MouseEvent) => {
    const target = e.target instanceof Element ? e.target : null;
    if (!allowsNativeMenu(target, dev, e.shiftKey)) e.preventDefault();
  };
  const keys = (e: KeyboardEvent) => {
    if (!dev && isBrowserAccelerator(e)) e.preventDefault();
  };
  window.addEventListener('contextmenu', menu);
  window.addEventListener('keydown', keys, true);
  return () => {
    window.removeEventListener('contextmenu', menu);
    window.removeEventListener('keydown', keys, true);
  };
}
````

## File: src/lib/utils/playlist-name.ts
````typescript
import { LOCAL_COLLECTION, shadowsBuiltIn } from './collections.ts';

export const PLAYLIST_NAME_MAX = 100;

/** Returns an error message, or null when the name can be saved. Rust repeats the length and duplicate checks. */
export function playlistNameError(
  value: string,
  existing: { id: number; name: string }[],
  selfId: number | null = null,
): string | null {
  const name = value.trim();
  if (!name) return '请输入歌单名称';
  if (name.length > PLAYLIST_NAME_MAX) return `歌单名称最多 ${PLAYLIST_NAME_MAX} 个字符`;
  const lower = name.toLowerCase();
  if (existing.some((p) => p.id !== selfId && p.name.trim().toLowerCase() === lower))
    return `已存在名为「${name}」的歌单`;
  const keepsOwnName =
    selfId != null &&
    existing.some((p) => p.id === selfId && p.name.trim().toLowerCase() === lower);
  if (shadowsBuiltIn(name) && !keepsOwnName)
    return `「${LOCAL_COLLECTION.name}」是系统歌单，会自动显示全部本地音乐`;
  return null;
}

/** Suggests “新建歌单 2” style names that do not collide with existing playlists. */
export function suggestPlaylistName(existing: { name: string }[], base = '新建歌单') {
  const names = new Set(existing.map((p) => p.name.trim().toLowerCase()));
  if (!names.has(base.toLowerCase())) return base;
  for (let i = 2; ; i++) {
    const candidate = `${base} ${i}`;
    if (!names.has(candidate.toLowerCase())) return candidate;
  }
}
````

## File: src/lib/utils/queue.ts
````typescript
import type { Track } from '../services/music';

/** True when two queue snapshots list the same tracks with the same displayed metadata, in order. */
export function sameTracks(a: readonly Track[], b: readonly Track[]) {
  if (a === b) return true;
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    const x = a[i],
      y = b[i];
    if (
      x.path !== y.path ||
      x.title !== y.title ||
      x.artist !== y.artist ||
      x.album !== y.album ||
      x.duration_ms !== y.duration_ms
    )
      return false;
  }
  return true;
}

/** Keep the previous array when a poll did not change the tracks, so Svelte does not re-diff every row. */
export function retainQueue<T extends Track>(
  rendered: readonly T[],
  next: readonly T[],
): readonly T[] {
  return sameTracks(rendered, next) ? rendered : next;
}
````

## File: src/lib/utils/row-visibility.ts
````typescript
// Shared observers per scroll container. `seen` is the row inside the container's box; `near`
// adds a preload margin. The observer root is the container itself: with the implicit viewport
// root a margin does not extend past the container's clipping, so nothing would be preloaded.
export interface RowState {
  near: boolean;
  seen: boolean;
}
type Listener = (state: RowState) => void;
interface Watch {
  listener: Listener;
  state: RowState;
}
interface Root {
  near: IntersectionObserver;
  seen: IntersectionObserver;
  rows: Map<Element, Watch>;
}

const roots = new WeakMap<Element, Map<string, Root>>();

function rootFor(container: Element, margin: string): Root {
  let byMargin = roots.get(container);
  if (!byMargin) {
    byMargin = new Map();
    roots.set(container, byMargin);
  }
  let root = byMargin.get(margin);
  if (root) return root;
  const rows = new Map<Element, Watch>();
  const update = (field: keyof RowState) => (entries: IntersectionObserverEntry[]) => {
    for (const entry of entries) {
      const watch = rows.get(entry.target);
      if (!watch || watch.state[field] === entry.isIntersecting) continue;
      watch.state = { ...watch.state, [field]: entry.isIntersecting };
      watch.listener(watch.state);
    }
  };
  root = {
    rows,
    near: new IntersectionObserver(update('near'), { root: container, rootMargin: margin }),
    seen: new IntersectionObserver(update('seen'), { root: container }),
  };
  byMargin.set(margin, root);
  return root;
}

/** Reports when `row` enters or leaves `container`'s box and its preload margin. */
export function watchRow(
  row: Element,
  container: Element,
  margin: string,
  listener: Listener,
): () => void {
  const root = rootFor(container, margin);
  root.rows.set(row, { listener, state: { near: false, seen: false } });
  root.near.observe(row);
  root.seen.observe(row);
  return () => {
    root.rows.delete(row);
    root.near.unobserve(row);
    root.seen.unobserve(row);
  };
}
````

## File: src/lib/utils/shortcuts.ts
````typescript
export type Shortcut = 'toggle' | 'search' | 'previous' | 'next' | 'lyrics' | 'open' | 'escape';
export const shortcutHelp = [
  ['Space', '播放 / 暂停'],
  ['Ctrl+F', '聚焦搜索'],
  ['Ctrl+←', '上一首'],
  ['Ctrl+→', '下一首'],
  ['Ctrl+L', '打开 / 关闭歌词'],
  ['Ctrl+O', '选择本地音乐文件'],
  ['Escape', '关闭弹窗或退出沉浸页面'],
];
export function shortcut(
  e: Pick<
    KeyboardEvent,
    | 'key'
    | 'ctrlKey'
    | 'altKey'
    | 'metaKey'
    | 'shiftKey'
    | 'repeat'
    | 'isComposing'
    | 'defaultPrevented'
  >,
  editable: boolean,
  interactive: boolean,
  modal: boolean,
): Shortcut | null {
  if (e.defaultPrevented || e.repeat || e.isComposing || e.altKey || e.metaKey || e.shiftKey)
    return null;
  if (e.key === 'Escape') return 'escape';
  if (modal) return null;
  if (e.ctrlKey) {
    if (e.key.toLowerCase() === 'f') return 'search';
    if (editable) return null;
    return (
      (
        { ArrowLeft: 'previous', ArrowRight: 'next', l: 'lyrics', o: 'open' } as Record<
          string,
          Shortcut
        >
      )[e.key.length === 1 ? e.key.toLowerCase() : e.key] ?? null
    );
  }
  return e.key === ' ' && !editable && !interactive ? 'toggle' : null;
}
````

## File: src/lib/utils/theme.ts
````typescript
/** Theme choice shared with public/theme-init.js, which applies it before the first paint. */
export type ThemeChoice = 'dark' | 'light' | 'system';
export type Theme = 'dark' | 'light';

export const THEME_KEY = 'music-mine-theme';
export const THEME_COLOR: Record<Theme, string> = { dark: '#121414', light: '#F3F5F1' };

/** Unknown or missing values are dark, so users from before themes existed see no change. */
export function parseTheme(value: unknown): ThemeChoice {
  return value === 'light' || value === 'system' ? value : 'dark';
}
export function resolveTheme(choice: ThemeChoice, systemDark: boolean): Theme {
  return choice === 'system' ? (systemDark ? 'dark' : 'light') : choice;
}
````

## File: src/lib/utils/virtual-range.ts
````typescript
/** Visible row window for a fixed-height list inside a scroll container. */
export function visibleRange(
  scrollTop: number,
  viewport: number,
  listTop: number,
  rowHeight: number,
  total: number,
  overscan = 8,
) {
  if (total <= 0 || rowHeight <= 0) return { start: 0, end: 0 };
  const first = Math.floor((scrollTop - listTop) / rowHeight);
  const last = Math.ceil((scrollTop - listTop + viewport) / rowHeight);
  const start = Math.max(0, Math.min(total, first - overscan));
  const end = Math.max(start, Math.min(total, last + overscan));
  return { start, end };
}

/** Page numbers (of `size` rows) needed to render rows [start, end). */
export function pagesFor(start: number, end: number, size: number) {
  const pages: number[] = [];
  if (end <= start) return pages;
  for (let p = Math.floor(start / size); p <= Math.floor((end - 1) / size); p++) pages.push(p);
  return pages;
}

/** Scroll offset that brings row `index` fully into view, or null when already visible. */
export function scrollToRow(
  index: number,
  scrollTop: number,
  viewport: number,
  listTop: number,
  rowHeight: number,
  padTop = 0,
) {
  const top = listTop + index * rowHeight;
  const bottom = top + rowHeight;
  if (top - padTop < scrollTop) return top - padTop;
  if (bottom > scrollTop + viewport) return bottom - viewport;
  return null;
}
````

## File: src/lib/utils/volume.ts
````typescript
/** Rust's player volume is a 0.0–1.0 gain; the UI shows the same value as a whole percentage. */
export const DEFAULT_PERCENT = 70;
export const toPercent = (gain: number) =>
  Number.isFinite(gain) ? Math.round(Math.min(1, Math.max(0, gain)) * 100) : 0;
export const toGain = (percent: number) => Math.min(100, Math.max(0, Math.round(percent))) / 100;

/** Volume a mute toggle should set: mute when audible, otherwise restore the last audible level. */
export function muteTarget(current: number, lastAudible: number) {
  return current > 0 ? 0 : lastAudible > 0 ? lastAudible : DEFAULT_PERCENT;
}

/**
 * Coalesces rapid values (slider drags) into IPC calls: at most one request is in flight,
 * intermediate values are dropped, and the most recent value is always sent last.
 */
export function latest<T, R>(
  send: (value: T) => Promise<R>,
  handlers: { result: (r: R) => void; error: (e: unknown) => void; settled: () => void },
) {
  let pending: { value: T } | null = null;
  let running = false;
  let sent: { value: T } | null = null;
  async function pump() {
    running = true;
    while (pending) {
      const { value } = pending;
      pending = null;
      if (sent && Object.is(sent.value, value)) continue;
      sent = { value };
      try {
        handlers.result(await send(value));
      } catch (e) {
        sent = null;
        handlers.error(e);
      }
    }
    running = false;
    sent = null;
    handlers.settled();
  }
  return {
    push(value: T) {
      pending = { value };
      if (!running) void pump();
    },
    get busy() {
      return running;
    },
  };
}
````

## File: src/lib/views/GridView.svelte
````svelte
<script lang="ts">
  import { untrack } from 'svelte';
  import { Play, Search, Disc3, MicVocal } from 'lucide-svelte';
  import Cover from '../components/Cover.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import Select from '../components/Select.svelte';
  import { tooltip } from '../components/tooltip';
  import { library } from '../stores/library.svelte';
  import { ui, openMenu } from '../stores/ui.svelte';
  import { go, memory } from '../stores/nav.svelte';
  import { playCollection, albumMenu, artistMenu } from '../actions';
  import type { Group } from '../services/music';
  import { initial, nameHue } from '../utils/format';

  let { kind }: { kind: 'albums' | 'artists' } = $props();

  let sort = $state(untrack(() => memory.get(kind)?.sort ?? 'name'));
  function changeSort(next: string) {
    sort = next;
    memory.set(kind, { scroll: 0, ...memory.get(kind), sort: next });
  }

  let groups = $derived.by(() => {
    const q = ui.query.trim().toLowerCase();
    const source = kind === 'albums' ? library.summary.albums : library.summary.artists;
    const list = q
      ? source.filter((g) => (g.name + ' ' + g.artist).toLowerCase().includes(q))
      : source.slice();
    return list.sort((a, b) =>
      sort === 'count'
        ? b.count - a.count || a.name.localeCompare(b.name, 'zh-CN')
        : a.name.localeCompare(b.name, 'zh-CN'),
    );
  });

  // Render progressively so huge libraries do not mount thousands of cards at once.
  let limit = $state(120);
  function sentinel(node: HTMLElement) {
    const observer = new IntersectionObserver(
      (entries) => {
        if (entries.some((e) => e.isIntersecting)) limit += 120;
      },
      { rootMargin: '600px' },
    );
    observer.observe(node);
    return {
      destroy() {
        observer.disconnect();
      },
    };
  }

  const open = (g: Group) =>
    kind === 'albums' ? go({ view: 'album', key: g.key }) : go({ view: 'artist', name: g.name });
  const play = (g: Group) =>
    playCollection(
      kind === 'albums' ? { album: g.key, sort: 'album' } : { artist: g.name, sort: 'album' },
    );
  const menu = (e: MouseEvent | HTMLElement, g: Group) =>
    openMenu(e, kind === 'albums' ? albumMenu(g) : artistMenu(g), g.name + ' 操作');
</script>

<header class="page-header">
  <div>
    <h2 class="page-title">{kind === 'albums' ? '专辑' : '艺术家'}</h2>
    <p class="page-meta">
      {groups.length}
      {kind === 'albums' ? '张专辑' : '位艺术家'}{#if ui.query.trim()}
        · 搜索“{ui.query.trim()}”{/if}
    </p>
  </div>
  <div class="page-actions">
    <span class="toolbar-label">排序</span>
    <Select
      label="排序方式"
      value={sort}
      onchange={changeSort}
      options={[
        { value: 'name', label: '名称' },
        { value: 'count', label: '歌曲数量' },
      ]}
    />
  </div>
</header>

{#if !groups.length}
  {#if ui.query.trim()}<EmptyState
      icon={Search}
      title={kind === 'albums' ? '没有找到匹配的专辑' : '没有找到匹配的艺术家'}
      message="试试其他关键词，或在“歌曲”中搜索。"
      ><button type="button" class="btn btn-ghost" onclick={() => go({ view: 'songs' })}
        >在歌曲中搜索</button
      ></EmptyState
    >
  {:else if library.loaded}<EmptyState
      icon={kind === 'albums' ? Disc3 : MicVocal}
      title={kind === 'albums' ? '还没有专辑' : '还没有艺术家'}
      message="导入音乐文件夹后会自动整理。"
    />{/if}
{:else}
  <div class="card-grid" class:artists={kind === 'artists'}>
    {#each groups.slice(0, limit) as g (g.key)}
      <article class="card" oncontextmenu={(e) => menu(e, g)}>
        <div class="card-art" class:round={kind === 'artists'}>
          {#if kind === 'albums'}<Cover id={g.cover_id} alt={g.name} size={320} />
          {:else}<span class="artist-initial" style:--hue={nameHue(g.name)} aria-hidden="true"
              >{initial(g.name)}</span
            >{/if}
          <button
            type="button"
            class="card-open"
            aria-label={'打开 ' + g.name}
            onclick={() => open(g)}
          ></button>
          <button
            type="button"
            class="card-play"
            aria-label={'播放 ' + g.name}
            use:tooltip={'播放'}
            onclick={() => play(g)}><Play size={18} fill="currentColor" /></button
          >
        </div>
        <button
          type="button"
          class="card-title"
          onclick={() => open(g)}
          use:tooltip={{ text: g.name, overflow: true }}>{g.name || '未知'}</button
        >
        <p class="card-sub" use:tooltip={{ text: g.artist, overflow: true }}>
          {kind === 'albums' ? g.artist || '未知艺术家' : `${g.count} 首歌曲`}
        </p>
      </article>
    {/each}
  </div>
  {#if limit < groups.length}<div class="grid-sentinel" use:sentinel aria-hidden="true"></div>{/if}
{/if}
````

## File: src/lib/views/HomeView.svelte
````svelte
<script lang="ts">
  import { Play, Pause, FolderPlus, ChevronRight, Music2 } from 'lucide-svelte';
  import Cover from '../components/Cover.svelte';
  import Artwork from '../components/Artwork.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import { tooltip } from '../components/tooltip';
  import {
    desktopAvailable,
    homeData,
    queryLibrary,
    playTrack,
    play,
    pause,
    type LibraryTrack,
  } from '../services/music';
  import { player, perform, control } from '../stores/player.svelte';
  import { library } from '../stores/library.svelte';
  import { ui, openMenu, notifyError } from '../stores/ui.svelte';
  import { go } from '../stores/nav.svelte';
  import {
    songMenu,
    playlistMenu,
    localMenu,
    artistMenu,
    openDetails,
    importFolder,
    playCollection,
  } from '../actions';
  import { LOCAL_COLLECTION, localQuery } from '../utils/collections';
  import { duration, initial, nameHue } from '../utils/format';

  let { artwork }: { artwork: string | null } = $props();

  let recent = $state<LibraryTrack[]>([]);
  let added = $state<LibraryTrack[]>([]);
  let artists = $state<[string, number][]>([]);
  let loading = $state(true);

  async function load() {
    if (!desktopAvailable) {
      loading = false;
      return;
    }
    try {
      const [h, a] = await Promise.all([homeData(), queryLibrary({ limit: 12, sort: 'added' })]);
      recent = h.recent;
      artists = h.artists;
      added = a.tracks;
    } catch (e) {
      notifyError(e);
    } finally {
      loading = false;
    }
  }
  // Reload when the library changes or a different song starts (recent plays move).
  let trackId = $derived(player.snapshot?.track_id);
  $effect(() => {
    library.revision;
    trackId;
    void load();
  });

  let s = $derived(player.snapshot);
  let playing = $derived(s?.playback_state === 'playing');
  let progress = $derived(
    s?.duration_ms ? Math.min(100, (s.position_ms / s.duration_ms) * 100) : 0,
  );

  const hour = new Date().getHours();
  const greeting =
    hour < 5
      ? '夜深了'
      : hour < 11
        ? '早上好'
        : hour < 13
          ? '中午好'
          : hour < 18
            ? '下午好'
            : '晚上好';

  function trackMenu(e: MouseEvent, t: LibraryTrack) {
    openMenu(e, songMenu(t, { onDetails: () => openDetails(t) }), t.title + ' 操作');
  }
  function artistGroup(name: string) {
    return (
      library.summary.artists.find((a) => a.name === name) ?? {
        key: name,
        name,
        artist: '',
        count: 0,
        cover_id: 0,
      }
    );
  }
</script>

<header class="home-header">
  <h2 class="page-title">{greeting}</h2>
  {#if library.loaded && library.summary.tracks}<p class="page-meta">
      {library.summary.tracks} 首歌曲 · {library.summary.albums.length} 张专辑 · {library.summary
        .artists.length} 位艺术家
    </p>{/if}
</header>

{#if !desktopAvailable}
  <EmptyState
    icon={Music2}
    title="请在 Music Mine 桌面应用中使用"
    message="浏览器预览无法读取本地音乐库。"
  />
{:else if library.loaded && !library.summary.tracks}
  <EmptyState
    icon={FolderPlus}
    title="欢迎使用 Music Mine"
    message="导入一个音乐文件夹，开始建立你的本地音乐库。音乐文件和标签不会被修改。"
  >
    <button type="button" class="btn btn-primary" onclick={importFolder}
      ><FolderPlus size={16} />导入音乐文件夹</button
    >
  </EmptyState>
{:else}
  {#if s?.current_track}
    <section class="home-section" aria-labelledby="home-continue">
      <h3 id="home-continue" class="section-title">继续播放</h3>
      <div class="continue">
        <button
          type="button"
          class="continue-art"
          aria-label="打开正在播放"
          onclick={() => (ui.immersive = true)}
          ><Artwork src={artwork} alt={s.current_track.album} /></button
        >
        <div class="continue-info">
          <strong use:tooltip={{ text: s.current_track.title, overflow: true }}
            >{s.current_track.title}</strong
          >
          <span
            >{s.current_track.artist || '未知艺术家'}{s.current_track.album
              ? ' · ' + s.current_track.album
              : ''}</span
          >
          <div class="continue-progress">
            <div class="meter" role="presentation"><i style:width={progress + '%'}></i></div>
            <small>{duration(s.position_ms)} / {duration(s.duration_ms)}</small>
          </div>
        </div>
        <button
          type="button"
          class="pb-play continue-play"
          aria-label={playing ? '暂停' : '继续播放'}
          disabled={player.busy}
          onclick={() => control(playing ? pause : play)}
        >
          {#if playing}<Pause size={18} fill="currentColor" />{:else}<Play
              size={18}
              fill="currentColor"
              class="pb-play-icon"
            />{/if}
        </button>
      </div>
    </section>
  {/if}

  {#snippet shelf(id: string, heading: string, tracks: LibraryTrack[], more?: () => void)}
    <section class="home-section" aria-labelledby={id}>
      <div class="section-head">
        <h3 {id} class="section-title">{heading}</h3>
        {#if more}<button type="button" class="link-btn" onclick={more}
            >查看全部<ChevronRight size={15} /></button
          >{/if}
      </div>
      <div class="shelf">
        {#each tracks as t (t.id)}
          <article class="card" oncontextmenu={(e) => trackMenu(e, t)}>
            <div class="card-art">
              <Cover id={t.id} alt={t.album || t.title} size={320} />
              <button
                type="button"
                class="card-open"
                aria-label={'播放 ' + t.title}
                onclick={() => perform(() => playTrack(t.id))}
              ></button>
              <span class="card-play" aria-hidden="true"
                ><Play size={18} fill="currentColor" /></span
              >
            </div>
            <span class="card-title" use:tooltip={{ text: t.title, overflow: true }}>{t.title}</span
            >
            <p class="card-sub">{t.artist || '未知艺术家'}</p>
          </article>
        {/each}
      </div>
    </section>
  {/snippet}

  {#if loading}
    <div class="shelf" aria-busy="true">
      {#each Array(6) as _, i (i)}<div class="card">
          <div class="card-art skeleton"></div>
          <span class="skeleton skeleton-text"></span>
        </div>{/each}
    </div>
  {:else}
    {#if recent.length}{@render shelf('home-recent', '最近播放', recent)}{/if}
    {#if added.length}{@render shelf('home-added', '最近添加', added, () =>
        go({ view: 'recent' }),
      )}{/if}

    {#if artists.length}
      <section class="home-section" aria-labelledby="home-artists">
        <div class="section-head">
          <h3 id="home-artists" class="section-title">常听艺术家</h3>
          <button type="button" class="link-btn" onclick={() => go({ view: 'artists' })}
            >全部艺术家<ChevronRight size={15} /></button
          >
        </div>
        <div class="artist-row">
          {#each artists as [name, plays] (name)}
            <button
              type="button"
              class="artist-pill"
              onclick={() => go({ view: 'artist', name })}
              oncontextmenu={(e) => openMenu(e, artistMenu(artistGroup(name)), name + ' 操作')}
            >
              <span class="artist-initial small" style:--hue={nameHue(name)} aria-hidden="true"
                >{initial(name)}</span
              >
              <span class="artist-pill-text"
                ><strong>{name || '未知艺术家'}</strong><small>{plays} 次播放</small></span
              >
            </button>
          {/each}
        </div>
      </section>
    {/if}

    <section class="home-section" aria-labelledby="home-playlists">
      <div class="section-head"><h3 id="home-playlists" class="section-title">我的歌单</h3></div>
      <div class="playlist-row">
        <div
          class="playlist-tile"
          role="group"
          aria-label={LOCAL_COLLECTION.name}
          oncontextmenu={(e) =>
            openMenu(e, localMenu(), '系统歌单「' + LOCAL_COLLECTION.name + '」操作')}
        >
          <button type="button" class="playlist-tile-main" onclick={() => go({ view: 'local' })}>
            <span class="playlist-tile-art"><Artwork kind="playlist" alt="" /></span>
            <span class="playlist-tile-text"
              ><strong>{LOCAL_COLLECTION.name}</strong><small
                >系统歌单 · {library.summary.tracks} 首歌曲</small
              ></span
            >
          </button>
          <button
            type="button"
            class="icon-btn icon-btn-sm playlist-tile-play"
            aria-label={'播放歌单 ' + LOCAL_COLLECTION.name}
            disabled={!library.summary.tracks}
            onclick={() => playCollection(localQuery('added', ''))}
            ><Play size={15} fill="currentColor" /></button
          >
        </div>
        {#each library.summary.playlists as p (p.id)}
          <div
            class="playlist-tile"
            role="group"
            aria-label={p.name}
            oncontextmenu={(e) => openMenu(e, playlistMenu(p), '歌单「' + p.name + '」操作')}
          >
            <button
              type="button"
              class="playlist-tile-main"
              onclick={() => go({ view: 'playlist', id: p.id })}
            >
              <span class="playlist-tile-art"><Artwork kind="playlist" alt="" /></span>
              <span class="playlist-tile-text"
                ><strong use:tooltip={{ text: p.name, overflow: true }}>{p.name}</strong><small
                  >{p.count} 首歌曲</small
                ></span
              >
            </button>
            <button
              type="button"
              class="icon-btn icon-btn-sm playlist-tile-play"
              aria-label={'播放歌单 ' + p.name}
              disabled={!p.count}
              onclick={() => playCollection({ playlist: p.id })}
              ><Play size={15} fill="currentColor" /></button
            >
          </div>
        {/each}
      </div>
      {#if !library.summary.playlists.length}<p class="hint">
          在侧边栏“我的歌单”旁点击 + 创建自己的歌单。
        </p>{/if}
    </section>
  {/if}
{/if}

<style>
  :global {
    /* ============================================================ home */
    .home-header {
      padding: var(--space-4) 0 var(--space-2);
    }

    .home-section {
      margin-top: var(--space-8);
    }

    .home-section:first-of-type {
      margin-top: var(--space-5);
    }

    .section-head {
      display: flex;
      align-items: baseline;
      justify-content: space-between;
      gap: 12px;
      margin-bottom: var(--space-4);
    }

    .section-title {
      font-family: var(--font-display);
      font-size: var(--fs-section);
      font-weight: var(--fw-semibold);
    }

    .home-section > .section-title {
      margin-bottom: var(--space-4);
    }

    .continue {
      display: flex;
      align-items: center;
      gap: var(--space-4);
      max-width: 720px;
      padding: var(--space-3);
      border-radius: var(--radius-lg);
      background: var(--surface);
    }

    .continue-art {
      position: relative;
      flex-shrink: 0;
      width: 72px;
      height: 72px;
      border-radius: var(--radius);
      overflow: hidden;
    }

    .continue-info {
      flex: 1;
      min-width: 0;
      display: flex;
      flex-direction: column;
      gap: 2px;
    }

    .continue-info strong {
      font-size: 15px;
      font-weight: var(--fw-semibold);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .continue-info > span {
      color: var(--text-2);
      font-size: var(--fs-meta);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .continue-progress {
      display: flex;
      align-items: center;
      gap: 10px;
      margin-top: 8px;
    }

    .continue-progress small {
      color: var(--text-3);
      font-size: var(--fs-small);
      font-variant-numeric: tabular-nums;
      white-space: nowrap;
    }

    .meter {
      flex: 1;
      height: 4px;
      border-radius: 2px;
      background: rgb(var(--ink) / 0.14);
      overflow: hidden;
    }

    .meter i {
      display: block;
      height: 100%;
      background: var(--accent);
      border-radius: inherit;
    }

    .artist-row,
    .playlist-row {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
      gap: var(--space-2);
    }

    .artist-pill {
      display: flex;
      align-items: center;
      gap: 12px;
      min-width: 0;
      padding: 8px 12px 8px 8px;
      border-radius: 999px;
      background: var(--surface);
      text-align: left;
      transition: background-color var(--dur-fast);
    }

    .artist-pill:hover {
      background: var(--surface-hover);
    }

    .artist-pill-text {
      min-width: 0;
      display: flex;
      flex-direction: column;
    }

    .artist-pill-text strong,
    .playlist-tile-text strong {
      font-size: var(--fs-body);
      font-weight: var(--fw-semibold);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .artist-pill-text small,
    .playlist-tile-text small {
      color: var(--text-3);
      font-size: var(--fs-small);
    }

    .playlist-tile {
      position: relative;
      display: flex;
      align-items: center;
      min-width: 0;
      border-radius: var(--radius);
      background: var(--surface);
      transition: background-color var(--dur-fast);
    }

    .playlist-tile:hover {
      background: var(--surface-hover);
    }

    .playlist-tile-main {
      flex: 1;
      min-width: 0;
      display: flex;
      align-items: center;
      gap: 12px;
      padding: 8px;
      text-align: left;
      border-radius: inherit;
    }

    .playlist-tile-art {
      position: relative;
      flex-shrink: 0;
      width: 44px;
      height: 44px;
      border-radius: var(--radius-sm);
      overflow: hidden;
    }

    .playlist-tile-text {
      min-width: 0;
      display: flex;
      flex-direction: column;
    }

    .playlist-tile-play {
      margin-right: 8px;
      background: var(--accent);
      color: var(--accent-ink) !important;
      opacity: 0;
    }

    .playlist-tile:hover .playlist-tile-play,
    .playlist-tile:focus-within .playlist-tile-play {
      opacity: 1;
    }

    .playlist-tile-play:hover:not(:disabled) {
      background: var(--accent-hover) !important;
    }
  }
</style>
````

## File: src/lib/views/NowPlaying.svelte
````svelte
<script lang="ts">
  import Select from '../components/Select.svelte';
  import { getLyrics, setLyricsOffset, seek, type Lyrics } from '../services/music';
  import { tick, onMount } from 'svelte';
  import { groupLines, activeLine, scrollTarget } from '../utils/lyrics-view';
  import Artwork from '../components/Artwork.svelte';
  import IconButton from '../components/IconButton.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import { ChevronDown, Music2, LocateFixed } from 'lucide-svelte';
  import { player, control } from '../stores/player.svelte';
  import { ui, notifyError } from '../stores/ui.svelte';
  import { ambientColor } from '../utils/color';

  let { artwork, onclose }: { artwork: string | null; onclose: () => void } = $props();
  let snapshot = $derived(player.snapshot);

  let lyrics = $state<Lyrics | null>(null);
  let loading = $state(false);
  let manual = $state(false);
  let layout = $state('split');
  let container = $state<HTMLDivElement>();
  let request = 0;
  let trackId = $derived(snapshot?.track_id);
  $effect(() => {
    const path = trackId;
    const ticket = ++request;
    lyrics = null;
    manual = false;
    if (path) {
      loading = true;
      getLyrics()
        .then((l) => {
          if (ticket === request) lyrics = l;
        })
        .catch(notifyError)
        .finally(() => {
          if (ticket === request) loading = false;
        });
    }
  });

  // Ambient colour: computed once per song from the already-loaded cover, never per progress tick.
  const ambientCache = new Map<string, string | null>();
  let ambient = $state<string | null>(null);
  $effect(() => {
    const id = trackId,
      src = artwork;
    if (!id || !src) {
      ambient = null;
      return;
    }
    if (ambientCache.has(id)) {
      ambient = ambientCache.get(id)!;
      return;
    }
    let cancelled = false;
    const img = new Image();
    img.src = src;
    img
      .decode()
      .then(() => {
        const canvas = document.createElement('canvas');
        canvas.width = canvas.height = 24;
        const ctx = canvas.getContext('2d', { willReadFrequently: true });
        if (!ctx) return;
        ctx.drawImage(img, 0, 0, 24, 24);
        const rgb = ambientColor(ctx.getImageData(0, 0, 24, 24).data);
        const value = rgb ? rgb.join(' ') : null;
        ambientCache.set(id, value);
        if (ambientCache.size > 60) ambientCache.delete(ambientCache.keys().next().value!);
        if (!cancelled) ambient = value;
      })
      .catch(() => {});
    return () => {
      cancelled = true;
    };
  });

  let fontSize = $state(22);
  onMount(() => {
    try {
      const saved = Number(localStorage.getItem('lyrics-font-size'));
      if (saved >= 16 && saved <= 32) fontSize = saved;
    } catch {
      /* optional preference */
    }
  });
  function changeSize(value: number) {
    fontSize = value;
    try {
      localStorage.setItem('lyrics-font-size', String(value));
    } catch {
      /* keep session preference */
    }
  }
  let rows = $derived(groupLines(lyrics?.lines ?? [], lyrics?.synced ?? false));
  let active = $derived(
    lyrics?.synced
      ? activeLine(rows, (snapshot?.position_ms ?? 0) - (snapshot?.lyrics_offset_ms ?? 0))
      : -1,
  );
  function watchSize(node: HTMLDivElement) {
    const observer = new ResizeObserver(() => {
      if (manual) return;
      const line = node.querySelector<HTMLElement>('[data-line="' + active + '"]');
      if (line)
        node.scrollTo({
          top: scrollTarget(
            line.offsetTop,
            line.offsetHeight,
            node.clientHeight,
            node.scrollHeight,
          ),
          behavior: 'instant',
        });
    });
    observer.observe(node);
    return {
      destroy() {
        observer.disconnect();
      },
    };
  }
  let scrollVersion = 0;
  $effect(() => {
    const index = active;
    const browsing = manual;
    const host = container;
    rows;
    fontSize;
    layout;
    const version = ++scrollVersion;
    if (!browsing && host) {
      void tick().then(() => {
        if (version !== scrollVersion) return;
        const line = host.querySelector<HTMLElement>('[data-line="' + index + '"]');
        host.scrollTo({
          top: line
            ? scrollTarget(line.offsetTop, line.offsetHeight, host.clientHeight, host.scrollHeight)
            : 0,
          behavior: ui.reduceMotion ? 'instant' : 'smooth',
        });
      });
    }
  });
  async function jump(time: number) {
    await control(() =>
      seek(
        Math.min(snapshot?.duration_ms ?? 0, Math.max(0, time + (snapshot?.lyrics_offset_ms ?? 0))),
      ),
    );
    manual = false;
  }
</script>

<section
  style:--lyric-size={fontSize + 'px'}
  style:--ambient={ambient ?? undefined}
  class="immersive"
  class:has-ambient={!!ambient && !ui.reduceMotion}
  aria-label="正在播放"
>
  <header class="immersive-header">
    <IconButton label="收起正在播放 (Esc)" onclick={onclose}><ChevronDown size={20} /></IconButton>
    <span class="immersive-caption">正在播放</span>
    <Select
      label="正在播放布局"
      bind:value={layout}
      options={[
        { value: 'split', label: '封面与歌词' },
        { value: 'cover', label: '大封面' },
        { value: 'lyrics', label: '纯歌词' },
      ]}
    />
  </header>
  <div
    class="immersive-content"
    class:cover-only={layout === 'cover'}
    class:lyrics-only={layout === 'lyrics'}
  >
    {#if layout !== 'lyrics'}<div class="artwork-panel">
        <div class="large-artwork"><Artwork src={artwork} alt="当前专辑封面" /></div>
        <h1 title={snapshot?.current_track?.title}>
          {snapshot?.current_track?.title ?? '未在播放'}
        </h1>
        <p>{snapshot?.current_track?.artist ?? '从音乐库选择一首歌曲'}</p>
        {#if snapshot?.current_track?.album}<small title={snapshot.current_track.album}
            >{snapshot.current_track.album}</small
          >{/if}
      </div>{/if}
    {#if layout !== 'cover'}<div class="lyrics-panel">
        <div class="lyric-tools">
          <span class="lyric-source">{lyrics?.source ?? '本地歌词'}</span>
          <label class="lyric-field"
            >字号 <input
              class="slider slider-sm"
              style:--fill={((fontSize - 16) / 16) * 100 + '%'}
              aria-label="歌词字号"
              type="range"
              min="16"
              max="32"
              step="1"
              value={fontSize}
              oninput={(e) => changeSize(Number(e.currentTarget.value))}
            /><output>{fontSize}</output></label
          >
          <label class="lyric-field"
            >偏移 <input
              class="input input-sm"
              aria-label="歌词时间偏移（毫秒，正数延后）"
              type="number"
              min="-60000"
              max="60000"
              step="100"
              value={snapshot?.lyrics_offset_ms ?? 0}
              onchange={(e) => control(() => setLyricsOffset(Number(e.currentTarget.value)))}
            /> ms</label
          >
          {#if manual}<button
              type="button"
              class="btn btn-ghost btn-sm"
              onclick={() => (manual = false)}><LocateFixed size={14} />回到当前行</button
            >{/if}
        </div>
        <div
          class="lyric-lines"
          class:empty-lyrics={!lyrics?.lines.length}
          bind:this={container}
          use:watchSize
          onwheel={() => (manual = true)}
          ontouchmove={() => (manual = true)}
          onpointerdown={() => (manual = true)}
          onkeydown={(e) => {
            if (['ArrowDown', 'ArrowUp', 'PageDown', 'PageUp', 'Home', 'End'].includes(e.key))
              manual = true;
          }}
          tabindex="0"
          role="listbox"
          aria-label="歌词，滚动后可回到当前行"
        >
          {#if loading}<p class="lyric-empty">
              正在读取歌词…
            </p>{:else if lyrics?.lines.length}{#each rows as line, i}<button
                role="option"
                aria-selected={active === i}
                data-line={i}
                class:current-line={active === i}
                class:past-line={active > i}
                disabled={!lyrics.synced || snapshot?.playback_state === 'idle'}
                onclick={() => jump(line.time_ms)}
                >{#each line.parts as part, j}<span class:lyric-secondary={j > 0}
                    >{part || '♪'}</span
                  >{/each}</button
              >{/each}{:else}<div class="lyric-empty">
              <EmptyState
                icon={Music2}
                title="暂无歌词"
                message="把同名 .lrc 文件放在歌曲旁边，或使用歌曲内嵌歌词。"
              />
            </div>{/if}
        </div>
      </div>{/if}
  </div>
</section>

<style>
  :global {
    /* ============================================================ now playing */
    .immersive-layer {
      position: fixed;
      inset: 0 0 var(--player-h) 0;
      z-index: var(--z-immersive);
    }

    .immersive {
      position: absolute;
      inset: 0;
      display: flex;
      flex-direction: column;
      overflow: hidden;
      background: var(--bg);
    }

    .immersive::before {
      content: '';
      position: absolute;
      inset: 0;
      pointer-events: none;
      background: radial-gradient(ellipse 70% 80% at 22% 40%, var(--glow), transparent 70%);
    }

    .immersive.has-ambient::before {
      background:
        radial-gradient(
          ellipse 75% 85% at 22% 40%,
          rgb(var(--ambient) / var(--ambient-a)),
          transparent 72%
        ),
        radial-gradient(
          ellipse 60% 70% at 85% 80%,
          rgb(var(--ambient) / var(--ambient-b)),
          transparent 70%
        );
    }

    .immersive > * {
      position: relative;
    }

    .immersive-header {
      display: flex;
      align-items: center;
      gap: var(--space-3);
      height: 56px;
      padding: 0 var(--page-x) 0 calc(var(--page-x) - 8px);
      flex-shrink: 0;
    }

    .immersive-caption {
      flex: 1;
      color: var(--text-2);
      font-size: var(--fs-small);
      font-weight: var(--fw-semibold);
    }

    .immersive-content {
      flex: 1;
      min-height: 0;
      display: grid;
      grid-template-columns: minmax(0, 1fr) minmax(0, 1.2fr);
      gap: clamp(24px, 5vw, 80px);
      width: 100%;
      max-width: 1240px;
      margin: 0 auto;
      padding: var(--space-4) clamp(20px, 4vw, 64px) var(--space-6);
      align-items: center;
    }

    .cover-only,
    .lyrics-only {
      grid-template-columns: minmax(0, 1fr);
      max-width: 760px;
    }

    .artwork-panel {
      min-width: 0;
      max-height: 100%;
      display: flex;
      flex-direction: column;
      align-items: center;
      text-align: center;
      overflow: hidden;
    }

    .large-artwork {
      position: relative;
      width: min(100%, 42vh, 440px);
      aspect-ratio: 1;
      border-radius: var(--radius-xl);
      overflow: hidden;
      box-shadow: var(--shadow-art);
    }

    .artwork-panel h1 {
      margin-top: var(--space-6);
      max-width: 100%;
      font-family: var(--font-display);
      font-size: clamp(20px, 2vw, 28px);
      font-weight: 700;
      line-height: 1.3;
      overflow-wrap: anywhere;
      display: -webkit-box;
      -webkit-line-clamp: 2;
      line-clamp: 2;
      -webkit-box-orient: vertical;
      overflow: hidden;
    }

    .artwork-panel p {
      margin-top: 6px;
      color: var(--text-2);
      font-size: 15px;
    }

    .artwork-panel small {
      margin-top: 2px;
      max-width: 100%;
      color: var(--text-3);
      font-size: var(--fs-meta);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .lyrics-panel {
      height: 100%;
      min-height: 0;
      min-width: 0;
      display: flex;
      flex-direction: column;
    }

    .lyric-tools {
      display: flex;
      align-items: center;
      flex-wrap: wrap;
      gap: 8px 16px;
      padding-bottom: var(--space-3);
      color: var(--text-3);
      font-size: var(--fs-small);
      flex-shrink: 0;
    }

    .lyric-source {
      max-width: 40%;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .lyric-field {
      display: inline-flex;
      align-items: center;
      gap: 8px;
    }

    .lyric-field output {
      min-width: 20px;
      font-variant-numeric: tabular-nums;
    }

    .lyric-lines {
      position: relative;
      flex: 1;
      min-height: 0;
      overflow-x: hidden;
      overflow-y: auto;
      overscroll-behavior: contain;
      padding: 38% 12px 60%;
      scrollbar-gutter: stable;
      overflow-anchor: none;
      mask-image: linear-gradient(to bottom, transparent, #000 12%, #000 82%, transparent);
      outline: none;
    }

    .lyric-lines:focus-visible {
      box-shadow: inset 0 0 0 2px var(--focus);
      border-radius: var(--radius);
    }

    .lyric-lines > button {
      display: block;
      width: 100%;
      max-width: 36rem;
      margin-inline: auto;
      padding: 10px 8px;
      border-radius: var(--radius);
      text-align: start;
      font-family: var(--font-display);
      font-size: clamp(16px, calc(var(--lyric-size) * 0.7 + 0.5vw), 32px);
      font-weight: 600;
      line-height: 1.5;
      color: rgb(var(--ink) / var(--lyric-dim));
      white-space: pre-wrap;
      overflow-wrap: anywhere;
      transition: color var(--dur) var(--ease);
    }

    .lyric-lines > button:not(:disabled):hover {
      color: rgb(var(--ink) / 0.7);
      background: rgb(var(--ink) / 0.04);
    }

    .lyric-lines > button.past-line {
      color: rgb(var(--ink) / var(--lyric-past));
    }

    .lyric-lines > button.current-line {
      color: var(--text);
    }

    .lyric-lines > button:disabled {
      cursor: default;
    }

    .lyric-lines > button > span {
      display: block;
    }

    .lyric-lines > button .lyric-secondary {
      font-size: 0.74em;
      font-weight: 400;
      margin-top: 0.2em;
    }

    .lyric-lines.empty-lyrics {
      display: grid;
      place-items: center;
      padding: 24px;
      mask-image: none;
    }

    .lyric-empty {
      color: var(--text-3);
    }
  }
</style>
````

## File: src/lib/views/SettingsView.svelte
````svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { FolderPlus, Database, X } from 'lucide-svelte';
  import Select from '../components/Select.svelte';
  import { shortcutHelp } from '../utils/shortcuts';
  import { desktopAvailable, mediaStatus, desktopPreferences, cancelScan } from '../services/music';
  import { player } from '../stores/player.svelte';
  import { library } from '../stores/library.svelte';
  import { ui, notify, notifyError } from '../stores/ui.svelte';
  import { importFolder, importLegacyDatabase } from '../actions';
  import { setReduceMotion, setTheme } from '../stores/preferences';
  import { parseTheme } from '../utils/theme';

  let media = $state('尚未连接桌面');
  let closeToTray = $state(false);
  let desktopReady = $state(false);
  let saving = $state(false);
  let importing = $state(false);

  onMount(() => {
    if (!desktopAvailable) return;
    void mediaStatus()
      .then((s) => (media = s))
      .catch(notifyError);
    void desktopPreferences()
      .then((p) => {
        closeToTray = p.close_to_tray;
        desktopReady = true;
      })
      .catch(notifyError);
  });

  async function changeClose(value: string) {
    saving = true;
    try {
      const enabled = value === 'tray';
      const p = await desktopPreferences(enabled);
      closeToTray = p.close_to_tray;
      if (enabled) notify('关闭窗口后音乐继续播放；右键托盘图标选择“退出”可结束程序。');
    } catch (e) {
      notifyError(e);
    } finally {
      saving = false;
    }
  }
  async function legacy() {
    importing = true;
    try {
      await importLegacyDatabase();
    } finally {
      importing = false;
    }
  }
  let scanning = $derived(!!library.progress?.running);
</script>

<header class="page-header">
  <div>
    <h2 class="page-title">设置</h2>
    <p class="page-meta">Music Mine v0.1.0 · 本地离线运行</p>
  </div>
</header>

<div class="settings">
  <section class="settings-card" aria-labelledby="set-library">
    <h3 id="set-library">音乐库</h3>
    <p class="settings-note">Music Mine 只读取音乐文件和标签，从不修改、移动或删除它们。</p>
    {#if library.summary.folders.length}
      <ul class="folder-list">
        {#each library.summary.folders as folder (folder)}<li class="mono" title={folder}>
            {folder}
          </li>{/each}
      </ul>
    {:else}<p class="settings-note">尚未添加音乐文件夹。</p>{/if}
    <div class="settings-actions">
      <button
        type="button"
        class="btn btn-primary"
        onclick={importFolder}
        disabled={scanning || !desktopAvailable}
        ><FolderPlus size={16} />添加或重新扫描文件夹</button
      >
    </div>
    {#if library.progress && (library.progress.running || library.progress.visited || library.progress.errors.length)}
      {@const p = library.progress}
      <div class="scan-status" aria-live="polite">
        <div class="scan-line">
          <strong
            >{p.running
              ? '正在扫描'
              : p.cancelled
                ? '扫描已取消，已导入的歌曲会保留'
                : '扫描完成'}</strong
          ><span>已检查 {p.visited} · 已导入或更新 {p.imported}</span>
          {#if p.running}<button
              type="button"
              class="btn btn-ghost btn-sm"
              onclick={() => cancelScan().catch(notifyError)}><X size={14} />取消</button
            >{/if}
        </div>
        {#if p.current}<small class="mono">{p.current}</small>{/if}
        {#if p.errors.length}<details>
            <summary>{p.errors.length} 条错误</summary>{#each p.errors as e, i (i)}<p class="mono">
                {e}
              </p>{/each}
          </details>{/if}
      </div>
    {/if}
  </section>

  <section class="settings-card" aria-labelledby="set-playback">
    <h3 id="set-playback">播放</h3>
    <dl class="settings-list">
      <div>
        <dt>音频输出</dt>
        <dd>{player.snapshot?.output_device_id ?? '尚未打开音频输出设备'}</dd>
      </div>
      <div>
        <dt>Windows 媒体控制</dt>
        <dd>{media}</dd>
      </div>
    </dl>
    <p class="settings-note">播放队列、进度、音量和播放模式会自动保存，重新启动后保持暂停。</p>
  </section>

  <section class="settings-card" aria-labelledby="set-appearance">
    <h3 id="set-appearance">外观</h3>
    <div class="settings-row">
      <div>
        <strong>主题</strong>
        <p>同时应用于窗口标题栏。“跟随系统”会随 Windows 的深色/浅色设置自动切换。</p>
      </div>
      <Select
        label="主题"
        value={ui.theme}
        onchange={(v) => setTheme(parseTheme(v))}
        options={[
          { value: 'dark', label: '深色' },
          { value: 'light', label: '浅色' },
          { value: 'system', label: '跟随系统' },
        ]}
      />
    </div>
    <div class="settings-row">
      <div>
        <strong>减少动态效果</strong>
        <p>关闭过渡动画与正在播放页面的氛围光晕。系统已开启“减少动画”时自动生效。</p>
      </div>
      <label class="switch"
        ><input
          type="checkbox"
          role="switch"
          checked={ui.reduceMotion}
          onchange={(e) => setReduceMotion(e.currentTarget.checked)}
        /><span aria-hidden="true"></span><span class="sr-only">减少动态效果</span></label
      >
    </div>
  </section>

  <section class="settings-card" aria-labelledby="set-window">
    <h3 id="set-window">窗口</h3>
    <div class="settings-row">
      <div>
        <strong>关闭主窗口时</strong>
        <p>选择托盘后，左键托盘图标恢复窗口，右键可控制播放或退出。</p>
      </div>
      <Select
        label="关闭主窗口时"
        value={closeToTray ? 'tray' : 'exit'}
        disabled={!desktopReady || saving}
        onchange={changeClose}
        options={[
          { value: 'exit', label: '退出程序' },
          { value: 'tray', label: '最小化到系统托盘' },
        ]}
      />
    </div>
  </section>

  <section class="settings-card" aria-labelledby="set-keys">
    <h3 id="set-keys">键盘快捷键</h3>
    <dl class="shortcut-list">
      {#each shortcutHelp as [key, label] (key)}<div>
          <dt><kbd>{key}</kbd></dt>
          <dd>{label}</dd>
        </div>{/each}
    </dl>
    <p class="settings-note">
      仅在窗口内生效，不占用其他应用的全局快捷键。输入文字时保留编辑操作。
    </p>
  </section>

  <section class="settings-card" aria-labelledby="set-legacy">
    <h3 id="set-legacy">从旧版迁移</h3>
    <p class="settings-note">
      选择旧版数据库，制作一致性副本后导入。仅支持空音乐库；旧数据库与音乐文件保持只读。
    </p>
    <div class="settings-actions">
      <button
        type="button"
        class="btn btn-secondary"
        onclick={legacy}
        disabled={importing || scanning || !desktopAvailable}
        ><Database size={16} />{importing ? '正在备份与校验…' : '导入旧版数据库'}</button
      >
    </div>
  </section>
</div>

<style>
  :global {
    /* ============================================================ settings */
    .settings {
      display: flex;
      flex-direction: column;
      gap: var(--space-4);
      max-width: 760px;
    }

    .settings-card {
      padding: var(--space-5);
      border-radius: var(--radius-lg);
      background: var(--surface);
    }

    .settings-card h3 {
      margin-bottom: var(--space-3);
      font-size: 15px;
      font-weight: var(--fw-semibold);
    }

    .settings-note {
      color: var(--text-2);
      font-size: var(--fs-meta);
      line-height: 1.6;
    }

    .settings-actions {
      display: flex;
      gap: 8px;
      margin-top: var(--space-4);
    }

    .folder-list {
      display: flex;
      flex-direction: column;
      gap: 4px;
      margin-top: var(--space-3);
    }

    .folder-list li {
      padding: 8px 12px;
      border-radius: var(--radius-sm);
      background: var(--bg);
      color: var(--text-2);
    }

    .scan-status {
      display: flex;
      flex-direction: column;
      gap: 6px;
      margin-top: var(--space-4);
      padding: var(--space-3);
      border-radius: var(--radius);
      background: var(--bg);
      font-size: var(--fs-meta);
    }

    .scan-line {
      display: flex;
      align-items: center;
      gap: 12px;
      flex-wrap: wrap;
    }

    .scan-line span {
      color: var(--text-2);
    }

    .scan-status small {
      color: var(--text-3);
    }

    .scan-status details {
      color: var(--danger);
    }

    .settings-list {
      display: grid;
      gap: 10px;
    }

    .settings-list > div,
    .settings-row {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: var(--space-4);
    }

    .settings-list dt {
      color: var(--text-2);
      font-size: var(--fs-meta);
    }

    .settings-list dd {
      color: var(--text);
      font-size: var(--fs-meta);
      text-align: right;
      overflow-wrap: anywhere;
    }

    .settings-row {
      padding: var(--space-3) 0;
      border-top: 1px solid var(--line);
    }

    .settings-row:first-of-type {
      border-top: 0;
      padding-top: 0;
    }

    .settings-row strong {
      font-size: var(--fs-body);
      font-weight: var(--fw-semibold);
    }

    .settings-row p {
      margin-top: 2px;
      color: var(--text-3);
      font-size: var(--fs-small);
      line-height: 1.5;
    }

    .shortcut-list {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
      gap: 6px 24px;
      margin-bottom: var(--space-3);
    }

    .shortcut-list > div {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 12px;
      padding: 6px 0;
      border-bottom: 1px solid var(--line);
    }

    .shortcut-list dd {
      color: var(--text-2);
      font-size: var(--fs-meta);
    }
  }
</style>
````

## File: src/lib/views/SongsView.svelte
````svelte
<script lang="ts">
  import {
    Play,
    Shuffle,
    ListEnd,
    Ellipsis,
    Search,
    Music2,
    FolderPlus,
    ListMusic,
  } from 'lucide-svelte';
  import TrackList from '../components/TrackList.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import Select from '../components/Select.svelte';
  import Cover from '../components/Cover.svelte';
  import Artwork from '../components/Artwork.svelte';
  import { nav, memory, routeKey, go } from '../stores/nav.svelte';
  import { library } from '../stores/library.svelte';
  import { ui, openMenu } from '../stores/ui.svelte';
  import { control } from '../stores/player.svelte';
  import { setMode, desktopAvailable, type LibraryQuery } from '../services/music';
  import { playCollection, playlistMenu, albumMenu, artistMenu, importFolder } from '../actions';
  import { LOCAL_COLLECTION, localQuery } from '../utils/collections';

  let route = $derived(nav.route);
  let key = $derived(routeKey(route));

  let search = $state(ui.query);
  $effect(() => {
    const q = ui.query;
    if (q === search) return;
    const t = setTimeout(() => (search = q), 180);
    return () => clearTimeout(t);
  });

  let sorts = $state<Record<string, string>>({});
  let sort = $derived(
    sorts[key] ?? memory.get(key)?.sort ?? (route.view === 'artist' ? 'album' : 'added'),
  );
  function changeSort(next: string) {
    sorts[key] = next;
    memory.set(key, { scroll: 0, ...memory.get(key), sort: next });
  }

  let album = $derived(
    route.view === 'album'
      ? (library.summary.albums.find((a) => a.key === route.key) ?? null)
      : null,
  );
  let artist = $derived(
    route.view === 'artist'
      ? (library.summary.artists.find((a) => a.name === route.name) ?? null)
      : null,
  );
  let playlist = $derived(
    route.view === 'playlist'
      ? (library.summary.playlists.find((p) => p.id === route.id) ?? null)
      : null,
  );

  let base = $derived.by((): LibraryQuery => {
    switch (route.view) {
      case 'album':
        return { album: route.key, sort: 'album' };
      case 'artist':
        return { artist: route.name, sort };
      case 'playlist':
        return { playlist: route.id };
      case 'recent':
        return { sort: 'added' };
      case 'local':
        return localQuery(sort, '');
      default:
        return { sort };
    }
  });
  let query = $derived<LibraryQuery>({ ...base, search: search.trim() });
  let sortable = $derived(
    route.view === 'songs' || route.view === 'local' || route.view === 'artist',
  );

  let total = $state(0);
  let loaded = $state(false);
  let title = $derived(
    route.view === 'songs'
      ? '歌曲'
      : route.view === 'recent'
        ? '最近添加'
        : route.view === 'local'
          ? LOCAL_COLLECTION.name
          : (album?.name ?? artist?.name ?? playlist?.name ?? ''),
  );

  async function shufflePlay() {
    await control(() => setMode('shuffle'));
    await playCollection(base);
  }
  function headerMenu(e: MouseEvent) {
    const target = e.currentTarget as HTMLElement;
    if (playlist) openMenu(target, playlistMenu(playlist), '歌单操作');
    else if (album) openMenu(target, albumMenu(album), '专辑操作');
    else if (artist) openMenu(target, artistMenu(artist), '艺术家操作');
  }
  const sortOptions = [
    { value: 'added', label: '最近添加' },
    { value: 'title', label: '标题' },
    { value: 'artist', label: '艺术家' },
    { value: 'album', label: '专辑与曲序' },
    { value: 'duration', label: '时长' },
  ];
</script>

{#if route.view === 'album' || route.view === 'artist' || route.view === 'playlist'}
  <header class="detail-hero">
    <div class="hero-art" class:round={route.view === 'artist'}>
      {#if album}<Cover id={album.cover_id} alt={album.name} size={360} />
      {:else if route.view === 'artist'}<Artwork kind="artist" alt={title} />
      {:else}<Artwork kind="playlist" alt={title} />{/if}
    </div>
    <div class="hero-info">
      <span class="hero-kind"
        >{route.view === 'album' ? '专辑' : route.view === 'artist' ? '艺术家' : '歌单'}</span
      >
      <h2 class="hero-title">{title || (route.view === 'playlist' ? '歌单不存在' : '未知')}</h2>
      <p class="hero-meta">
        {#if album && album.artist}<button
            type="button"
            class="link"
            onclick={() => go({ view: 'artist', name: album!.artist })}>{album.artist}</button
          ><span aria-hidden="true"> · </span>{/if}
        {loaded ? total : (album?.count ?? artist?.count ?? playlist?.count ?? 0)} 首歌曲
      </p>
      <div class="hero-actions">
        <button
          type="button"
          class="btn btn-primary"
          disabled={!total}
          onclick={() => playCollection(base)}><Play size={16} fill="currentColor" />播放</button
        >
        <button type="button" class="btn btn-secondary" disabled={!total} onclick={shufflePlay}
          ><Shuffle size={16} />随机播放</button
        >
        <button
          type="button"
          class="btn btn-ghost"
          disabled={!total}
          onclick={() => playCollection(base, false, title)}><ListEnd size={16} />加入队列</button
        >
        {#if album || artist || playlist}<button
            type="button"
            class="icon-btn"
            aria-label="更多操作"
            aria-haspopup="menu"
            onclick={headerMenu}><Ellipsis size={18} /></button
          >{/if}
      </div>
    </div>
  </header>
{:else}
  <header class="page-header">
    <div>
      {#if route.view === 'local'}<span class="hero-kind"
          >系统歌单 · 自动包含音乐库中的全部歌曲</span
        >{/if}
      <h2 class="page-title">{title}</h2>
      <p class="page-meta">
        {loaded ? `${total} 首歌曲` : '正在读取…'}{#if search.trim()}
          · 搜索“{search.trim()}”{/if}
      </p>
    </div>
    <div class="page-actions">
      <button
        type="button"
        class="btn btn-primary"
        disabled={!total}
        onclick={() => playCollection({ ...base, search: search.trim() })}
        ><Play size={16} fill="currentColor" />播放全部</button
      >
      <button type="button" class="btn btn-secondary" disabled={!total} onclick={shufflePlay}
        ><Shuffle size={16} />随机播放</button
      >
    </div>
  </header>
  {#if route.view === 'songs' || route.view === 'local'}
    <div class="toolbar">
      <span class="toolbar-spacer"></span>
      <span class="toolbar-label">排序</span><Select
        label="歌曲排序"
        value={sort}
        options={sortOptions}
        onchange={changeSort}
      />
    </div>
  {/if}
{/if}

{#if !desktopAvailable}
  <EmptyState
    icon={Music2}
    title="请在 Music Mine 桌面应用中使用"
    message="浏览器预览无法读取本地音乐库。"
  />
{:else if library.loaded && !library.summary.tracks}
  <EmptyState
    icon={FolderPlus}
    title="音乐库还是空的"
    message="导入你的 MP3、FLAC、WAV 文件夹。Music Mine 只读取文件，不会修改音乐或标签。"
  >
    <button type="button" class="btn btn-primary" onclick={importFolder}
      ><FolderPlus size={16} />导入音乐文件夹</button
    >
  </EmptyState>
{:else if route.view === 'playlist' && !playlist && library.loaded}
  <EmptyState icon={ListMusic} title="这个歌单已不存在" message="它可能已被删除。" />
{:else}
  {#key key}
    <TrackList
      {query}
      memoryKey={key}
      playlistId={playlist?.id ?? null}
      numbering={route.view === 'album' ? 'track' : 'index'}
      {sortable}
      onsort={sortable ? changeSort : undefined}
      ontotal={(t, l) => {
        total = t;
        loaded = l;
      }}
    />
  {/key}
  {#if loaded && !total}
    {#if search.trim()}<EmptyState
        icon={Search}
        title="没有找到匹配的歌曲"
        message={`没有与“${search.trim()}”匹配的歌曲。试试歌曲名、艺术家或专辑的其他关键词。`}
        ><button type="button" class="btn btn-ghost" onclick={() => (ui.query = '')}
          >清除搜索</button
        ></EmptyState
      >
    {:else if route.view === 'playlist'}<EmptyState
        icon={ListMusic}
        title="歌单还是空的"
        message="在任意歌曲上点右键，选择“添加到歌单”。"
        ><button type="button" class="btn btn-ghost" onclick={() => go({ view: 'songs' })}
          >浏览歌曲</button
        ></EmptyState
      >
    {:else}<EmptyState icon={Music2} title="这里还没有歌曲" />{/if}
  {/if}
{/if}
````

## File: src/lib/actions.ts
````typescript
import {
  Play,
  ListEnd,
  ListStart,
  Info,
  Pencil,
  Trash2,
  ListPlus,
  Disc3,
  MicVocal,
  ListMinus,
  Shuffle,
} from 'lucide-svelte';
import {
  playTrack,
  queueAction,
  queueTracks,
  collectTrackIds,
  addToPlaylist,
  removeFromPlaylist,
  setMode,
  createPlaylist,
  renamePlaylist,
  deletePlaylist,
  trackDetails,
  scanFolder,
  scanProgress,
  importLegacy,
  desktopAvailable,
  type LibraryTrack,
  type LibraryQuery,
  type Playlist,
  type Group,
} from './services/music';
import { perform, control } from './stores/player.svelte';
import { library, refreshSummary } from './stores/library.svelte';
import { go, forgetPlaylist } from './stores/nav.svelte';
import {
  ui,
  confirmAction,
  promptText,
  notify,
  notifyError,
  type MenuItem,
} from './stores/ui.svelte';
import { playlistNameError, suggestPlaylistName } from './utils/playlist-name';
import { LOCAL_COLLECTION, localQuery } from './utils/collections';

export async function playCollection(query: LibraryQuery, replace = true, label = '') {
  try {
    const ids = await collectTrackIds(query);
    if (!ids.length) {
      notify('这里还没有可以播放的歌曲');
      return;
    }
    await perform(
      () => queueTracks(ids, replace),
      replace ? undefined : `已将 ${ids.length} 首歌曲${label ? '（' + label + '）' : ''}加入队列`,
    );
  } catch (e) {
    notifyError(e);
  }
}

export async function importFolder() {
  if (!desktopAvailable) {
    notify('请在 Music Mine 桌面窗口中导入音乐文件夹');
    return;
  }
  try {
    await scanFolder();
    library.progress = await scanProgress();
    if (!library.progress.running) await refreshSummary();
  } catch (e) {
    notifyError(e);
  }
}

export async function importLegacyDatabase() {
  try {
    const r = await importLegacy();
    if (r) {
      notify(`已安全导入 ${r.tracks} 首歌曲、${r.playlists} 个歌单。${r.warnings.join(' ')}`);
      await refreshSummary();
    }
  } catch (e) {
    notifyError(e);
  }
}

export async function openDetails(track: LibraryTrack) {
  try {
    ui.details = await trackDetails(track.id);
  } catch (e) {
    notifyError(e);
  }
}

async function addTo(playlist: Playlist, track: LibraryTrack) {
  try {
    await addToPlaylist(playlist.id, track.id);
    await refreshSummary();
    notify(`已加入「${playlist.name}」`);
  } catch (e) {
    notifyError(e);
  }
}

export function songMenu(
  track: LibraryTrack,
  options: { playlistId?: number | null; onDetails: () => void },
): MenuItem[] {
  const items: MenuItem[] = [
    { label: '播放', icon: Play, action: () => perform(() => playTrack(track.id)) },
    {
      label: '下一首播放',
      icon: ListStart,
      action: () => perform(() => queueAction('play_next', track.id), '将在当前歌曲之后播放'),
    },
    {
      label: '加入播放队列',
      icon: ListEnd,
      action: () => perform(() => queueAction('add', track.id), '已加入播放队列'),
    },
  ];
  const targets = library.summary.playlists.filter((p) => p.id !== options.playlistId);
  items.push({ kind: 'separator' });
  if (targets.length) {
    items.push({ kind: 'heading', label: '添加到歌单' });
    for (const p of targets.slice(0, 12))
      items.push({ label: p.name, icon: ListPlus, action: () => addTo(p, track) });
  }
  items.push({ label: '新建歌单并添加…', icon: ListPlus, action: () => newPlaylist(track) });
  if (options.playlistId != null) {
    const id = options.playlistId;
    items.push(
      { kind: 'separator' },
      {
        label: '从此歌单移除',
        icon: ListMinus,
        action: async () => {
          try {
            await removeFromPlaylist(id, track.id);
            await refreshSummary();
            notify('已从歌单移除，音乐文件不受影响');
          } catch (e) {
            notifyError(e);
          }
        },
      },
    );
  }
  items.push({ kind: 'separator' }, { label: '歌曲详情', icon: Info, action: options.onDetails });
  return items;
}

export function albumMenu(album: Group): MenuItem[] {
  const query = { album: album.key, sort: 'album' };
  return [
    { label: '播放专辑', icon: Play, action: () => playCollection(query) },
    {
      label: '加入播放队列',
      icon: ListEnd,
      action: () => playCollection(query, false, album.name),
    },
    { kind: 'separator' },
    { label: '打开专辑', icon: Disc3, action: () => go({ view: 'album', key: album.key }) },
  ];
}

export function artistMenu(artist: Group): MenuItem[] {
  const query = { artist: artist.name, sort: 'album' };
  return [
    { label: '播放全部', icon: Play, action: () => playCollection(query) },
    {
      label: '加入播放队列',
      icon: ListEnd,
      action: () => playCollection(query, false, artist.name),
    },
    { kind: 'separator' },
    {
      label: '查看艺术家',
      icon: MicVocal,
      action: () => go({ view: 'artist', name: artist.name }),
    },
  ];
}

export function playlistMenu(playlist: Playlist): MenuItem[] {
  return [
    {
      label: '播放歌单',
      icon: Play,
      disabled: !playlist.count,
      action: () => playCollection({ playlist: playlist.id }),
    },
    {
      label: '加入播放队列',
      icon: ListEnd,
      disabled: !playlist.count,
      action: () => playCollection({ playlist: playlist.id }, false, playlist.name),
    },
    { kind: 'separator' },
    { label: '重命名…', icon: Pencil, action: () => renamePlaylistDialog(playlist) },
    {
      label: '删除歌单…',
      icon: Trash2,
      danger: true,
      action: () => deletePlaylistDialog(playlist),
    },
  ];
}

/** The built-in collection can be played but never renamed or deleted: it owns no rows and no files. */
export function localMenu(): MenuItem[] {
  const query = localQuery('added', '');
  return [
    {
      label: '播放全部',
      icon: Play,
      disabled: !library.summary.tracks,
      action: () => playCollection(query),
    },
    {
      label: '随机播放',
      icon: Shuffle,
      disabled: !library.summary.tracks,
      action: async () => {
        if (await control(() => setMode('shuffle'))) await playCollection(query);
      },
    },
    {
      label: '加入播放队列',
      icon: ListEnd,
      disabled: !library.summary.tracks,
      action: () => playCollection(query, false, LOCAL_COLLECTION.name),
    },
  ];
}

export async function newPlaylist(track?: LibraryTrack) {
  let created: number | null = null;
  const ok = await promptText({
    title: track ? '新建歌单并添加歌曲' : '新建歌单',
    label: '歌单名称',
    confirmLabel: '创建',
    value: suggestPlaylistName(library.summary.playlists),
    validate: (v) => playlistNameError(v, library.summary.playlists),
    submit: async (v) => {
      created = await createPlaylist(v.trim());
      if (track) await addToPlaylist(created, track.id);
    },
  });
  if (!ok || created == null) return;
  await refreshSummary();
  if (track) notify('已创建歌单并加入歌曲');
  else go({ view: 'playlist', id: created });
}

export async function renamePlaylistDialog(playlist: Playlist) {
  const ok = await promptText({
    title: '重命名歌单',
    label: '歌单名称',
    value: playlist.name,
    confirmLabel: '保存',
    validate: (v) => playlistNameError(v, library.summary.playlists, playlist.id),
    submit: (v) => renamePlaylist(playlist.id, v.trim()),
  });
  if (ok) {
    await refreshSummary();
    notify('歌单已重命名');
  }
}

export async function deletePlaylistDialog(playlist: Playlist) {
  const ok = await confirmAction({
    title: `删除歌单「${playlist.name}」？`,
    message: '只删除歌单，不删除本地音乐文件。歌曲仍保留在音乐库中。',
    detail: playlist.count
      ? `歌单内有 ${playlist.count} 首歌曲。播放队列中的歌曲不受影响，当前播放会继续。`
      : undefined,
    confirmLabel: '删除歌单',
    danger: true,
  });
  if (!ok) return;
  try {
    await deletePlaylist(playlist.id);
    forgetPlaylist(playlist.id);
    notify(`已删除歌单「${playlist.name}」`);
  } catch (e) {
    notifyError(e);
  } finally {
    await refreshSummary();
  }
}
````

## File: src/app.css
````css
@import 'tailwindcss';
@import './lib/styles/tokens.css';

/* ============================================================ base */
*,
*::before,
*::after {
  box-sizing: border-box;
}

html,
body,
#app {
  height: 100%;
  margin: 0;
  overflow: hidden;
}

body {
  background: var(--bg);
  color: var(--text);
  font-family: var(--font-sans);
  font-size: var(--fs-body);
  line-height: 1.45;
  font-synthesis: none;
  -webkit-font-smoothing: antialiased;
  text-rendering: optimizeLegibility;
}

h1,
h2,
h3,
h4,
p,
dl,
dd,
ul {
  margin: 0;
  padding: 0;
}

ul {
  list-style: none;
}

button,
input,
select,
textarea {
  font: inherit;
  color: inherit;
}

button {
  background: none;
  border: 0;
  padding: 0;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
}

button:disabled {
  cursor: default;
}

img {
  display: block;
}

kbd {
  font-family: var(--font-sans);
  font-size: 11px;
  color: var(--text-3);
  border: 1px solid var(--line-strong);
  border-radius: var(--radius-xs);
  padding: 1px 5px;
  white-space: nowrap;
  line-height: 1.4;
}

:focus-visible {
  outline: 2px solid var(--focus);
  outline-offset: 2px;
}

::selection {
  background: var(--accent-soft);
}

.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  overflow: hidden;
  clip-path: inset(50%);
  white-space: nowrap;
}

.mono {
  font-family: var(--font-mono);
  font-size: var(--fs-small);
  overflow-wrap: anywhere;
}

.num,
.pb-time,
.col-duration,
.col-index,
.queue-duration {
  font-variant-numeric: tabular-nums;
}

.muted {
  color: var(--text-3);
}

/* Thin, quiet scrollbars that still show where content scrolls. */
* {
  scrollbar-width: thin;
  scrollbar-color: rgb(var(--ink) / 0.18) transparent;
}

::-webkit-scrollbar {
  width: 12px;
  height: 12px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: rgb(var(--ink) / 0.16);
  border: 3px solid transparent;
  background-clip: padding-box;
  border-radius: 8px;
}

::-webkit-scrollbar-thumb:hover {
  background-color: rgb(var(--ink) / 0.3);
}

::-webkit-scrollbar-corner {
  background: transparent;
}

/* ============================================================ shell */
.app {
  height: 100dvh;
  display: grid;
  grid-template-columns: var(--sidebar-w) minmax(0, 1fr);
  grid-template-rows: minmax(0, 1fr) var(--player-h);
  overflow: hidden;
  background: var(--bg);
}

.workspace {
  grid-column: 2;
  grid-row: 1;
  min-width: 0;
  min-height: 0;
  display: grid;
  grid-template-rows: var(--topbar-h) minmax(0, 1fr);
}

.content {
  position: relative;
  min-height: 0;
  min-width: 0;
  display: grid;
  grid-template-columns: minmax(0, 1fr);
  overflow: hidden;
}

.main {
  min-height: 0;
  min-width: 0;
  overflow-x: hidden;
  overflow-y: auto;
  overscroll-behavior: contain;
  padding: var(--page-top) var(--page-x) var(--space-8);
  scrollbar-gutter: stable;
}

/* ============================================================ buttons & inputs */
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  height: var(--control-h);
  padding: 0 14px;
  border-radius: 999px;
  font-size: var(--fs-meta);
  font-weight: var(--fw-semibold);
  white-space: nowrap;
  transition:
    background-color var(--dur-fast) var(--ease),
    color var(--dur-fast),
    transform var(--dur-fast);
  user-select: none;
}

.btn:disabled {
  opacity: 0.42;
}

.btn:active:not(:disabled) {
  transform: scale(0.97);
}

.btn-primary {
  background: var(--accent);
  color: var(--accent-ink);
}

.btn-primary:hover:not(:disabled) {
  background: var(--accent-hover);
}

.btn-secondary {
  background: var(--surface-hover);
  color: var(--text);
}

.btn-secondary:hover:not(:disabled) {
  background: var(--surface-active);
}

.btn-ghost {
  color: var(--text-2);
}

.btn-ghost:hover:not(:disabled) {
  background: var(--hover);
  color: var(--text);
}

.btn-ghost.is-on {
  color: var(--accent);
}

.btn-danger {
  background: var(--danger-bg);
  color: var(--on-danger);
}

.btn-danger:hover:not(:disabled) {
  background: var(--danger-bg-hover);
}

.btn-sm {
  height: 28px;
  padding: 0 10px;
  font-size: var(--fs-small);
}

.icon-btn {
  position: relative;
  display: inline-grid;
  place-items: center;
  flex-shrink: 0;
  width: var(--control-h);
  height: var(--control-h);
  border-radius: 50%;
  color: var(--text-2);
  transition:
    background-color var(--dur-fast) var(--ease),
    color var(--dur-fast);
}

.icon-btn:hover:not(:disabled) {
  background: var(--surface-hover);
  color: var(--text);
}

.icon-btn:disabled {
  opacity: 0.35;
}

.icon-btn.active {
  color: var(--accent);
}

.icon-btn.active::after {
  content: '';
  position: absolute;
  left: 50%;
  bottom: 0;
  width: 4px;
  height: 4px;
  border-radius: 50%;
  background: currentColor;
  transform: translateX(-50%);
}

.icon-btn-sm {
  width: 26px;
  height: 26px;
}

.icon-btn-lg {
  width: 36px;
  height: 36px;
}

.link,
.link-btn {
  color: var(--text-2);
  border-radius: var(--radius-xs);
}

.link:hover,
.link-btn:hover {
  color: var(--text);
  text-decoration: underline;
}

.link-btn {
  display: inline-flex;
  align-items: center;
  gap: 2px;
  font-size: var(--fs-small);
  font-weight: var(--fw-semibold);
}

.chip {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  height: 28px;
  padding: 0 12px;
  border-radius: 999px;
  background: var(--surface);
  color: var(--text-2);
  font-size: var(--fs-small);
  font-weight: var(--fw-semibold);
  transition: background-color var(--dur-fast);
}

.chip:hover {
  background: var(--surface-hover);
  color: var(--text);
}

.chip.active {
  background: var(--accent);
  color: var(--accent-ink);
}

.input {
  width: 100%;
  height: 36px;
  padding: 0 12px;
  border-radius: var(--radius);
  border: 1px solid var(--line-strong);
  background: var(--bg);
  color: var(--text);
  font-size: var(--fs-body);
  outline: none;
  transition: border-color var(--dur-fast);
}

.input:hover {
  border-color: rgb(var(--ink) / 0.24);
}

.input:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 1px var(--accent);
}

.input[aria-invalid='true'] {
  border-color: var(--danger);
  box-shadow: none;
}

.input[readonly] {
  color: var(--text-2);
}

.input-sm {
  width: 84px;
  height: 28px;
  padding: 0 8px;
  font-size: var(--fs-small);
}

.field-label {
  display: block;
  margin-bottom: 6px;
  color: var(--text-2);
  font-size: var(--fs-small);
  font-weight: var(--fw-semibold);
}

.field-error {
  min-height: 18px;
  margin-top: 6px;
  color: var(--danger);
  font-size: var(--fs-small);
}

/* Range slider shared by progress, volume and lyric size. --fill is set inline. */
.slider {
  -webkit-appearance: none;
  appearance: none;
  width: 100%;
  height: 16px;
  margin: 0;
  background: transparent;
  cursor: pointer;
  --fill: 0%;
}

.slider::-webkit-slider-runnable-track {
  height: 4px;
  border-radius: 2px;
  background: linear-gradient(
    to right,
    var(--accent) var(--fill),
    rgb(var(--ink) / 0.18) var(--fill)
  );
}

.slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 12px;
  height: 12px;
  margin-top: -4px;
  border-radius: 50%;
  background: var(--text);
  box-shadow: var(--shadow-thumb);
  opacity: 0;
  transition:
    opacity var(--dur-fast),
    transform var(--dur-fast) var(--ease);
}

.slider:hover::-webkit-slider-thumb,
.slider:focus-visible::-webkit-slider-thumb,
.slider:active::-webkit-slider-thumb {
  opacity: 1;
}

.slider:active::-webkit-slider-thumb {
  transform: scale(1.25);
}

.slider:hover::-webkit-slider-runnable-track {
  background: linear-gradient(
    to right,
    var(--accent-hover) var(--fill),
    rgb(var(--ink) / 0.24) var(--fill)
  );
}

.slider:focus-visible {
  outline: none;
}

.slider:focus-visible::-webkit-slider-runnable-track {
  outline: 2px solid var(--focus);
  outline-offset: 4px;
}

.slider:disabled {
  cursor: default;
  opacity: 0.45;
}

.slider:disabled::-webkit-slider-thumb {
  opacity: 0;
}

.slider-sm {
  width: 80px;
}

.switch {
  position: relative;
  display: inline-flex;
  flex-shrink: 0;
  width: 40px;
  height: 22px;
  cursor: pointer;
}

.switch input {
  position: absolute;
  inset: 0;
  opacity: 0;
  margin: 0;
  cursor: pointer;
}

.switch span[aria-hidden] {
  position: absolute;
  inset: 0;
  border-radius: 999px;
  background: var(--surface-active);
  transition: background-color var(--dur);
}

.switch span[aria-hidden]::after {
  content: '';
  position: absolute;
  left: 3px;
  top: 3px;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: var(--text-2);
  transition:
    transform var(--dur) var(--ease),
    background-color var(--dur);
}

.switch input:checked + span {
  background: var(--accent);
}

.switch input:checked + span::after {
  transform: translateX(18px);
  background: var(--accent-ink);
}

.switch input:focus-visible + span {
  outline: 2px solid var(--focus);
  outline-offset: 2px;
}

/* Select (Select.svelte): trigger + popover listbox. */
.select-trigger {
  display: inline-flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  height: var(--control-h);
  max-width: 100%;
  min-width: 120px;
  padding: 0 10px 0 12px;
  border-radius: var(--radius);
  background: var(--surface);
  color: var(--text);
  font-size: var(--fs-meta);
  transition: background-color var(--dur-fast);
}

.select-trigger:hover:not(:disabled) {
  background: var(--surface-hover);
}

.select-trigger[aria-expanded='true'] {
  background: var(--surface-hover);
}

.select-trigger:disabled {
  opacity: 0.45;
}

.select-trigger > span {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.select-trigger > svg {
  flex-shrink: 0;
  color: var(--text-3);
}

/* ============================================================ popovers: menu, select, tooltip */
.menu,
.select-panel {
  position: fixed;
  inset: auto;
  margin: 0;
  min-width: 200px;
  max-width: min(320px, calc(100vw - 16px));
  padding: 4px;
  border: 0;
  border-radius: var(--radius);
  background: var(--surface-overlay);
  color: var(--text);
  box-shadow: var(--shadow-pop);
  overflow-y: auto;
  overscroll-behavior: contain;
}

.menu[data-state='open'],
.select-panel[data-state='open'] {
  display: flex;
  flex-direction: column;
  animation: pop-in var(--dur-fast) var(--ease);
}

.menu-item,
.select-panel button {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  min-height: 34px;
  padding: 6px 10px;
  border-radius: var(--radius-xs);
  font-size: var(--fs-meta);
  text-align: left;
  color: var(--text);
  outline: none;
  flex-shrink: 0;
}

.menu-item:hover:not(:disabled),
.menu-item:focus-visible,
.select-panel button:hover,
.select-panel button:focus-visible {
  background: var(--surface-hover);
}

.menu-item:focus-visible,
.select-panel button:focus-visible {
  box-shadow: inset 0 0 0 1px var(--accent-line);
}

.menu {
  position: relative;
  z-index: var(--z-toast);
  max-height: var(--bits-dropdown-menu-content-available-height);
}

/* Bits UI positions the wrapper and exposes the anchor/available size. */
.select-panel {
  position: relative;
  z-index: var(--z-toast);
  min-width: max(160px, var(--bits-select-anchor-width));
  max-height: min(320px, var(--bits-select-content-available-height));
  overflow: hidden;
}

.select-viewport {
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow-y: auto;
  overscroll-behavior: contain;
}

.select-panel button[data-highlighted] {
  background: var(--surface-hover);
  box-shadow: inset 0 0 0 1px var(--accent-line);
}

.menu-item:disabled {
  color: var(--text-3);
}

.menu-item.danger {
  color: var(--danger);
}

.menu-item.danger:hover:not(:disabled),
.menu-item.danger:focus-visible {
  background: var(--danger-soft);
}

.menu-icon {
  display: grid;
  place-items: center;
  width: 16px;
  color: var(--text-2);
  flex-shrink: 0;
}

.menu-item.danger .menu-icon {
  color: inherit;
}

.menu-label {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.menu-item > svg:last-child {
  color: var(--accent);
  flex-shrink: 0;
}

.menu-separator {
  height: 1px;
  margin: 4px 6px;
  background: var(--line-strong);
  flex-shrink: 0;
}

.menu-heading {
  padding: 8px 10px 4px;
  color: var(--text-3);
  font-size: 11px;
  font-weight: var(--fw-semibold);
  flex-shrink: 0;
}

.select-panel button {
  justify-content: space-between;
}

.select-panel button[aria-selected='true'] {
  color: var(--accent);
}

.tooltip {
  position: fixed;
  inset: auto;
  margin: 0;
  padding: 5px 9px;
  border: 0;
  border-radius: var(--radius-sm);
  background: var(--tooltip-bg);
  color: var(--tooltip-text);
  font-size: var(--fs-small);
  line-height: 1.35;
  max-width: 320px;
  overflow-wrap: anywhere;
  box-shadow: var(--shadow-tooltip);
  pointer-events: none;
  opacity: 0;
  transition: opacity var(--dur-fast);
}

.tooltip[data-visible='true'] {
  opacity: 1;
}

@keyframes pop-in {
  from {
    opacity: 0;
    transform: translateY(-3px) scale(0.98);
  }

  to {
    opacity: 1;
    transform: none;
  }
}

/* ============================================================ dialog & toast */
/* Tailwind preflight zeroes every margin; a modal <dialog> needs margin:auto to centre in the top layer. */
.dialog {
  position: fixed;
  inset: 0;
  margin: auto;
  width: min(440px, calc(100vw - 32px));
  max-width: none;
  max-height: calc(100dvh - 48px);
  padding: 0;
  border: 0;
  border-radius: var(--radius-lg);
  background: var(--surface-overlay);
  color: var(--text);
  box-shadow: var(--shadow-dialog);
  overflow: hidden;
}

.dialog[open] {
  display: flex;
  flex-direction: column;
  animation: dialog-in var(--dur) var(--ease);
}

.dialog > form {
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.dialog::backdrop {
  background: var(--scrim);
}

.dialog-wide {
  width: min(600px, calc(100vw - 32px));
}

.dialog-wide > .dialog-body {
  overscroll-behavior: contain;
}

.dialog-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 20px 20px 8px;
}

.dialog-header h2 {
  flex: 1;
  font-size: var(--fs-section);
  font-weight: var(--fw-semibold);
  line-height: 1.35;
  overflow-wrap: anywhere;
}

.dialog-heading-sm {
  font-size: 15px !important;
}

.dialog-badge {
  display: grid;
  place-items: center;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  flex-shrink: 0;
}

.dialog-badge.danger {
  background: var(--danger-soft);
  color: var(--danger);
}

.dialog-body {
  padding: 8px 20px 16px;
  min-height: 0;
  overflow-y: auto;
  color: var(--text-2);
  font-size: var(--fs-body);
  line-height: 1.6;
}

.dialog-detail {
  margin-top: 8px;
  color: var(--text-3);
  font-size: var(--fs-meta);
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 12px 20px 20px;
}

@keyframes dialog-in {
  from {
    opacity: 0;
    transform: translateY(6px) scale(0.98);
  }

  to {
    opacity: 1;
    transform: none;
  }
}

.toast {
  position: fixed;
  left: 50%;
  bottom: calc(var(--player-h) + 16px);
  z-index: var(--z-toast);
  display: flex;
  align-items: center;
  gap: 12px;
  max-width: min(560px, calc(100vw - 32px));
  padding: 8px 8px 8px 16px;
  border-radius: var(--radius);
  background: var(--toast-bg);
  color: var(--text);
  font-size: var(--fs-meta);
  box-shadow: var(--shadow-pop);
  transform: translateX(-50%);
}

.toast.error {
  background: var(--toast-error-bg);
  color: var(--toast-error-text);
}

.toast > span {
  min-width: 0;
  overflow-wrap: anywhere;
}

/* ============================================================ artwork */
.artwork {
  position: relative;
  width: 100%;
  height: 100%;
  overflow: hidden;
  border-radius: inherit;
  background: var(--surface-hover);
}

.artwork img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  opacity: 0;
  transition: opacity var(--dur) var(--ease);
}

.artwork img.loaded {
  opacity: 1;
}

.artwork-placeholder {
  display: grid;
  place-items: center;
  width: 100%;
  height: 100%;
  background: linear-gradient(145deg, var(--art-from), var(--art-to));
  color: var(--art-icon);
}

.artwork-placeholder svg {
  width: 42%;
  height: 42%;
  max-width: 72px;
  max-height: 72px;
  stroke-width: 1.5;
}

.artwork-artist .artwork-placeholder {
  background: linear-gradient(145deg, var(--art-from), var(--art-to));
}

.cover-fill {
  position: absolute;
  inset: 0;
  display: block;
  border-radius: inherit;
}

.skeleton {
  background: linear-gradient(
    90deg,
    var(--surface) 0%,
    var(--surface-hover) 50%,
    var(--surface) 100%
  );
  background-size: 200% 100%;
  animation: shimmer 1.4s linear infinite;
  border-radius: var(--radius-xs);
}

.skeleton-text {
  display: block;
  height: 10px;
  width: 60%;
}

.skeleton-text.short {
  width: 40%;
}

@keyframes shimmer {
  from {
    background-position: 100% 0;
  }

  to {
    background-position: -100% 0;
  }
}

/* ============================================================ page headers */
.page-header {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  gap: var(--space-4);
  flex-wrap: wrap;
  padding: var(--space-4) 0 var(--space-5);
}

.page-title {
  font-family: var(--font-display);
  font-size: var(--fs-page);
  font-weight: var(--fw-semibold);
  line-height: 1.2;
  letter-spacing: -0.2px;
}

.page-meta {
  margin-top: 6px;
  color: var(--text-3);
  font-size: var(--fs-meta);
}

.page-actions {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  flex-wrap: wrap;
}

.toolbar {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-bottom: var(--space-3);
}

.toolbar-spacer {
  flex: 1;
}

.toolbar-label {
  color: var(--text-3);
  font-size: var(--fs-small);
}

.scan-banner {
  display: flex;
  align-items: center;
  gap: 10px;
  margin: var(--space-2) 0;
  padding: 6px 6px 6px 12px;
  border-radius: var(--radius);
  background: var(--accent-soft);
  color: var(--text);
  font-size: var(--fs-meta);
}

.scan-banner .btn {
  margin-left: auto;
}

.spinner {
  width: 14px;
  height: 14px;
  border-radius: 50%;
  border: 2px solid var(--accent-line);
  border-top-color: var(--accent);
  animation: spin 0.8s linear infinite;
  flex-shrink: 0;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.hint {
  color: var(--text-3);
  font-size: var(--fs-meta);
}

.detail-hero {
  display: flex;
  align-items: flex-end;
  gap: var(--space-6);
  padding: var(--space-4) 0 var(--space-6);
}

.hero-art {
  position: relative;
  flex-shrink: 0;
  width: clamp(140px, 16vw, 200px);
  aspect-ratio: 1;
  border-radius: var(--radius-lg);
  overflow: hidden;
  box-shadow: var(--shadow-card);
}

.hero-art.round {
  border-radius: 50%;
}

.hero-info {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.hero-kind {
  color: var(--text-2);
  font-size: var(--fs-small);
  font-weight: var(--fw-semibold);
}

.hero-title {
  font-family: var(--font-display);
  font-size: clamp(24px, 3vw, 36px);
  font-weight: 700;
  line-height: 1.15;
  letter-spacing: -0.4px;
  overflow-wrap: anywhere;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.hero-meta {
  color: var(--text-2);
  font-size: var(--fs-meta);
}

.hero-actions {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-top: var(--space-3);
  flex-wrap: wrap;
}

/* Animated “now playing” bars; static while paused. */
.eq {
  display: inline-flex;
  align-items: flex-end;
  gap: 2px;
  height: 14px;
}

.eq i {
  width: 3px;
  height: 100%;
  border-radius: 1px;
  background: var(--accent);
  transform-origin: bottom;
  animation: eq 1s ease-in-out infinite;
}

.eq i:nth-child(2) {
  animation-delay: -0.4s;
}

.eq i:nth-child(3) {
  animation-delay: -0.7s;
}

.eq.paused i {
  animation: none;
  transform: scaleY(0.35);
}

.eq.paused i:nth-child(2) {
  transform: scaleY(0.6);
}

@keyframes eq {
  0%,
  100% {
    transform: scaleY(0.3);
  }

  50% {
    transform: scaleY(1);
  }
}

/* ============================================================ cards (albums, artists, home shelves) */
.card-grid,
.shelf {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(156px, 1fr));
  gap: var(--space-5) var(--space-4);
}

.shelf {
  grid-template-columns: repeat(auto-fill, minmax(144px, 1fr));
  grid-auto-rows: auto;
  max-height: calc((144px + 64px) * 2 + var(--space-5));
  overflow: hidden;
}

.card {
  position: relative;
  min-width: 0;
  padding: var(--space-2);
  margin: calc(var(--space-2) * -1);
  border-radius: var(--radius-lg);
  transition: background-color var(--dur) var(--ease);
}

.card:hover {
  background: var(--hover);
}

.card-art {
  position: relative;
  aspect-ratio: 1;
  border-radius: var(--radius);
  overflow: hidden;
  background: var(--surface-hover);
  box-shadow: var(--shadow-card);
}

.card-art.round {
  border-radius: 50%;
}

.card-open {
  position: absolute;
  inset: 0;
  z-index: 1;
  border-radius: inherit;
}

.card-play {
  position: absolute;
  right: 8px;
  bottom: 8px;
  z-index: 2;
  display: grid;
  place-items: center;
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: var(--accent);
  color: var(--accent-ink);
  box-shadow: var(--shadow-control);
  opacity: 0;
  transform: translateY(6px);
  transition:
    opacity var(--dur) var(--ease),
    transform var(--dur) var(--ease);
  pointer-events: none;
}

button.card-play {
  pointer-events: auto;
}

.card:hover .card-play,
.card:focus-within .card-play {
  opacity: 1;
  transform: none;
}

.card-play:hover {
  background: var(--accent-hover);
}

.card-play svg {
  margin-left: 2px;
}

.card-title {
  display: block;
  width: 100%;
  margin-top: 10px;
  color: var(--text);
  font-size: var(--fs-body);
  font-weight: var(--fw-semibold);
  text-align: left;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.card-sub {
  margin-top: 2px;
  color: var(--text-3);
  font-size: var(--fs-small);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.card-grid.artists .card-title,
.card-grid.artists .card-sub {
  text-align: center;
}

.artist-initial {
  display: grid;
  place-items: center;
  width: 100%;
  height: 100%;
  background: linear-gradient(150deg, hsl(var(--hue) 18% 30%), hsl(calc(var(--hue) + 40) 14% 18%));
  color: hsl(var(--hue) 30% 82%);
  font-family: var(--font-display);
  font-size: clamp(36px, 4vw, 52px);
  font-weight: 600;
}

.artist-initial.small {
  width: 44px;
  height: 44px;
  border-radius: 50%;
  font-size: 18px;
  flex-shrink: 0;
}

.grid-sentinel {
  height: 1px;
}

/* ============================================================ responsive */
@media (max-width: 1279px) {
  :root {
    --queue-w: 320px;
  }

  .player-bar {
    grid-template-columns: minmax(160px, 1fr) minmax(300px, 1.8fr) minmax(150px, 1fr);
  }
}

/* Narrow windows only cap the drawer width. Scrim colour, fade and hit-testing live on .queue-scrim above;
   repeating them here would drift from the base rule. */
@media (max-width: 1199px) {
  .app.immersive-open.queue-open .immersive-layer {
    right: 0;
  }

  .queue {
    width: min(var(--queue-w), calc(100vw - var(--sidebar-w) - 24px));
  }
}

@media (max-width: 1099px) {
  :root {
    --sidebar-w: 196px;
  }

  .local-label {
    display: none;
  }

  .pb-volume .slider {
    width: 72px;
  }
}

@media (max-width: 999px) {
  :root {
    --sidebar-w: 184px;
  }

  .search kbd {
    display: none;
  }

  .player-bar {
    grid-template-columns: minmax(120px, 1fr) minmax(280px, 1.6fr) auto;
    gap: var(--space-3);
    padding: 0 var(--space-3);
  }

  .pb-cover {
    width: 48px;
    height: 48px;
  }

  .pb-volume .slider,
  .pb-volume-value {
    display: none;
  }

  .crumb-parent,
  .crumb-sep {
    display: none;
  }

  .detail-hero {
    align-items: center;
  }
}

@media (max-height: 700px) {
  :root {
    --player-h: 80px;
  }

  .large-artwork {
    width: min(100%, 34vh, 360px);
  }

  .detail-hero {
    padding-bottom: var(--space-4);
  }

  .hero-art {
    width: clamp(120px, 14vw, 160px);
  }
}

/* Same width as the player-bar collapse above, so Now Playing and the bar change together. */
@media (max-width: 999px) {
  .immersive-content:not(.cover-only):not(.lyrics-only) {
    grid-template-columns: minmax(0, 1fr);
    grid-template-rows: auto minmax(0, 1fr);
    gap: var(--space-4);
  }

  .immersive-content:not(.cover-only):not(.lyrics-only) .artwork-panel {
    flex-direction: row;
    text-align: left;
    gap: var(--space-4);
  }

  .immersive-content:not(.cover-only):not(.lyrics-only) .large-artwork {
    width: clamp(72px, 14vh, 112px);
    flex-shrink: 0;
  }

  .immersive-content:not(.cover-only):not(.lyrics-only) .artwork-panel h1 {
    margin-top: 0;
  }

  .lyric-lines {
    padding-top: 24%;
    padding-bottom: 45%;
  }
}

/* ============================================================ motion preferences */
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
    /* Initial transition-property is all. A duration on * would transition every element. */
    transition-property: none !important;
    scroll-behavior: auto !important;
  }

  .eq i {
    transform: scaleY(0.6);
  }

  .queue {
    --drawer-shift: 0px;
  }
}

.reduce-motion *,
.reduce-motion *::before,
.reduce-motion *::after {
  animation-duration: 0.01ms !important;
  animation-iteration-count: 1 !important;
  transition-duration: 0.01ms !important;
  /* Same as the media query above: duration on * without this starts a transition per element. */
  transition-property: none !important;
  scroll-behavior: auto !important;
}

.reduce-motion .eq i {
  transform: scaleY(0.6);
}

.reduce-motion .queue {
  --drawer-shift: 0px;
}
````

## File: src/App.svelte
````svelte
<script lang="ts">
  import { onMount, tick, setContext } from 'svelte';
  import { fly } from 'svelte/transition';
  import { X } from 'lucide-svelte';
  import { shortcut } from './lib/utils/shortcuts';
  import { diagnostics } from './lib/utils/cover-diagnostics';
  import {
    desktopAvailable,
    verifyConnection,
    getPlayerState,
    getArtwork,
    desktopPreferences,
    openFile,
    play,
    pause,
    queueAction,
    cancelScan,
  } from './lib/services/music';
  import { player, accept, control } from './lib/stores/player.svelte';
  import { library, refreshSummary, watchScans } from './lib/stores/library.svelte';
  import { nav, routeKey, memory } from './lib/stores/nav.svelte';
  import { ui, dismissToast, notifyError } from './lib/stores/ui.svelte';
  import { loadPreferences, watchSystemTheme, motionMs } from './lib/stores/preferences';
  import { installNativeMenuGuard } from './lib/utils/native-menu';
  import { escapeLayer } from './lib/utils/layers';
  import Sidebar from './lib/layout/Sidebar.svelte';
  import TopBar from './lib/layout/TopBar.svelte';
  import PlayerBar from './lib/layout/PlayerBar.svelte';
  import Queue from './lib/layout/Queue.svelte';
  import NowPlaying from './lib/views/NowPlaying.svelte';
  import TrackDetails from './lib/components/TrackDetails.svelte';
  import ContextMenu from './lib/components/ContextMenu.svelte';
  import Dialog from './lib/components/Dialog.svelte';
  import HomeView from './lib/views/HomeView.svelte';
  import SongsView from './lib/views/SongsView.svelte';
  import GridView from './lib/views/GridView.svelte';
  import SettingsView from './lib/views/SettingsView.svelte';

  let snapshot = $derived(player.snapshot);
  let searchInput = $state<HTMLInputElement>();
  let queueTrigger = $state<HTMLButtonElement>();
  let main = $state<HTMLElement>();
  setContext('scroller', () => main);

  // Cover for the current song: one request per track change, stale results ignored.
  let artwork = $state<string | null>(null);
  let artworkRequest = 0;
  let artworkTrackId = $derived(snapshot?.track_id);
  $effect(() => {
    const path = artworkTrackId;
    if (diagnostics) {
      diagnostics.trackChanges++;
      if (path && desktopAvailable) diagnostics.coverRequests++;
    }
    const ticket = ++artworkRequest;
    artwork = null;
    if (path && desktopAvailable)
      void getArtwork()
        .then((a) => {
          if (ticket === artworkRequest) artwork = a;
        })
        .catch(() => {});
  });
  async function connect() {
    try {
      const r = await verifyConnection();
      accept(r.snapshot);
      player.connection = 'Rust 播放引擎已连接';
      player.connected = true;
    } catch (e) {
      player.connection = desktopAvailable ? '连接失败，请重试' : '浏览器预览';
      notifyError(e);
    }
  }

  // Each route keeps its own scroll position for the session.
  let lastKey = routeKey(nav.route);
  $effect.pre(() => {
    const key = routeKey(nav.route);
    if (key === lastKey) return;
    if (main)
      memory.set(lastKey, {
        ...memory.get(lastKey),
        scroll: main.scrollTop,
      });
    lastKey = key;
    void tick().then(() => {
      if (main) main.scrollTop = memory.get(key)?.scroll ?? 0;
    });
  });

  // Mounted once (while idle after startup, or on first open) and kept: closing only animates it away,
  // so scroll position survives and the first open is not the frame that builds every row.
  let queueMounted = $state(false);
  $effect.pre(() => {
    if (ui.queueOpen) queueMounted = true;
  });

  async function toggleQueue(value = !ui.queueOpen) {
    ui.queueOpen = value;
    if (desktopAvailable) desktopPreferences(null, value).catch(notifyError);
    if (!value) queueTrigger?.focus();
  }

  function keydown(e: KeyboardEvent) {
    const target = e.composedPath()[0];
    const el = target instanceof HTMLElement ? target : null;
    const modal = !!document.querySelector('dialog[open]') || !!ui.menu;
    const action = shortcut(
      e,
      !!el?.closest(
        'input,textarea,select,[contenteditable]:not([contenteditable="false"]),[role="textbox"]',
      ),
      !!el?.closest('button,a,[role="button"],summary'),
      modal,
    );
    if (!action) return;
    if (action === 'escape') {
      if (modal) return;
      const layer = escapeLayer({
        queue: ui.queueOpen,
        immersive: ui.immersive,
        toast: !!ui.toast,
      });
      if (layer === 'queue') void toggleQueue(false);
      else if (layer === 'immersive') ui.immersive = false;
      else if (layer === 'toast') dismissToast();
      else return;
    } else if (action === 'search') {
      ui.immersive = false;
      void tick().then(() => {
        searchInput?.focus();
        searchInput?.select();
      });
    } else if (action === 'lyrics') {
      if (snapshot?.current_track || ui.immersive) ui.immersive = !ui.immersive;
    } else if (action === 'open') {
      if (desktopAvailable) void control(openFile);
    } else if (action === 'toggle') {
      if (snapshot?.current_track)
        void control(snapshot.playback_state === 'playing' ? pause : play);
    } else if (action === 'previous' || action === 'next') {
      if (snapshot?.queue.length) void control(() => queueAction(action));
    }
    e.preventDefault();
  }

  onMount(() => {
    loadPreferences();
    const stopTheme = watchSystemTheme();
    const removeGuard = installNativeMenuGuard(import.meta.env.DEV);
    if (!desktopAvailable) {
      player.connection = '浏览器预览';
      return () => {
        stopTheme();
        removeGuard();
      };
    }
    void desktopPreferences()
      .then((p) => {
        ui.queueOpen = p.queue_open;
      })
      .catch(notifyError);
    void connect();
    void refreshSummary();
    const stopScans = watchScans();
    let disposed = false;
    let timer: ReturnType<typeof setTimeout>;
    async function refresh() {
      try {
        const next = await getPlayerState();
        if (!disposed) {
          accept(next);
          player.connected = true;
        }
      } catch (e) {
        if (!disposed) {
          player.connected = false;
          player.connection = '连接失败';
          notifyError(e);
        }
      }
      if (!disposed) timer = setTimeout(refresh, 200);
    }
    void refresh();
    const idle = requestIdleCallback(() => (queueMounted = true), {
      timeout: 4000,
    });
    return () => {
      disposed = true;
      cancelIdleCallback(idle);
      clearTimeout(timer);
      stopScans();
      stopTheme();
      removeGuard();
    };
  });

  let route = $derived(nav.route);
</script>

<div
  class="app"
  class:reduce-motion={ui.reduceMotion}
  class:queue-open={ui.queueOpen}
  class:immersive-open={ui.immersive}
>
  <Sidebar />
  <div class="workspace">
    <TopBar bind:searchInput />
    <div class="content">
      <main class="main" bind:this={main} id="main-content">
        {#if library.progress?.running}
          <div class="scan-banner" role="status">
            <span class="spinner" aria-hidden="true"></span>正在扫描音乐文件夹 · 已检查 {library
              .progress.visited} · 已导入 {library.progress.imported}<button
              type="button"
              class="btn btn-ghost btn-sm"
              onclick={() => cancelScan().catch(notifyError)}>取消</button
            >
          </div>
        {/if}
        {#if route.view === 'home'}<HomeView {artwork} />
        {:else if route.view === 'albums' || route.view === 'artists'}{#key route.view}<GridView
              kind={route.view}
            />{/key}
        {:else if route.view === 'settings'}<SettingsView />
        {:else}<SongsView />{/if}
      </main>
    </div>
  </div>
  <PlayerBar {artwork} ontogglequeue={() => toggleQueue()} bind:queueTrigger />
  {#if ui.immersive}
    <div class="immersive-layer" transition:fly={{ y: 24, duration: motionMs(220) }}>
      <NowPlaying {artwork} onclose={() => (ui.immersive = false)} />
    </div>
  {/if}
  <!-- App-level so it opens over every page, including Now Playing. -->
  {#if queueMounted}
    <button
      type="button"
      class="queue-scrim"
      aria-label="关闭播放队列"
      tabindex="-1"
      inert={!ui.queueOpen}
      onclick={() => toggleQueue(false)}
    ></button>
    <Queue open={ui.queueOpen} onclose={() => toggleQueue(false)} />
  {/if}

  <ContextMenu />
  <Dialog />
  {#if ui.details}{#key ui.details.id}<TrackDetails
        track={ui.details}
        onclose={() => (ui.details = null)}
      />{/key}{/if}
  {#if ui.toast}
    {#key ui.toast.id}
      <div
        class="toast"
        class:error={ui.toast.tone === 'error'}
        role={ui.toast.tone === 'error' ? 'alert' : 'status'}
        transition:fly={{ y: 12, duration: motionMs(160) }}
      >
        <span>{ui.toast.message}</span><button
          type="button"
          class="icon-btn icon-btn-sm"
          aria-label="关闭提示"
          onclick={dismissToast}><X size={15} /></button
        >
      </div>
    {/key}
  {/if}
</div>
<svelte:window onkeydown={keydown} />
````

## File: src/main.ts
````typescript
import { mount } from 'svelte';
import App from './App.svelte';
import './app.css';
mount(App, { target: document.getElementById('app')! });
````

## File: src-tauri/capabilities/default.json
````json
{"identifier":"main-window","description":"Only the main application window, with no filesystem or shell access.","windows":["main"],"permissions":["core:default","core:window:allow-set-theme"]}
````

## File: src-tauri/src/artwork/covers.rs
````rust
//! Bounded cover thumbnails. Decoding happens on the caller's blocking thread,
//! never on the audio callback. Disk files live under the app cache directory.
use std::collections::{HashMap, VecDeque};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Instant, UNIX_EPOCH};

use base64::Engine;

const MAX_SOURCE_BYTES: usize = 16 * 1024 * 1024;
const MAX_EDGE: u32 = 960;
const MIN_EDGE: u32 = 32;
const MAX_PIXELS: u32 = 8_192;
/// A 4000 x 4000 RGBA picture needs 64 MB; two decode slots bound the peak.
const MAX_DECODE_ALLOC: u64 = 80 * 1024 * 1024;
/// WebP and GIF are not decoded here; small ones go to the WebView unchanged.
const MAX_PASSTHROUGH_BYTES: usize = 512 * 1024;
const MEMORY_ENTRIES: usize = 240;
const MEMORY_BYTES: usize = 24 * 1024 * 1024;
const DISK_BYTES: u64 = 64 * 1024 * 1024;
const DECODE_SLOTS: u32 = 2;
/// Disk entries written by an older policy (which stored read failures as "no cover") are dropped.
const CACHE_VERSION: &str = "v2";

#[derive(Default)]
pub struct Stats {
    pub requests: AtomicU64,
    pub decoded: AtomicU64,
    pub passthrough: AtomicU64,
    pub memory_hits: AtomicU64,
    pub disk_hits: AtomicU64,
    /// Definitive "no usable cover" results, split by reason below.
    pub missing: AtomicU64,
    pub no_picture: AtomicU64,
    pub unsupported: AtomicU64,
    pub too_large: AtomicU64,
    pub corrupt: AtomicU64,
    /// File missing or unreadable right now. Never cached.
    pub transient: AtomicU64,
    pub coalesced: AtomicU64,
    pub bytes_out: AtomicU64,
    pub decode_us: AtomicU64,
    pub decode_max_us: AtomicU64,
    pub slot_wait_us: AtomicU64,
}

impl Stats {
    pub fn snapshot(&self) -> serde_json::Value {
        let n = |v: &AtomicU64| v.load(Ordering::Relaxed);
        serde_json::json!({
            "requests": n(&self.requests),
            "decoded": n(&self.decoded),
            "passthrough": n(&self.passthrough),
            "memoryHits": n(&self.memory_hits),
            "diskHits": n(&self.disk_hits),
            "coalesced": n(&self.coalesced),
            "noPicture": n(&self.no_picture),
            "unsupported": n(&self.unsupported),
            "tooLarge": n(&self.too_large),
            "corrupt": n(&self.corrupt),
            "transient": n(&self.transient),
            "bytesOut": n(&self.bytes_out),
            "decodeMs": n(&self.decode_us) / 1000,
            "decodeMaxMs": n(&self.decode_max_us) / 1000,
            "slotWaitMs": n(&self.slot_wait_us) / 1000,
        })
    }
}

/// The cover cannot be produced now but may be later. Returned to the page as a code, never cached.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoverError {
    FileMissing,
    Unreadable,
}

impl CoverError {
    pub fn code(self) -> &'static str {
        match self {
            CoverError::FileMissing => "cover-file-missing",
            CoverError::Unreadable => "cover-unreadable",
        }
    }
}

/// Definitive reasons a file has no usable cover. These are cached until the file changes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Absent {
    NoPicture,
    Unsupported,
    TooLarge,
    Corrupt,
}

struct Image {
    mime: &'static str,
    bytes: Vec<u8>,
}

type Cached = Option<Arc<Image>>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Key {
    /// 1 = library row id, 2 = playback path that is not a library row.
    kind: u8,
    id: u64,
    mtime_ns: u128,
    len: u64,
    edge: u32,
}

struct Slots {
    free: Mutex<u32>,
    cv: Condvar,
}

struct SlotGuard<'a> {
    slots: &'a Slots,
}

impl Drop for SlotGuard<'_> {
    fn drop(&mut self) {
        let mut free = self.slots.free.lock().unwrap_or_else(|e| e.into_inner());
        *free += 1;
        self.slots.cv.notify_one();
    }
}

impl Slots {
    fn new(n: u32) -> Self {
        Self {
            free: Mutex::new(n),
            cv: Condvar::new(),
        }
    }
    fn enter(&self) -> SlotGuard<'_> {
        let mut free = self.free.lock().unwrap_or_else(|e| e.into_inner());
        while *free == 0 {
            free = self.cv.wait(free).unwrap_or_else(|e| e.into_inner());
        }
        *free -= 1;
        SlotGuard { slots: self }
    }
}

enum FlightState {
    Pending,
    Ready(Result<Cached, CoverError>),
}

struct Flight {
    state: Mutex<FlightState>,
    cv: Condvar,
}

impl Flight {
    fn pending() -> Self {
        Self {
            state: Mutex::new(FlightState::Pending),
            cv: Condvar::new(),
        }
    }
    fn wait(&self) -> Result<Cached, CoverError> {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            match &*state {
                FlightState::Ready(value) => return value.clone(),
                FlightState::Pending => {
                    state = self.cv.wait(state).unwrap_or_else(|e| e.into_inner());
                }
            }
        }
    }
    fn finish(&self, value: Result<Cached, CoverError>) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        *state = FlightState::Ready(value);
        self.cv.notify_all();
    }
}

struct Memory {
    map: HashMap<Key, Cached>,
    order: VecDeque<Key>,
    bytes: usize,
}

impl Memory {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            order: VecDeque::new(),
            bytes: 0,
        }
    }
    fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
        self.bytes = 0;
    }
    fn get(&mut self, key: &Key) -> Option<Cached> {
        if !self.map.contains_key(key) {
            return None;
        }
        self.order.retain(|item| item != key);
        self.order.push_back(key.clone());
        Some(self.map.get(key).cloned().unwrap_or(None))
    }
    fn put(&mut self, key: Key, value: Cached) {
        if let Some(old) = self.map.remove(&key) {
            self.bytes = self
                .bytes
                .saturating_sub(old.as_ref().map(|b| b.bytes.len()).unwrap_or(0));
            self.order.retain(|item| item != &key);
        }
        self.bytes += value.as_ref().map(|b| b.bytes.len()).unwrap_or(0);
        self.map.insert(key.clone(), value);
        self.order.push_back(key);
        while self.map.len() > MEMORY_ENTRIES || self.bytes > MEMORY_BYTES {
            let Some(old) = self.order.pop_front() else {
                break;
            };
            if let Some(value) = self.map.remove(&old) {
                self.bytes = self
                    .bytes
                    .saturating_sub(value.as_ref().map(|b| b.bytes.len()).unwrap_or(0));
            }
        }
    }
}

pub struct Store {
    dir: PathBuf,
    memory: Mutex<Memory>,
    inflight: Mutex<HashMap<Key, Arc<Flight>>>,
    slots: Slots,
    writes: AtomicU64,
    pub stats: Stats,
}

impl Store {
    pub fn new(dir: PathBuf) -> Self {
        let _ = std::fs::create_dir_all(&dir);
        let stale = dir.clone();
        std::thread::spawn(move || drop_old_versions(&stale));
        Self {
            dir,
            memory: Mutex::new(Memory::new()),
            inflight: Mutex::new(HashMap::new()),
            slots: Slots::new(DECODE_SLOTS),
            writes: AtomicU64::new(0),
            stats: Stats::default(),
        }
    }
    pub fn clear_memory(&self) {
        self.memory
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clear();
    }
    /// Thumbnail for a track row already loaded from the library. `path` is the stored path.
    /// `Ok(None)` means the file has no usable cover; errors are worth retrying later.
    pub fn for_track(&self, id: i64, path: &Path, edge: u32) -> Result<Option<String>, CoverError> {
        self.render(1, id as u64, path, edge)
    }
    /// Current playback file. The path comes from the player, not from a frontend argument.
    pub fn for_playback(&self, path: &Path, edge: u32) -> Result<Option<String>, CoverError> {
        self.render(2, path_key(path), path, edge)
    }
    fn render(
        &self,
        kind: u8,
        id: u64,
        path: &Path,
        edge: u32,
    ) -> Result<Option<String>, CoverError> {
        self.stats.requests.fetch_add(1, Ordering::Relaxed);
        let edge = edge.clamp(MIN_EDGE, MAX_EDGE);
        let meta = std::fs::metadata(path).map_err(|e| {
            self.stats.transient.fetch_add(1, Ordering::Relaxed);
            if e.kind() == std::io::ErrorKind::NotFound {
                CoverError::FileMissing
            } else {
                CoverError::Unreadable
            }
        })?;
        if !meta.is_file() {
            self.stats.transient.fetch_add(1, Ordering::Relaxed);
            return Err(CoverError::FileMissing);
        }
        let mtime_ns = meta
            .modified()
            .ok()
            .and_then(|m| m.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let key = Key {
            kind,
            id,
            mtime_ns,
            len: meta.len(),
            edge,
        };
        let Some(image) = self.load(key, path, edge)? else {
            return Ok(None);
        };
        self.stats
            .bytes_out
            .fetch_add(image.bytes.len() as u64, Ordering::Relaxed);
        Ok(Some(data_url(image.mime, &image.bytes)))
    }
    fn load(&self, key: Key, path: &Path, edge: u32) -> Result<Cached, CoverError> {
        if let Some(hit) = self
            .memory
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&key)
        {
            self.stats.memory_hits.fetch_add(1, Ordering::Relaxed);
            return Ok(hit);
        }
        let (flight, leader) = {
            let mut inflight = self.inflight.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(existing) = inflight.get(&key) {
                (existing.clone(), false)
            } else {
                let created = Arc::new(Flight::pending());
                inflight.insert(key.clone(), created.clone());
                (created, true)
            }
        };
        if !leader {
            self.stats.coalesced.fetch_add(1, Ordering::Relaxed);
            let value = flight.wait();
            self.finish_wait(&key);
            return value;
        }
        let value = match self.read_disk(&key) {
            Some(hit) => {
                self.stats.disk_hits.fetch_add(1, Ordering::Relaxed);
                Ok(hit)
            }
            None => {
                let waited = Instant::now();
                let _slot = self.slots.enter();
                self.stats
                    .slot_wait_us
                    .fetch_add(waited.elapsed().as_micros() as u64, Ordering::Relaxed);
                let started = Instant::now();
                let encoded = encode_file(path, edge, &self.stats);
                let spent = started.elapsed().as_micros() as u64;
                self.stats.decode_us.fetch_add(spent, Ordering::Relaxed);
                self.stats.decode_max_us.fetch_max(spent, Ordering::Relaxed);
                match encoded {
                    Ok(Ok(image)) => {
                        let shared = Arc::new(image);
                        self.write_disk(&key, Some(&shared));
                        Ok(Some(shared))
                    }
                    Ok(Err(reason)) => {
                        self.count_absent(reason);
                        self.write_disk(&key, None);
                        Ok(None)
                    }
                    Err(error) => {
                        self.stats.transient.fetch_add(1, Ordering::Relaxed);
                        Err(error)
                    }
                }
            }
        };
        if let Ok(cached) = &value {
            self.memory
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .put(key.clone(), cached.clone());
        }
        flight.finish(value.clone());
        self.inflight
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(&key);
        value
    }
    fn finish_wait(&self, key: &Key) {
        let mut inflight = self.inflight.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(flight) = inflight.get(key) {
            let done = matches!(
                &*flight.state.lock().unwrap_or_else(|e| e.into_inner()),
                FlightState::Ready(_)
            );
            if done {
                inflight.remove(key);
            }
        }
    }
    fn count_absent(&self, reason: Absent) {
        self.stats.missing.fetch_add(1, Ordering::Relaxed);
        let counter = match reason {
            Absent::NoPicture => &self.stats.no_picture,
            Absent::Unsupported => &self.stats.unsupported,
            Absent::TooLarge => &self.stats.too_large,
            Absent::Corrupt => &self.stats.corrupt,
        };
        counter.fetch_add(1, Ordering::Relaxed);
    }
    fn file_path(&self, key: &Key, ext: &str) -> PathBuf {
        self.dir.join(format!(
            "{CACHE_VERSION}-{}-{}-{}-{}-{}.{}",
            key.kind, key.id, key.mtime_ns, key.len, key.edge, ext
        ))
    }
    fn read_disk(&self, key: &Key) -> Option<Cached> {
        for (ext, mime) in IMAGE_FILES {
            let file = self.file_path(key, ext);
            if !file.is_file() {
                continue;
            }
            if let Ok(bytes) = std::fs::read(&file) {
                if !bytes.is_empty() {
                    return Some(Some(Arc::new(Image { mime, bytes })));
                }
            }
            let _ = std::fs::remove_file(&file);
        }
        if self.file_path(key, "none").is_file() {
            return Some(None);
        }
        None
    }
    fn write_disk(&self, key: &Key, image: Option<&Image>) {
        let (path, payload): (PathBuf, &[u8]) = match image {
            Some(image) => {
                let ext = IMAGE_FILES
                    .iter()
                    .find(|(_, mime)| *mime == image.mime)
                    .map(|(ext, _)| *ext)
                    .unwrap_or("jpg");
                (self.file_path(key, ext), &image.bytes)
            }
            None => (self.file_path(key, "none"), &[]),
        };
        if std::fs::write(&path, payload).is_ok() {
            let n = self.writes.fetch_add(1, Ordering::Relaxed);
            if n.is_multiple_of(16) {
                trim_disk(&self.dir);
            }
        }
    }
}

fn trim_disk(dir: &Path) {
    let mut files = Vec::new();
    let mut total = 0u64;
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let Ok(meta) = entry.metadata() else { continue };
        if !meta.is_file() {
            continue;
        }
        let Ok(modified) = meta.modified() else {
            continue;
        };
        let stamp = modified
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        total += meta.len();
        files.push((stamp, entry.path(), meta.len()));
    }
    if total <= DISK_BYTES {
        return;
    }
    files.sort_by_key(|item| item.0);
    for (_, path, len) in files {
        if total <= DISK_BYTES * 3 / 4 {
            break;
        }
        if std::fs::remove_file(path).is_ok() {
            total = total.saturating_sub(len);
        }
    }
}

fn path_key(path: &Path) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in path.to_string_lossy().as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

const IMAGE_FILES: [(&str, &str); 3] = [
    ("jpg", "image/jpeg"),
    ("webp", "image/webp"),
    ("gif", "image/gif"),
];

/// Removes disk entries without the current version prefix. Runs once, off the setup thread.
fn drop_old_versions(dir: &Path) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let prefix = format!("{CACHE_VERSION}-");
    for entry in entries.flatten() {
        let name = entry.file_name();
        if !name.to_string_lossy().starts_with(&prefix) && entry.path().is_file() {
            let _ = std::fs::remove_file(entry.path());
        }
    }
}

fn data_url(mime: &str, bytes: &[u8]) -> String {
    format!(
        "data:{mime};base64,{}",
        base64::engine::general_purpose::STANDARD.encode(bytes)
    )
}

fn encode_file(path: &Path, edge: u32, stats: &Stats) -> Result<Result<Image, Absent>, CoverError> {
    use crate::catalog::metadata::CoverReadError;
    match crate::catalog::metadata::read_cover_checked(path) {
        Ok(Some(source)) => Ok(encode_source(&source, edge, stats)),
        Ok(None) => Ok(Err(Absent::NoPicture)),
        Err(CoverReadError::Tags) => Ok(Err(Absent::Unsupported)),
        Err(CoverReadError::Io) => Err(CoverError::Unreadable),
    }
}

/// JPEG thumbnail of a JPEG or PNG picture. Other inputs give `None`.
#[cfg(test)]
fn encode_bytes(source: &[u8], edge: u32, stats: &Stats) -> Option<Vec<u8>> {
    encode_source(source, edge, stats)
        .ok()
        .filter(|image| image.mime == "image/jpeg")
        .map(|image| image.bytes)
}

fn encode_source(source: &[u8], edge: u32, stats: &Stats) -> Result<Image, Absent> {
    if source.is_empty() {
        return Err(Absent::NoPicture);
    }
    if source.len() > MAX_SOURCE_BYTES {
        return Err(Absent::TooLarge);
    }
    let format = image::guess_format(source).map_err(|_| Absent::Corrupt)?;
    let passthrough = match format {
        image::ImageFormat::WebP => Some("image/webp"),
        image::ImageFormat::Gif => Some("image/gif"),
        image::ImageFormat::Jpeg | image::ImageFormat::Png => None,
        _ => return Err(Absent::Unsupported),
    };
    if let Some(mime) = passthrough {
        if source.len() > MAX_PASSTHROUGH_BYTES {
            return Err(Absent::TooLarge);
        }
        stats.passthrough.fetch_add(1, Ordering::Relaxed);
        return Ok(Image {
            mime,
            bytes: source.to_vec(),
        });
    }
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(MAX_PIXELS);
    limits.max_image_height = Some(MAX_PIXELS);
    limits.max_alloc = Some(MAX_DECODE_ALLOC);
    let mut reader = image::ImageReader::with_format(Cursor::new(source), format);
    reader.limits(limits);
    stats.decoded.fetch_add(1, Ordering::Relaxed);
    let image = reader.decode().map_err(|e| match e {
        image::ImageError::Limits(_) => Absent::TooLarge,
        _ => Absent::Corrupt,
    })?;
    let fitted = fit(image, edge);
    let rgb = fitted.into_rgb8();
    let mut output = Cursor::new(Vec::new());
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, 80);
    encoder
        .encode(
            rgb.as_raw(),
            rgb.width(),
            rgb.height(),
            image::ExtendedColorType::Rgb8,
        )
        .map_err(|_| Absent::Corrupt)?;
    let bytes = output.into_inner();
    if bytes.is_empty() {
        return Err(Absent::Corrupt);
    }
    Ok(Image {
        mime: "image/jpeg",
        bytes,
    })
}

fn fit(image: image::DynamicImage, edge: u32) -> image::DynamicImage {
    let width = image.width().max(1);
    let height = image.height().max(1);
    let longest = width.max(height);
    if longest <= edge {
        return image;
    }
    let target_w = (u64::from(width) * u64::from(edge) / u64::from(longest)).max(1) as u32;
    let target_h = (u64::from(height) * u64::from(edge) / u64::from(longest)).max(1) as u32;
    // Area sampling: a 3000 px cover down to a thumbnail is far cheaper than a Triangle filter.
    image.thumbnail(target_w, target_h)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GenericImageView, Rgb, RgbImage};

    fn jpeg(width: u32, height: u32, color: [u8; 3], quality: u8) -> Vec<u8> {
        let mut image = RgbImage::new(width, height);
        for pixel in image.pixels_mut() {
            *pixel = Rgb(color);
        }
        let mut output = Cursor::new(Vec::new());
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, quality);
        encoder
            .encode(
                image.as_raw(),
                width,
                height,
                image::ExtendedColorType::Rgb8,
            )
            .unwrap();
        output.into_inner()
    }

    fn dimensions(bytes: &[u8]) -> (u32, u32) {
        let image = image::load_from_memory(bytes).unwrap();
        (image.width(), image.height())
    }

    #[test]
    fn thumbnail_keeps_aspect_and_does_not_upscale() {
        let stats = Stats::default();
        let wide = encode_bytes(&jpeg(200, 100, [20, 40, 200], 90), 40, &stats).unwrap();
        assert_eq!(dimensions(&wide), (40, 20));
        let small = encode_bytes(&jpeg(10, 10, [1, 2, 3], 90), 96, &stats).unwrap();
        assert_eq!(dimensions(&small), (10, 10));
        assert_eq!(stats.decoded.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn corrupt_and_oversized_images_are_rejected() {
        let stats = Stats::default();
        assert!(encode_bytes(b"not an image", 96, &stats).is_none());
        assert!(encode_bytes(&vec![0u8; MAX_SOURCE_BYTES + 1], 96, &stats).is_none());
        assert_eq!(stats.decoded.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn small_thumbnail_is_far_smaller_than_the_original_ipc_payload() {
        let mut image = RgbImage::new(1600, 1200);
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            *pixel = Rgb([(x % 251) as u8, (y % 241) as u8, ((x * 3 + y) % 255) as u8]);
        }
        let mut original = Cursor::new(Vec::new());
        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut original, 95)
            .encode(image.as_raw(), 1600, 1200, image::ExtendedColorType::Rgb8)
            .unwrap();
        let original = original.into_inner();
        let before = data_url("image/jpeg", &original).len();
        let stats = Stats::default();
        let thumb = encode_bytes(&original, 96, &stats).unwrap();
        let after = data_url("image/jpeg", &thumb).len();
        eprintln!(
            "cover-bytes original_ipc={before} thumb_ipc={after} thumb_jpeg={}",
            thumb.len()
        );
        assert!(
            after * 8 < before,
            "thumb {after} should be much smaller than {before}"
        );
        assert_eq!(dimensions(&thumb).0.max(dimensions(&thumb).1), 96);
        assert_eq!(stats.decoded.load(Ordering::Relaxed), 1);
    }

    fn fixture_copy(dir: &Path, name: &str) -> PathBuf {
        let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tone.flac");
        let before = std::fs::read(&source).unwrap();
        let dest = dir.join(name);
        std::fs::copy(&source, &dest).unwrap();
        assert_eq!(
            std::fs::read(&source).unwrap(),
            before,
            "fixture must stay unchanged"
        );
        dest
    }

    fn embed(path: &Path, picture: Vec<u8>) {
        use lofty::config::WriteOptions;
        use lofty::file::{AudioFile, TaggedFileExt};
        use lofty::picture::{MimeType, Picture, PictureType};
        let mut tagged = lofty::read_from_path(path).unwrap();
        let picture =
            Picture::new_unchecked(PictureType::CoverFront, Some(MimeType::Jpeg), None, picture);
        if let Some(tag) = tagged.primary_tag_mut() {
            tag.remove_picture_type(PictureType::CoverFront);
            tag.push_picture(picture);
        } else {
            let mut tag = lofty::tag::Tag::new(tagged.primary_tag_type());
            tag.push_picture(picture);
            tagged.insert_tag(tag);
        }
        tagged.save_to_path(path, WriteOptions::default()).unwrap();
    }

    #[test]
    fn memory_and_disk_cache_decode_once_and_drop_when_the_file_changes() {
        let dir = std::env::temp_dir().join(format!("mm-covers-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let audio = fixture_copy(&dir, "song.flac");
        embed(&audio, jpeg(80, 40, [220, 10, 10], 90));
        let store = Store::new(dir.join("cache"));
        let first = store.for_track(7, &audio, 32).unwrap().unwrap();
        let second = store.for_track(7, &audio, 32).unwrap().unwrap();
        assert_eq!(first, second);
        assert_eq!(store.stats.decoded.load(Ordering::Relaxed), 1);
        assert!(store.stats.memory_hits.load(Ordering::Relaxed) >= 1);
        let reopened = Store::new(dir.join("cache"));
        assert!(reopened.for_track(7, &audio, 32).unwrap().is_some());
        assert_eq!(reopened.stats.decoded.load(Ordering::Relaxed), 0);
        assert!(reopened.stats.disk_hits.load(Ordering::Relaxed) >= 1);
        store.clear_memory();
        std::thread::sleep(std::time::Duration::from_millis(20));
        embed(&audio, jpeg(80, 40, [10, 220, 10], 90));
        let changed = store.for_track(7, &audio, 32).unwrap().unwrap();
        assert_ne!(first, changed);
        assert!(store.stats.decoded.load(Ordering::Relaxed) >= 2);
        let plain = fixture_copy(&dir, "plain.flac");
        assert!(store.for_track(8, &plain, 32).unwrap().is_none());
        assert_eq!(store.stats.decoded.load(Ordering::Relaxed), 2);
        assert!(store.for_track(8, &plain, 32).unwrap().is_none());
        assert_eq!(store.stats.decoded.load(Ordering::Relaxed), 2);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn same_title_files_keep_their_own_covers() {
        let dir = std::env::temp_dir().join(format!("mm-covers-pair-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let red = fixture_copy(&dir, "red.flac");
        let green = fixture_copy(&dir, "green.flac");
        embed(&red, jpeg(60, 40, [230, 20, 20], 95));
        embed(&green, jpeg(40, 60, [20, 230, 20], 95));
        let store = Store::new(dir.join("cache"));
        let left = store.for_track(1, &red, 40).unwrap().unwrap();
        let right = store.for_track(2, &green, 40).unwrap().unwrap();
        assert_ne!(left, right);
        let red_px = image::load_from_memory(&{
            // The stored JPEG is inside the data URL; compare the decoded thumb colors.
            let raw = store.for_track(1, &red, 40).unwrap().unwrap();
            let b64 = raw.split_once(',').unwrap().1;
            base64::engine::general_purpose::STANDARD
                .decode(b64)
                .unwrap()
        })
        .unwrap();
        let green_px = image::load_from_memory(&{
            let raw = right;
            let b64 = raw.split_once(',').unwrap().1;
            base64::engine::general_purpose::STANDARD
                .decode(b64)
                .unwrap()
        })
        .unwrap();
        let red_sample = red_px.get_pixel(0, 0);
        let green_sample = green_px.get_pixel(0, 0);
        assert!(red_sample[0] > red_sample[1]);
        assert!(green_sample[1] > green_sample[0]);
        if std::env::var_os("MM_EXPORT_COVERS").is_some() {
            let out = std::env::temp_dir().join("music-mine-cover-audio");
            let data = std::env::temp_dir().join("music-mine-cover-data");
            let _ = std::fs::remove_dir_all(&out);
            let _ = std::fs::remove_dir_all(&data);
            std::fs::create_dir_all(&out).unwrap();
            let exported_red = fixture_copy(&out, "red.flac");
            let exported_green = fixture_copy(&out, "green.flac");
            let _plain = fixture_copy(&out, "plain.flac");
            embed(&exported_red, jpeg(80, 60, [230, 20, 20], 95));
            embed(&exported_green, jpeg(60, 80, [20, 230, 20], 95));
            let catalog = crate::catalog::Catalog::new(data.join("library-tauri-v1.sqlite"));
            catalog.scan(out.clone()).unwrap();
            eprintln!("exported {} {}", out.display(), data.display());
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn concurrent_requests_share_one_decode() {
        let dir = std::env::temp_dir().join(format!("mm-covers-flight-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let audio = fixture_copy(&dir, "song.flac");
        embed(&audio, jpeg(120, 80, [8, 16, 32], 90));
        let store = Arc::new(Store::new(dir.join("cache")));
        let mut threads = Vec::new();
        for _ in 0..8 {
            let store = store.clone();
            let audio = audio.clone();
            threads.push(std::thread::spawn(move || store.for_track(3, &audio, 48)));
        }
        let results: Vec<_> = threads.into_iter().map(|t| t.join().unwrap()).collect();
        assert!(results
            .iter()
            .all(|item| item == &results[0] && matches!(item, Ok(Some(_)))));
        assert_eq!(store.stats.decoded.load(Ordering::Relaxed), 1);
        assert!(store.stats.coalesced.load(Ordering::Relaxed) >= 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn embed_picture(
        path: &Path,
        kind: lofty::picture::PictureType,
        mime: lofty::picture::MimeType,
        data: Vec<u8>,
    ) {
        use lofty::config::WriteOptions;
        use lofty::file::{AudioFile, TaggedFileExt};
        let mut tagged = lofty::read_from_path(path).unwrap();
        if tagged.primary_tag().is_none() {
            tagged.insert_tag(lofty::tag::Tag::new(tagged.primary_tag_type()));
        }
        let tag = tagged.primary_tag_mut().unwrap();
        tag.push_picture(lofty::picture::Picture::new_unchecked(
            kind,
            Some(mime),
            None,
            data,
        ));
        tagged.save_to_path(path, WriteOptions::default()).unwrap();
    }

    fn cache_names(dir: &Path) -> Vec<String> {
        let mut names: Vec<_> = std::fs::read_dir(dir)
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        names.sort();
        names
    }

    #[test]
    fn outcomes_distinguish_formats_limits_and_corruption() {
        let stats = Stats::default();
        let webp = base64::engine::general_purpose::STANDARD
            .decode("UklGRhoAAABXRUJQVlA4TA0AAAAvAAAAEAcQERGIiP4HAA==")
            .unwrap();
        let gif = base64::engine::general_purpose::STANDARD
            .decode("R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw==")
            .unwrap();
        assert_eq!(encode_source(&webp, 96, &stats).unwrap().mime, "image/webp");
        assert_eq!(encode_source(&gif, 96, &stats).unwrap().mime, "image/gif");
        let mut big_webp = webp.clone();
        big_webp.resize(MAX_PASSTHROUGH_BYTES + 1, 0);
        assert_eq!(
            encode_source(&big_webp, 96, &stats).err(),
            Some(Absent::TooLarge)
        );
        let mut broken = jpeg(64, 64, [1, 2, 3], 90);
        broken.truncate(40);
        assert_eq!(
            encode_source(&broken, 96, &stats).err(),
            Some(Absent::Corrupt)
        );
        assert_eq!(
            encode_source(b"not an image", 96, &stats).err(),
            Some(Absent::Corrupt)
        );
        assert_eq!(
            encode_source(b"BM\0\0\0\0\0\0\0\0\0\0\0\0", 96, &stats).err(),
            Some(Absent::Unsupported)
        );
        assert_eq!(
            encode_source(&vec![0u8; MAX_SOURCE_BYTES + 1], 96, &stats).err(),
            Some(Absent::TooLarge)
        );
        // 3000 x 3000 RGBA needs 36 MB to decode; the old 32 MB limit cached it as "no cover".
        let rgba = image::RgbaImage::from_pixel(3000, 3000, image::Rgba([200, 30, 30, 255]));
        let mut png = Cursor::new(Vec::new());
        rgba.write_to(&mut png, image::ImageFormat::Png).unwrap();
        let thumb = encode_source(png.get_ref(), 112, &stats).unwrap();
        assert_eq!(thumb.mime, "image/jpeg");
        assert_eq!(dimensions(&thumb.bytes), (112, 112));
    }

    #[test]
    fn transient_failures_are_not_cached_as_no_cover() {
        let dir = std::env::temp_dir().join(format!("mm-covers-transient-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let store = Store::new(dir.join("cache"));
        let audio = dir.join("later.flac");
        assert_eq!(store.for_track(5, &audio, 64), Err(CoverError::FileMissing));
        assert!(cache_names(&dir.join("cache")).is_empty());
        fixture_copy(&dir, "later.flac");
        embed(&audio, jpeg(80, 80, [9, 9, 200], 90));
        assert!(store.for_track(5, &audio, 64).unwrap().is_some());
        let plain = fixture_copy(&dir, "plain.flac");
        assert_eq!(store.for_track(6, &plain, 64), Ok(None));
        let names = cache_names(&dir.join("cache"));
        assert!(names.iter().all(|n| n.starts_with("v2-")), "{names:?}");
        assert_eq!(names.iter().filter(|n| n.ends_with(".none")).count(), 1);
        assert_eq!(store.stats.transient.load(Ordering::Relaxed), 1);
        assert_eq!(store.stats.no_picture.load(Ordering::Relaxed), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn old_cache_entries_are_dropped_and_front_cover_wins() {
        use lofty::picture::{MimeType, PictureType};
        let dir = std::env::temp_dir().join(format!("mm-covers-front-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let cache = dir.join("cache");
        std::fs::create_dir_all(&cache).unwrap();
        std::fs::write(cache.join("1-4-0-0-112.none"), b"").unwrap();
        let audio = fixture_copy(&dir, "two.flac");
        embed_picture(
            &audio,
            PictureType::Other,
            MimeType::Jpeg,
            jpeg(40, 40, [230, 10, 10], 95),
        );
        embed_picture(
            &audio,
            PictureType::CoverFront,
            MimeType::Jpeg,
            jpeg(40, 40, [10, 230, 10], 95),
        );
        let store = Store::new(cache.clone());
        let url = store.for_track(4, &audio, 40).unwrap().unwrap();
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(url.split_once(',').unwrap().1)
            .unwrap();
        let pixel = image::load_from_memory(&bytes).unwrap().get_pixel(5, 5);
        assert!(pixel[1] > pixel[0], "front cover expected, got {pixel:?}");
        for _ in 0..50 {
            if !cache.join("1-4-0-0-112.none").exists() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert!(!cache.join("1-4-0-0-112.none").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
````

## File: src-tauri/src/artwork/mod.rs
````rust
pub(crate) mod covers;
````

## File: src-tauri/src/audio/mod.rs
````rust
pub mod player;
````

## File: src-tauri/src/audio/player.rs
````rust
//! Playback engine.
//!
//! Pipeline: file → Symphonia probe/decode → downmix to stereo → linear
//! resample to the output device's rate → interleaved output channels → ring
//! buffer → cpal/WASAPI callback → speakers.
//!
//! The UI's position comes from `frames_played`, which only the audio callback
//! increments, so the progress bar always reflects audio that really reached
//! the device. If the output device cannot be opened, playback refuses to
//! start and the error is reported instead of faking progress.

use std::collections::VecDeque;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleFormat, SizedSample};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::{FormatOptions, SeekMode, SeekTo};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::units::Time;

use crate::infrastructure::logging::LogBuffer;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutputInfo {
    pub device: String,
    pub sample_rate: u32,
    pub channels: u16,
    pub sample_format: String,
}

#[derive(Clone, Debug)]
pub struct PlayerStatus {
    pub state: PlaybackState,
    pub position: Duration,
    pub duration: Duration,
    pub current_path: Option<String>,
    /// Perceived UI volume ratio; converted to amplitude only in the output callback.
    pub volume: f32,
    pub output: Option<OutputInfo>,
    pub error: Option<String>,
    pub underruns: u64,
}

/// Event sent from the engine to the UI.
#[derive(Clone, Debug)]
pub enum PlaybackEvent {
    /// A track played to its end (buffers drained).
    Finished,
    /// Something went wrong; the message is user-facing.
    Failed(String),
}

enum Cmd {
    Play {
        path: PathBuf,
        duration_ms: u64,
    },
    /// Load a track paused at a position (session restore). Nothing is heard
    /// until the person presses play.
    Prepare {
        path: PathBuf,
        duration_ms: u64,
        position_ms: u64,
    },
    Pause,
    Resume,
    Seek {
        seconds: f64,
    },
    Stop,
    SetVolume {
        volume: f32,
    },
    ClearError,
    Synchronize(Sender<PlayerStatus>),
    Shutdown(Sender<()>),
}

pub struct Player {
    cmd_tx: Sender<Cmd>,
    event_rx: Receiver<PlaybackEvent>,
    status: Arc<Mutex<PlayerStatus>>,
    logs: Arc<LogBuffer>,
}

impl Player {
    pub fn new(log_path: Option<PathBuf>) -> Player {
        let logs = Arc::new(LogBuffer::new(log_path));
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();
        let status = Arc::new(Mutex::new(PlayerStatus {
            state: PlaybackState::Stopped,
            position: Duration::ZERO,
            duration: Duration::ZERO,
            current_path: None,
            volume: 1.0,
            output: None,
            error: None,
            underruns: 0,
        }));
        let status_thread = status.clone();
        let logs_thread = logs.clone();
        std::thread::Builder::new()
            .name("audio-engine".into())
            .spawn(move || engine_loop(cmd_rx, event_tx, status_thread, logs_thread))
            .expect("spawn audio engine");
        Player {
            cmd_tx,
            event_rx,
            status,
            logs,
        }
    }

    /// FIFO barrier acknowledged after publishing the actual engine status.
    pub fn synchronize(&self) -> Result<PlayerStatus, String> {
        let (tx, rx) = mpsc::channel();
        self.cmd_tx
            .send(Cmd::Synchronize(tx))
            .map_err(|_| "音频线程已退出".to_string())?;
        rx.recv_timeout(Duration::from_secs(15))
            .map_err(|_| "音频引擎响应超时".to_string())
    }

    pub fn status(&self) -> PlayerStatus {
        self.status
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    pub fn poll(&self) -> Option<PlaybackEvent> {
        self.event_rx.try_recv().ok()
    }

    pub fn log_lines(&self) -> Vec<String> {
        self.logs.lines()
    }

    pub fn play(&self, path: PathBuf, duration_ms: u64) {
        let _ = self.cmd_tx.send(Cmd::Play { path, duration_ms });
    }
    /// Load a track paused at `position_ms` without producing sound.
    pub fn prepare(&self, path: PathBuf, duration_ms: u64, position_ms: u64) {
        let _ = self.cmd_tx.send(Cmd::Prepare {
            path,
            duration_ms,
            position_ms,
        });
    }
    pub fn pause(&self) {
        let _ = self.cmd_tx.send(Cmd::Pause);
    }
    pub fn resume(&self) {
        let _ = self.cmd_tx.send(Cmd::Resume);
    }
    pub fn seek(&self, seconds: f64) {
        let _ = self.cmd_tx.send(Cmd::Seek { seconds });
    }
    pub fn shutdown(&self) -> Result<(), String> {
        let (tx, rx) = mpsc::channel();
        self.cmd_tx
            .send(Cmd::Shutdown(tx))
            .map_err(|_| "音频线程已退出".to_string())?;
        rx.recv_timeout(Duration::from_secs(15))
            .map_err(|_| "释放音频资源超时".to_string())
    }
    pub fn stop(&self) {
        let _ = self.cmd_tx.send(Cmd::Stop);
    }
    pub fn set_volume(&self, volume: f32) {
        let _ = self.cmd_tx.send(Cmd::SetVolume { volume });
    }
    pub fn clear_error(&self) {
        let _ = self.cmd_tx.send(Cmd::ClearError);
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

// ---------------------------------------------------------------- shared state

struct Shared {
    ring: Mutex<VecDeque<f32>>,
    capacity: usize,
    channels: usize,
    rate: u32,
    volume_bits: AtomicU32,
    paused: AtomicBool,
    /// True while a track is loaded (playing or draining its tail). The
    /// playback clock only advances then.
    clock_on: AtomicBool,
    /// True while audio is expected, so the callback can count underruns.
    active: AtomicBool,
    frames_played: AtomicU64,
    underruns: AtomicU64,
    stream_dead: AtomicBool,
    capture: Mutex<Option<Vec<f32>>>,
    peak_bits: AtomicU32,
}

impl Shared {
    fn new(rate: u32, channels: usize, volume: f32) -> Shared {
        let capacity = (rate as usize * channels * 3) / 10; // 300 ms
        Shared {
            ring: Mutex::new(VecDeque::with_capacity(capacity)),
            capacity,
            channels,
            rate,
            volume_bits: AtomicU32::new(volume.to_bits()),
            paused: AtomicBool::new(false),
            clock_on: AtomicBool::new(false),
            active: AtomicBool::new(false),
            frames_played: AtomicU64::new(0),
            underruns: AtomicU64::new(0),
            stream_dead: AtomicBool::new(false),
            capture: Mutex::new(None),
            peak_bits: AtomicU32::new(0.0f32.to_bits()),
        }
    }

    fn clear_ring(&self) {
        self.ring.lock().unwrap_or_else(|e| e.into_inner()).clear();
    }

    fn fill(&self) -> usize {
        self.ring.lock().unwrap_or_else(|e| e.into_inner()).len()
    }

    fn reset_clock(&self) {
        self.frames_played.store(0, Ordering::SeqCst);
    }
}

struct Output {
    // The stream stops when dropped, so it must stay alive with the engine.
    stream: Option<cpal::Stream>,
    pump: Option<std::thread::JoinHandle<()>>,
    stop_pump: Option<Arc<AtomicBool>>,
    shared: Arc<Shared>,
    info: OutputInfo,
}

fn null_output_requested() -> bool {
    std::env::var_os("MUSIC_MINE_NULL_OUTPUT").is_some()
}

/// WASAPI faults (0xc0000005) when several tests open or drop streams at once.
fn cpal_gate() -> std::sync::MutexGuard<'static, ()> {
    static GATE: Mutex<()> = Mutex::new(());
    GATE.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

impl Drop for Output {
    fn drop(&mut self) {
        self.shared.active.store(false, Ordering::SeqCst);
        self.shared.paused.store(true, Ordering::SeqCst);
        if let Some(stop) = self.stop_pump.take() {
            stop.store(true, Ordering::SeqCst);
        }
        if let Some(pump) = self.pump.take() {
            let _ = pump.join();
        }
        let _gate = cpal_gate();
        if let Some(stream) = self.stream.take() {
            let _ = stream.pause();
            // Let the callback return before the stream is freed.
            std::thread::sleep(Duration::from_millis(20));
            drop(stream);
        }
    }
}

/// Clock that drains the ring without calling cpal. Used when the check must
/// not touch the Windows audio device.
fn open_null_output(logs: &Arc<LogBuffer>, volume: f32) -> Output {
    let shared = Arc::new(Shared::new(48_000, 2, volume));
    let stop = Arc::new(AtomicBool::new(false));
    let stop_thread = stop.clone();
    let shared_thread = shared.clone();
    let pump = std::thread::Builder::new()
        .name("null-output".into())
        .spawn(move || {
            let mut buf = vec![0.0f32; 480];
            while !stop_thread.load(Ordering::SeqCst) {
                fill_output(&mut buf, &shared_thread);
                std::thread::sleep(Duration::from_millis(5));
            }
        })
        .expect("spawn null output");
    logs.push("使用空音频输出，未打开系统设备");
    Output {
        stream: None,
        pump: Some(pump),
        stop_pump: Some(stop),
        shared,
        info: OutputInfo {
            device: "null".into(),
            sample_rate: 48_000,
            channels: 2,
            sample_format: "F32".into(),
        },
    }
}

fn open_output(logs: &Arc<LogBuffer>, volume: f32) -> Result<Output, String> {
    if null_output_requested() {
        return Ok(open_null_output(logs, volume));
    }
    let _gate = cpal_gate();
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| "系统没有可用的音频输出设备".to_string())?;
    let device_name = device.name().unwrap_or_else(|_| "未知设备".to_string());
    logs.push(format!("音频输出设备: {device_name}"));

    // The device's own default config is what Windows uses in shared mode and
    // is the most reliable choice; fall back to every supported config.
    let mut candidates: Vec<cpal::SupportedStreamConfig> = Vec::new();
    match device.default_output_config() {
        Ok(cfg) => {
            logs.push(format!(
                "设备默认格式: {} Hz, {} 声道, {:?}",
                cfg.sample_rate().0,
                cfg.channels(),
                cfg.sample_format()
            ));
            candidates.push(cfg);
        }
        Err(e) => logs.push(format!("读取设备默认格式失败: {e}")),
    }
    if let Ok(list) = device.supported_output_configs() {
        for c in list {
            let rate = 48_000u32.clamp(c.min_sample_rate().0, c.max_sample_rate().0);
            let cfg = c.with_sample_rate(cpal::SampleRate(rate));
            if !candidates.iter().any(|x| {
                x.sample_rate() == cfg.sample_rate()
                    && x.channels() == cfg.channels()
                    && x.sample_format() == cfg.sample_format()
            }) {
                candidates.push(cfg);
            }
        }
    }
    // Prefer float, stereo, 48 kHz — but only after the device default.
    candidates.sort_by_key(|c| {
        (
            c.sample_format() != SampleFormat::F32,
            c.channels() != 2,
            c.sample_rate().0 != 48_000,
        )
    });

    let mut last_err = String::from("没有可尝试的输出格式");
    for cand in candidates {
        let format = cand.sample_format();
        let config = cand.config();
        let shared = Arc::new(Shared::new(
            config.sample_rate.0,
            config.channels as usize,
            volume,
        ));
        let built = match format {
            SampleFormat::F32 => build::<f32>(&device, &config, shared.clone(), logs.clone()),
            SampleFormat::F64 => build::<f64>(&device, &config, shared.clone(), logs.clone()),
            SampleFormat::I16 => build::<i16>(&device, &config, shared.clone(), logs.clone()),
            SampleFormat::I32 => build::<i32>(&device, &config, shared.clone(), logs.clone()),
            SampleFormat::I8 => build::<i8>(&device, &config, shared.clone(), logs.clone()),
            SampleFormat::U8 => build::<u8>(&device, &config, shared.clone(), logs.clone()),
            SampleFormat::U16 => build::<u16>(&device, &config, shared.clone(), logs.clone()),
            SampleFormat::U32 => build::<u32>(&device, &config, shared.clone(), logs.clone()),
            other => {
                last_err = format!("不支持的采样格式: {other}");
                continue;
            }
        };
        match built {
            Ok(stream) => {
                if let Err(e) = stream.play() {
                    drop(stream);
                    last_err = format!("启动音频流失败: {e}");
                    logs.push(format!("{last_err} ({format:?})"));
                    continue;
                }
                logs.push(format!(
                    "音频流已启动: {} Hz, {} 声道, {format:?}",
                    config.sample_rate.0, config.channels
                ));
                return Ok(Output {
                    stream: Some(stream),
                    pump: None,
                    stop_pump: None,
                    shared,
                    info: OutputInfo {
                        device: device_name,
                        sample_rate: config.sample_rate.0,
                        channels: config.channels,
                        sample_format: format!("{format:?}"),
                    },
                });
            }
            Err(e) => {
                last_err = e;
                logs.push(format!("无法用 {format:?} 打开音频流: {last_err}"));
            }
        }
    }
    Err(format!("无法启动音频输出: {last_err}"))
}

/// Result of loading a track into the engine.
struct Started {
    decoder: Option<Decoder>,
    draining_since: Option<Instant>,
    log_line: String,
}

/// Open a file, seek to `position`, pre-fill the device buffer and put the
/// engine in the requested state. Shared by Play and Prepare (session restore).
fn begin_track(
    output: &mut Option<Output>,
    logs: &Arc<LogBuffer>,
    volume: f32,
    path: &Path,
    duration_ms: u64,
    position: Duration,
    paused: bool,
) -> Result<Started, String> {
    if output.is_none() {
        *output = Some(open_output(logs, volume)?);
    }
    let out = output.as_ref().unwrap();
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    let mut d = Decoder::open(path, out.shared.rate, out.shared.channels)
        .map_err(|e| format!("无法播放「{name}」：{e}"))?;

    let log_line = format!(
        "开始播放: {name} | {} | {} Hz | {} 声道 | 预计 {} ms | 输出 {} Hz / {} 声道 | 起点 {:.1}s",
        d.codec,
        d.source_rate,
        d.source_channels,
        duration_ms,
        out.shared.rate,
        out.shared.channels,
        position.as_secs_f64()
    );

    out.shared.paused.store(true, Ordering::SeqCst);
    out.shared.clear_ring();
    out.shared.reset_clock();
    if position > Duration::ZERO {
        let secs = position.as_secs_f64();
        d.seek(secs)?;
        out.shared
            .frames_played
            .store((secs * out.shared.rate as f64) as u64, Ordering::SeqCst);
    }

    // Hold the device while the first chunk is decoded so playback never
    // starts on an empty buffer.
    out.shared.paused.store(true, Ordering::SeqCst);
    let target = out.shared.capacity;
    let mut filled = 0usize;
    let mut hit_end = false;
    while filled < target {
        match d.next_output() {
            Some(chunk) => {
                filled += chunk.len();
                let mut ring = out.shared.ring.lock().unwrap_or_else(|e| e.into_inner());
                ring.extend(chunk);
            }
            None => {
                hit_end = true;
                break;
            }
        }
    }
    if filled == 0 {
        // Probed fine but produced no audio: report it instead of pretending.
        out.shared.paused.store(false, Ordering::SeqCst);
        out.shared.clock_on.store(false, Ordering::SeqCst);
        out.shared.active.store(false, Ordering::SeqCst);
        out.shared.clear_ring();
        return Err(format!("无法播放「{name}」：文件中没有可解码的音频数据"));
    }
    out.shared.clock_on.store(true, Ordering::SeqCst);
    out.shared.active.store(!paused, Ordering::SeqCst);
    out.shared.paused.store(paused, Ordering::SeqCst);
    Ok(Started {
        decoder: if hit_end { None } else { Some(d) },
        draining_since: if hit_end { Some(Instant::now()) } else { None },
        log_line,
    })
}

fn build<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    shared: Arc<Shared>,
    logs: Arc<LogBuffer>,
) -> Result<cpal::Stream, String>
where
    T: SizedSample + FromSample<f32> + Send + 'static,
{
    let error_shared = shared.clone();
    let error_logs = logs;
    let err_fn = move |e: cpal::StreamError| {
        error_logs.push(format!("音频流错误: {e}"));
        error_shared.stream_dead.store(true, Ordering::SeqCst);
    };
    let data_fn = move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
        fill_output(data, &shared);
    };
    device
        .build_output_stream(config, data_fn, err_fn, None)
        .map_err(|e| e.to_string())
}

/// Maps linear UI volume [0.0, 1.0] to amplitude gain using a cubic audio taper (x^3).
/// 0% -> 0.0 (-inf dB), 20% -> 0.008 (-42 dB), 50% -> 0.125 (-18 dB), 100% -> 1.0 (0 dB).
#[inline]
pub fn amplitude_gain(volume: f32) -> f32 {
    let v = volume.clamp(0.0, 1.0);
    v * v * v
}

/// The audio callback. Public only so the self-test can drive it.
fn fill_output<T>(data: &mut [T], shared: &Shared)
where
    T: SizedSample + FromSample<f32>,
{
    let channels = shared.channels.max(1);
    if shared.paused.load(Ordering::Relaxed) {
        for s in data.iter_mut() {
            *s = T::from_sample(0.0f32);
        }
        return;
    }
    let gain = amplitude_gain(f32::from_bits(shared.volume_bits.load(Ordering::Relaxed)));
    let mut peak = 0.0f32;
    let mut missing = 0usize;
    let capturing = shared.capture.lock().map(|c| c.is_some()).unwrap_or(false);
    let mut captured: Vec<f32> = if capturing {
        Vec::with_capacity(data.len())
    } else {
        Vec::new()
    };
    {
        let mut ring = shared.ring.lock().unwrap_or_else(|e| e.into_inner());
        for s in data.iter_mut() {
            match ring.pop_front() {
                Some(v) => {
                    let v = (v * gain).clamp(-1.0, 1.0);
                    peak = peak.max(v.abs());
                    *s = T::from_sample(v);
                    if capturing {
                        captured.push(v);
                    }
                }
                None => {
                    missing += 1;
                    *s = T::from_sample(0.0f32);
                    if capturing {
                        captured.push(0.0);
                    }
                }
            }
        }
    }
    if missing > 0 && shared.active.load(Ordering::Relaxed) {
        shared.underruns.fetch_add(1, Ordering::Relaxed);
    }
    if capturing {
        if let Ok(mut cap) = shared.capture.lock() {
            if let Some(buf) = cap.as_mut() {
                let room = 480_000usize.saturating_sub(buf.len());
                buf.extend(captured.into_iter().take(room));
            }
        }
    }
    shared.peak_bits.store(peak.to_bits(), Ordering::Relaxed);
    if shared.clock_on.load(Ordering::Relaxed) {
        shared
            .frames_played
            .fetch_add((data.len() / channels) as u64, Ordering::Relaxed);
    }
}

// ------------------------------------------------------------------- engine

fn engine_loop(
    cmd_rx: Receiver<Cmd>,
    events: Sender<PlaybackEvent>,
    status: Arc<Mutex<PlayerStatus>>,
    logs: Arc<LogBuffer>,
) {
    let mut output: Option<Output> = None;
    let mut decoder: Option<Decoder> = None;
    let mut state = PlaybackState::Stopped;
    let mut duration = Duration::ZERO;
    let mut current_path: Option<String> = None;
    let mut volume = 1.0f32;
    let mut error: Option<String> = None;
    let mut draining_since: Option<Instant> = None;

    // Aggregate decoded levels for a once-a-second log line.
    let mut window_start = Instant::now();
    let mut win_peak = 0.0f32;
    let mut win_sq = 0.0f64;
    let mut win_n = 0u64;

    logs.push("播放引擎已启动");

    // Open the device up front so a missing/blocked output is visible before
    // the person presses play, instead of failing silently later.
    match open_output(&logs, volume) {
        Ok(o) => output = Some(o),
        Err(e) => {
            logs.push(format!("音频输出不可用: {e}"));
            error = Some(format!("音频输出不可用：{e}"));
        }
    }

    loop {
        let mut acknowledgement = None;
        // ------------------------------------------------------ commands
        loop {
            match cmd_rx.try_recv() {
                Ok(Cmd::Play { path, duration_ms }) => {
                    match begin_track(
                        &mut output,
                        &logs,
                        volume,
                        &path,
                        duration_ms,
                        Duration::ZERO,
                        false,
                    ) {
                        Ok(started) => {
                            logs.push(started.log_line);
                            decoder = started.decoder;
                            draining_since = started.draining_since;
                            state = PlaybackState::Playing;
                            duration = Duration::from_millis(duration_ms);
                            current_path = Some(path.to_string_lossy().to_string());
                            error = None;
                            window_start = Instant::now();
                            win_peak = 0.0;
                            win_sq = 0.0;
                            win_n = 0;
                        }
                        Err(msg) => {
                            logs.push(msg.clone());
                            error = Some(msg.clone());
                            decoder = None;
                            state = PlaybackState::Stopped;
                            duration = Duration::ZERO;
                            current_path = None;
                            let _ = events.send(PlaybackEvent::Failed(msg));
                        }
                    }
                }
                Ok(Cmd::Prepare {
                    path,
                    duration_ms,
                    position_ms,
                }) => {
                    let position = Duration::from_millis(position_ms);
                    match begin_track(
                        &mut output,
                        &logs,
                        volume,
                        &path,
                        duration_ms,
                        position,
                        true,
                    ) {
                        Ok(started) => {
                            logs.push(format!("恢复会话: {}", started.log_line));
                            decoder = started.decoder;
                            draining_since = started.draining_since;
                            state = PlaybackState::Paused;
                            duration = Duration::from_millis(duration_ms);
                            current_path = Some(path.to_string_lossy().to_string());
                            error = None;
                        }
                        Err(msg) => {
                            logs.push(msg.clone());
                            error = Some(msg.clone());
                            decoder = None;
                            state = PlaybackState::Stopped;
                            duration = Duration::ZERO;
                            current_path = None;
                            let _ = events.send(PlaybackEvent::Failed(msg));
                        }
                    }
                }
                Ok(Cmd::Pause) => {
                    if state == PlaybackState::Playing {
                        state = PlaybackState::Paused;
                        if let Some(o) = &output {
                            o.shared.paused.store(true, Ordering::SeqCst);
                            o.shared.active.store(false, Ordering::SeqCst);
                        }
                        logs.push("已暂停");
                    }
                }
                Ok(Cmd::Resume) => {
                    if state == PlaybackState::Paused {
                        state = PlaybackState::Playing;
                        if let Some(o) = &output {
                            o.shared.paused.store(false, Ordering::SeqCst);
                            o.shared.active.store(true, Ordering::SeqCst);
                        }
                        logs.push("继续播放");
                    }
                }
                Ok(Cmd::Seek { seconds }) => {
                    if let Some(path) = current_path.as_ref() {
                        let target = seconds.max(0.0);
                        if duration > Duration::ZERO && target >= duration.as_secs_f64() {
                            decoder = None;
                            draining_since = None;
                            state = PlaybackState::Stopped;
                            error = None;
                            if let Some(o) = &output {
                                o.shared.paused.store(true, Ordering::SeqCst);
                                o.shared.clear_ring();
                                o.shared.clock_on.store(false, Ordering::SeqCst);
                                o.shared.active.store(false, Ordering::SeqCst);
                                o.shared.frames_played.store(
                                    (duration.as_secs_f64() * o.shared.rate as f64) as u64,
                                    Ordering::SeqCst,
                                );
                            }
                            continue;
                        }
                        match begin_track(
                            &mut output,
                            &logs,
                            volume,
                            Path::new(path),
                            duration.as_millis() as u64,
                            Duration::from_secs_f64(target),
                            state == PlaybackState::Paused,
                        ) {
                            Ok(started) => {
                                decoder = started.decoder;
                                draining_since = started.draining_since;
                                error = None;
                                logs.push(format!("跳转到 {target:.1}s"));
                            }
                            Err(msg) => {
                                state = PlaybackState::Stopped;
                                decoder = None;
                                error = Some(msg.clone());
                                let _ = events.send(PlaybackEvent::Failed(msg));
                            }
                        }
                    } else {
                        error = Some("歌曲已进入结尾，请重新播放后跳转".into());
                    }
                }
                Ok(Cmd::Shutdown(reply)) => {
                    drop(decoder);
                    drop(output);
                    let _ = reply.send(());
                    return;
                }
                Ok(Cmd::Stop) => {
                    decoder = None;
                    state = PlaybackState::Stopped;
                    duration = Duration::ZERO;
                    current_path = None;
                    draining_since = None;
                    if let Some(o) = &output {
                        o.shared.clear_ring();
                        o.shared.reset_clock();
                        o.shared.paused.store(false, Ordering::SeqCst);
                        o.shared.clock_on.store(false, Ordering::SeqCst);
                        o.shared.active.store(false, Ordering::SeqCst);
                    }
                }
                Ok(Cmd::SetVolume { volume: v }) => {
                    volume = v.clamp(0.0, 1.0);
                    if let Some(o) = &output {
                        o.shared
                            .volume_bits
                            .store(volume.to_bits(), Ordering::Relaxed);
                    }
                }
                Ok(Cmd::ClearError) => {
                    error = None;
                }
                Ok(Cmd::Synchronize(reply)) => {
                    acknowledgement = Some(reply);
                    break;
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return,
            }
        }

        // Failed switches/seeks must not drain a previous song as if it succeeded.
        if state == PlaybackState::Stopped && error.is_some() {
            draining_since = None;
            if let Some(o) = &output {
                o.shared.paused.store(true, Ordering::SeqCst);
                o.shared.clock_on.store(false, Ordering::SeqCst);
                o.shared.active.store(false, Ordering::SeqCst);
                o.shared.clear_ring();
                o.shared.reset_clock();
            }
        }

        // A dead stream (device removed, format lost) must not look like silence.
        if let Some(o) = &output {
            if o.shared.stream_dead.swap(false, Ordering::SeqCst) {
                let msg = format!("音频输出已中断（{}），请检查设备后重试。", o.info.device);
                logs.push(msg.clone());
                error = Some(msg.clone());
                state = PlaybackState::Stopped;
                decoder = None;
                output = None;
                let _ = events.send(PlaybackEvent::Failed(msg));
            }
        }

        // -------------------------------------------------------- decoding
        if state == PlaybackState::Playing {
            if let Some(o) = &output {
                if decoder.is_some() && o.shared.fill() < o.shared.capacity {
                    let chunk = decoder.as_mut().and_then(|d| d.next_output());
                    match chunk {
                        Some(samples) => {
                            let mut ring = o.shared.ring.lock().unwrap_or_else(|e| e.into_inner());
                            for s in &samples {
                                ring.push_back(*s);
                                win_peak = win_peak.max(s.abs());
                                win_sq += (*s as f64) * (*s as f64);
                            }
                            win_n += samples.len() as u64;
                        }
                        None => {
                            // End of stream: let the buffered tail play out.
                            // No more audio is coming, so stop counting
                            // underruns for the (intentionally) empty buffer.
                            draining_since = Some(Instant::now());
                            decoder = None;
                            o.shared.active.store(false, Ordering::SeqCst);
                        }
                    }
                } else {
                    std::thread::sleep(Duration::from_millis(2));
                }
            }
        }

        // -------------------------------------------------------- draining
        if state == PlaybackState::Playing && draining_since.is_some() {
            let empty = output
                .as_ref()
                .map(|o| o.shared.fill() == 0)
                .unwrap_or(true);
            if empty {
                logs.push("播放到结尾");
                draining_since = None;
                state = PlaybackState::Stopped;
                if let Some(o) = &output {
                    o.shared.active.store(false, Ordering::SeqCst);
                    o.shared.clock_on.store(false, Ordering::SeqCst);
                }
                let _ = events.send(PlaybackEvent::Finished);
            }
        }

        // ---------------------------------------------------- level logging
        if state == PlaybackState::Playing && window_start.elapsed() >= Duration::from_secs(1) {
            if win_n > 0 {
                let rms = (win_sq / win_n as f64).sqrt() as f32;
                let (buffered, capacity) = output
                    .as_ref()
                    .map(|o| {
                        (
                            (o.shared.fill() / o.shared.channels.max(1))
                                .min(o.shared.capacity / o.shared.channels.max(1)),
                            o.shared.capacity / o.shared.channels.max(1),
                        )
                    })
                    .unwrap_or((0, 0));
                logs.push(format!(
                    "PCM 电平: 峰值 {:.3} / RMS {:.3} | 缓冲区 {buffered}/{capacity} 帧 | 欠载 {}",
                    win_peak,
                    rms,
                    output
                        .as_ref()
                        .map(|o| o.shared.underruns.load(Ordering::Relaxed))
                        .unwrap_or(0)
                ));
            }
            window_start = Instant::now();
            win_peak = 0.0;
            win_sq = 0.0;
            win_n = 0;
        }

        // --------------------------------------------------------- status
        {
            let (position, out_info, underruns) = match &output {
                Some(o) => (
                    Duration::from_secs_f64(
                        o.shared.frames_played.load(Ordering::Relaxed) as f64
                            / o.shared.rate as f64,
                    ),
                    Some(o.info.clone()),
                    o.shared.underruns.load(Ordering::Relaxed),
                ),
                None => (Duration::ZERO, None, 0),
            };
            let position = if duration > Duration::ZERO && position > duration {
                duration
            } else {
                position
            };
            let mut st = status.lock().unwrap_or_else(|e| e.into_inner());
            *st = PlayerStatus {
                state,
                position,
                duration,
                current_path: current_path.clone(),
                volume,
                output: out_info,
                error: error.clone(),
                underruns,
            };
        }

        if let Some(reply) = acknowledgement {
            let _ = reply.send(status.lock().unwrap_or_else(|e| e.into_inner()).clone());
        }
        std::thread::sleep(Duration::from_millis(3));
    }
}

// ------------------------------------------------------------------ decoder

struct Decoder {
    format: Box<dyn symphonia::core::formats::FormatReader>,
    decoder: Box<dyn symphonia::core::codecs::Decoder>,
    track_id: u32,
    source_rate: u32,
    source_channels: usize,
    out_rate: u32,
    out_channels: usize,
    resampler: StreamResampler,
    codec: String,
}

impl Decoder {
    fn open(path: &Path, out_rate: u32, out_channels: usize) -> Result<Decoder, String> {
        let file = File::open(path).map_err(|e| format!("打开文件失败: {e}"))?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());
        let mut hint = Hint::new();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        }
        let probed = symphonia::default::get_probe()
            .format(
                &hint,
                mss,
                &FormatOptions {
                    enable_gapless: true,
                    ..Default::default()
                },
                &MetadataOptions::default(),
            )
            .map_err(|e| format!("无法识别的音频格式: {e}"))?;
        let format = probed.format;
        let track = format
            .default_track()
            .ok_or_else(|| "文件中没有音频轨道".to_string())?;
        let track_id = track.id;
        let params = track.codec_params.clone();
        let codec = format!("{:?}", params.codec);
        let source_rate = params.sample_rate.unwrap_or(44_100);
        let source_channels = params.channels.map(|c| c.count()).unwrap_or(2);
        let decoder = symphonia::default::get_codecs()
            .make(&params, &DecoderOptions::default())
            .map_err(|e| format!("无法创建解码器: {e}"))?;
        Ok(Decoder {
            format,
            decoder,
            track_id,
            source_rate,
            source_channels,
            out_rate,
            out_channels: out_channels.max(1),
            resampler: StreamResampler::new(source_rate as f64, out_rate as f64),
            codec,
        })
    }

    fn seek(&mut self, seconds: f64) -> Result<(), String> {
        let secs = seconds.max(0.0);
        let time = Time::new(secs as u64, secs.fract());
        self.format
            .seek(
                SeekMode::Accurate,
                SeekTo::Time {
                    time,
                    track_id: Some(self.track_id),
                },
            )
            .map_err(|e| format!("跳转失败：{e}"))?;
        self.decoder.reset();
        self.resampler = StreamResampler::new(self.source_rate as f64, self.out_rate as f64);
        Ok(())
    }

    /// Decode packets until some audio is ready. Returns interleaved samples in
    /// the output channel layout, or None at end of stream.
    fn next_output(&mut self) -> Option<Vec<f32>> {
        loop {
            let packet = match self.format.next_packet() {
                Ok(p) => p,
                Err(_) => return None,
            };
            if packet.track_id() != self.track_id {
                continue;
            }
            let decoded = match self.decoder.decode(&packet) {
                Ok(d) => d,
                // A damaged packet is not the end of the file.
                Err(e) => {
                    eprintln!("[player] decode error (skipping packet): {e}");
                    continue;
                }
            };
            let spec = *decoded.spec();
            let frames = decoded.frames();
            if frames == 0 {
                continue;
            }
            let channels = spec.channels.count().max(1);
            let mut sbuf = SampleBuffer::<f32>::new(frames as u64, spec);
            sbuf.copy_interleaved_ref(decoded);
            let samples = sbuf.samples();

            // Downmix to stereo, resample, then map to the device's channels.
            let mut out: Vec<f32> = Vec::new();
            let mut stage = Vec::with_capacity(frames * 2);
            for f in 0..frames {
                let base = f * channels;
                let (l, r) = match channels {
                    1 => {
                        let m = samples[base];
                        (m, m)
                    }
                    2 => (samples[base], samples[base + 1]),
                    _ => {
                        let c = samples.get(base + 2).copied().unwrap_or(0.0);
                        (samples[base] + 0.707 * c, samples[base + 1] + 0.707 * c)
                    }
                };
                self.resampler.process([l, r], &mut stage);
            }
            match self.out_channels {
                1 => {
                    for pair in stage.as_chunks::<2>().0 {
                        out.push((pair[0] + pair[1]) * 0.5);
                    }
                }
                2 => out = stage,
                n => {
                    for pair in stage.as_chunks::<2>().0 {
                        out.push(pair[0]);
                        out.push(pair[1]);
                        out.extend(std::iter::repeat_n(0.0, n - 2));
                    }
                }
            }
            if !out.is_empty() {
                return Some(out);
            }
        }
    }
}

/// Streaming linear resampler (stereo). Carries the previous frame across calls.
struct StreamResampler {
    ratio: f64,
    next_t: f64,
    prev: Option<[f32; 2]>,
    i: u64,
}

impl StreamResampler {
    fn new(in_rate: f64, out_rate: f64) -> Self {
        Self {
            ratio: in_rate / out_rate,
            next_t: 0.0,
            prev: None,
            i: 0,
        }
    }

    fn process(&mut self, cur: [f32; 2], out: &mut Vec<f32>) {
        match self.prev {
            None => self.prev = Some(cur),
            Some(p) => {
                while self.next_t < self.i as f64 {
                    let local = (self.next_t - (self.i as f64 - 1.0)) as f32;
                    let l = p[0] + (cur[0] - p[0]) * local;
                    let r = p[1] + (cur[1] - p[1]) * local;
                    out.push(l);
                    out.push(r);
                    self.next_t += self.ratio;
                }
                self.prev = Some(cur);
            }
        }
        self.i += 1;
    }
}

// ------------------------------------------------------- diagnostics API

#[derive(Clone, Debug)]
pub struct AudioReport {
    pub codec: String,
    pub sample_rate: u32,
    pub channels: usize,
    pub frames_out: u64,
    pub seconds: f32,
    pub peak: f32,
    pub rms: f32,
}

impl AudioReport {
    pub fn is_silent(&self) -> bool {
        self.peak < 0.001
    }
}

/// Decode a file with the real engine decoder (no device needed) and measure
/// the PCM it produces. Used by tests and troubleshooting.
pub fn analyze_file(path: &Path, max_seconds: f32) -> Result<AudioReport, String> {
    let mut d = Decoder::open(path, 48_000, 2)?;
    let limit = (max_seconds.max(0.1) * 48_000.0 * 2.0) as usize;
    let mut peak = 0.0f32;
    let mut sum_sq = 0.0f64;
    let mut n = 0usize;
    let mut frames = 0u64;
    while n < limit {
        match d.next_output() {
            Some(chunk) => {
                for s in &chunk {
                    peak = peak.max(s.abs());
                    sum_sq += (*s as f64) * (*s as f64);
                }
                n += chunk.len();
                frames += (chunk.len() / 2) as u64;
            }
            None => break,
        }
    }
    Ok(AudioReport {
        codec: d.codec.clone(),
        sample_rate: d.source_rate,
        channels: d.source_channels,
        frames_out: frames,
        seconds: frames as f32 / 48_000.0,
        peak,
        rms: if n > 0 {
            (sum_sq / n as f64).sqrt() as f32
        } else {
            0.0
        },
    })
}

#[derive(Clone, Debug)]
pub struct SelfTestReport {
    pub device: String,
    pub sample_rate: u32,
    pub channels: u16,
    pub format: String,
    pub frames_delivered: u64,
    pub peak: f32,
}

/// Open the real output device, feed it a 440 Hz tone through the engine's
/// ring buffer, and report what the audio callback actually wrote to the
/// device. Proves audio data reaches WASAPI (it cannot prove the speakers are
/// turned up).
pub fn output_selftest(millis: u64) -> Result<SelfTestReport, String> {
    if null_output_requested() {
        return Err("MUSIC_MINE_NULL_OUTPUT：未打开真实设备".into());
    }
    let logs = Arc::new(LogBuffer::new(None));
    let out = open_output(&logs, 1.0)?;
    // Route the callback's samples into a capture buffer for inspection.
    out.shared
        .capture
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .replace(Vec::with_capacity(48_000 * 2));

    let rate = out.shared.rate as f32;
    let channels = out.shared.channels;
    // Pre-fill ~50 ms of tone so the first callbacks are not silent.
    {
        let mut ring = out.shared.ring.lock().unwrap_or_else(|e| e.into_inner());
        for i in 0..(rate as usize * channels / 20) {
            let t = i as f32 / (rate * channels as f32);
            ring.push_back((2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.5);
        }
    }
    out.shared.paused.store(false, Ordering::SeqCst);
    out.shared.clock_on.store(true, Ordering::SeqCst);
    out.shared.active.store(true, Ordering::SeqCst);

    // Keep the ring fed while the device pulls frames in real time.
    let deadline = Instant::now() + Duration::from_millis(millis);
    let mut t = 0.0f32; // seconds
    while Instant::now() < deadline {
        {
            let mut ring = out.shared.ring.lock().unwrap_or_else(|e| e.into_inner());
            while ring.len() + channels <= out.shared.capacity {
                let v = (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.5;
                for _ in 0..channels {
                    ring.push_back(v);
                }
                t += 1.0 / rate;
            }
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    let captured = out
        .shared
        .capture
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .take()
        .unwrap_or_default();
    let peak = captured.iter().fold(0.0f32, |m, s| m.max(s.abs()));
    Ok(SelfTestReport {
        device: out.info.device.clone(),
        sample_rate: out.info.sample_rate,
        channels: out.info.channels,
        format: out.info.sample_format.clone(),
        frames_delivered: out.shared.frames_played.load(Ordering::SeqCst),
        peak,
    })
}

// -------------------------------------------------------------------- tests

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn poisoned_status_remains_readable_and_engine_publishes() {
        let player = Player::new(None);
        player.synchronize().unwrap();
        let status = player.status.clone();
        assert!(std::thread::spawn(move || {
            let _guard = status.lock().unwrap_or_else(|e| e.into_inner());
            panic!("intentional status poison");
        })
        .join()
        .is_err());
        assert!(player.status.is_poisoned());
        assert_eq!(player.status().state, PlaybackState::Stopped);
        player.set_volume(0.25);
        assert_eq!(player.synchronize().unwrap().volume, 0.25);
        player.shutdown().unwrap();
    }

    #[test]
    fn poisoned_ring_supports_fill_clear_and_callback() {
        let shared = Arc::new(Shared::new(48_000, 2, 1.0));
        let poisoned = shared.clone();
        assert!(std::thread::spawn(move || {
            let mut ring = poisoned.ring.lock().unwrap_or_else(|e| e.into_inner());
            ring.extend([0.5, -0.5]);
            panic!("intentional ring poison");
        })
        .join()
        .is_err());
        assert!(shared.ring.is_poisoned());
        assert_eq!(shared.fill(), 2);
        let mut data = [1.0f32; 4];
        fill_output(&mut data, &shared);
        assert_eq!(data, [0.5, -0.5, 0.0, 0.0]);
        shared
            .ring
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push_back(0.5);
        shared.clear_ring();
        assert_eq!(shared.fill(), 0);
        fill_output(&mut data, &shared);
        assert_eq!(data, [0.0; 4]);
    }

    /// Write a small stereo 16-bit PCM WAV file for testing.
    pub fn write_test_wav(path: &Path, rate: u32, seconds: f32) {
        let frames = (rate as f32 * seconds) as u32;
        let mut data = Vec::new();
        for i in 0..frames {
            let t = i as f32 / rate as f32;
            let v = (2.0 * std::f32::consts::PI * 440.0 * t).sin();
            let s = (v * 12000.0) as i16;
            data.extend_from_slice(&s.to_le_bytes());
            data.extend_from_slice(&s.to_le_bytes());
        }
        let byte_rate = rate * 4;
        let mut f = Vec::new();
        f.extend_from_slice(b"RIFF");
        f.extend_from_slice(&(36 + data.len() as u32).to_le_bytes());
        f.extend_from_slice(b"WAVE");
        f.extend_from_slice(b"fmt ");
        f.extend_from_slice(&16u32.to_le_bytes());
        f.extend_from_slice(&1u16.to_le_bytes());
        f.extend_from_slice(&2u16.to_le_bytes());
        f.extend_from_slice(&rate.to_le_bytes());
        f.extend_from_slice(&byte_rate.to_le_bytes());
        f.extend_from_slice(&4u16.to_le_bytes());
        f.extend_from_slice(&16u16.to_le_bytes());
        f.extend_from_slice(b"data");
        f.extend_from_slice(&(data.len() as u32).to_le_bytes());
        f.extend_from_slice(&data);
        std::fs::write(path, f).unwrap();
    }

    #[test]
    fn resampler_upsamples_two_to_one() {
        let mut r = StreamResampler::new(24000.0, 48000.0);
        let mut out = Vec::new();
        for v in [[0.0f32, 0.0], [1.0, 1.0], [2.0, 2.0], [3.0, 3.0]] {
            r.process(v, &mut out);
        }
        assert_eq!(out.len(), 12);
        assert_eq!(out[0], 0.0);
        assert!((out[2] - 0.5).abs() < 1e-6);
        assert_eq!(out[4], 1.0);
        assert_eq!(out[8], 2.0);
        assert!((out[10] - 2.5).abs() < 1e-6);
    }

    #[test]
    fn resampler_passthrough() {
        let mut r = StreamResampler::new(48000.0, 48000.0);
        let mut out = Vec::new();
        for v in [[1.0f32, 2.0], [3.0, 4.0], [5.0, 6.0]] {
            r.process(v, &mut out);
        }
        assert_eq!(out, vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn decoded_wav_is_not_silent_and_has_expected_length() {
        let dir = std::env::temp_dir().join("music-mine-tests-decode");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("tone.wav");
        write_test_wav(&path, 44100, 1.0);

        let report = analyze_file(&path, 2.0).unwrap();
        assert!(!report.is_silent(), "decoded PCM is silent: {report:?}");
        assert!(report.peak > 0.2, "peak too low: {report:?}");
        assert!(report.rms > 0.1, "rms too low: {report:?}");
        assert!(
            (report.seconds - 1.0).abs() < 0.05,
            "duration off: {report:?}"
        );
    }

    #[test]
    fn seeks_within_wav() {
        let dir = std::env::temp_dir().join("music-mine-tests-seek");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("long.wav");
        write_test_wav(&path, 44100, 3.0);

        let mut d = Decoder::open(&path, 48_000, 2).unwrap();
        d.seek(2.0).unwrap();
        let first = d.next_output().expect("audio after seek");
        assert!(!first.is_empty());
        assert!(first.iter().any(|s| s.abs() > 0.01), "silence after seek");
        assert!(first.iter().all(|s| s.is_finite()));
    }

    #[test]
    fn callback_moves_samples_to_the_device_buffer() {
        // Drives the actual callback logic without a device.
        let shared = Shared::new(48_000, 2, 1.0);
        shared.active.store(true, Ordering::SeqCst);
        shared.clock_on.store(true, Ordering::SeqCst);
        {
            let mut ring = shared.ring.lock().unwrap_or_else(|e| e.into_inner());
            for i in 0..480 {
                ring.push_back(if i % 2 == 0 { 0.5 } else { -0.5 });
            }
        }
        let mut data = vec![0.0f32; 192];
        fill_output(&mut data, &shared);
        assert_eq!(data.len(), 192);
        assert!(data.iter().any(|s| s.abs() > 0.4), "callback wrote silence");
        assert_eq!(shared.frames_played.load(Ordering::SeqCst), 96);
    }

    #[test]
    fn pause_writes_silence_and_freezes_the_clock() {
        let shared = Shared::new(48_000, 2, 1.0);
        shared.active.store(true, Ordering::SeqCst);
        shared.paused.store(true, Ordering::SeqCst);
        {
            let mut ring = shared.ring.lock().unwrap_or_else(|e| e.into_inner());
            ring.push_back(0.8);
            ring.push_back(0.8);
        }
        let mut data = vec![0.0f32; 64];
        fill_output(&mut data, &shared);
        assert!(data.iter().all(|s| *s == 0.0));
        assert_eq!(shared.frames_played.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn amplitude_gain_maps_perceived_volume_and_clamps_bounds() {
        for (volume, expected) in [
            (-0.5, 0.0),
            (0.0, 0.0),
            (0.2, 0.008),
            (0.5, 0.125),
            (1.0, 1.0),
            (1.5, 1.0),
        ] {
            assert!(
                (amplitude_gain(volume) - expected).abs() < 0.000001,
                "volume {volume}"
            );
        }
    }

    #[test]
    fn volume_scales_the_actual_callback_samples() {
        for (volume, expected) in [
            (0.0, 0.0),
            (0.2, 0.0064),
            (0.25, 0.0125),
            (0.5, 0.1),
            (1.0, 0.8),
        ] {
            let shared = Shared::new(48_000, 2, volume);
            shared
                .ring
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .extend([0.8, -0.8]);
            let mut samples = [0.0f32; 2];
            fill_output(&mut samples, &shared);
            assert!((samples[0] - expected).abs() < 0.000001, "volume {volume}");
            assert!((samples[1] + expected).abs() < 0.000001, "volume {volume}");
        }
    }

    #[test]
    fn paused_short_track_keeps_its_buffer_until_resumed() {
        let path =
            std::env::temp_dir().join(format!("music-mine-tauri-short-{}.wav", std::process::id()));
        write_test_wav(&path, 44100, 0.1);
        let player = Player::new(None);
        player.set_volume(0.0);
        player.prepare(path.clone(), 100, 0);
        assert_eq!(player.synchronize().unwrap().state, PlaybackState::Paused);
        std::thread::sleep(Duration::from_millis(2200));
        assert_eq!(player.synchronize().unwrap().state, PlaybackState::Paused);
        player.resume();
        player.synchronize().unwrap();
        std::thread::sleep(Duration::from_millis(400));
        assert_eq!(player.synchronize().unwrap().state, PlaybackState::Stopped);
        drop(player);
        std::thread::sleep(Duration::from_millis(20));
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn real_output_device_delivers_audio() {
        // Skips (with a visible message) only when the machine has no device.
        match output_selftest(400) {
            Ok(r) => {
                eprintln!("selftest: {r:?}");
                assert!(r.frames_delivered > 0, "device never pulled frames");
                assert!(r.peak > 0.05, "device received silence: {r:?}");
            }
            Err(e) => eprintln!("SKIPPED (no output device): {e}"),
        }
    }
}
````

## File: src-tauri/src/catalog/catalog_tests.rs
````rust
use super::*;

#[test]
fn scan_real_formats_search_and_cancel() {
    let dir = std::env::temp_dir().join(format!("mm-catalog-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let state = Catalog::new(dir.join("library.db"));
    state
        .scan(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures"))
        .unwrap();
    let page = state.query(Query::default()).unwrap();
    assert_eq!(page.total, 3);
    state
        .scan(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures"))
        .unwrap();
    assert_eq!(
        state.query(Query::default()).unwrap().total,
        3,
        "rescans must not duplicate paths"
    );
    assert!(page.tracks.iter().all(|t| t.duration_ms > 2800));
    let query = Query {
        search: "tone".into(),
        limit: 1,
        offset: 1,
        ..Default::default()
    };
    assert_eq!(state.query(query).unwrap().tracks.len(), 1);
    assert_eq!(
        state
            .query(Query {
                search: "%".into(),
                ..Default::default()
            })
            .unwrap()
            .total,
        0
    );
    state.cancel.store(true, Ordering::Relaxed);
    state.scan(PathBuf::from("missing")).unwrap();
    assert!(state.progress.lock().unwrap().cancelled);
    assert_eq!(state.query(Query::default()).unwrap().total, 3);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn wal_backup_preserves_ids_collections_and_settings() {
    let dir = std::env::temp_dir().join(format!("mm-backup-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let source = dir.join("old.db");
    let conn = db::open(&source).unwrap();
    conn.execute_batch("PRAGMA journal_mode=WAL;").unwrap();
    let track = metadata::read_track(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tone.flac"),
        0,
    )
    .unwrap();
    db::upsert_tracks(&conn, &[track]).unwrap();
    let second = metadata::read_track(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tone.mp3"),
        0,
    )
    .unwrap();
    db::upsert_tracks(&conn, &[second]).unwrap();
    let id = db::load_tracks(&conn).unwrap()[0].id;
    let second_id = db::load_tracks(&conn).unwrap()[1].id;
    collections::set_favorite(&conn, id, true).unwrap();
    let playlist = collections::create_playlist(&conn, "保留歌单").unwrap();
    collections::add_to_playlist(&conn, playlist, id).unwrap();
    collections::add_to_playlist(&conn, playlist, second_id).unwrap();
    collections::move_in_playlist(&conn, playlist, 0, 1).unwrap();
    db::add_folder(&conn, "C:/legacy-music").unwrap();
    db::set_setting(&conn, "session_v2", "{\"queue\":[]}").unwrap();
    let dest = Catalog::new(dir.join("new.db"));
    let report = dest.import_copy(&source).unwrap();
    assert_eq!(
        (report.tracks, report.favorites, report.playlists),
        (2, 1, 1)
    );
    assert!(*dest.initialized.lock().unwrap());
    assert_eq!(dest.summary().unwrap().tracks, 2);
    assert_eq!(dest.initialization_count.load(Ordering::SeqCst), 1);
    let copied = db::open(&dest.path).unwrap();
    assert_eq!(
        collections::playlist_track_ids(&copied, playlist),
        vec![second_id, id]
    );
    assert_eq!(db::load_folders(&copied).unwrap(), vec!["C:/legacy-music"]);
    assert_eq!(
        db::get_setting(&copied, "session_v2"),
        Some("{\"queue\":[]}".into())
    );
    assert!(dest.import_copy(&source).is_err());
    assert_eq!(db::load_tracks(&conn).unwrap().len(), 2);
    drop(copied);
    drop(conn);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn nested_scan_reports_corrupt_files_and_preserves_audio_bytes() {
    let dir = std::env::temp_dir().join(format!("mm-readonly-{}", std::process::id()));
    std::fs::create_dir_all(dir.join("album/nested")).unwrap();
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tone.wav");
    let expected = std::fs::read(source).unwrap();
    let audio = dir.join("album/nested/01 - Read only.WAV");
    std::fs::write(&audio, &expected).unwrap();
    std::fs::write(dir.join("album/broken.mp3"), b"broken").unwrap();
    std::fs::write(dir.join("notes.txt"), b"ignore").unwrap();
    let catalog = Catalog::new(dir.join("catalog.db"));
    catalog.scan(dir.join("album")).unwrap();
    let page = catalog.query(Query::default()).unwrap();
    assert_eq!(page.total, 1);
    assert_eq!(page.tracks[0].title, "Read only");
    assert_eq!(page.tracks[0].track_number, Some(1));
    assert_eq!(catalog.progress.lock().unwrap().errors.len(), 1);
    assert_eq!(std::fs::read(audio).unwrap(), expected);
    std::fs::remove_dir_all(dir).unwrap();
}
/// The built-in “本地” collection is the unfiltered library query: it follows imports and removals
/// and stays independent of a user playlist that has the same name.
#[test]
fn local_collection_tracks_the_library_and_leaves_user_playlists_alone() {
    let dir = std::env::temp_dir().join(format!("mm-local-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let state = Catalog::new(dir.join("library.db"));
    let local = || {
        state
            .query(Query {
                sort: "added".into(),
                ..Default::default()
            })
            .unwrap()
            .total
    };
    assert_eq!(local(), 0);
    let conn = state.connection().unwrap();
    let user = collections::create_playlist(&conn, "本地").unwrap();
    let other = collections::create_playlist(&conn, "夜晚").unwrap();
    drop(conn);
    let folder = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    state.scan(folder.clone()).unwrap();
    assert_eq!(local(), 3, "new imports appear without copying rows");
    let conn = state.connection().unwrap();
    let first = db::load_tracks(&conn).unwrap()[0].id;
    collections::add_to_playlist(&conn, other, first).unwrap();
    let in_playlist = |id| {
        state
            .query(Query {
                playlist: Some(id),
                ..Default::default()
            })
            .unwrap()
            .total
    };
    assert_eq!(
        in_playlist(user),
        0,
        "same-name user playlist keeps its own (empty) membership"
    );
    assert_eq!(in_playlist(other), 1);
    let rows: i64 = conn
        .query_row("SELECT COUNT(*) FROM playlist_tracks", [], |r| r.get(0))
        .unwrap();
    assert_eq!(
        rows, 1,
        "no membership rows are written for the built-in collection"
    );
    collections::delete_playlist(&conn, user).unwrap();
    assert_eq!(
        local(),
        3,
        "deleting a playlist never removes library tracks"
    );
    db::remove_folder(&conn, &folder.to_string_lossy()).unwrap();
    drop(conn);
    assert_eq!(local(), 0, "removed library files leave the collection");
    assert_eq!(in_playlist(other), 0);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn listening_history_is_real_bounded_and_persistent() {
    let dir = std::env::temp_dir().join(format!("mm-home-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let state = Catalog::new(dir.join("library.db"));
    state
        .scan(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures"))
        .unwrap();
    assert!(state.home().unwrap().recent.is_empty());
    state.record_listen("not-a-library-track").unwrap();
    assert!(state.home().unwrap().recent.is_empty());
    let track = state.query(Query::default()).unwrap().tracks.remove(0);
    state.record_listen(&track.path).unwrap();
    state.record_listen(&track.path).unwrap();
    let reopened = Catalog::new(state.path.clone());
    let home = reopened.home().unwrap();
    assert_eq!(home.recent.len(), 1);
    assert_eq!(home.recent[0].id, track.id);
    assert_eq!(home.artists[0], (track.artist, 2));
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn same_title_resolves_by_exact_path() {
    let dir = std::env::temp_dir().join(format!("mm-path-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let state = Catalog::new(dir.join("library.db"));
    let conn = state.connection().unwrap();
    for (path, album) in [("C:\\one.flac", "Album A"), ("C:\\two.flac", "Album B")] {
        conn.execute(
            "INSERT INTO tracks (path,title,artist,album,album_artist,genre,duration_ms,album_source,added_at)
             VALUES (?1,'Same Song','Same Artist',?2,'','',1000,'tag',1)",
            [path, album],
        )
        .unwrap();
    }
    drop(conn);
    let first = state.track_by_path("C:\\one.flac").unwrap().unwrap();
    let second = state.track_by_path("C:\\two.flac").unwrap().unwrap();
    assert_ne!(first.id, second.id);
    assert_eq!(first.title, second.title);
    assert_ne!(first.album, second.album);
    assert!(state.track_by_path("C:\\missing.flac").unwrap().is_none());
    assert!(state.track_by_path("").unwrap().is_none());
    let _ = std::fs::remove_dir_all(dir);
}

fn initialization_test_dir(label: &str) -> PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("mm-init-{label}-{}-{nonce}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn concurrent_first_access_initializes_once() {
    let dir = initialization_test_dir("concurrent");
    let catalog = std::sync::Arc::new(Catalog::new(dir.join("library.db")));
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(8));
    let workers: Vec<_> = (0..8)
        .map(|_| {
            let catalog = catalog.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                assert_eq!(catalog.query(Query::default()).unwrap().total, 0);
                assert_eq!(catalog.summary().unwrap().tracks, 0);
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
    assert_eq!(catalog.initialization_count.load(Ordering::SeqCst), 1);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn hot_connections_preserve_zero_added_at_and_configure_pragmas() {
    let dir = initialization_test_dir("hot");
    let catalog = Catalog::new(dir.join("library.db"));
    let conn = catalog.connection().unwrap();
    conn.execute("INSERT INTO tracks (path,title,artist,album,album_artist,genre,duration_ms,added_at) VALUES ('fixture.flac','Song','Artist','Album','','',1000,0)", []).unwrap();
    let id = conn.last_insert_rowid();
    drop(conn);
    for _ in 0..3 {
        assert_eq!(
            catalog.query(Query::default()).unwrap().tracks[0].added_at,
            0
        );
        assert_eq!(catalog.track(id).unwrap().added_at, 0);
        assert_eq!(catalog.tracks_by_ids(&[id]).unwrap()[0].added_at, 0);
        assert_eq!(catalog.summary().unwrap().tracks, 1);
        catalog.record_listen("fixture.flac").unwrap();
        assert_eq!(catalog.home().unwrap().recent[0].added_at, 0);
        assert_eq!(
            catalog
                .track_by_path("fixture.flac")
                .unwrap()
                .unwrap()
                .added_at,
            0
        );
        assert_eq!(catalog.track_present(id).unwrap().unwrap().added_at, 0);
    }
    assert_eq!(catalog.initialization_count.load(Ordering::SeqCst), 1);
    for conn in [
        catalog.connection().unwrap(),
        db::open_light(&catalog.path).unwrap(),
        db::open_read(&catalog.path).unwrap(),
        db::open(&catalog.path).unwrap(),
    ] {
        assert_eq!(
            conn.query_row("PRAGMA foreign_keys", [], |r| r.get::<_, i64>(0))
                .unwrap(),
            1
        );
        assert_eq!(
            conn.query_row("PRAGMA busy_timeout", [], |r| r.get::<_, i64>(0))
                .unwrap(),
            2000
        );
    }
    assert!(catalog.track(id).unwrap().added_at > 0);
    std::fs::remove_file(&catalog.path).unwrap();
    assert!(catalog.connection().is_err());
    assert!(!catalog.path.exists());
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn initialization_failure_is_retryable_and_poison_is_reported() {
    let dir = initialization_test_dir("retry");
    let parent = dir.join("blocked");
    std::fs::write(&parent, b"not a directory").unwrap();
    let catalog = std::sync::Arc::new(Catalog::new(parent.join("library.db")));
    assert!(catalog.connection().is_err());
    assert!(!*catalog.initialized.lock().unwrap());
    std::fs::remove_file(&parent).unwrap();
    assert_eq!(catalog.summary().unwrap().tracks, 0);
    assert!(*catalog.initialized.lock().unwrap());
    let poisoned = catalog.clone();
    assert!(std::thread::spawn(move || {
        let _guard = poisoned.initialized.lock().unwrap();
        panic!("intentional initialization lock poison");
    })
    .join()
    .is_err());
    let error = catalog.connection().expect_err("poison must fail");
    assert!(error.contains("初始化锁已毒化"));
    std::fs::remove_dir_all(dir).unwrap();
}
````

## File: src-tauri/src/catalog/collections.rs
````rust
//! User collections: favourites and custom playlists (V0.3).
//!
//! All state lives in SQLite next to the library, so it survives restarts and
//! database migrations. Deleting a playlist only removes rows in
//! `playlists` / `playlist_tracks`; the `tracks` table (and therefore the music
//! files on disk) is never touched.

use std::collections::HashSet;

use rusqlite::{params, Connection};

use super::db::now_secs;

// ------------------------------------------------------------------ favourites

pub fn favorite_ids(conn: &Connection) -> HashSet<i64> {
    let mut stmt = match conn.prepare("SELECT track_id FROM favorites") {
        Ok(s) => s,
        Err(_) => return HashSet::new(),
    };
    stmt.query_map([], |r| r.get::<_, i64>(0))
        .map(|rows| rows.flatten().collect())
        .unwrap_or_default()
}

#[cfg(test)]
pub fn is_favorite(conn: &Connection, track_id: i64) -> bool {
    conn.query_row(
        "SELECT 1 FROM favorites WHERE track_id = ?1",
        [track_id],
        |_| Ok(true),
    )
    .unwrap_or(false)
}

pub fn set_favorite(conn: &Connection, track_id: i64, on: bool) -> Result<(), String> {
    if on {
        conn.execute(
            "INSERT OR IGNORE INTO favorites (track_id, added_at) VALUES (?1, ?2)",
            params![track_id, now_secs()],
        )
        .map_err(|e| e.to_string())?;
    } else {
        conn.execute("DELETE FROM favorites WHERE track_id = ?1", [track_id])
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Returns the new state.
#[cfg(test)]
pub fn toggle_favorite(conn: &Connection, track_id: i64) -> Result<bool, String> {
    let now = is_favorite(conn, track_id);
    set_favorite(conn, track_id, !now)?;
    Ok(!now)
}

/// Favourite / un-favourite many songs in one transaction (one disk sync for a
/// selection of thousands).
#[cfg(test)]
pub fn set_favorites(conn: &Connection, ids: &[i64], on: bool) -> Result<usize, String> {
    if ids.is_empty() {
        return Ok(0);
    }
    let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
    let mut changed = 0usize;
    {
        let mut stmt = tx
            .prepare(if on {
                "INSERT OR IGNORE INTO favorites (track_id, added_at) VALUES (?1, ?2)"
            } else {
                "DELETE FROM favorites WHERE track_id = ?1"
            })
            .map_err(|e| e.to_string())?;
        let now = now_secs();
        for id in ids {
            let rows = if on {
                stmt.execute(params![id, now])
            } else {
                stmt.execute([id])
            }
            .map_err(|e| e.to_string())?;
            changed += rows;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;
    Ok(changed)
}

// ------------------------------------------------------------------- playlists

#[derive(Clone, Debug, PartialEq)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
    pub created_at: i64,
    pub track_count: usize,
}

pub fn list_playlists(conn: &Connection) -> Vec<Playlist> {
    let mut stmt = match conn.prepare(
        "SELECT p.id, p.name, p.created_at, COUNT(pt.track_id)
         FROM playlists p
         LEFT JOIN playlist_tracks pt ON pt.playlist_id = p.id
         GROUP BY p.id
         ORDER BY p.created_at, p.id",
    ) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };
    stmt.query_map([], |r| {
        Ok(Playlist {
            id: r.get(0)?,
            name: r.get(1)?,
            created_at: r.get(2)?,
            track_count: r.get::<_, i64>(3)? as usize,
        })
    })
    .map(|rows| rows.flatten().collect())
    .unwrap_or_default()
}

fn ensure_unique_name(conn: &Connection, name: &str, except: Option<i64>) -> Result<(), String> {
    let taken: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM playlists WHERE name = ?1 COLLATE NOCASE AND id IS NOT ?2)",
            params![name, except],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;
    if taken {
        return Err(format!("已存在名为「{name}」的歌单"));
    }
    Ok(())
}

pub fn create_playlist(conn: &Connection, name: &str) -> Result<i64, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("歌单名称不能为空".to_string());
    }
    ensure_unique_name(conn, name, None)?;
    conn.execute(
        "INSERT INTO playlists (name, created_at) VALUES (?1, ?2)",
        params![name, now_secs()],
    )
    .map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn rename_playlist(conn: &Connection, id: i64, name: &str) -> Result<(), String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("歌单名称不能为空".to_string());
    }
    ensure_unique_name(conn, name, Some(id))?;
    let changed = conn
        .execute(
            "UPDATE playlists SET name = ?1 WHERE id = ?2",
            params![name, id],
        )
        .map_err(|e| e.to_string())?;
    if changed == 0 {
        return Err("歌单不存在或已被删除".to_string());
    }
    Ok(())
}

/// Deletes the playlist and its membership rows. Tracks and files stay.
pub fn delete_playlist(conn: &Connection, id: i64) -> Result<(), String> {
    let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM playlist_tracks WHERE playlist_id = ?1", [id])
        .map_err(|e| e.to_string())?;
    let changed = tx
        .execute("DELETE FROM playlists WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    if changed == 0 {
        return Err("歌单不存在或已被删除".to_string());
    }
    tx.commit().map_err(|e| e.to_string())
}

pub fn playlist_track_ids(conn: &Connection, playlist_id: i64) -> Vec<i64> {
    let mut stmt = match conn
        .prepare("SELECT track_id FROM playlist_tracks WHERE playlist_id = ?1 ORDER BY position")
    {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };
    stmt.query_map([playlist_id], |r| r.get::<_, i64>(0))
        .map(|rows| rows.flatten().collect())
        .unwrap_or_default()
}

/// Adds a track at the end. Returns false when it was already in the playlist.
pub fn add_to_playlist(conn: &Connection, playlist_id: i64, track_id: i64) -> Result<bool, String> {
    let next_pos: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(position) + 1, 0) FROM playlist_tracks WHERE playlist_id = ?1",
            [playlist_id],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;
    let changed = conn
        .execute(
            "INSERT OR IGNORE INTO playlist_tracks (playlist_id, track_id, position, added_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![playlist_id, track_id, next_pos, now_secs()],
        )
        .map_err(|e| e.to_string())?;
    Ok(changed > 0)
}

/// Append many tracks in order, skipping those already in the playlist, in one
/// transaction. Returns how many were added.
#[cfg(test)]
pub fn add_many_to_playlist(
    conn: &Connection,
    playlist_id: i64,
    ids: &[i64],
) -> Result<usize, String> {
    if ids.is_empty() {
        return Ok(0);
    }
    let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
    let mut next_pos: i64 = tx
        .query_row(
            "SELECT COALESCE(MAX(position) + 1, 0) FROM playlist_tracks WHERE playlist_id = ?1",
            [playlist_id],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;
    let now = now_secs();
    let mut added = 0usize;
    {
        let mut stmt = tx
            .prepare(
                "INSERT OR IGNORE INTO playlist_tracks (playlist_id, track_id, position, added_at)
                 VALUES (?1, ?2, ?3, ?4)",
            )
            .map_err(|e| e.to_string())?;
        for id in ids {
            let rows = stmt
                .execute(params![playlist_id, id, next_pos, now])
                .map_err(|e| e.to_string())?;
            if rows > 0 {
                added += 1;
                next_pos += 1;
            }
        }
    }
    tx.commit().map_err(|e| e.to_string())?;
    Ok(added)
}

pub fn remove_from_playlist(
    conn: &Connection,
    playlist_id: i64,
    track_id: i64,
) -> Result<(), String> {
    let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
    tx.execute(
        "DELETE FROM playlist_tracks WHERE playlist_id = ?1 AND track_id = ?2",
        params![playlist_id, track_id],
    )
    .map_err(|e| e.to_string())?;
    renumber(&tx, playlist_id)?;
    tx.commit().map_err(|e| e.to_string())
}

/// Moves the entry at `from` to index `to` (clamped), renumbering positions.
#[cfg(test)]
pub fn move_in_playlist(
    conn: &Connection,
    playlist_id: i64,
    from: usize,
    to: usize,
) -> Result<(), String> {
    let ids = playlist_track_ids(conn, playlist_id);
    if from >= ids.len() {
        return Ok(());
    }
    let to = to.min(ids.len().saturating_sub(1));
    if from == to {
        return Ok(());
    }
    let mut order = ids;
    let moved = order.remove(from);
    order.insert(to, moved);
    let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
    for (pos, id) in order.iter().enumerate() {
        tx.execute(
            "UPDATE playlist_tracks SET position = ?1 WHERE playlist_id = ?2 AND track_id = ?3",
            params![pos as i64, playlist_id, id],
        )
        .map_err(|e| e.to_string())?;
    }
    tx.commit().map_err(|e| e.to_string())
}

fn renumber(conn: &Connection, playlist_id: i64) -> Result<(), String> {
    let ids = playlist_track_ids(conn, playlist_id);
    for (pos, id) in ids.iter().enumerate() {
        conn.execute(
            "UPDATE playlist_tracks SET position = ?1 WHERE playlist_id = ?2 AND track_id = ?3",
            params![pos as i64, playlist_id, id],
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::db;
    use super::super::library::{AlbumSource, Track};
    use super::*;

    fn seed_tracks(conn: &Connection, count: i64) -> Vec<i64> {
        let mut ids = Vec::new();
        for i in 0..count {
            let t = Track {
                id: 0,
                path: format!("C:\\Music\\{i}.flac"),
                title: format!("Song {i}"),
                artist: "Artist".into(),
                album: "Album".into(),
                album_artist: String::new(),
                genre: "Pop".into(),
                year: None,
                track_number: Some(i as u32 + 1),
                disc_number: None,
                duration_ms: 1000,
                album_source: AlbumSource::Tag,
                added_at: 0,
            };
            db::upsert_tracks(conn, &[t]).unwrap();
            ids.push(conn.last_insert_rowid());
        }
        ids
    }

    #[test]
    fn favorites_toggle_and_persist() {
        let conn = db::open_in_memory();
        let ids = seed_tracks(&conn, 3);
        assert!(!is_favorite(&conn, ids[0]));
        assert!(toggle_favorite(&conn, ids[0]).unwrap());
        assert!(is_favorite(&conn, ids[0]));
        assert_eq!(favorite_ids(&conn).len(), 1);
        assert!(!toggle_favorite(&conn, ids[0]).unwrap());
        assert!(favorite_ids(&conn).is_empty());

        set_favorite(&conn, ids[1], true).unwrap();
        set_favorite(&conn, ids[1], true).unwrap(); // idempotent
        assert_eq!(favorite_ids(&conn).len(), 1);
    }

    #[test]
    fn playlist_crud_and_order() {
        let conn = db::open_in_memory();
        let ids = seed_tracks(&conn, 4);
        let pl = create_playlist(&conn, "  夜晚  ").unwrap();
        assert_eq!(list_playlists(&conn)[0].name, "夜晚");

        for id in &ids[0..3] {
            assert!(add_to_playlist(&conn, pl, *id).unwrap());
        }
        // Adding twice does not duplicate.
        assert!(!add_to_playlist(&conn, pl, ids[0]).unwrap());
        assert_eq!(playlist_track_ids(&conn, pl), ids[0..3].to_vec());

        move_in_playlist(&conn, pl, 0, 2).unwrap();
        assert_eq!(playlist_track_ids(&conn, pl), vec![ids[1], ids[2], ids[0]]);

        remove_from_playlist(&conn, pl, ids[2]).unwrap();
        assert_eq!(playlist_track_ids(&conn, pl), vec![ids[1], ids[0]]);

        rename_playlist(&conn, pl, "新しい名前 🎵").unwrap();
        assert_eq!(list_playlists(&conn)[0].name, "新しい名前 🎵");
        assert_eq!(list_playlists(&conn)[0].track_count, 2);

        delete_playlist(&conn, pl).unwrap();
        assert!(list_playlists(&conn).is_empty());
        assert!(playlist_track_ids(&conn, pl).is_empty());
        // The library itself is untouched.
        assert_eq!(db::load_tracks(&conn).unwrap().len(), 4);
    }

    #[test]
    fn deleting_a_track_removes_it_from_collections() {
        let conn = db::open_in_memory();
        let ids = seed_tracks(&conn, 2);
        let pl = create_playlist(&conn, "P").unwrap();
        add_to_playlist(&conn, pl, ids[0]).unwrap();
        set_favorite(&conn, ids[0], true).unwrap();

        db::remove_folder(&conn, "C:\\Music").unwrap();
        assert_eq!(db::load_tracks(&conn).unwrap().len(), 0);
        assert!(playlist_track_ids(&conn, pl).is_empty());
        assert!(favorite_ids(&conn).is_empty());
    }

    #[test]
    fn batch_favourites_and_playlist_append() {
        let conn = db::open_in_memory();
        let ids = seed_tracks(&conn, 4);
        let added = set_favorites(&conn, &ids[0..3], true).unwrap();
        assert_eq!(added, 3);
        assert_eq!(favorite_ids(&conn).len(), 3);
        // Re-favouriting already-favourited rows changes nothing.
        assert_eq!(set_favorites(&conn, &ids[0..3], true).unwrap(), 0);
        assert_eq!(set_favorites(&conn, &ids[0..2], false).unwrap(), 2);
        assert_eq!(favorite_ids(&conn), [ids[2]].into_iter().collect());

        let pl = create_playlist(&conn, "批量").unwrap();
        assert_eq!(add_many_to_playlist(&conn, pl, &ids).unwrap(), 4);
        assert_eq!(
            add_many_to_playlist(&conn, pl, &ids[1..3]).unwrap(),
            0,
            "already in playlist"
        );
        assert_eq!(playlist_track_ids(&conn, pl), ids);
    }

    #[test]
    fn duplicate_names_and_missing_playlists_report_errors() {
        let conn = db::open_in_memory();
        let ids = seed_tracks(&conn, 2);
        let local = create_playlist(&conn, "本地").unwrap();
        let other = create_playlist(&conn, "Night").unwrap();
        assert!(create_playlist(&conn, " 本地 ").is_err());
        assert!(create_playlist(&conn, "night").is_err());
        assert!(rename_playlist(&conn, other, "本地").is_err());
        rename_playlist(&conn, other, "NIGHT").unwrap();
        add_to_playlist(&conn, local, ids[0]).unwrap();

        delete_playlist(&conn, local).unwrap();
        assert_eq!(list_playlists(&conn).len(), 1);
        assert!(delete_playlist(&conn, local).is_err());
        assert!(rename_playlist(&conn, local, "x").is_err());
        assert_eq!(db::load_tracks(&conn).unwrap().len(), 2);
        assert!(create_playlist(&conn, "本地").is_ok());
    }

    #[test]
    fn deleted_playlist_stays_deleted_after_reopening_the_database() {
        let path = std::env::temp_dir().join(format!(
            "music-mine-playlist-delete-{}.sqlite",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        {
            let conn = db::open(&path).unwrap();
            let ids = seed_tracks(&conn, 1);
            let pl = create_playlist(&conn, "本地").unwrap();
            add_to_playlist(&conn, pl, ids[0]).unwrap();
            delete_playlist(&conn, pl).unwrap();
        }
        let conn = db::open(&path).unwrap();
        assert!(list_playlists(&conn).is_empty());
        assert_eq!(db::load_tracks(&conn).unwrap().len(), 1);
        drop(conn);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn empty_playlist_names_are_rejected() {
        let conn = db::open_in_memory();
        assert!(create_playlist(&conn, "   ").is_err());
        let pl = create_playlist(&conn, "ok").unwrap();
        assert!(rename_playlist(&conn, pl, "").is_err());
        assert_eq!(list_playlists(&conn)[0].name, "ok");
    }
}
````

## File: src-tauri/src/catalog/db.rs
````rust
use std::path::Path;

use rusqlite::Connection;

use super::library::{AlbumSource, Track};

pub fn open(path: &Path) -> Result<Connection, String> {
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let conn = Connection::open(path).map_err(|e| e.to_string())?;
    configure_connection(&conn)?;
    init(&conn)?;
    Ok(conn)
}

fn configure_connection(conn: &Connection) -> Result<(), String> {
    conn.execute_batch("PRAGMA foreign_keys = ON;")
        .map_err(|e| e.to_string())?;
    conn.busy_timeout(std::time::Duration::from_secs(2))
        .map_err(|e| e.to_string())
}

/// Opens an initialized database without creating it or running migrations.
pub fn open_light(path: &Path) -> Result<Connection, String> {
    let conn = Connection::open_with_flags(path, rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE)
        .map_err(|e| e.to_string())?;
    configure_connection(&conn)?;
    Ok(conn)
}

/// Read-only connection for hot lookups: no schema checks and no write lock per call.
pub fn open_read(path: &Path) -> Result<Connection, String> {
    let flags =
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX;
    let conn = Connection::open_with_flags(path, flags).map_err(|e| e.to_string())?;
    configure_connection(&conn)?;
    Ok(conn)
}

#[cfg(test)]
pub fn open_in_memory() -> Connection {
    let conn = Connection::open_in_memory().expect("open in-memory db");
    configure_connection(&conn).expect("configure in-memory db");
    init(&conn).expect("init in-memory db");
    conn
}

fn init(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS tracks (
            id INTEGER PRIMARY KEY,
            path TEXT UNIQUE NOT NULL,
            title TEXT NOT NULL,
            artist TEXT NOT NULL,
            album TEXT NOT NULL,
            album_artist TEXT NOT NULL,
            genre TEXT NOT NULL,
            year INTEGER,
            track_number INTEGER,
            duration_ms INTEGER NOT NULL,
            disc_number INTEGER,
            album_source TEXT NOT NULL DEFAULT 'tag',
            added_at INTEGER NOT NULL DEFAULT 0
        );
        CREATE TABLE IF NOT EXISTS listening_history (track_id INTEGER PRIMARY KEY REFERENCES tracks(id) ON DELETE CASCADE, last_played INTEGER NOT NULL, plays INTEGER NOT NULL);
        CREATE TABLE IF NOT EXISTS folders (
            path TEXT PRIMARY KEY
        );
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS favorites (
            track_id INTEGER PRIMARY KEY REFERENCES tracks(id) ON DELETE CASCADE,
            added_at INTEGER NOT NULL
        );
        CREATE TABLE IF NOT EXISTS playlists (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            created_at INTEGER NOT NULL
        );
        CREATE TABLE IF NOT EXISTS playlist_tracks (
            playlist_id INTEGER NOT NULL REFERENCES playlists(id) ON DELETE CASCADE,
            track_id INTEGER NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,
            position INTEGER NOT NULL,
            added_at INTEGER NOT NULL,
            PRIMARY KEY (playlist_id, track_id)
        );
        CREATE INDEX IF NOT EXISTS idx_playlist_tracks ON playlist_tracks(playlist_id, position);",
    )
    .map_err(|e| e.to_string())?;
    // Migration for databases created before V0.2 / V0.3.
    ensure_column(conn, "tracks", "disc_number", "INTEGER")?;
    ensure_column(
        conn,
        "tracks",
        "album_source",
        "TEXT NOT NULL DEFAULT 'tag'",
    )?;
    ensure_column(conn, "tracks", "added_at", "INTEGER NOT NULL DEFAULT 0")?;
    // Existing libraries get a stable "added" time so sorting works.
    conn.execute(
        "UPDATE tracks SET added_at = strftime('%s','now') WHERE added_at = 0",
        [],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn ensure_column(conn: &Connection, table: &str, column: &str, decl: &str) -> Result<(), String> {
    let mut stmt = conn
        .prepare(&format!("PRAGMA table_info({table})"))
        .map_err(|e| e.to_string())?;
    let names: Vec<String> = stmt
        .query_map([], |r| r.get::<_, String>(1))
        .map_err(|e| e.to_string())?
        .flatten()
        .collect();
    if names.iter().any(|n| n.eq_ignore_ascii_case(column)) {
        return Ok(());
    }
    conn.execute(
        &format!("ALTER TABLE {table} ADD COLUMN {column} {decl}"),
        [],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_tracks(conn: &Connection) -> Result<Vec<Track>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, path, title, artist, album, album_artist, genre, year, track_number, duration_ms, disc_number, album_source, added_at
             FROM tracks
             ORDER BY artist COLLATE NOCASE, album COLLATE NOCASE, disc_number, track_number, title COLLATE NOCASE",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Track {
                id: row.get(0)?,
                path: row.get(1)?,
                title: row.get(2)?,
                artist: row.get(3)?,
                album: row.get(4)?,
                album_artist: row.get(5)?,
                genre: row.get(6)?,
                year: row.get::<_, Option<i64>>(7)?.map(|v| v as u32),
                track_number: row.get::<_, Option<i64>>(8)?.map(|v| v as u32),
                duration_ms: row.get::<_, i64>(9)? as u64,
                disc_number: row.get::<_, Option<i64>>(10)?.map(|v| v as u32),
                album_source: AlbumSource::parse(&row.get::<_, String>(11).unwrap_or_default()),
                added_at: row.get::<_, i64>(12).unwrap_or(0),
            })
        })
        .map_err(|e| e.to_string())?;
    let mut tracks = Vec::new();
    for r in rows {
        tracks.push(r.map_err(|e| e.to_string())?);
    }
    Ok(tracks)
}

pub fn upsert_tracks(conn: &Connection, tracks: &[Track]) -> Result<(), String> {
    let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
    for t in tracks {
        tx.execute(
            "INSERT INTO tracks (path, title, artist, album, album_artist, genre, year, track_number, duration_ms, disc_number, album_source, added_at)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)
             ON CONFLICT(path) DO UPDATE SET
               title=excluded.title, artist=excluded.artist, album=excluded.album,
               album_artist=excluded.album_artist, genre=excluded.genre, year=excluded.year,
               track_number=excluded.track_number, duration_ms=excluded.duration_ms,
               disc_number=excluded.disc_number, album_source=excluded.album_source",
            rusqlite::params![
                t.path,
                t.title,
                t.artist,
                t.album,
                t.album_artist,
                t.genre,
                t.year.map(|v| v as i64),
                t.track_number.map(|v| v as i64),
                t.duration_ms as i64,
                t.disc_number.map(|v| v as i64),
                t.album_source.as_str(),
                if t.added_at > 0 {
                    t.added_at
                } else {
                    now_secs()
                }
            ],
        )
        .map_err(|e| e.to_string())?;
    }
    tx.commit().map_err(|e| e.to_string())
}

pub fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub fn load_folders(conn: &Connection) -> Result<Vec<String>, String> {
    let mut stmt = conn
        .prepare("SELECT path FROM folders ORDER BY path")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |r| r.get::<_, String>(0))
        .map_err(|e| e.to_string())?;
    Ok(rows.filter_map(|r| r.ok()).collect())
}

pub fn add_folder(conn: &Connection, path: &str) -> Result<(), String> {
    conn.execute("INSERT OR IGNORE INTO folders (path) VALUES (?1)", [path])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
pub fn remove_folder(conn: &Connection, path: &str) -> Result<(), String> {
    conn.execute("DELETE FROM folders WHERE path = ?1", [path])
        .map_err(|e| e.to_string())?;
    let prefix = format!("{}\\", path.trim_end_matches(['\\', '/']));
    conn.execute(
        "DELETE FROM tracks WHERE path = ?1 OR path LIKE ?2",
        rusqlite::params![path, format!("{prefix}%")],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_setting(conn: &Connection, key: &str) -> Option<String> {
    conn.query_row("SELECT value FROM settings WHERE key = ?1", [key], |r| {
        r.get::<_, String>(0)
    })
    .ok()
}

#[cfg(test)]
pub fn set_setting(conn: &Connection, key: &str, value: &str) -> Result<(), String> {
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1,?2)
         ON CONFLICT(key) DO UPDATE SET value=excluded.value",
        [key, value],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::library::Track;
    use super::*;

    fn sample_track(path: &str, title: &str) -> Track {
        Track {
            id: 0,
            path: path.to_string(),
            title: title.to_string(),
            artist: "A".into(),
            album: "B".into(),
            album_artist: String::new(),
            genre: String::new(),
            year: Some(2000),
            track_number: Some(1),
            disc_number: Some(1),
            duration_ms: 123456,
            album_source: AlbumSource::Tag,
            added_at: 0,
        }
    }

    #[test]
    fn roundtrip_tracks_and_settings() {
        let conn = open_in_memory();
        upsert_tracks(
            &conn,
            &[
                sample_track("C:\\Music\\a.mp3", "First"),
                sample_track("C:\\Music\\b.flac", "Second"),
            ],
        )
        .unwrap();
        let loaded = load_tracks(&conn).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].title, "First");
        assert!(loaded[0].id != 0);

        // Upserting the same path updates rather than duplicates.
        upsert_tracks(&conn, &[sample_track("C:\\Music\\a.mp3", "FirstUpdated")]).unwrap();
        let reloaded = load_tracks(&conn).unwrap();
        assert_eq!(reloaded.len(), 2);
        assert!(reloaded.iter().any(|t| t.title == "FirstUpdated"));

        set_setting(&conn, "volume", "0.5").unwrap();
        assert_eq!(get_setting(&conn, "volume").as_deref(), Some("0.5"));

        add_folder(&conn, "C:\\Music").unwrap();
        assert_eq!(load_folders(&conn).unwrap(), vec!["C:\\Music".to_string()]);
        remove_folder(&conn, "C:\\Music").unwrap();
        assert!(load_folders(&conn).unwrap().is_empty());
    }
}

#[cfg(test)]
mod migration_tests {
    use super::*;

    /// A V0.2 database must keep every track when opened by V0.3: new tables
    /// appear, new columns are added, and existing rows survive untouched.
    #[test]
    fn upgrading_a_v02_database_preserves_the_library() {
        let dir = std::env::temp_dir().join("music-mine-tests-migration");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join(format!("v02-{}.db", std::process::id()));
        let _ = std::fs::remove_file(&path);
        {
            let conn = Connection::open(&path).unwrap();
            conn.execute_batch(
                "CREATE TABLE tracks (
                    id INTEGER PRIMARY KEY, path TEXT UNIQUE NOT NULL, title TEXT NOT NULL,
                    artist TEXT NOT NULL, album TEXT NOT NULL, album_artist TEXT NOT NULL,
                    genre TEXT NOT NULL, year INTEGER, track_number INTEGER,
                    duration_ms INTEGER NOT NULL, disc_number INTEGER,
                    album_source TEXT NOT NULL DEFAULT 'tag');
                 CREATE TABLE folders (path TEXT PRIMARY KEY);
                 CREATE TABLE settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);",
            )
            .unwrap();
            conn.execute(
                "INSERT INTO tracks (path,title,artist,album,album_artist,genre,year,track_number,duration_ms,disc_number,album_source)
                 VALUES ('C:\\Music\\a.flac','Song','Artist','Album','','Pop',2001,1,1234,NULL,'tag')",
                [],
            )
            .unwrap();
            conn.execute(
                "INSERT INTO settings (key,value) VALUES ('session_v2','{\"queue\":[]}')",
                [],
            )
            .unwrap();
        }

        let conn = open(&path).unwrap();
        let tracks = load_tracks(&conn).unwrap();
        assert_eq!(tracks.len(), 1, "existing library rows must survive");
        assert_eq!(tracks[0].title, "Song");
        assert!(tracks[0].added_at > 0, "migration assigns an added time");
        assert_eq!(
            get_setting(&conn, "session_v2").as_deref(),
            Some("{\"queue\":[]}")
        );
        // The new tables exist and work.
        let favorites: i64 = conn
            .query_row("SELECT COUNT(*) FROM favorites", [], |r| r.get(0))
            .unwrap();
        let playlists: i64 = conn
            .query_row("SELECT COUNT(*) FROM playlists", [], |r| r.get(0))
            .unwrap();
        assert_eq!((favorites, playlists), (0, 0));
    }
}

#[cfg(test)]
mod scan_tests {
    use super::super::library;
    use super::super::metadata;

    #[test]
    fn scans_a_wav_and_reads_its_duration() {
        let dir = std::env::temp_dir().join("music-mine-tests-scan");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("nested")).unwrap();
        let wav = dir.join("nested").join("tone.wav");
        std::fs::copy(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tone.wav"),
            &wav,
        )
        .unwrap();
        // Files we don't support must be ignored.
        std::fs::write(dir.join("notes.txt"), b"hello").unwrap();

        let tracks = library::scan_folder(&dir);
        assert_eq!(tracks.len(), 1, "expected only the wav file");
        let t = &tracks[0];
        assert_eq!(t.title, "tone", "falls back to the file name");
        assert!(
            t.duration_ms >= 2900 && t.duration_ms <= 3100,
            "duration {}",
            t.duration_ms
        );
        assert!(metadata::read_cover(std::path::Path::new(&t.path)).is_none());
    }
}
````

## File: src-tauri/src/catalog/library.rs
````rust
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[cfg(test)]
use super::metadata;

/// Where a track's album value came from. Grouping must not treat a folder
/// guess the same way as a real album tag.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlbumSource {
    /// Read from the file's album tag.
    Tag,
    /// Tag missing; the containing folder was used (it holds several songs).
    Folder,
    /// Tag missing and the folder had only this song.
    Unknown,
}

impl AlbumSource {
    pub fn as_str(self) -> &'static str {
        match self {
            AlbumSource::Tag => "tag",
            AlbumSource::Folder => "folder",
            AlbumSource::Unknown => "unknown",
        }
    }
    pub fn parse(s: &str) -> AlbumSource {
        match s {
            "folder" => AlbumSource::Folder,
            "unknown" => AlbumSource::Unknown,
            _ => AlbumSource::Tag,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Track {
    pub id: i64,
    pub path: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
    pub genre: String,
    pub year: Option<u32>,
    pub track_number: Option<u32>,
    pub disc_number: Option<u32>,
    pub duration_ms: u64,
    pub album_source: AlbumSource,
    /// Unix time the track first entered the library (0 = assigned on insert).
    pub added_at: i64,
}

#[derive(Default)]
#[allow(dead_code)]
pub struct Library {
    pub tracks: Vec<Track>,
    pub folders: Vec<String>,
}

const EXTENSIONS: &[&str] = &["mp3", "flac", "wav"];

/// Recursively scan a folder for supported audio files and read their tags.
/// Returns tracks with id = 0 (the database assigns real ids on reload).
#[cfg(test)]
pub fn scan_folder(folder: &Path) -> Vec<Track> {
    let mut out = Vec::new();
    walk(folder, &mut out);
    fill_album_fallbacks(&mut out);
    sort_tracks(&mut out);
    out
}

#[cfg(test)]
fn walk(dir: &Path, out: &mut Vec<Track>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, out);
        } else if is_supported(&path) {
            if let Some(t) = metadata::read_track(&path, 0) {
                out.push(t);
            }
        }
    }
}

/// Files without an album tag are grouped by their containing folder when that
/// folder holds several songs (the usual "one folder per album" layout). A
/// folder with a single song would otherwise create a fake album per track, so
/// those land in one "未知专辑" bucket instead.
pub fn fill_album_fallbacks(tracks: &mut [Track]) {
    let mut per_folder: HashMap<PathBuf, usize> = HashMap::new();
    for t in tracks.iter() {
        if let Some(parent) = Path::new(&t.path).parent() {
            *per_folder.entry(parent.to_path_buf()).or_default() += 1;
        }
    }
    for t in tracks.iter_mut() {
        if t.album.trim().is_empty() {
            let parent = Path::new(&t.path).parent().map(|p| p.to_path_buf());
            let count = parent
                .as_ref()
                .and_then(|p| per_folder.get(p).copied())
                .unwrap_or(1);
            let folder_name = parent
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|s| s.to_str())
                .map(|s| s.trim().to_string());
            match (count >= 2, folder_name) {
                (true, Some(name)) if !name.is_empty() => {
                    t.album = name;
                    t.album_source = AlbumSource::Folder;
                }
                _ => {
                    t.album = "未知专辑".to_string();
                    t.album_source = AlbumSource::Unknown;
                }
            }
        }
    }
}

/// Case/space/punctuation-insensitive form used for comparing album names.
/// Keeps distinct titles distinct; only folds cosmetic differences.
pub fn normalize_key(s: &str) -> String {
    let mut out = String::new();
    let mut last_space = false;
    for ch in s.trim().to_lowercase().chars() {
        let c = match ch {
            '’' | '‘' | '`' | '´' => '\'',
            '“' | '”' => '"',
            '（' => '(',
            '）' => ')',
            '：' => ':',
            '，' => ',',
            '、' => ',',
            '　' => ' ',
            _ => ch,
        };
        if c.is_whitespace() {
            if !last_space {
                out.push(' ');
            }
            last_space = true;
        } else {
            out.push(c);
            last_space = false;
        }
    }
    out
}

/// The album name a track is grouped under. Empty album tags become a single
/// "未知专辑" group rather than one album per song.
pub fn album_name(t: &Track) -> String {
    let album = t.album.trim();
    if album.is_empty() {
        "未知专辑".to_string()
    } else {
        album.to_string()
    }
}

pub const VARIOUS_ARTISTS: &str = "Various Artists";

/// One album as the UI shows it: a name, a performing artist label and the
/// tracks in play order.
#[derive(Clone, Debug)]
pub struct AlbumGroup {
    /// Stable identity for selection; derived from name + artist label.
    pub key: String,
    pub name: String,
    pub artist: String,
    #[cfg_attr(not(test), allow(dead_code))]
    pub is_compilation: bool,
    #[allow(dead_code)]
    pub source: AlbumSource,
    /// Track indices (into the slice passed to `group_albums`), play order.
    pub tracks: Vec<usize>,
    #[cfg_attr(not(test), allow(dead_code))]
    pub disc_count: usize,
}

/// Album grouping rules, driven by real metadata:
///
/// * A real album tag groups by album name (ignoring case/space/punctuation)
///   **plus** the album-artist value, so two different artists' same-named
///   albums stay apart.
/// * No album-artist tag: one shared artist labels the album; several artists
///   mean a compilation, labelled "Various Artists" — unless the album name is
///   also a song title, which marks same-named singles (e.g. covers) that must
///   stay separate per artist.
/// * An album guessed from a folder groups by that folder, never mixed with a
///   same-named tag album elsewhere.
/// * Untagged single files share one "未知专辑" group.
pub fn group_albums(tracks: &[Track]) -> Vec<AlbumGroup> {
    use std::collections::BTreeMap;

    // Bucket 1: what the metadata says the album is.
    let mut buckets: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, t) in tracks.iter().enumerate() {
        let base = match t.album_source {
            AlbumSource::Tag => format!("tag:{}", normalize_key(&album_name(t))),
            AlbumSource::Folder => {
                let parent = std::path::Path::new(&t.path)
                    .parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();
                format!("folder:{}", normalize_key(&parent))
            }
            AlbumSource::Unknown => "untagged".to_string(),
        };
        buckets.entry(base).or_default().push(i);
    }

    let mut out: Vec<AlbumGroup> = Vec::new();
    for (base, indices) in buckets {
        let source = tracks[indices[0]].album_source;
        let name = album_name(&tracks[indices[0]]);

        let mut album_artists: Vec<String> = Vec::new();
        let mut artists: Vec<String> = Vec::new();
        for &i in &indices {
            let aa = tracks[i].album_artist.trim();
            if !aa.is_empty()
                && !album_artists
                    .iter()
                    .any(|x| normalize_key(x) == normalize_key(aa))
            {
                album_artists.push(aa.to_string());
            }
            let ar = tracks[i].artist.trim();
            if !ar.is_empty()
                && !artists
                    .iter()
                    .any(|x| normalize_key(x) == normalize_key(ar))
            {
                artists.push(ar.to_string());
            }
        }

        // Same-named singles: the album name is also one of the song titles.
        let name_is_a_title = indices
            .iter()
            .any(|&i| normalize_key(&tracks[i].title) == normalize_key(&name));

        let single_label: Option<String> = match source {
            AlbumSource::Folder => Some(if artists.len() == 1 {
                artists[0].clone()
            } else if artists.is_empty() {
                "未知艺术家".to_string()
            } else {
                VARIOUS_ARTISTS.to_string()
            }),
            AlbumSource::Unknown => Some(if artists.len() == 1 {
                artists[0].clone()
            } else if artists.is_empty() {
                "未知艺术家".to_string()
            } else {
                "多位艺术家".to_string()
            }),
            AlbumSource::Tag => {
                if album_artists.len() == 1 {
                    Some(album_artists[0].clone())
                } else if album_artists.len() > 1 {
                    // Explicitly different album artists: two albums that happen
                    // to share a name. Split, never merge into a fake compilation.
                    None
                } else if artists.len() == 1 {
                    Some(artists[0].clone())
                } else if artists.is_empty() {
                    Some("未知艺术家".to_string())
                } else if name_is_a_title {
                    None // same-named singles: split per artist below
                } else {
                    Some(VARIOUS_ARTISTS.to_string())
                }
            }
        };

        match single_label {
            Some(label) => out.push(make_group(&base, name, label, source, indices, tracks)),
            None => {
                // Split same-named singles per performing artist.
                let mut by_artist: BTreeMap<String, (String, Vec<usize>)> = BTreeMap::new();
                for &i in &indices {
                    let label = {
                        let aa = tracks[i].album_artist.trim();
                        if !aa.is_empty() {
                            aa.to_string()
                        } else if !tracks[i].artist.trim().is_empty() {
                            tracks[i].artist.trim().to_string()
                        } else {
                            "未知艺术家".to_string()
                        }
                    };
                    by_artist
                        .entry(normalize_key(&label))
                        .or_insert_with(|| (label, Vec::new()))
                        .1
                        .push(i);
                }
                for (_, (label, idx)) in by_artist {
                    out.push(make_group(&base, name.clone(), label, source, idx, tracks));
                }
            }
        }
    }

    out.sort_by(|a, b| {
        a.name
            .to_lowercase()
            .cmp(&b.name.to_lowercase())
            .then(a.artist.to_lowercase().cmp(&b.artist.to_lowercase()))
    });
    out
}

fn make_group(
    base: &str,
    name: String,
    artist: String,
    source: AlbumSource,
    mut indices: Vec<usize>,
    tracks: &[Track],
) -> AlbumGroup {
    indices.sort_by_key(|&i| {
        (
            tracks[i].disc_number.unwrap_or(1),
            tracks[i].track_number.unwrap_or(u32::MAX),
            tracks[i].title.to_lowercase(),
        )
    });
    let disc_count = {
        let mut discs: Vec<u32> = indices
            .iter()
            .filter_map(|&i| tracks[i].disc_number)
            .collect();
        discs.sort_unstable();
        discs.dedup();
        discs.len().max(1)
    };
    let is_compilation = artist == VARIOUS_ARTISTS;
    AlbumGroup {
        key: format!("{base}|{}", normalize_key(&artist)),
        name,
        artist,
        is_compilation,
        source,
        tracks: indices,
        disc_count,
    }
}

/// The performer shown for an album: the album-artist tag when present, the
/// single shared artist when all tracks agree, otherwise "多位艺术家".
#[cfg(test)]
pub fn album_artist_label(album_artist: &str, artists: &[String]) -> String {
    if !album_artist.trim().is_empty() {
        return album_artist.trim().to_string();
    }
    match artists.len() {
        0 => "未知艺术家".to_string(),
        1 => artists[0].clone(),
        _ => "多位艺术家".to_string(),
    }
}

/// Sortable fields offered by the song list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg(test)]
pub enum SortKey {
    Title,
    Artist,
    Album,
    Duration,
    AddedAt,
}

/// Sort track indices in place. Kept pure so the ordering rules are testable.
#[cfg(test)]
pub fn sort_track_indices(tracks: &[Track], indices: &mut [usize], key: SortKey, ascending: bool) {
    indices.sort_by(|a, b| {
        let (ta, tb) = (&tracks[*a], &tracks[*b]);
        let ord = match key {
            SortKey::Title => ta
                .title
                .to_lowercase()
                .cmp(&tb.title.to_lowercase())
                .then(ta.artist.to_lowercase().cmp(&tb.artist.to_lowercase())),
            SortKey::Artist => ta
                .artist
                .to_lowercase()
                .cmp(&tb.artist.to_lowercase())
                .then(ta.album.to_lowercase().cmp(&tb.album.to_lowercase()))
                .then(ta.disc_number.cmp(&tb.disc_number))
                .then(ta.track_number.cmp(&tb.track_number)),
            SortKey::Album => ta
                .album
                .to_lowercase()
                .cmp(&tb.album.to_lowercase())
                .then(ta.disc_number.cmp(&tb.disc_number))
                .then(ta.track_number.cmp(&tb.track_number)),
            SortKey::Duration => ta.duration_ms.cmp(&tb.duration_ms),
            SortKey::AddedAt => ta.added_at.cmp(&tb.added_at),
        };
        if ascending {
            ord
        } else {
            ord.reverse()
        }
    });
}

#[cfg(test)]
pub fn sort_tracks(tracks: &mut [Track]) {
    tracks.sort_by(|a, b| {
        a.artist
            .to_lowercase()
            .cmp(&b.artist.to_lowercase())
            .then(a.album.to_lowercase().cmp(&b.album.to_lowercase()))
            .then(a.disc_number.cmp(&b.disc_number))
            .then(a.track_number.cmp(&b.track_number))
            .then(a.title.to_lowercase().cmp(&b.title.to_lowercase()))
    });
}

pub fn is_supported(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| EXTENSIONS.contains(&e.to_ascii_lowercase().as_str()))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn track(path: &str, album: &str, artist: &str) -> Track {
        Track {
            id: 0,
            path: path.into(),
            title: "t".into(),
            artist: artist.into(),
            album: album.into(),
            album_artist: String::new(),
            genre: String::new(),
            year: None,
            track_number: None,
            disc_number: None,
            duration_ms: 1000,
            album_source: if album.is_empty() {
                AlbumSource::Unknown
            } else {
                AlbumSource::Tag
            },
            added_at: 0,
        }
    }

    fn titled(mut t: Track, title: &str) -> Track {
        t.title = title.into();
        t
    }

    #[test]
    fn album_artist_label_prefers_the_tag_then_the_shared_artist() {
        assert_eq!(
            album_artist_label("Band", &["A".into(), "B".into()]),
            "Band"
        );
        assert_eq!(album_artist_label("", &["A".into()]), "A");
        assert_eq!(
            album_artist_label("", &["A".into(), "B".into()]),
            "多位艺术家"
        );
        assert_eq!(album_artist_label("", &[]), "未知艺术家");
    }

    #[test]
    fn untagged_albums_use_folder_only_when_it_holds_several_tracks() {
        let mut tracks = vec![
            track("D:\\A\\01.wav", "", "X"),
            track("D:\\A\\02.wav", "", "X"),
            track("D:\\Singles\\only.wav", "", "Y"),
            track("D:\\Tagged\\03.wav", "Real Album", "Z"),
        ];
        fill_album_fallbacks(&mut tracks);
        assert_eq!(tracks[0].album, "A");
        assert_eq!(tracks[0].album_source, AlbumSource::Folder);
        assert_eq!(tracks[1].album, "A");
        assert_eq!(tracks[2].album, "未知专辑");
        assert_eq!(tracks[2].album_source, AlbumSource::Unknown);
        assert_eq!(tracks[3].album, "Real Album");
        assert_eq!(tracks[3].album_source, AlbumSource::Tag);
    }

    #[test]
    fn a_compilation_becomes_one_various_artists_album() {
        let tracks = vec![
            titled(
                track("D:\\OST\\1.flac", "Some Soundtrack", "Singer A"),
                "Opening",
            ),
            titled(
                track("D:\\OST\\2.flac", "Some Soundtrack", "Singer B"),
                "Theme",
            ),
            titled(
                track("D:\\OST\\3.flac", "Some Soundtrack", "Singer C"),
                "Ending",
            ),
        ];
        let groups = group_albums(&tracks);
        assert_eq!(groups.len(), 1, "{groups:?}");
        assert_eq!(groups[0].artist, VARIOUS_ARTISTS);
        assert!(groups[0].is_compilation);
        assert_eq!(groups[0].tracks.len(), 3);
    }

    #[test]
    fn same_named_singles_stay_separate_per_artist() {
        // The album name is also the song title: two covers, not one album.
        let tracks = vec![
            titled(track("D:\\1.flac", "牵丝戏", "银临"), "牵丝戏"),
            titled(track("D:\\2.flac", "牵丝戏", "黄龄"), "牵丝戏 (梦幻西游版)"),
        ];
        let groups = group_albums(&tracks);
        assert_eq!(groups.len(), 2, "{groups:?}");
    }

    #[test]
    fn same_album_name_with_different_album_artists_stays_apart() {
        let mut a = titled(track("D:\\1.flac", "Greatest Hits", "Band A"), "One");
        a.album_artist = "Band A".into();
        let mut b = titled(track("D:\\2.flac", "Greatest Hits", "Band B"), "Two");
        b.album_artist = "Band B".into();
        let groups = group_albums(&[a, b]);
        assert_eq!(groups.len(), 2, "{groups:?}");
    }

    #[test]
    fn album_artist_tag_keeps_a_compilation_together() {
        let mut a = titled(track("D:\\1.flac", "Comp", "Singer A"), "One");
        a.album_artist = "Various Artists".into();
        let mut b = titled(track("D:\\2.flac", "Comp", "Singer B"), "Two");
        b.album_artist = "Various Artists".into();
        let groups = group_albums(&[a, b]);
        assert_eq!(groups.len(), 1, "{groups:?}");
        assert!(groups[0].is_compilation);
    }

    #[test]
    fn cosmetic_album_differences_do_not_split_an_album() {
        let tracks = vec![
            titled(track("D:\\1.flac", "LOVE  Story", "Band"), "One"),
            titled(track("D:\\2.flac", " love story", "Band"), "Two"),
        ];
        let groups = group_albums(&tracks);
        assert_eq!(groups.len(), 1, "{groups:?}");
        assert_eq!(groups[0].tracks.len(), 2);
    }

    #[test]
    fn folder_albums_do_not_merge_with_same_named_tag_albums() {
        let mut folder_track = titled(track("D:\\Music\\Holiday\\01.flac", "", "X"), "One");
        folder_track.album = "Holiday".into();
        folder_track.album_source = AlbumSource::Folder;
        let tag_track = titled(track("D:\\Other\\2.flac", "Holiday", "Y"), "Two");
        let groups = group_albums(&[folder_track, tag_track]);
        assert_eq!(groups.len(), 2, "{groups:?}");
    }

    #[test]
    fn untagged_single_files_share_one_unknown_album_group() {
        let tracks = vec![
            titled(track("D:\\S1\\a.wav", "", "A"), "One"),
            titled(track("D:\\S2\\b.wav", "", "B"), "Two"),
        ];
        let groups = group_albums(&tracks);
        assert_eq!(groups.len(), 1, "{groups:?}");
        assert_eq!(groups[0].name, "未知专辑");
    }

    #[test]
    fn sorting_orders_by_each_field_in_both_directions() {
        let mut a = titled(track("D:\\1.flac", "A", "Beta"), "Zebra");
        a.duration_ms = 3000;
        a.added_at = 20;
        let mut b = titled(track("D:\\2.flac", "A", "Alpha"), "apple");
        b.duration_ms = 1000;
        b.added_at = 10;
        let mut c = titled(track("D:\\3.flac", "A", "Gamma"), "Mango");
        c.duration_ms = 2000;
        c.added_at = 30;
        let tracks = vec![a, b, c];

        let order = |key, asc| {
            let mut idx: Vec<usize> = (0..tracks.len()).collect();
            sort_track_indices(&tracks, &mut idx, key, asc);
            idx
        };
        assert_eq!(order(SortKey::Title, true), vec![1, 2, 0]); // apple, Mango, Zebra
        assert_eq!(order(SortKey::Title, false), vec![0, 2, 1]);
        assert_eq!(order(SortKey::Artist, true), vec![1, 0, 2]); // Alpha, Beta, Gamma
        assert_eq!(order(SortKey::Duration, true), vec![1, 2, 0]);
        assert_eq!(order(SortKey::Duration, false), vec![0, 2, 1]);
        assert_eq!(order(SortKey::AddedAt, true), vec![1, 0, 2]);
        assert_eq!(order(SortKey::AddedAt, false), vec![2, 0, 1]);
        assert_eq!(order(SortKey::Album, true), vec![0, 1, 2]); // same album: stable
    }

    #[test]
    fn multi_disc_albums_order_by_disc_then_track() {
        let mut t1 = titled(track("D:\\1.flac", "Big", "Band"), "Disc2 Track1");
        t1.disc_number = Some(2);
        t1.track_number = Some(1);
        let mut t2 = titled(track("D:\\2.flac", "Big", "Band"), "Disc1 Track2");
        t2.disc_number = Some(1);
        t2.track_number = Some(2);
        let mut t3 = titled(track("D:\\3.flac", "Big", "Band"), "Disc1 Track1");
        t3.disc_number = Some(1);
        t3.track_number = Some(1);
        let tracks = vec![t1, t2, t3];
        let groups = group_albums(&tracks);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].disc_count, 2);
        let order: Vec<&str> = groups[0]
            .tracks
            .iter()
            .map(|&i| tracks[i].title.as_str())
            .collect();
        assert_eq!(order, vec!["Disc1 Track1", "Disc1 Track2", "Disc2 Track1"]);
    }
}
````

## File: src-tauri/src/catalog/metadata.rs
````rust
use std::path::Path;

use lofty::file::{AudioFile, TaggedFileExt};
use lofty::tag::{Accessor, ItemKey};

use super::library::{AlbumSource, Track};

fn clean<S: Into<String>>(value: Option<S>) -> Option<String> {
    value
        .map(|s| s.into().trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Read metadata from an audio file using lofty. Tags missing from the primary
/// tag are looked up in every other tag the file carries, because some files
/// keep title/album in a secondary tag (e.g. ID3v1 next to ID3v2, or RIFF INFO
/// in a WAV).
pub fn read_track(path: &Path, id: i64) -> Option<Track> {
    let tagged = lofty::read_from_path(path).ok()?;
    let duration_ms = tagged.properties().duration().as_millis() as u64;

    let mut title = None;
    let mut artist = None;
    let mut album = None;
    let mut album_artist = None;
    let mut genre = None;
    let mut year = None;
    let mut track_number = None;
    let mut disc_number = None;

    // Primary tag first, then any remaining tag as a fallback.
    let primary = tagged.primary_tag().map(|t| t as *const _);
    let mut tags: Vec<&lofty::tag::Tag> = Vec::new();
    if let Some(p) = tagged.primary_tag() {
        tags.push(p);
    }
    for t in tagged.tags() {
        if Some(t as *const _) != primary {
            tags.push(t);
        }
    }

    for t in tags {
        if title.is_none() {
            title = clean(t.title());
        }
        if artist.is_none() {
            artist = clean(t.artist());
        }
        if album.is_none() {
            album = clean(t.album());
        }
        if album_artist.is_none() {
            album_artist = clean(t.get_string(&ItemKey::AlbumArtist));
        }
        if genre.is_none() {
            genre = clean(t.genre());
        }
        if year.is_none() {
            year = t.year();
        }
        if track_number.is_none() {
            track_number = t.track();
        }
        if disc_number.is_none() {
            disc_number = t.disk();
        }
    }

    // Untagged files fall back to the file name (and its leading track number).
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("未知歌曲");
    let (file_title, file_track_no) = split_track_number(stem);

    let has_album_tag = album.is_some();
    Some(Track {
        id,
        path: path.to_string_lossy().to_string(),
        title: title.unwrap_or(file_title),
        artist: artist.unwrap_or_else(|| "未知艺术家".to_string()),
        // Album stays empty when the file has no album tag; the scanner decides
        // whether the containing folder is a better guess.
        album: album.unwrap_or_default(),
        album_artist: album_artist.unwrap_or_default(),
        genre: genre.unwrap_or_default(),
        year,
        track_number: track_number.or(file_track_no),
        disc_number,
        duration_ms,
        album_source: if has_album_tag {
            AlbumSource::Tag
        } else {
            AlbumSource::Unknown
        },
        // Filled in by the database on first insert, preserved on rescans.
        added_at: 0,
    })
}

/// Split a leading track number off a file name: "01 - Title", "02. Title",
/// "03 Title" and "04_Title" all yield (title, Some(number)).
fn split_track_number(stem: &str) -> (String, Option<u32>) {
    let s = stem.trim();
    let digits: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() || digits.len() > 3 {
        return (s.to_string(), None);
    }
    let rest = &s[digits.len()..];
    let trimmed = rest
        .trim_start_matches([' ', '-', '.', '_', '　'])
        .trim_start();
    let number = digits.parse::<u32>().ok();
    if trimmed.is_empty() {
        return (s.to_string(), number);
    }
    (trimmed.to_string(), number)
}

/// Extract the embedded cover image (bytes) from a file's tags.
pub fn read_cover(path: &Path) -> Option<Vec<u8>> {
    read_cover_checked(path).ok().flatten()
}

/// Why tags could not be read. `Io` can succeed on a later attempt; `Tags` cannot.
pub enum CoverReadError {
    Io,
    Tags,
}

/// Front cover first, then any other non-empty picture. `Ok(None)` means the tags have no picture.
pub fn read_cover_checked(path: &Path) -> Result<Option<Vec<u8>>, CoverReadError> {
    let tagged = lofty::read_from_path(path).map_err(|e| match e.kind() {
        lofty::error::ErrorKind::Io(_) => CoverReadError::Io,
        _ => CoverReadError::Tags,
    })?;
    let pictures = || {
        tagged
            .tags()
            .iter()
            .flat_map(|tag| tag.pictures())
            .filter(|p| !p.data().is_empty())
    };
    let front = pictures().find(|p| p.pic_type() == lofty::picture::PictureType::CoverFront);
    Ok(front
        .or_else(|| pictures().next())
        .map(|p| p.data().to_vec()))
}

#[cfg(test)]
mod tests {
    use super::split_track_number;

    #[test]
    fn splits_track_numbers() {
        assert_eq!(split_track_number("01 - Night"), ("Night".into(), Some(1)));
        assert_eq!(split_track_number("02. Sea"), ("Sea".into(), Some(2)));
        assert_eq!(split_track_number("03 Song"), ("Song".into(), Some(3)));
        assert_eq!(split_track_number("04_Song"), ("Song".into(), Some(4)));
        assert_eq!(split_track_number("Song"), ("Song".into(), None));
        // Four leading digits are a year, not a track number.
        assert_eq!(split_track_number("1984"), ("1984".into(), None));
        assert_eq!(split_track_number("1000 days"), ("1000 days".into(), None));
    }
}
````

## File: src-tauri/src/catalog/mod.rs
````rust
pub(crate) mod collections;
mod db;
pub(crate) mod library;
pub(crate) mod metadata;
use library::Track;
const FIELDS: &str = "t.id,t.path,t.title,t.artist,t.album,t.album_artist,t.genre,t.year,t.track_number,t.duration_ms,t.disc_number,t.album_source,t.added_at";
fn row_track(r: &rusqlite::Row<'_>) -> rusqlite::Result<Track> {
    Ok(Track {
        id: r.get(0)?,
        path: r.get(1)?,
        title: r.get(2)?,
        artist: r.get(3)?,
        album: r.get(4)?,
        album_artist: r.get(5)?,
        genre: r.get(6)?,
        year: r.get(7)?,
        track_number: r.get(8)?,
        duration_ms: r.get(9)?,
        disc_number: r.get(10)?,
        album_source: library::AlbumSource::parse(&r.get::<_, String>(11)?),
        added_at: r.get(12)?,
    })
}
// Compare stable legacy data before/after migrating the backup, including membership order.
fn legacy_snapshot(conn: &Connection) -> Result<Vec<Vec<Vec<String>>>, String> {
    let mut snapshot = Vec::new();
    for (table, fields) in [
        (
            "tracks",
            "id,path,title,artist,album,album_artist,genre,year,track_number,duration_ms",
        ),
        ("favorites", "*"),
        ("playlists", "*"),
        ("playlist_tracks", "*"),
        ("folders", "*"),
        ("settings", "*"),
    ] {
        let exists: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name=?1)",
                [table],
                |r| r.get(0),
            )
            .map_err(|e| e.to_string())?;
        if !exists {
            if table == "tracks" {
                return Err("所选文件不是支持的旧版音乐库".into());
            }
            snapshot.push(vec![]);
            continue;
        }
        let mut statement = conn
            .prepare(&format!("SELECT {fields} FROM {table} ORDER BY 1"))
            .map_err(|e| e.to_string())?;
        let columns = statement.column_count();
        let mut rows = statement.query([]).map_err(|e| e.to_string())?;
        let mut values = Vec::new();
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            values.push(
                (0..columns)
                    .map(|i| row.get_ref(i).map(|v| format!("{v:?}")))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| e.to_string())?,
            );
        }
        values.sort();
        snapshot.push(values);
    }
    Ok(snapshot)
}
use rusqlite::{params, Connection, OpenFlags};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
};

pub struct Catalog {
    initialized: Mutex<bool>,
    #[cfg(test)]
    initialization_count: std::sync::atomic::AtomicUsize,
    pub path: PathBuf,
    pub progress: Mutex<Progress>,
    pub cancel: AtomicBool,
    pub operation: Mutex<()>,
}
#[derive(Clone, Default, Serialize)]
pub struct Progress {
    pub running: bool,
    pub visited: usize,
    pub imported: usize,
    pub current: String,
    pub cancelled: bool,
    pub errors: Vec<String>,
}
#[derive(Deserialize)]
#[serde(default)]
pub struct Query {
    pub search: String,
    pub sort: String,
    pub offset: usize,
    pub limit: usize,
    pub favorite: bool,
    pub playlist: Option<i64>,
    pub album: Option<String>,
    pub artist: Option<String>,
}
impl Default for Query {
    fn default() -> Self {
        Self {
            search: String::new(),
            sort: "added".into(),
            offset: 0,
            limit: 100,
            favorite: false,
            playlist: None,
            album: None,
            artist: None,
        }
    }
}
#[derive(Serialize)]
pub struct Home {
    pub recent: Vec<Track>,
    pub artists: Vec<(String, i64)>,
}
#[derive(Serialize)]
pub struct Page {
    pub tracks: Vec<Track>,
    pub total: usize,
    pub favorites: Vec<i64>,
}
#[derive(Serialize)]
pub struct Group {
    pub key: String,
    pub name: String,
    pub artist: String,
    pub count: usize,
    pub cover_id: i64,
}
#[derive(Serialize)]
pub struct Summary {
    pub tracks: usize,
    pub albums: Vec<Group>,
    pub artists: Vec<Group>,
    pub folders: Vec<String>,
    pub playlists: Vec<Playlist>,
}
#[derive(Serialize)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
    pub count: usize,
}
#[derive(Serialize, PartialEq, Debug)]
pub struct ImportReport {
    pub tracks: i64,
    pub favorites: i64,
    pub playlists: i64,
    pub entries: i64,
    pub warnings: Vec<String>,
}
impl Catalog {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            initialized: Mutex::new(false),
            #[cfg(test)]
            initialization_count: std::sync::atomic::AtomicUsize::new(0),
            progress: Mutex::new(Progress::default()),
            cancel: AtomicBool::new(false),
            operation: Mutex::new(()),
        }
    }
    fn ensure_initialized(&self) -> Result<(), String> {
        let mut initialized = self
            .initialized
            .lock()
            .map_err(|_| "曲库初始化锁已毒化，无法安全初始化".to_string())?;
        if !*initialized {
            #[cfg(test)]
            self.initialization_count.fetch_add(1, Ordering::SeqCst);
            drop(db::open(&self.path)?);
            *initialized = true;
        }
        Ok(())
    }
    pub fn connection(&self) -> Result<Connection, String> {
        self.ensure_initialized()?;
        db::open_light(&self.path)
    }
    pub fn record_listen(&self, path: &str) -> Result<(), String> {
        self.connection()?.execute("INSERT INTO listening_history(track_id,last_played,plays) SELECT id,strftime('%s','now'),1 FROM tracks WHERE path=?1 ON CONFLICT(track_id) DO UPDATE SET last_played=excluded.last_played,plays=plays+1", [path]).map_err(|e|e.to_string())?;
        Ok(())
    }
    pub fn home(&self) -> Result<Home, String> {
        let conn = self.connection()?;
        let mut stmt=conn.prepare(&format!("SELECT {FIELDS} FROM tracks t JOIN listening_history h ON h.track_id=t.id ORDER BY h.last_played DESC,t.id DESC LIMIT 12")).map_err(|e|e.to_string())?;
        let recent = stmt
            .query_map([], row_track)
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        let mut stmt=conn.prepare("SELECT t.artist,SUM(h.plays) FROM tracks t JOIN listening_history h ON h.track_id=t.id GROUP BY t.artist ORDER BY SUM(h.plays) DESC,t.artist LIMIT 8").map_err(|e|e.to_string())?;
        let artists = stmt
            .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(Home { recent, artists })
    }
    pub fn track(&self, id: i64) -> Result<Track, String> {
        self.connection()?
            .query_row(
                &format!("SELECT {FIELDS} FROM tracks t WHERE t.id=?1"),
                [id],
                row_track,
            )
            .map_err(|e| format!("读取歌曲失败：{e}"))
    }
    /// A missing row is `Ok(None)`. Database failures stay errors.
    pub fn track_present(&self, id: i64) -> Result<Option<Track>, String> {
        self.lookup(&format!("SELECT {FIELDS} FROM tracks t WHERE t.id=?1"), &id)
            .map_err(|e| format!("读取歌曲失败：{e}"))
    }
    /// Exact library path. Titles are never consulted. Unknown paths are `Ok(None)`.
    pub fn track_by_path(&self, path: &str) -> Result<Option<Track>, String> {
        if path.is_empty() || path.len() > 1024 {
            return Ok(None);
        }
        self.lookup(
            &format!("SELECT {FIELDS} FROM tracks t WHERE t.path=?1"),
            &path,
        )
    }
    /// One-row lookup after initialization, with a lightweight read-write fallback.
    fn lookup(&self, sql: &str, param: &dyn rusqlite::ToSql) -> Result<Option<Track>, String> {
        self.ensure_initialized()?;
        let run = |conn: &Connection| match conn.query_row(sql, [param], row_track) {
            Ok(track) => Ok(Some(track)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        };
        if let Ok(conn) = db::open_read(&self.path) {
            if let Ok(found) = run(&conn) {
                return Ok(found);
            }
        }
        run(&db::open_light(&self.path)?).map_err(|e| e.to_string())
    }
    /// Loads tracks in the given order over one connection, skipping unknown IDs.
    pub fn tracks_by_ids(&self, ids: &[i64]) -> Result<Vec<Track>, String> {
        let conn = self.connection()?;
        let mut stmt = conn
            .prepare(&format!("SELECT {FIELDS} FROM tracks t WHERE t.id=?1"))
            .map_err(|e| e.to_string())?;
        let mut tracks = Vec::with_capacity(ids.len());
        for id in ids {
            match stmt.query_row([id], row_track) {
                Ok(t) => tracks.push(t),
                Err(rusqlite::Error::QueryReturnedNoRows) => {}
                Err(e) => return Err(e.to_string()),
            }
        }
        Ok(tracks)
    }
    pub fn query(&self, q: Query) -> Result<Page, String> {
        let conn = self.connection()?;
        // All user values are bound parameters; LIKE metacharacters are literal search text.
        let pattern = format!(
            "%{}%",
            q.search
                .replace('!', "!!")
                .replace('%', "!%")
                .replace('_', "!_")
        );
        let order = match q.sort.as_str() {
            "title" => "t.title COLLATE NOCASE,t.id",
            "artist" => "t.artist COLLATE NOCASE,t.id",
            "album" => "t.album COLLATE NOCASE,t.disc_number,t.track_number,t.id",
            "duration" => "t.duration_ms,t.id",
            _ => "t.added_at DESC,t.id DESC",
        };
        let filter = "FROM tracks t WHERE (t.title LIKE ?1 ESCAPE '!' OR t.artist LIKE ?1 ESCAPE '!' OR t.album LIKE ?1 ESCAPE '!') AND (?2=0 OR EXISTS(SELECT 1 FROM favorites f WHERE f.track_id=t.id)) AND (?3 IS NULL OR EXISTS(SELECT 1 FROM playlist_tracks p WHERE p.track_id=t.id AND p.playlist_id=?3)) AND (?4 IS NULL OR t.id IN (SELECT value FROM json_each(?4))) AND (?5 IS NULL OR t.artist=?5)";
        let tracks = if q.album.is_some() {
            db::load_tracks(&conn)?
        } else {
            Vec::new()
        };
        let album_ids = q.album.as_ref().map(|key| {
            let indices = library::group_albums(&tracks)
                .into_iter()
                .find(|g| &g.key == key)
                .map(|g| g.tracks)
                .unwrap_or_default();
            serde_json::to_string(
                &indices
                    .into_iter()
                    .map(|i| tracks[i].id)
                    .collect::<Vec<_>>(),
            )
            .unwrap()
        });
        let total = conn
            .query_row(
                &format!("SELECT COUNT(*) {filter}"),
                params![pattern, q.favorite, q.playlist, album_ids, q.artist],
                |r| r.get::<_, usize>(0),
            )
            .map_err(|e| e.to_string())?;
        let order = if q.playlist.is_some() {
            "(SELECT position FROM playlist_tracks p WHERE p.track_id=t.id AND p.playlist_id=?3)"
        } else {
            order
        };
        let mut stmt = conn
            .prepare(&format!(
                "SELECT {FIELDS} {filter} ORDER BY {order} LIMIT ?6 OFFSET ?7"
            ))
            .map_err(|e| e.to_string())?;
        let tracks = stmt
            .query_map(
                params![
                    pattern,
                    q.favorite,
                    q.playlist,
                    album_ids,
                    q.artist,
                    q.limit.clamp(1, 200),
                    q.offset.min(i64::MAX as usize)
                ],
                row_track,
            )
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(Page {
            tracks,
            total,
            favorites: collections::favorite_ids(&conn).into_iter().collect(),
        })
    }
    pub fn summary(&self) -> Result<Summary, String> {
        let conn = self.connection()?;
        let tracks = db::load_tracks(&conn)?;
        let albums = library::group_albums(&tracks)
            .into_iter()
            .map(|g| Group {
                key: g.key,
                name: g.name,
                artist: g.artist,
                count: g.tracks.len(),
                cover_id: tracks[g.tracks[0]].id,
            })
            .collect();
        let mut artists: std::collections::BTreeMap<String, Vec<&Track>> = Default::default();
        for t in &tracks {
            artists.entry(t.artist.clone()).or_default().push(t);
        }
        Ok(Summary {
            tracks: tracks.len(),
            albums,
            artists: artists
                .into_iter()
                .map(|(name, ts)| Group {
                    key: name.clone(),
                    name,
                    artist: String::new(),
                    count: ts.len(),
                    cover_id: ts[0].id,
                })
                .collect(),
            folders: db::load_folders(&conn)?,
            playlists: collections::list_playlists(&conn)
                .into_iter()
                .map(|p| Playlist {
                    id: p.id,
                    name: p.name,
                    count: p.track_count,
                })
                .collect(),
        })
    }
    fn error(&self, value: String) {
        let mut p = self.progress.lock().unwrap();
        if p.errors.len() < 100 {
            p.errors.push(value);
        }
    }
    pub fn scan(&self, folder: PathBuf) -> Result<(), String> {
        let _guard = self.operation.lock().map_err(|e| e.to_string())?;
        let conn = self.connection()?;
        if folder.is_dir() {
            db::add_folder(&conn, &folder.to_string_lossy())?;
        }
        let mut dirs = vec![folder];
        while let Some(dir) = dirs.pop() {
            if self.cancel.load(Ordering::Relaxed) {
                self.progress.lock().unwrap().cancelled = true;
                break;
            }
            let entries = match std::fs::read_dir(&dir) {
                Ok(e) => e,
                Err(e) => {
                    self.error(format!("{}：{e}", dir.display()));
                    continue;
                }
            };
            let mut batch = Vec::new();
            for entry in entries {
                if self.cancel.load(Ordering::Relaxed) {
                    self.progress.lock().unwrap().cancelled = true;
                    break;
                }
                let entry = match entry {
                    Ok(e) => e,
                    Err(e) => {
                        self.error(e.to_string());
                        continue;
                    }
                };
                let path = entry.path();
                let ty = match entry.file_type() {
                    Ok(t) => t,
                    Err(e) => {
                        self.error(format!("{}：{e}", path.display()));
                        continue;
                    }
                };
                // Never traverse links/junctions, avoiding recursive cycles and duplicate imports.
                if ty.is_symlink() {
                    continue;
                }
                if ty.is_dir() {
                    dirs.push(path);
                    continue;
                }
                if !library::is_supported(&path) {
                    continue;
                }
                {
                    let mut p = self.progress.lock().unwrap();
                    p.visited += 1;
                    p.current = path.to_string_lossy().into_owned();
                }
                match metadata::read_track(&path, 0) {
                    Some(t) => batch.push(t),
                    None => self.error(format!("无法读取元数据：{}", path.display())),
                }
            }
            library::fill_album_fallbacks(&mut batch);
            db::upsert_tracks(&conn, &batch)?;
            self.progress.lock().unwrap().imported += batch.len();
        }
        Ok(())
    }
    pub fn import_copy(&self, source: &Path) -> Result<ImportReport, String> {
        let _guard = self
            .operation
            .try_lock()
            .map_err(|_| "扫描或导入正在进行")?;
        let conn = self.connection()?;
        for table in ["tracks", "favorites", "playlists", "settings", "folders"] {
            let count: i64 = conn
                .query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |r| r.get(0))
                .map_err(|e| e.to_string())?;
            if count > 0 {
                return Err("仅可导入到空曲库；现有新版数据不会被覆盖".into());
            }
        }
        drop(conn);
        let src = Connection::open_with_flags(source, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .map_err(|e| e.to_string())?;
        let temp = self.path.with_extension("importing.db");
        if temp.exists() {
            return Err("存在未完成的导入副本，请先检查 importing.db".into());
        }
        let result = (|| {
            let mut dest = Connection::open(&temp).map_err(|e| e.to_string())?;
            {
                let backup =
                    rusqlite::backup::Backup::new(&src, &mut dest).map_err(|e| e.to_string())?;
                backup
                    .run_to_completion(128, std::time::Duration::from_millis(5), None)
                    .map_err(|e| e.to_string())?;
            }
            let integrity: String = dest
                .query_row("PRAGMA integrity_check", [], |r| r.get(0))
                .map_err(|e| e.to_string())?;
            if integrity != "ok" {
                return Err(format!("旧库完整性检查失败：{integrity}"));
            }
            let before = legacy_snapshot(&dest)?;
            drop(dest);
            // Only the consistent backup is migrated, never the source connection.
            let dest = db::open(&temp)?;
            if legacy_snapshot(&dest)? != before {
                return Err("迁移前后歌曲、收藏、歌单顺序或设置不一致，已拒绝导入".into());
            }
            let count = |table: &str| {
                dest.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |r| {
                    r.get::<_, i64>(0)
                })
                .map_err(|e| e.to_string())
            };
            let mut report = ImportReport {
                tracks: count("tracks")?,
                favorites: count("favorites")?,
                playlists: count("playlists")?,
                entries: count("playlist_tracks")?,
                warnings: vec![],
            };
            db::load_tracks(&dest)?;
            let bad: i64 = dest
                .query_row("SELECT COUNT(*) FROM pragma_foreign_key_check", [], |r| {
                    r.get(0)
                })
                .map_err(|e| e.to_string())?;
            if bad != 0 {
                return Err(format!(
                    "旧库有 {bad} 个无效关联，副本已拒绝导入；原库保持不变"
                ));
            }
            if db::get_setting(&dest, "session_v2").is_some() {
                report.warnings.push("旧版播放队列、循环/随机模式和上次播放位置已原样保留在 session_v2，尚未自动恢复；不会静默丢弃。".into());
            }
            dest.execute_batch("PRAGMA wal_checkpoint(TRUNCATE); PRAGMA journal_mode=DELETE;")
                .map_err(|e| e.to_string())?;
            drop(dest);
            std::fs::rename(&temp, &self.path).map_err(|e| e.to_string())?;
            Ok(report)
        })();
        if result.is_err() {
            let _ = std::fs::remove_file(&temp);
        }
        result
    }
}
#[cfg(test)]
#[path = "catalog_tests.rs"]
mod tests;
````

## File: src-tauri/src/commands/catalog.rs
````rust
use crate::catalog::{collections, metadata, *};
use crate::{execute, Action, PlayerSnapshot, Shared};
use base64::Engine;
use std::sync::{atomic::Ordering, Arc};
type State<'a> = tauri::State<'a, Arc<Catalog>>;
async fn work<T: Send + 'static>(
    catalog: Arc<Catalog>,
    f: impl FnOnce(&Catalog) -> Result<T, String> + Send + 'static,
) -> Result<T, String> {
    tauri::async_runtime::spawn_blocking(move || f(&catalog))
        .await
        .map_err(|e| e.to_string())?
}
#[tauri::command]
pub async fn library_query(query: Query, state: State<'_>) -> Result<Page, String> {
    work(state.inner().clone(), move |s| s.query(query)).await
}
#[tauri::command]
pub async fn library_summary(state: State<'_>) -> Result<Summary, String> {
    work(state.inner().clone(), |s| s.summary()).await
}
#[tauri::command]
pub fn library_progress(state: State<'_>) -> Progress {
    state.progress.lock().unwrap().clone()
}
#[tauri::command]
pub fn library_cancel(state: State<'_>) {
    state.cancel.store(true, Ordering::Relaxed);
}
#[tauri::command]
pub async fn library_scan(
    state: State<'_>,
    covers: tauri::State<'_, Arc<crate::artwork::covers::Store>>,
) -> Result<(), String> {
    let folder = tauri::async_runtime::spawn_blocking(|| rfd::FileDialog::new().pick_folder())
        .await
        .map_err(|e| e.to_string())?;
    let Some(folder) = folder else {
        return Ok(());
    };
    let catalog = state.inner().clone();
    let covers = covers.inner().clone();
    {
        let mut p = catalog.progress.lock().unwrap();
        if p.running {
            return Err("扫描已经在进行".into());
        }
        *p = Progress {
            running: true,
            ..Default::default()
        };
    }
    catalog.cancel.store(false, Ordering::Relaxed);
    covers.clear_memory();
    tauri::async_runtime::spawn_blocking(move || {
        let result = catalog.scan(folder);
        let mut p = catalog.progress.lock().unwrap();
        p.running = false;
        if let Err(e) = result {
            p.errors.push(e);
        }
    });
    Ok(())
}
#[tauri::command]
pub async fn library_track(
    id: i64,
    state: State<'_>,
) -> Result<crate::catalog::library::Track, String> {
    work(state.inner().clone(), move |s| s.track(id)).await
}
#[derive(serde::Serialize)]
pub struct AudioInfo {
    format: String,
    sample_rate: Option<u32>,
    channels: Option<u8>,
    bitrate: Option<u32>,
}
/// Read-only file inspection; no library schema changes or audio engine work.
#[tauri::command]
pub async fn library_audio_info(id: i64, state: State<'_>) -> Result<AudioInfo, String> {
    work(state.inner().clone(), move |s| {
        let track = s.track(id)?;
        read_audio_info(std::path::Path::new(&track.path))
    })
    .await
}
fn read_audio_info(path: &std::path::Path) -> Result<AudioInfo, String> {
    use lofty::file::{AudioFile, TaggedFileExt};
    let file = lofty::read_from_path(path).map_err(|e| e.to_string())?;
    let properties = file.properties();
    Ok(AudioInfo {
        format: format!("{:?}", file.file_type()),
        sample_rate: properties.sample_rate().filter(|v| *v > 0),
        channels: properties.channels().filter(|v| *v > 0),
        bitrate: properties.audio_bitrate().filter(|v| *v > 0),
    })
}
#[cfg(test)]
mod audio_info_tests {
    use super::*;
    #[test]
    fn reads_real_properties_without_changing_fixture() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tone.wav");
        let before = std::fs::read(&path).unwrap();
        let info = read_audio_info(&path).unwrap();
        assert_eq!(info.format, "Wav");
        let rate = u32::from_le_bytes(before[24..28].try_into().unwrap());
        let channels = u16::from_le_bytes(before[22..24].try_into().unwrap());
        assert_eq!(info.sample_rate, Some(rate));
        assert_eq!(info.channels, Some(channels as u8));
        assert_eq!(std::fs::read(&path).unwrap(), before);
        assert!(read_audio_info(&path.with_extension("missing")).is_err());
    }
}
#[tauri::command]
pub async fn library_cover(id: i64, state: State<'_>) -> Result<Option<String>, String> {
    work(state.inner().clone(), move |s| {
        let t = s.track(id)?;
        Ok(
            metadata::read_cover(std::path::Path::new(&t.path)).and_then(|bytes| {
                let mime = if bytes.starts_with(b"\x89PNG") {
                    "image/png"
                } else if bytes.starts_with(b"\xff\xd8") {
                    "image/jpeg"
                } else {
                    return None;
                };
                if bytes.len() > 8 * 1024 * 1024 {
                    return None;
                }
                Some(format!(
                    "data:{mime};base64,{}",
                    base64::engine::general_purpose::STANDARD.encode(bytes)
                ))
            }),
        )
    })
    .await
}
#[tauri::command]
pub async fn library_thumb(
    id: i64,
    edge: u32,
    state: State<'_>,
    covers: tauri::State<'_, Arc<crate::artwork::covers::Store>>,
) -> Result<Option<String>, String> {
    let catalog = state.inner().clone();
    let covers = covers.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let Some(track) = catalog.track_present(id)? else {
            return Ok(None);
        };
        covers
            .for_track(track.id, std::path::Path::new(&track.path), edge)
            .map_err(|e| e.code().to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}
#[tauri::command]
pub async fn library_queue_thumb(
    path: String,
    edge: u32,
    state: State<'_>,
    covers: tauri::State<'_, Arc<crate::artwork::covers::Store>>,
) -> Result<Option<String>, String> {
    let catalog = state.inner().clone();
    let covers = covers.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let Some(track) = catalog.track_by_path(&path)? else {
            return Err("cover-not-in-library".to_string());
        };
        covers
            .for_track(track.id, std::path::Path::new(&track.path), edge)
            .map_err(|e| e.code().to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}
/// Cover cache counters for diagnostics. Counts and durations only, no paths or image data.
#[tauri::command]
pub fn library_cover_stats(
    covers: tauri::State<'_, Arc<crate::artwork::covers::Store>>,
) -> serde_json::Value {
    covers.stats.snapshot()
}
#[tauri::command]
pub async fn library_play(
    id: i64,
    state: State<'_>,
    player: tauri::State<'_, Shared>,
) -> Result<PlayerSnapshot, String> {
    let track = work(state.inner().clone(), move |s| s.track(id)).await?;
    execute(player.inner().clone(), Action::Start(track.path.into())).await
}
#[tauri::command]
pub async fn library_queue(
    action: String,
    id: Option<i64>,
    path: Option<String>,
    state: State<'_>,
    player: tauri::State<'_, Shared>,
) -> Result<PlayerSnapshot, String> {
    let action = match action.as_str() {
        "next" => Action::Next,
        "previous" => Action::Previous,
        "clear" => Action::ClearUpcoming,
        "remove" => Action::Remove(path.ok_or("缺少歌曲路径")?),
        "add" | "play_next" => {
            let t = work(state.inner().clone(), move |s| {
                s.track(id.ok_or("缺少歌曲 ID")?)
            })
            .await?;
            let track = queue_track(t);
            if action == "play_next" {
                Action::PlayNext(track)
            } else {
                Action::Enqueue(track)
            }
        }
        _ => return Err("不支持的队列操作".into()),
    };
    execute(player.inner().clone(), action).await
}
fn queue_track(t: crate::catalog::library::Track) -> crate::playback::controller::Track {
    crate::playback::controller::Track {
        path: t.path,
        title: t.title,
        artist: t.artist,
        album: t.album,
        duration_ms: t.duration_ms,
    }
}
/// Plays (replacing the queue) or appends a list of library tracks, e.g. an album or playlist.
#[tauri::command]
pub async fn library_queue_tracks(
    ids: Vec<i64>,
    replace: bool,
    state: State<'_>,
    player: tauri::State<'_, Shared>,
) -> Result<PlayerSnapshot, String> {
    if ids.len() > 20_000 {
        return Err("一次加入的歌曲过多".into());
    }
    let tracks = work(state.inner().clone(), move |s| s.tracks_by_ids(&ids)).await?;
    if tracks.is_empty() {
        return Err("没有可播放的歌曲".into());
    }
    let tracks = tracks.into_iter().map(queue_track).collect();
    let action = if replace {
        Action::Replace(tracks)
    } else {
        Action::EnqueueMany(tracks)
    };
    execute(player.inner().clone(), action).await
}
#[tauri::command]
pub async fn library_favorite(id: i64, on: bool, state: State<'_>) -> Result<(), String> {
    work(state.inner().clone(), move |s| {
        let _guard = s.operation.lock().map_err(|e| e.to_string())?;
        collections::set_favorite(&s.connection()?, id, on)
    })
    .await
}
#[tauri::command]
pub async fn library_playlist(
    name: Option<String>,
    playlist_id: Option<i64>,
    track_id: Option<i64>,
    state: State<'_>,
) -> Result<Option<i64>, String> {
    work(state.inner().clone(), move |s| {
        let _guard = s.operation.lock().map_err(|e| e.to_string())?;
        let conn = s.connection()?;
        if let Some(name) = name {
            return collections::create_playlist(&conn, &name).map(Some);
        } else if let (Some(p), Some(t)) = (playlist_id, track_id) {
            collections::add_to_playlist(&conn, p, t)?;
        } else {
            return Err("请选择歌单和歌曲".into());
        }
        Ok(None)
    })
    .await
}
/// Rename, delete or remove a track from a playlist by its ID. Only SQLite rows change;
/// library tracks and audio files are never touched.
#[tauri::command]
pub async fn library_playlist_edit(
    action: String,
    playlist_id: i64,
    name: Option<String>,
    track_id: Option<i64>,
    state: State<'_>,
) -> Result<(), String> {
    work(state.inner().clone(), move |s| {
        let _guard = s.operation.lock().map_err(|e| e.to_string())?;
        let conn = s.connection()?;
        match action.as_str() {
            "rename" => {
                collections::rename_playlist(&conn, playlist_id, &name.ok_or("缺少歌单名称")?)
            }
            "delete" => collections::delete_playlist(&conn, playlist_id),
            "remove_track" => collections::remove_from_playlist(
                &conn,
                playlist_id,
                track_id.ok_or("缺少歌曲 ID")?,
            ),
            _ => Err("不支持的歌单操作".into()),
        }
    })
    .await
}
#[tauri::command]
pub async fn library_import(state: State<'_>) -> Result<Option<ImportReport>, String> {
    let file = tauri::async_runtime::spawn_blocking(|| {
        rfd::FileDialog::new()
            .set_title("选择旧版 SQLite 数据库（只读安全备份）")
            .add_filter("SQLite", &["db", "sqlite", "sqlite3"])
            .pick_file()
    })
    .await
    .map_err(|e| e.to_string())?;
    match file {
        Some(file) => {
            work(state.inner().clone(), move |s| {
                s.import_copy(&file).map(Some)
            })
            .await
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn library_home(state: State<'_>) -> Result<Home, String> {
    work(state.inner().clone(), |s| s.home()).await
}
````

## File: src-tauri/src/commands/mod.rs
````rust
pub(crate) mod catalog;
````

## File: src-tauri/src/infrastructure/logging.rs
````rust
use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_LINES: usize = 1000;

/// A small, shared log: every line goes to stderr, to a file in the app's data
/// folder, and to an in-memory buffer the UI can show.
pub struct LogBuffer {
    lines: Mutex<VecDeque<String>>,
    file: Mutex<Option<File>>,
}

impl LogBuffer {
    pub fn new(path: Option<PathBuf>) -> LogBuffer {
        let file = path.and_then(|p| {
            if let Some(dir) = p.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            OpenOptions::new().create(true).append(true).open(p).ok()
        });
        LogBuffer {
            lines: Mutex::new(VecDeque::new()),
            file: Mutex::new(file),
        }
    }

    pub fn push(&self, message: impl AsRef<str>) {
        let line = format!("[{}] {}", timestamp(), message.as_ref());
        eprintln!("{line}");
        if let Ok(mut f) = self.file.lock() {
            if let Some(file) = f.as_mut() {
                let _ = writeln!(file, "{line}");
            }
        }
        if let Ok(mut lines) = self.lines.lock() {
            lines.push_back(line);
            while lines.len() > MAX_LINES {
                lines.pop_front();
            }
        }
    }

    pub fn lines(&self) -> Vec<String> {
        self.lines
            .lock()
            .map(|l| l.iter().cloned().collect())
            .unwrap_or_default()
    }
}

fn timestamp() -> String {
    // UTC wall clock, second resolution: enough to correlate a session.
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let day = secs % 86_400;
    format!("{:02}:{:02}:{:02}", day / 3600, (day % 3600) / 60, day % 60)
}
````

## File: src-tauri/src/infrastructure/mod.rs
````rust
pub(crate) mod logging;
````

## File: src-tauri/src/platform/desktop.rs
````rust
use crate::{playback::controller::Action, Shared};
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Preferences {
    pub close_to_tray: bool,
    pub queue_open: bool,
    /// "dark", "light" or "system"; files written before themes existed read as dark.
    pub theme: String,
    bounds: Option<(i32, i32, u32, u32)>,
    maximized: bool,
}
/// Native title bar theme; `None` lets Windows follow the system setting.
fn window_theme(theme: &str) -> Option<tauri::Theme> {
    match theme {
        "light" => Some(tauri::Theme::Light),
        "system" => None,
        _ => Some(tauri::Theme::Dark),
    }
}
fn window_background(theme: tauri::Theme) -> tauri::window::Color {
    match theme {
        tauri::Theme::Light => tauri::window::Color(0xf3, 0xf5, 0xf1, 0xff),
        _ => tauri::window::Color(0x12, 0x14, 0x14, 0xff),
    }
}
pub struct Desktop {
    preferences: Mutex<Preferences>,
    path: std::path::PathBuf,
    pub exiting: AtomicBool,
}
impl Desktop {
    fn save(&self, value: &Preferences) -> Result<(), String> {
        let data = serde_json::to_vec(value).map_err(|e| e.to_string())?;
        let temp = self.path.with_extension("tmp");
        std::fs::write(&temp, data).map_err(|e| e.to_string())?;
        std::fs::rename(temp, &self.path).map_err(|e| e.to_string())
    }
}
#[tauri::command]
pub fn desktop_preferences(
    app: tauri::AppHandle,
    close_to_tray: Option<bool>,
    queue_open: Option<bool>,
    theme: Option<String>,
) -> Result<Preferences, String> {
    let state = app.state::<Desktop>();
    let mut guard = state.preferences.lock().map_err(|e| e.to_string())?;
    let mut next = guard.clone();
    if let Some(v) = close_to_tray {
        next.close_to_tray = v;
    }
    if let Some(v) = queue_open {
        next.queue_open = v;
    }
    if let Some(v) = theme {
        if !matches!(v.as_str(), "dark" | "light" | "system") {
            return Err(format!("未知的外观主题: {v}"));
        }
        next.theme = v;
    }
    state.save(&next)?;
    *guard = next.clone();
    Ok(next)
}
// Require the entire window to fit one monitor's work area; disconnected monitors
// and changed DPI therefore fall back to the current primary work area.
fn fits(bounds: (i32, i32, u32, u32), area: (i32, i32, u32, u32)) -> bool {
    let (x, y, w, h) = bounds;
    let (ax, ay, aw, ah) = area;
    w >= 320
        && h >= 240
        && x >= ax
        && y >= ay
        && i64::from(x) + i64::from(w) <= i64::from(ax) + i64::from(aw)
        && i64::from(y) + i64::from(h) <= i64::from(ay) + i64::from(ah)
}
pub fn ensure_visible(window: &tauri::WebviewWindow) -> tauri::Result<()> {
    let pos = window.outer_position()?;
    let size = window.outer_size()?;
    let monitors = window.available_monitors()?;
    if window.is_maximized()? {
        let center_x = i64::from(pos.x) + i64::from(size.width) / 2;
        let center_y = i64::from(pos.y) + i64::from(size.height) / 2;
        if monitors.iter().any(|m| {
            let a = m.work_area();
            center_x >= i64::from(a.position.x)
                && center_x < i64::from(a.position.x) + i64::from(a.size.width)
                && center_y >= i64::from(a.position.y)
                && center_y < i64::from(a.position.y) + i64::from(a.size.height)
        }) {
            return Ok(());
        }
        window.unmaximize()?;
        if let Some(m) = window.primary_monitor()? {
            window.set_position(m.work_area().position)?;
        }
        return window.maximize();
    }
    if !monitors.iter().any(|m| {
        let a = m.work_area();
        fits(
            (pos.x, pos.y, size.width, size.height),
            (a.position.x, a.position.y, a.size.width, a.size.height),
        )
    }) {
        if let Some(m) = window.primary_monitor()? {
            let a = m.work_area();
            let inner = window.inner_size()?;
            let frame_width = size.width.saturating_sub(inner.width);
            let frame_height = size.height.saturating_sub(inner.height);
            let max_width = a.size.width.saturating_sub(frame_width).max(1);
            let max_height = a.size.height.saturating_sub(frame_height).max(1);
            let scale = window.scale_factor()?;
            window.set_min_size(Some(tauri::PhysicalSize::new(
                ((900.0 * scale) as u32).min(max_width),
                ((640.0 * scale) as u32).min(max_height),
            )))?;
            window.set_size(tauri::PhysicalSize::new(
                inner.width.min(max_width),
                inner.height.min(max_height),
            ))?;
            window.set_position(a.position)?;
        }
    }
    Ok(())
}
pub fn show(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let result = (|| {
            w.unminimize()?;
            ensure_visible(&w)?;
            w.show()?;
            w.set_focus()
        })();
        if let Err(e) = result {
            eprintln!("恢复窗口失败: {e}");
        }
    }
}
pub fn window_event(window: &tauri::Window, event: &tauri::WindowEvent) {
    if window.label() != "main" {
        return;
    }
    let Some(state) = window.try_state::<Desktop>() else {
        return;
    };
    if matches!(event, tauri::WindowEvent::ScaleFactorChanged { .. }) {
        let app = window.app_handle().clone();
        let handle = app.clone();
        if let Err(e) = app.run_on_main_thread(move || {
            if let Some(w) = handle.get_webview_window("main") {
                if let Err(e) = ensure_visible(&w) {
                    eprintln!("调整显示范围失败: {e}");
                }
            }
        }) {
            eprintln!("调整 DPI 失败: {e}");
        }
    }
    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
        if !state.exiting.load(Ordering::SeqCst)
            && state.preferences.lock().is_ok_and(|p| p.close_to_tray)
        {
            // Only prevent closing after hiding succeeds: a failed tray operation
            // must never strand an invisible process.
            if window.hide().is_ok() {
                api.prevent_close();
            }
        }
    }
    if matches!(
        event,
        tauri::WindowEvent::Moved(_) | tauri::WindowEvent::Resized(_)
    ) && !window.is_minimized().unwrap_or(true)
    {
        if let Ok(mut p) = state.preferences.lock() {
            p.maximized = window.is_maximized().unwrap_or(false);
            if !p.maximized {
                if let (Ok(pos), Ok(size)) = (window.outer_position(), window.inner_size()) {
                    p.bounds = Some((pos.x, pos.y, size.width, size.height));
                }
            }
            if let Err(e) = state.save(&p) {
                eprintln!("保存窗口状态失败: {e}");
            }
        }
    }
}
pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let path = app.path().app_data_dir()?.join("desktop-v1.json");
    let preferences = match std::fs::read(&path) {
        Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
        Err(_) => Preferences::default(),
    };
    app.manage(Desktop {
        preferences: Mutex::new(preferences.clone()),
        path,
        exiting: AtomicBool::new(false),
    });
    if let Some(w) = app.get_webview_window("main") {
        w.set_theme(window_theme(&preferences.theme))?;
        w.set_background_color(Some(window_background(w.theme()?)))?;
        if let Some((x, y, width, height)) = preferences.bounds {
            w.set_size(tauri::PhysicalSize::new(
                width.clamp(900, 16384),
                height.clamp(640, 16384),
            ))?;
            w.set_position(tauri::PhysicalPosition::new(x, y))?;
        }
        ensure_visible(&w)?;
        if preferences.maximized {
            w.maximize()?;
        }
    }
    let show_item = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
    let status = MenuItem::with_id(app, "status", "未选择歌曲", false, None::<&str>)?;
    let toggle = MenuItem::with_id(app, "toggle", "播放", false, None::<&str>)?;
    let previous = MenuItem::with_id(app, "previous", "上一首", false, None::<&str>)?;
    let next = MenuItem::with_id(app, "next", "下一首", false, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(
        app,
        &[&show_item, &status, &toggle, &previous, &next, &quit],
    )?;
    TrayIconBuilder::with_id("music-mine")
        .icon(tauri::image::Image::from_bytes(include_bytes!(
            "../../icons/icon.png"
        ))?)
        .tooltip("Music Mine")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if matches!(
                event,
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                }
            ) {
                show(tray.app_handle());
            }
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => show(app),
            "quit" => app.exit(0),
            id => {
                let app = app.clone();
                let id = id.to_string();
                tauri::async_runtime::spawn_blocking(move || {
                    let result = (|| {
                        let state = app.state::<Shared>();
                        let mut c = state.lock().map_err(|e| e.to_string())?;
                        let action = match id.as_str() {
                            "toggle" => {
                                if c.execute(Action::State)?.playback_state == "playing" {
                                    Action::Pause
                                } else {
                                    Action::Play
                                }
                            }
                            "previous" => Action::Previous,
                            "next" => Action::Next,
                            _ => return Ok(()),
                        };
                        c.execute(action)?;
                        Ok::<_, String>(())
                    })();
                    if let Err(e) = result {
                        let _ = status_error(&app, &e);
                    }
                });
            }
        })
        .build(app)?;
    app.manage(TrayItems {
        status,
        toggle,
        previous,
        next,
        last: Mutex::new(String::new()),
    });
    Ok(())
}
struct TrayItems {
    status: MenuItem<tauri::Wry>,
    toggle: MenuItem<tauri::Wry>,
    previous: MenuItem<tauri::Wry>,
    next: MenuItem<tauri::Wry>,
    last: Mutex<String>,
}
fn status_error(app: &tauri::AppHandle, error: &str) -> tauri::Result<()> {
    app.state::<TrayItems>()
        .status
        .set_text(format!("操作失败: {error}"))
}
pub fn update(app: &tauri::AppHandle, s: &crate::playback::controller::PlayerSnapshot) {
    let items = app.state::<TrayItems>();
    let title = s
        .current_track
        .as_ref()
        .map(|t| t.title.as_str())
        .unwrap_or("未选择歌曲");
    let key = format!("{}:{title}:{}", s.playback_state, s.queue.len());
    if let Ok(mut last) = items.last.lock() {
        if *last == key {
            return;
        }
        *last = key;
    }
    let result = (|| {
        items.status.set_text(format!(
            "{} · {title}",
            match s.playback_state {
                "playing" => "正在播放",
                "paused" => "已暂停",
                _ => "已停止",
            }
        ))?;
        items.toggle.set_text(if s.playback_state == "playing" {
            "暂停"
        } else {
            "播放"
        })?;
        items.toggle.set_enabled(s.current_track.is_some())?;
        items.previous.set_enabled(!s.queue.is_empty())?;
        items.next.set_enabled(!s.queue.is_empty())
    })();
    if let Err(e) = result {
        eprintln!("更新托盘失败: {e}");
    }
}
pub fn shutdown(app: &tauri::AppHandle) {
    let state = app.state::<Desktop>();
    if state.exiting.swap(true, Ordering::SeqCst) {
        return;
    }
    if let Ok(mut c) = app.state::<Shared>().lock() {
        if let Err(e) = c.flush_session() {
            eprintln!("保存播放状态失败: {e}");
        }
        if let Err(e) = c.release_output() {
            eprintln!("释放音频输出失败: {e}");
        }
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn close_defaults_to_exit() {
        let p: Preferences = serde_json::from_str("{}").unwrap();
        assert!(!p.close_to_tray);
        assert!(!p.queue_open);
    }
    #[test]
    fn existing_users_keep_the_dark_title_bar() {
        let old: Preferences = serde_json::from_str(r#"{"close_to_tray":true}"#).unwrap();
        assert_eq!(window_theme(&old.theme), Some(tauri::Theme::Dark));
        assert_eq!(window_theme("light"), Some(tauri::Theme::Light));
        assert_eq!(window_theme("system"), None);
    }
    #[test]
    fn disconnected_and_negative_monitors() {
        assert!(!fits((1920, 0, 900, 640), (0, 0, 1920, 1080)));
        assert!(fits((-1800, 0, 900, 640), (-1920, 0, 1920, 1080)));
        assert!(!fits((0, 0, 2000, 1400), (0, 0, 1920, 1080)));
    }
    #[test]
    fn preferences_replace_existing_file() {
        let directory =
            std::env::temp_dir().join(format!("music-mine-desktop-test-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let state = Desktop {
            preferences: Mutex::new(Preferences::default()),
            path: directory.join("desktop.json"),
            exiting: AtomicBool::new(false),
        };
        state.save(&Preferences::default()).unwrap();
        let next = Preferences {
            close_to_tray: true,
            queue_open: true,
            ..Default::default()
        };
        state.save(&next).unwrap();
        let restored: Preferences =
            serde_json::from_slice(&std::fs::read(&state.path).unwrap()).unwrap();
        assert!(restored.close_to_tray && restored.queue_open);
        std::fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn shutdown_acknowledges_resource_release() {
        let player = crate::audio::player::Player::new(None);
        player.shutdown().unwrap();
        assert!(player.synchronize().is_err());
    }
    #[test]
    fn preferences_roundtrip() {
        let p = Preferences {
            close_to_tray: true,
            queue_open: true,
            theme: "light".into(),
            bounds: Some((-1200, 30, 1000, 700)),
            maximized: true,
        };
        let q: Preferences = serde_json::from_slice(&serde_json::to_vec(&p).unwrap()).unwrap();
        assert_eq!(p.bounds, q.bounds);
        assert_eq!(q.theme, "light");
        assert!(q.maximized && q.queue_open && q.close_to_tray);
    }
}
````

## File: src-tauri/src/platform/media.rs
````rust
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};
use tauri::Manager;
#[cfg(windows)]
use windows::{
    core::HSTRING,
    Foundation::{EventRegistrationToken, TypedEventHandler},
    Media::{
        MediaPlaybackStatus, MediaPlaybackType, SystemMediaTransportControls,
        SystemMediaTransportControlsButton, SystemMediaTransportControlsButtonPressedEventArgs,
    },
    Win32::System::WinRT::ISystemMediaTransportControlsInterop,
};

pub struct Media {
    stopped: AtomicBool,
    status: Mutex<String>,
    #[cfg(windows)]
    controls: Option<(SystemMediaTransportControls, EventRegistrationToken)>,
    last: Mutex<String>,
}
#[tauri::command]
pub fn media_status(app: tauri::AppHandle) -> Result<String, String> {
    app.state::<Media>()
        .status
        .lock()
        .map(|s| s.clone())
        .map_err(|e| e.to_string())
}
pub fn setup(app: &mut tauri::App) {
    #[cfg(windows)]
    let result = (|| -> windows::core::Result<_> {
        let window = app
            .get_webview_window("main")
            .ok_or_else(windows::core::Error::from_win32)?;
        let hwnd = window
            .hwnd()
            .map_err(|_| windows::core::Error::from_win32())?;
        let factory: ISystemMediaTransportControlsInterop = windows::core::factory::<
            SystemMediaTransportControls,
            ISystemMediaTransportControlsInterop,
        >()?;
        // Desktop WinRT interop uses our existing top-level HWND, never a MediaPlayer.
        let controls: SystemMediaTransportControls =
            unsafe { factory.GetForWindow(windows::Win32::Foundation::HWND(hwnd.0))? };
        controls.SetIsPlayEnabled(true)?;
        controls.SetIsPauseEnabled(true)?;
        controls.SetIsNextEnabled(true)?;
        controls.SetIsPreviousEnabled(true)?;
        let handle = app.handle().clone();
        let token = controls.ButtonPressed(&TypedEventHandler::new(
            move |_, args: &Option<SystemMediaTransportControlsButtonPressedEventArgs>| {
                if let Some(args) = args {
                    let action = match args.Button()? {
                        SystemMediaTransportControlsButton::Play => {
                            crate::playback::controller::Action::Play
                        }
                        SystemMediaTransportControlsButton::Pause => {
                            crate::playback::controller::Action::Pause
                        }
                        SystemMediaTransportControlsButton::Next => {
                            crate::playback::controller::Action::Next
                        }
                        SystemMediaTransportControlsButton::Previous => {
                            crate::playback::controller::Action::Previous
                        }
                        _ => return Ok(()),
                    };
                    let handle = handle.clone();
                    tauri::async_runtime::spawn_blocking(move || {
                        if handle.state::<Media>().stopped.load(Ordering::SeqCst) {
                            return;
                        }
                        let result = handle
                            .state::<crate::Shared>()
                            .lock()
                            .map_err(|e| e.to_string())
                            .and_then(|mut c| c.execute(action));
                        if let Err(e) = result {
                            if let Ok(mut s) = handle.state::<Media>().status.lock() {
                                *s = format!("SMTC 操作失败：{e}");
                            }
                        }
                    });
                }
                Ok(())
            },
        ))?;
        Ok((controls, token))
    })();
    #[cfg(windows)]
    let (controls, status) = match result {
        Ok(c) => (
            Some(c),
            "SMTC 已连接，等待实机媒体键验收；封面暂未接入".to_string(),
        ),
        Err(e) => (None, format!("SMTC 不可用：{e}")),
    };
    #[cfg(not(windows))]
    let status = "仅支持 Windows SMTC".to_string();
    app.manage(Media {
        stopped: AtomicBool::new(false),
        #[cfg(windows)]
        controls,
        status: Mutex::new(status),
        last: Mutex::new(String::new()),
    });
}
pub fn update(app: &tauri::AppHandle, s: &crate::playback::controller::PlayerSnapshot) {
    let state = app.state::<Media>();
    let key = format!(
        "{}:{}:{}",
        s.playback_state,
        serde_json::to_string(&s.current_track).unwrap_or_default(),
        s.queue.len()
    );
    let Ok(mut last) = state.last.lock() else {
        return;
    };
    if state.stopped.load(Ordering::SeqCst) {
        return;
    }
    if *last == key {
        return;
    }
    #[cfg(windows)]
    if let Some((c, _)) = &state.controls {
        let result = (|| -> windows::core::Result<()> {
            c.SetIsEnabled(s.current_track.is_some())?;
            c.SetPlaybackStatus(match s.playback_state {
                "playing" => MediaPlaybackStatus::Playing,
                "paused" => MediaPlaybackStatus::Paused,
                _ => MediaPlaybackStatus::Stopped,
            })?;
            c.SetIsNextEnabled(!s.queue.is_empty())?;
            c.SetIsPreviousEnabled(!s.queue.is_empty())?;
            let display = c.DisplayUpdater()?;
            display.ClearAll()?;
            if let Some(t) = &s.current_track {
                display.SetType(MediaPlaybackType::Music)?;
                let music = display.MusicProperties()?;
                music.SetTitle(&HSTRING::from(&t.title))?;
                music.SetArtist(&HSTRING::from(&t.artist))?;
                music.SetAlbumTitle(&HSTRING::from(&t.album))?;
            }
            display.Update()
        })();
        if let Err(e) = result {
            if let Ok(mut status) = state.status.lock() {
                *status = format!("SMTC 同步失败：{e}");
            }
            return;
        }
    }
    *last = key;
}
pub fn shutdown(app: &tauri::AppHandle) {
    let state = app.state::<Media>();
    let Ok(_last) = state.last.lock() else { return };
    if state.stopped.swap(true, Ordering::SeqCst) {
        return;
    }
    #[cfg(windows)]
    if let Some((c, token)) = &app.state::<Media>().controls {
        if let Err(e) = c
            .RemoveButtonPressed(*token)
            .and_then(|_| c.SetIsEnabled(false))
        {
            eprintln!("关闭 SMTC 失败：{e}");
        }
    }
}
````

## File: src-tauri/src/platform/mod.rs
````rust
pub(crate) mod desktop;
pub(crate) mod media;
````

## File: src-tauri/src/playback/controller.rs
````rust
use crate::audio::player::{PlaybackEvent, PlaybackState, Player};
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::tag::Accessor;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
#[derive(Clone, Serialize, Deserialize)]
pub struct Track {
    pub path: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name)
    }
    #[test]
    fn engine_eof_drives_sequential_shuffle_and_repeat_modes() {
        for mode in ["sequential", "shuffle", "repeat_one", "repeat_all"] {
            let mut c = Controller::new();
            c.execute(Action::Volume(0.)).unwrap();
            c.execute(Action::Open(fixture("tone.flac"))).unwrap();
            let first = c.track.clone().unwrap();
            c.execute(Action::Open(fixture("tone.mp3"))).unwrap();
            let last = c.track.clone().unwrap();
            c.execute(Action::Mode(mode.into())).unwrap();
            let duration = last.duration_ms;
            c.execute(Action::Seek(duration.saturating_sub(120) as f64))
                .unwrap();
            c.execute(Action::Play).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(250));
            c.execute(Action::Volume(0.)).unwrap(); // volume changes must not swallow EOF
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
            loop {
                std::thread::sleep(std::time::Duration::from_millis(30));
                let snapshot = c.execute(Action::State).unwrap();
                let done = match mode {
                    "sequential" => snapshot.playback_state == "idle",
                    "repeat_one" => {
                        snapshot.track_id == Some(last.path.clone())
                            && snapshot.position_ms < duration.saturating_sub(500)
                    }
                    _ => snapshot.track_id == Some(first.path.clone()),
                };
                if done {
                    break;
                }
                assert!(
                    std::time::Instant::now() < deadline,
                    "EOF did not apply mode {mode}"
                );
            }
            c.execute(Action::Stop).unwrap();
        }
    }
    #[test]
    fn session_restores_queue_position_mode_and_pauses() {
        let dir = std::env::temp_dir().join(format!("music-mine-session-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("session.json");
        let _ = std::fs::remove_file(&path);
        let mut c = Controller::new();
        c.attach_session(path.clone()).unwrap();
        c.execute(Action::Volume(0.)).unwrap();
        c.execute(Action::Open(fixture("tone.flac"))).unwrap();
        c.execute(Action::Seek(500.)).unwrap();
        c.execute(Action::Mode("repeat_all".into())).unwrap();
        c.execute(Action::Offset(-300)).unwrap();
        let mut restored = Controller::new();
        restored.attach_session(path.clone()).unwrap();
        let state = restored.execute(Action::State).unwrap();
        assert_eq!(state.playback_state, "paused");
        assert_eq!(state.playback_mode, "repeat_all");
        assert_eq!(state.lyrics_offset_ms, -300);
        assert_eq!(state.queue.len(), 1);
        assert!(state.position_ms.abs_diff(500) < 50);
        assert_eq!(state.volume, 0.);
        restored.execute(Action::Offset(100)).unwrap(); // replacing an existing session on Windows
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn queue_reorders_inserts_next_and_respects_modes() {
        let mut c = Controller::new();
        let make = |path: &str| Track {
            path: path.into(),
            title: path.into(),
            artist: String::new(),
            album: String::new(),
            duration_ms: 3000,
        };
        c.queue = vec![make("a"), make("b"), make("c")];
        c.track = Some(make("a"));
        c.execute(Action::Reorder("c".into(), 0)).unwrap();
        assert_eq!(
            c.queue.iter().map(|t| t.path.as_str()).collect::<Vec<_>>(),
            vec!["c", "a", "b"]
        );
        c.execute(Action::PlayNext(make("c"))).unwrap();
        assert_eq!(
            c.queue.iter().map(|t| t.path.as_str()).collect::<Vec<_>>(),
            vec!["a", "c", "b"]
        );
        assert_eq!(c.next_index(false), Some(1));
        assert_eq!(c.next_index(true), None);
        c.mode = "repeat_all".into();
        assert_eq!(c.next_index(true), Some(2));
        c.mode = "shuffle".into();
        for _ in 0..100 {
            assert_ne!(c.next_index(false), Some(0));
        }
        assert!(c.execute(Action::Mode("invalid".into())).is_err());
        assert!(c.execute(Action::Reorder("a".into(), 8)).is_err());
    }
    #[test]
    fn bulk_queue_actions_dedupe_and_clear_keeps_current() {
        let mut c = Controller::new();
        let make = |path: &str| Track {
            path: path.into(),
            title: path.into(),
            artist: String::new(),
            album: String::new(),
            duration_ms: 3000,
        };
        c.queue = vec![make("a"), make("b")];
        c.track = Some(make("b"));
        c.execute(Action::EnqueueMany(vec![
            make("c"),
            make("a"),
            make("c"),
            make("d"),
        ]))
        .unwrap();
        assert_eq!(
            c.queue.iter().map(|t| t.path.as_str()).collect::<Vec<_>>(),
            vec!["a", "b", "c", "d"]
        );
        let state = c.execute(Action::ClearUpcoming).unwrap();
        assert_eq!(state.queue.len(), 1);
        assert_eq!(state.queue[0].path, "b");
        c.track = None;
        assert!(c.execute(Action::ClearUpcoming).unwrap().queue.is_empty());
        assert!(c.execute(Action::Replace(Vec::new())).is_err());
    }
    #[test]
    fn replace_queue_starts_first_real_file() {
        let mut c = Controller::new();
        c.execute(Action::Volume(0.0)).unwrap();
        let track = |name: &str| Track {
            path: fixture(name).to_string_lossy().into_owned(),
            title: name.into(),
            artist: String::new(),
            album: String::new(),
            duration_ms: 3000,
        };
        let state = c
            .execute(Action::Replace(vec![
                track("tone.wav"),
                track("tone.flac"),
                track("tone.wav"),
            ]))
            .unwrap();
        assert_eq!(state.queue.len(), 2);
        assert_eq!(state.track_id, Some(track("tone.wav").path));
        assert_eq!(state.playback_state, "playing");
        c.execute(Action::Stop).unwrap();
    }
    #[test]
    fn queue_uses_existing_engine_for_selection_next_and_previous() {
        let mut c = Controller::new();
        c.execute(Action::Volume(0.0)).unwrap();
        let first = c.execute(Action::Open(fixture("tone.flac"))).unwrap();
        let first_track = first.current_track.unwrap();
        let second = Track {
            path: fixture("tone.mp3").to_string_lossy().into_owned(),
            title: "second".into(),
            artist: String::new(),
            album: String::new(),
            duration_ms: 3000,
        };
        let s = c.execute(Action::Enqueue(second.clone())).unwrap();
        assert_eq!(s.queue.len(), 2);
        assert_eq!(
            c.execute(Action::Enqueue(second.clone()))
                .unwrap()
                .queue
                .len(),
            2
        );
        let next = c.execute(Action::Next).unwrap();
        assert_eq!(next.track_id, Some(second.path.clone()));
        assert_eq!(next.playback_state, "playing");
        let previous = c.execute(Action::Previous).unwrap();
        assert_eq!(previous.track_id, Some(first_track.path));
        c.execute(Action::Stop).unwrap();
        assert_eq!(
            c.execute(Action::Remove(second.path)).unwrap().queue.len(),
            1
        );
    }
}
#[derive(Clone, Serialize)]
pub struct PlayerSnapshot {
    revision: u64,
    track_id: Option<String>,
    pub current_track: Option<Track>,
    pub playback_state: &'static str,
    position_ms: u64,
    duration_ms: u64,
    volume: f32,
    playback_mode: String,
    lyrics_offset_ms: i64,
    output_device_id: Option<String>,
    error: Option<String>,
    pub queue: Vec<Track>,
}
pub enum Action {
    State,
    Open(PathBuf),
    Start(PathBuf),
    Select(String),
    Play,
    Pause,
    Stop,
    Seek(f64),
    Volume(f64),
    Enqueue(Track),
    EnqueueMany(Vec<Track>),
    /// Replaces the queue with these tracks (deduplicated by path) and starts the first.
    Replace(Vec<Track>),
    /// Keeps only the current track in the queue.
    ClearUpcoming,
    Remove(String),
    Next,
    Previous,
    Mode(String),
    Reorder(String, usize),
    PlayNext(Track),
    Offset(i64),
}
pub struct Controller {
    engine: Player,
    track: Option<Track>,
    revision: u64,
    queue: Vec<Track>,
    mode: String,
    offset: i64,
    session_path: Option<PathBuf>,
    saved: String,
    last_save: std::time::Instant,
    random: u64,
}
impl Controller {
    pub fn new() -> Self {
        Self {
            engine: Player::new(None),
            track: None,
            revision: 0,
            queue: Vec::new(),
            mode: "sequential".into(),
            offset: 0,
            session_path: None,
            saved: String::new(),
            last_save: std::time::Instant::now(),
            random: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64
                | 1,
        }
    }
    pub fn execute(&mut self, action: Action) -> Result<PlayerSnapshot, String> {
        let is_state = matches!(action, Action::State);
        let replaces_transport = matches!(
            action,
            Action::Open(_)
                | Action::Start(_)
                | Action::Select(_)
                | Action::Play
                | Action::Pause
                | Action::Stop
                | Action::Seek(_)
                | Action::Next
                | Action::Previous
        );
        if matches!(action, Action::State) {
            let mut finished = false;
            while let Some(event) = self.engine.poll() {
                if matches!(event, PlaybackEvent::Finished) {
                    finished = true;
                }
            }
            if finished {
                if self.mode == "repeat_one" {
                    if let Some(t) = &self.track {
                        return self.execute(Action::Start(t.path.clone().into()));
                    }
                }
                if let Some(index) = self.next_index(false) {
                    return self.execute(Action::Start(self.queue[index].path.clone().into()));
                }
            }
        }

        let mut prepared_status = None;
        match action {
            Action::Select(path) => {
                if !self.queue.iter().any(|t| t.path == path) {
                    return Err("歌曲不在队列中".into());
                }
                return self.execute(Action::Start(path.into()));
            }
            Action::Start(path) => {
                self.execute(Action::Open(path))?;
                return self.execute(Action::Play);
            }
            Action::Enqueue(track) => {
                if !self.queue.iter().any(|t| t.path == track.path) {
                    self.queue.push(track);
                }
            }
            Action::EnqueueMany(tracks) => {
                for track in tracks {
                    if !self.queue.iter().any(|t| t.path == track.path) {
                        self.queue.push(track);
                    }
                }
            }
            Action::Replace(tracks) => {
                let mut queue: Vec<Track> = Vec::with_capacity(tracks.len());
                for track in tracks {
                    if !queue.iter().any(|t| t.path == track.path) {
                        queue.push(track);
                    }
                }
                let first = queue.first().ok_or("没有可播放的歌曲")?.path.clone();
                self.queue = queue;
                return self.execute(Action::Start(first.into()));
            }
            Action::ClearUpcoming => {
                let current = self.track.as_ref().map(|t| t.path.clone());
                self.queue.retain(|t| Some(&t.path) == current.as_ref());
            }
            Action::Remove(path) => {
                self.queue.retain(|t| t.path != path);
            }
            Action::Next | Action::Previous => {
                let index = self
                    .next_index(matches!(action, Action::Previous))
                    .ok_or("已经到达队列边界")?;
                let path = PathBuf::from(&self.queue[index].path);
                self.execute(Action::Open(path))?;
                return self.execute(Action::Play);
            }
            Action::Mode(mode) => {
                if !["sequential", "shuffle", "repeat_one", "repeat_all"].contains(&mode.as_str()) {
                    return Err("无效播放模式".into());
                }
                self.mode = mode;
            }
            Action::Offset(value) => {
                if !(-60_000..=60_000).contains(&value) {
                    return Err("歌词偏移范围为 ±60 秒".into());
                }
                self.offset = value;
            }
            Action::Reorder(path, target) => {
                if target >= self.queue.len() {
                    return Err("无效队列位置".into());
                }
                let from = self
                    .queue
                    .iter()
                    .position(|t| t.path == path)
                    .ok_or("歌曲不在队列中")?;
                let track = self.queue.remove(from);
                self.queue.insert(target, track);
            }
            Action::PlayNext(track) => {
                self.queue.retain(|t| t.path != track.path);
                let index = self
                    .track
                    .as_ref()
                    .and_then(|t| self.queue.iter().position(|q| q.path == t.path))
                    .map_or(0, |i| i + 1);
                self.queue.insert(index, track);
            }
            Action::State => {}
            Action::Open(path) => {
                let extension = path
                    .extension()
                    .and_then(|v| v.to_str())
                    .unwrap_or("")
                    .to_ascii_lowercase();
                if !["mp3", "flac", "wav"].contains(&extension.as_str()) {
                    return Err("请选择 MP3、FLAC 或 WAV 文件".into());
                }
                let file =
                    lofty::read_from_path(&path).map_err(|e| format!("读取音频失败：{e}"))?;
                let text = |field: u8| {
                    file.primary_tag()
                        .into_iter()
                        .chain(file.tags().iter())
                        .find_map(|tag| {
                            let value = match field {
                                0 => tag.title(),
                                1 => tag.artist(),
                                _ => tag.album(),
                            };
                            value.map(|s| s.trim().to_owned()).filter(|s| !s.is_empty())
                        })
                };
                let track = Track {
                    path: path.to_string_lossy().into_owned(),
                    title: text(0).unwrap_or_else(|| {
                        path.file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned()
                    }),
                    artist: text(1).unwrap_or_else(|| "未知歌手".into()),
                    album: text(2).unwrap_or_default(),
                    duration_ms: file.properties().duration().as_millis() as u64,
                };
                self.engine.prepare(path, track.duration_ms, 0);
                let status = self.engine.synchronize()?;
                self.track = if status.current_path.as_deref() == Some(track.path.as_str()) {
                    if !self.queue.iter().any(|t| t.path == track.path) {
                        self.queue.push(track.clone());
                    }
                    Some(track)
                } else {
                    None
                };
                prepared_status = Some(status);
            }
            Action::Play => {
                let track = self.track.as_ref().ok_or("请先添加真实音频文件")?;
                match self.engine.status().state {
                    PlaybackState::Paused => self.engine.resume(),
                    PlaybackState::Stopped => self
                        .engine
                        .play(PathBuf::from(&track.path), track.duration_ms),
                    PlaybackState::Playing => {}
                }
            }
            Action::Pause => self.engine.pause(),
            Action::Stop => self.engine.stop(),
            Action::Seek(ms) => {
                let status = self.engine.status();
                if !ms.is_finite() || ms < 0.0 || ms > status.duration.as_millis() as f64 {
                    return Err("跳转位置超出歌曲范围".into());
                }
                if status.state == PlaybackState::Stopped {
                    return Err("请先播放歌曲再跳转".into());
                }
                self.engine.seek(ms / 1000.0);
            }
            Action::Volume(value) => {
                if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                    return Err("音量应在 0 到 100% 之间".into());
                }
                self.engine.set_volume(value as f32);
            }
        }
        let status = match prepared_status {
            Some(status) => status,
            None => self.engine.synchronize()?,
        };
        // Only transport changes invalidate old EOF. Queue/config/volume actions preserve it.
        if replaces_transport {
            while self.engine.poll().is_some() {}
        }
        self.revision += 1;
        let snapshot = PlayerSnapshot {
            revision: self.revision,
            track_id: self.track.as_ref().map(|t| t.path.clone()),
            current_track: self.track.clone(),
            playback_state: match status.state {
                PlaybackState::Stopped => "idle",
                PlaybackState::Playing => "playing",
                PlaybackState::Paused => "paused",
            },
            position_ms: status.position.as_millis() as u64,
            duration_ms: status.duration.as_millis() as u64,
            volume: status.volume,
            playback_mode: self.mode.clone(),
            lyrics_offset_ms: self.offset,
            output_device_id: status.output.map(|o| o.device),
            error: status.error,
            queue: self.queue.clone(),
        };
        if !is_state || self.last_save.elapsed().as_secs() >= 2 {
            self.persist(&snapshot)?;
        }
        Ok(snapshot)
    }
}

#[derive(Serialize, Deserialize)]
struct Session {
    queue: Vec<Track>,
    current: Option<Track>,
    position: u64,
    volume: f32,
    mode: String,
    offset: i64,
}
impl Controller {
    fn next_index(&mut self, previous: bool) -> Option<usize> {
        let len = self.queue.len();
        if len == 0 {
            return None;
        }
        let current = self
            .track
            .as_ref()
            .and_then(|t| self.queue.iter().position(|q| q.path == t.path));
        let Some(i) = current else {
            return Some(0);
        };
        if self.mode == "shuffle" && len > 1 {
            self.random ^= self.random << 13;
            self.random ^= self.random >> 7;
            self.random ^= self.random << 17;
            return Some((i + 1 + self.random as usize % (len - 1)) % len);
        }
        if previous {
            i.checked_sub(1)
                .or_else(|| (self.mode == "repeat_all").then_some(len - 1))
        } else if i + 1 < len {
            Some(i + 1)
        } else {
            (self.mode == "repeat_all").then_some(0)
        }
    }
    pub fn attach_session(&mut self, path: PathBuf) -> Result<(), String> {
        if path.exists() {
            let session: Session =
                serde_json::from_slice(&std::fs::read(&path).map_err(|e| e.to_string())?)
                    .map_err(|e| format!("播放会话读取失败：{e}"))?;
            self.queue = session.queue;
            if ["sequential", "shuffle", "repeat_one", "repeat_all"]
                .contains(&session.mode.as_str())
            {
                self.mode = session.mode;
            }
            self.offset = session.offset.clamp(-60_000, 60_000);
            if session.volume.is_finite() {
                self.engine.set_volume(session.volume.clamp(0., 1.));
            }
            if let Some(track) = session.current {
                if std::path::Path::new(&track.path).is_file() {
                    self.engine.prepare(
                        track.path.clone().into(),
                        track.duration_ms,
                        session.position.min(track.duration_ms),
                    );
                    self.engine.synchronize()?;
                    self.track = Some(track);
                }
            }
        }
        self.session_path = Some(path);
        Ok(())
    }
    fn persist(&mut self, s: &PlayerSnapshot) -> Result<(), String> {
        let Some(path) = &self.session_path else {
            return Ok(());
        };
        let json = serde_json::to_string(&Session {
            queue: self.queue.clone(),
            current: self.track.clone(),
            position: s.position_ms,
            volume: s.volume,
            mode: self.mode.clone(),
            offset: self.offset,
        })
        .map_err(|e| e.to_string())?;
        if json != self.saved {
            let temp = path.with_extension("tmp");
            std::fs::write(&temp, &json).map_err(|e| format!("保存播放会话失败：{e}"))?;
            std::fs::rename(&temp, path).map_err(|e| format!("替换播放会话失败：{e}"))?;
            self.saved = json;
        }
        self.last_save = std::time::Instant::now();
        Ok(())
    }
    pub fn release_output(&mut self) -> Result<(), String> {
        self.engine.shutdown()
    }
    pub fn flush_session(&mut self) -> Result<(), String> {
        let snapshot = self.execute(Action::State)?;
        self.persist(&snapshot)
    }
    pub fn current_path(&self) -> Option<PathBuf> {
        self.track.as_ref().map(|t| t.path.clone().into())
    }
}
````

## File: src-tauri/src/playback/lyrics.rs
````rust
//! Local lyrics: reading (embedded / sidecar `.lrc`), decoding and parsing
//! standard LRC, and the timing logic that maps a playback position to a line.
//!
//! This module has no UI and no player state: it is pure data + text work, so
//! it can be tested on its own. The player's real position is passed in by the
//! caller every frame.

use std::path::{Path, PathBuf};

use lofty::file::TaggedFileExt;
use lofty::tag::ItemKey;

/// One lyric line. `time_ms` is -1 for files without any timestamps.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct LyricLine {
    pub time_ms: i64,
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize)]
pub struct Lyrics {
    pub lines: Vec<LyricLine>,
    /// `[offset:+/-ms]` applied to every timestamp.
    pub offset_ms: i64,
    /// True when at least one line carries a LRC timestamp.
    pub synced: bool,
    /// Where the text came from, for the UI ("内嵌歌词" / "song.lrc").
    pub source: String,
}

impl Lyrics {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Index of the line that is playing at `position_ms`, or None before the
    /// first timestamp.
    pub fn index_at(&self, position_ms: i64) -> Option<usize> {
        if !self.synced || self.lines.is_empty() {
            return None;
        }
        // Lines are sorted; find the last one with time <= position.
        let idx = self.lines.partition_point(|l| l.time_ms <= position_ms);
        if idx == 0 {
            None
        } else {
            Some(idx - 1)
        }
    }

    /// The time of the line that should be highlighted, for seek buttons.
    pub fn time_of(&self, index: usize) -> Option<i64> {
        self.lines.get(index).map(|l| l.time_ms)
    }
}

// ------------------------------------------------------------------ parsing

/// Parse LRC text. Accepts `[mm:ss]`, `[mm:ss.xx]`, `[mm:ss.xxx]`, several
/// timestamps on one line, and the `[offset:...]` tag. Lines without any
/// timestamp produce an unsynchronised lyric sheet.
pub fn parse_lrc(text: &str) -> Lyrics {
    let mut lines: Vec<LyricLine> = Vec::new();
    let mut offset_ms: i64 = 0;
    let mut any_stamp = false;

    for raw in text.lines() {
        let line = raw.trim_matches(['\u{feff}', '\r', '\n']).trim();
        if line.is_empty() {
            continue;
        }
        let mut rest = line;
        let mut stamps: Vec<i64> = Vec::new();
        // Consume leading [..] tags: timestamps or metadata.
        while let Some(start) = rest.strip_prefix('[') {
            let Some(end) = start.find(']') else {
                break;
            };
            let tag = &start[..end];
            let after = &start[end + 1..];
            if let Some(ms) = parse_timestamp(tag) {
                stamps.push(ms);
                rest = after;
                continue;
            }
            // Metadata like [ti:...], [offset:+500].
            if let Some((key, value)) = tag.split_once(':') {
                if key.eq_ignore_ascii_case("offset") {
                    let v = value.trim();
                    let sign = if v.starts_with('-') { -1 } else { 1 };
                    let digits: String = v.chars().filter(|c| c.is_ascii_digit()).collect();
                    if let Ok(n) = digits.parse::<i64>() {
                        offset_ms = sign * n;
                    }
                }
            }
            rest = after;
        }
        let text_part = rest.trim().to_string();
        if stamps.is_empty() {
            if !text_part.is_empty() {
                lines.push(LyricLine {
                    time_ms: -1,
                    text: text_part,
                });
            }
            continue;
        }
        any_stamp = true;
        for ms in stamps {
            lines.push(LyricLine {
                time_ms: ms,
                text: text_part.clone(),
            });
        }
    }

    let synced = any_stamp;
    if synced {
        // Apply the offset and sort; several tags on one line mean repeats.
        for l in lines.iter_mut() {
            if l.time_ms >= 0 {
                l.time_ms = l.time_ms.saturating_add(offset_ms);
            }
        }
        lines.retain(|l| l.time_ms >= 0);
        lines.sort_by_key(|l| l.time_ms);
    } else {
        for l in lines.iter_mut() {
            l.time_ms = -1;
        }
    }

    Lyrics {
        lines,
        offset_ms,
        synced,
        source: String::new(),
    }
}

/// `mm:ss`, `mm:ss.xx`, `mm:ss.xxx`, `mm:ss:xx` → milliseconds.
fn parse_timestamp(tag: &str) -> Option<i64> {
    let (minutes, rest) = tag.split_once(':')?;
    let minutes: i64 = minutes.trim().parse().ok()?;
    let (seconds, fraction) = match rest.split_once(['.', ':']) {
        Some((s, f)) => (s, Some(f)),
        None => (rest, None),
    };
    let seconds: i64 = seconds.trim().parse().ok()?;
    if minutes < 0 || !(0..60).contains(&seconds) {
        return None;
    }
    let frac_ms = match fraction {
        None => 0,
        Some(f) => {
            if !f.chars().all(|c| c.is_ascii_digit()) {
                return None;
            }
            let digits: String = f.chars().take(3).collect();
            let value: i64 = digits.parse().unwrap_or(0);
            match digits.len() {
                0 => 0,
                1 => value * 100,
                2 => value * 10,
                _ => value / 10_i64.pow(digits.len().saturating_sub(3) as u32),
            }
        }
    };
    minutes
        .checked_mul(60_000)?
        .checked_add(seconds * 1000)?
        .checked_add(frac_ms)
}

// ------------------------------------------------------------------ decoding

/// Decode lyric bytes: UTF-8 (with or without BOM), UTF-16 with BOM, UTF-16
/// without BOM when the zero-byte pattern is obvious, else charset detection
/// (GBK/GB18030, Shift_JIS, Big5, …).
pub fn decode_bytes(bytes: &[u8]) -> String {
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return String::from_utf8_lossy(&bytes[3..]).to_string();
    }
    if bytes.starts_with(&[0xFF, 0xFE]) {
        let (text, _, _) = encoding_rs::UTF_16LE.decode(&bytes[2..]);
        return text.replace('\u{feff}', "");
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        let (text, _, _) = encoding_rs::UTF_16BE.decode(&bytes[2..]);
        return text.replace('\u{feff}', "");
    }
    // UTF-16 without a BOM: ASCII-heavy text has a zero byte every other byte.
    let sample = &bytes[..bytes.len().min(512)];
    let even_zeros = sample.iter().step_by(2).filter(|b| **b == 0).count();
    let odd_zeros = sample
        .iter()
        .skip(1)
        .step_by(2)
        .filter(|b| **b == 0)
        .count();
    let pairs = (sample.len() / 2).max(1);
    if odd_zeros > pairs / 3 && even_zeros < pairs / 10 {
        let (text, _, _) = encoding_rs::UTF_16LE.decode(bytes);
        return text.replace('\u{feff}', "");
    }
    if even_zeros > pairs / 3 && odd_zeros < pairs / 10 {
        let (text, _, _) = encoding_rs::UTF_16BE.decode(bytes);
        return text.replace('\u{feff}', "");
    }
    if let Ok(text) = std::str::from_utf8(bytes) {
        return text.to_string();
    }
    let mut detector = chardetng::EncodingDetector::new();
    detector.feed(bytes, true);
    let encoding = detector.guess(None, true);
    let (text, _, _) = encoding.decode(bytes);
    text.to_string()
}

// ------------------------------------------------------------------- loading

const MAX_LYRIC_FILE: u64 = 4 * 1024 * 1024;

/// Load lyrics for a track: a same-name `.lrc` next to the file first (usually
/// the synced one), then the lyrics embedded in the audio tags. Files are only
/// read, never written.
pub fn load_for_track(path: &Path) -> Option<Lyrics> {
    if let Some(sidecar) = sidecar_path(path) {
        if let Ok(meta) = std::fs::metadata(&sidecar) {
            if meta.len() <= MAX_LYRIC_FILE {
                if let Ok(bytes) = std::fs::read(&sidecar) {
                    let text = decode_bytes(&bytes);
                    let mut lyrics = parse_lrc(&text);
                    if !lyrics.is_empty() {
                        lyrics.source = sidecar
                            .file_name()
                            .map(|s| s.to_string_lossy().to_string())
                            .unwrap_or_else(|| "lrc".to_string());
                        return Some(lyrics);
                    }
                }
            }
        }
    }
    embedded_lyrics(path)
}

fn sidecar_path(audio: &Path) -> Option<PathBuf> {
    let candidate = audio.with_extension("lrc");
    if candidate.is_file() {
        return Some(candidate);
    }
    // Some collections store an upper-case extension; try it explicitly for
    // case-sensitive filesystems.
    let upper = audio.with_extension("LRC");
    if upper.is_file() {
        return Some(upper);
    }
    None
}

fn embedded_lyrics(path: &Path) -> Option<Lyrics> {
    let tagged = lofty::read_from_path(path).ok()?;
    for tag in tagged.tags() {
        let text = tag.get_string(&ItemKey::Lyrics).map(|s| s.to_string());
        if let Some(text) = text {
            if text.trim().is_empty() {
                continue;
            }
            let mut lyrics = parse_lrc(&text);
            if !lyrics.is_empty() {
                lyrics.source = "内嵌歌词".to_string();
                return Some(lyrics);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_timestamps_and_sorts_them() {
        let lrc = "[00:12.50]Second\n[00:00.00]First\n[00:20.123]Third\n";
        let lyrics = parse_lrc(lrc);
        assert!(lyrics.synced);
        assert_eq!(lyrics.lines.len(), 3);
        assert_eq!(lyrics.lines[0].text, "First");
        assert_eq!(lyrics.lines[0].time_ms, 0);
        assert_eq!(lyrics.lines[1].time_ms, 12_500);
        assert_eq!(lyrics.lines[2].time_ms, 20_123);
    }

    #[test]
    fn one_line_with_several_timestamps_repeats_the_text() {
        let lyrics = parse_lrc("[00:05.00][01:30.00]Chorus\n[00:10.00]Verse\n");
        let times: Vec<i64> = lyrics.lines.iter().map(|l| l.time_ms).collect();
        assert_eq!(times, vec![5_000, 10_000, 90_000]);
        assert_eq!(lyrics.lines[0].text, "Chorus");
        assert_eq!(lyrics.lines[2].text, "Chorus");
    }

    #[test]
    fn offset_tag_shifts_all_timestamps() {
        let lyrics = parse_lrc("[offset:+500]\n[00:01.00]A\n[00:02.00]B\n");
        assert_eq!(lyrics.offset_ms, 500);
        assert_eq!(lyrics.lines[0].time_ms, 1_500);
        assert_eq!(lyrics.lines[1].time_ms, 2_500);

        let earlier = parse_lrc("[offset:-250]\n[00:01.00]A\n");
        assert_eq!(earlier.lines[0].time_ms, 750);
    }

    #[test]
    fn metadata_tags_and_blank_lines_are_ignored() {
        let lrc = "[ti:Song]\n[ar:Artist]\n[al:Album]\n\n[00:01.00]Hello\n";
        let lyrics = parse_lrc(lrc);
        assert_eq!(lyrics.lines.len(), 1);
        assert_eq!(lyrics.lines[0].text, "Hello");
        // "Song" must not become a lyric line.
        assert!(!lyrics.lines.iter().any(|l| l.text == "Song"));
    }

    #[test]
    fn text_without_timestamps_is_unsynchronised() {
        let lyrics = parse_lrc("Just a plain lyric\nSecond line\n");
        assert!(!lyrics.synced);
        assert_eq!(lyrics.lines.len(), 2);
        assert_eq!(lyrics.index_at(10_000), None);
    }

    #[test]
    fn index_at_handles_boundaries_and_empty_positions() {
        let lyrics = parse_lrc("[00:01.00]One\n[00:02.00]Two\n[00:03.00]Three\n");
        assert_eq!(lyrics.index_at(0), None);
        assert_eq!(lyrics.index_at(999), None);
        assert_eq!(lyrics.index_at(1_000), Some(0));
        assert_eq!(lyrics.index_at(1_999), Some(0));
        assert_eq!(lyrics.index_at(2_000), Some(1));
        assert_eq!(lyrics.index_at(99_000), Some(2));
    }

    #[test]
    fn pause_and_seek_are_reproducible() {
        let lyrics = parse_lrc("[00:01.00]One\n[00:05.00]Two\n");
        // A paused position maps to the same line no matter how often asked.
        for _ in 0..3 {
            assert_eq!(lyrics.index_at(5_000), Some(1));
        }
        // Seeking backwards moves the highlight back.
        assert_eq!(lyrics.index_at(1_200), Some(0));
    }

    #[test]
    fn fractional_timestamps_of_various_precision() {
        let lyrics = parse_lrc("[00:00.5]a\n[00:01.05]b\n[00:02.005]c\n[00:03:25]d\n");
        let times: Vec<i64> = lyrics.lines.iter().map(|l| l.time_ms).collect();
        assert_eq!(times, vec![500, 1_050, 2_005, 3_250]);
    }

    #[test]
    fn decodes_utf8_bom_utf16_and_gbk() {
        let text = "[00:01.00]你好，世界\n";
        let mut utf8_bom = vec![0xEF, 0xBB, 0xBF];
        utf8_bom.extend_from_slice(text.as_bytes());
        assert_eq!(decode_bytes(&utf8_bom), text);

        let mut utf16 = vec![0xFF, 0xFE];
        for unit in text.encode_utf16() {
            utf16.extend_from_slice(&unit.to_le_bytes());
        }
        assert_eq!(decode_bytes(&utf16), text);

        let (gbk, _, _) = encoding_rs::GBK.encode(text);
        assert_eq!(decode_bytes(&gbk), text);

        // UTF-16LE without a BOM, detected by the zero-byte pattern.
        let mut raw16 = Vec::new();
        for unit in text.encode_utf16() {
            raw16.extend_from_slice(&unit.to_le_bytes());
        }
        assert_eq!(decode_bytes(&raw16), text);
    }

    #[test]
    fn rejects_overflow_and_decodes_japanese_and_big5() {
        assert!(!parse_lrc("[9223372036854775807:00]bad").synced);
        for (encoding, text) in [
            (
                encoding_rs::SHIFT_JIS,
                "[00:01.00]こんにちは、世界。音楽を聴いています。",
            ),
            (
                encoding_rs::BIG5,
                "[00:01.00]你好世界，這是我們喜歡的音樂。",
            ),
        ] {
            let (bytes, _, _) = encoding.encode(text);
            assert_eq!(decode_bytes(&bytes), text);
        }
    }
    #[test]
    fn garbage_input_does_not_panic() {
        let _ = decode_bytes(&[0, 159, 146, 150, 255, 1, 2, 3]);
        let _ = parse_lrc("");
        let _ = parse_lrc("[]\n[::]\n[99:99.99]x");
    }
}
````

## File: src-tauri/src/playback/mod.rs
````rust
pub(crate) mod controller;
pub mod lyrics;
````

## File: src-tauri/src/test_support/cover_corpus.rs
````rust
//! Isolated large cover corpus for queue cover measurements. Never touches a user library:
//! every audio file is a copy of the repository fixtures with a generated picture.
//!
//! `MM_COVER_CORPUS=<dir> cargo test --release cover_corpus -- --ignored --nocapture`
//! writes `<dir>/music` and scans it into `<dir>/app-data/library-tauri-v1.sqlite`.
//! With `MM_COVER_CORPUS_RECOVER=1` it only replaces the pictures of the first five files,
//! keeping their names, to exercise cache invalidation.
use std::io::Cursor;
use std::path::{Path, PathBuf};

use image::{Rgb, RgbImage, Rgba, RgbaImage};
use lofty::config::WriteOptions;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::picture::{MimeType, Picture, PictureType};
use lofty::tag::{Accessor, Tag};

const WEBP_1X1: &str = "UklGRhoAAABXRUJQVlA4TA0AAAAvAAAAEAcQERGIiP4HAA==";
const GIF_1X1: &str = "R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw==";

struct Seed(u32);
impl Seed {
    fn next(&mut self) -> u8 {
        self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (self.0 >> 24) as u8
    }
}

/// Smooth gradient with mild noise: realistic JPEG sizes without multi-megabyte noise.
fn picture_rgb(edge: u32, seed: u32) -> RgbImage {
    let mut noise = Seed(seed);
    let base = [
        (seed * 53 % 256) as u8,
        (seed * 97 % 256) as u8,
        (seed * 29 % 256) as u8,
    ];
    RgbImage::from_fn(edge, edge, |x, y| {
        let n = noise.next() / 16;
        Rgb([
            base[0]
                .wrapping_add((x * 255 / edge) as u8 / 2)
                .wrapping_add(n),
            base[1]
                .wrapping_add((y * 255 / edge) as u8 / 2)
                .wrapping_add(n),
            base[2]
                .wrapping_add(((x + y) * 127 / edge) as u8)
                .wrapping_add(n),
        ])
    })
}

fn jpeg(edge: u32, seed: u32) -> Vec<u8> {
    let image = picture_rgb(edge, seed);
    let mut out = Cursor::new(Vec::new());
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 88)
        .encode(image.as_raw(), edge, edge, image::ExtendedColorType::Rgb8)
        .unwrap();
    out.into_inner()
}

fn png_rgb(edge: u32, seed: u32) -> Vec<u8> {
    let mut out = Cursor::new(Vec::new());
    picture_rgb(edge, seed)
        .write_to(&mut out, image::ImageFormat::Png)
        .unwrap();
    out.into_inner()
}

fn png_rgba(edge: u32, seed: u32, noisy: bool) -> Vec<u8> {
    let mut noise = Seed(seed);
    let image = RgbaImage::from_fn(edge, edge, |x, y| {
        if noisy {
            Rgba([noise.next(), noise.next(), noise.next(), 255])
        } else {
            Rgba([(x % 256) as u8, (y % 256) as u8, (seed % 256) as u8, 200])
        }
    });
    let mut out = Cursor::new(Vec::new());
    image.write_to(&mut out, image::ImageFormat::Png).unwrap();
    out.into_inner()
}

fn b64(text: &str) -> Vec<u8> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(text)
        .unwrap()
}

fn embed(
    path: &Path,
    title: &str,
    album: &str,
    artist: &str,
    picture: Option<(Vec<u8>, MimeType)>,
) {
    let mut tagged = lofty::read_from_path(path).unwrap();
    if tagged.primary_tag().is_none() {
        tagged.insert_tag(Tag::new(tagged.primary_tag_type()));
    }
    let tag = tagged.primary_tag_mut().unwrap();
    tag.set_title(title.into());
    tag.set_album(album.into());
    tag.set_artist(artist.into());
    tag.remove_picture_type(PictureType::CoverFront);
    if let Some((data, mime)) = picture {
        tag.push_picture(Picture::new_unchecked(
            PictureType::CoverFront,
            Some(mime),
            None,
            data,
        ));
    }
    tagged.save_to_path(path, WriteOptions::default()).unwrap();
}

struct Entry {
    kind: &'static str,
    folder: String,
    file: String,
    picture: Option<(Vec<u8>, MimeType)>,
}

fn entries() -> Vec<Entry> {
    let mut out = Vec::new();
    let mut push = |kind: &'static str, picture: Option<(Vec<u8>, MimeType)>| {
        let i = out.len();
        out.push(Entry {
            kind,
            folder: kind.to_string(),
            file: format!("{i:03} {kind}"),
            picture,
        });
    };
    for i in 0..194u32 {
        push(
            "jpeg-small",
            Some((jpeg([300, 500, 800][i as usize % 3], i), MimeType::Jpeg)),
        );
    }
    for i in 0..50u32 {
        push("jpeg-3000", Some((jpeg(3000, 1000 + i), MimeType::Jpeg)));
    }
    for i in 0..20u32 {
        push("png-1000", Some((png_rgb(1000, 2000 + i), MimeType::Png)));
    }
    for i in 0..8u32 {
        push(
            "png-3000-rgba",
            Some((png_rgba(3000, 3000 + i, false), MimeType::Png)),
        );
    }
    for i in 0..4u32 {
        push(
            "png-noise-13mb",
            Some((png_rgba(1800, 4000 + i, true), MimeType::Png)),
        );
    }
    for _ in 0..10 {
        push(
            "webp",
            Some((b64(WEBP_1X1), MimeType::Unknown("image/webp".into()))),
        );
    }
    for _ in 0..5 {
        push("gif", Some((b64(GIF_1X1), MimeType::Gif)));
    }
    for i in 0..5u8 {
        let mut broken = vec![0xff, 0xd8, 0xff, 0xe0];
        broken.extend(std::iter::repeat_n(i, 4096));
        push("corrupt-jpeg", Some((broken, MimeType::Jpeg)));
    }
    for _ in 0..30 {
        push("no-cover", None);
    }
    for i in 0..10u32 {
        out.push(Entry {
            kind: "same-name",
            folder: format!("same-name/{i:02}"),
            file: "Same Name".into(),
            picture: Some((jpeg(400, 5000 + i), MimeType::Jpeg)),
        });
    }
    out
}

#[test]
#[ignore]
fn cover_corpus() {
    let Some(root) = std::env::var_os("MM_COVER_CORPUS").map(PathBuf::from) else {
        eprintln!("MM_COVER_CORPUS is not set; nothing generated");
        return;
    };
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let music = root.join("music");
    if std::env::var_os("MM_COVER_CORPUS_RECOVER").is_some() {
        for (i, entry) in entries().into_iter().take(5).enumerate() {
            let ext = if i % 2 == 0 { "flac" } else { "mp3" };
            let path = music
                .join(&entry.folder)
                .join(format!("{}.{ext}", entry.file));
            let title = format!("Cover {i:03} {}", entry.kind);
            embed(
                &path,
                &title,
                "Album jpeg-small 0",
                "Corpus Artist 0",
                Some((jpeg(500, 9000 + i as u32), MimeType::Jpeg)),
            );
        }
        eprintln!("recovered 5 pictures under {}", music.display());
        return;
    }
    assert!(
        !music.exists(),
        "corpus already exists: {}",
        music.display()
    );
    let data = root.join("app-data");
    std::fs::create_dir_all(&data).unwrap();
    let mut kinds = std::collections::BTreeMap::<&str, (usize, usize)>::new();
    for (i, entry) in entries().into_iter().enumerate() {
        let ext = if i % 2 == 0 { "flac" } else { "mp3" };
        let dir = music.join(&entry.folder);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join(format!("{}.{ext}", entry.file));
        std::fs::copy(fixtures.join(format!("tone.{ext}")), &path).unwrap();
        let title = format!("Cover {i:03} {}", entry.kind);
        let album = format!("Album {} {}", entry.kind, i / 12);
        let artist = format!("Corpus Artist {}", i % 9);
        let bytes = entry.picture.as_ref().map(|p| p.0.len()).unwrap_or(0);
        embed(&path, &title, &album, &artist, entry.picture);
        let slot = kinds.entry(entry.kind).or_default();
        slot.0 += 1;
        slot.1 = slot.1.max(bytes);
    }
    let catalog = crate::catalog::Catalog::new(data.join("library-tauri-v1.sqlite"));
    catalog.scan(music.clone()).unwrap();
    for (kind, (count, max)) in kinds {
        eprintln!("corpus {kind}: {count} files, largest picture {max} bytes");
    }
    eprintln!("corpus music={} data={}", music.display(), data.display());
}
````

## File: src-tauri/src/test_support/mod.rs
````rust
mod cover_corpus;
````

## File: src-tauri/src/lib.rs
````rust
mod artwork;
mod audio;
mod catalog;
mod commands;
mod infrastructure;
mod platform;
mod playback;
#[cfg(test)]
mod test_support;

pub use audio::player;
pub use playback::lyrics;

use playback::controller::{Action, Controller, PlayerSnapshot};
use std::sync::{Arc, Mutex};
type Shared = Arc<Mutex<Controller>>;
async fn execute(state: Shared, action: Action) -> Result<PlayerSnapshot, String> {
    tauri::async_runtime::spawn_blocking(move || {
        state
            .lock()
            .map_err(|_| "播放器控制器不可用".to_string())?
            .execute(action)
    })
    .await
    .map_err(|e| e.to_string())?
}
#[tauri::command]
async fn player_get_state(state: tauri::State<'_, Shared>) -> Result<PlayerSnapshot, String> {
    execute(state.inner().clone(), Action::State).await
}
#[tauri::command]
async fn player_open_file(state: tauri::State<'_, Shared>) -> Result<PlayerSnapshot, String> {
    let path = tauri::async_runtime::spawn_blocking(|| {
        rfd::FileDialog::new()
            .add_filter("音频 MP3 / FLAC / WAV", &["mp3", "flac", "wav"])
            .pick_file()
    })
    .await
    .map_err(|e| e.to_string())?;
    execute(
        state.inner().clone(),
        path.map(Action::Open).unwrap_or(Action::State),
    )
    .await
}
#[tauri::command]
async fn player_play(state: tauri::State<'_, Shared>) -> Result<PlayerSnapshot, String> {
    execute(state.inner().clone(), Action::Play).await
}
#[tauri::command]
async fn player_pause(state: tauri::State<'_, Shared>) -> Result<PlayerSnapshot, String> {
    execute(state.inner().clone(), Action::Pause).await
}
#[tauri::command]
async fn player_stop(state: tauri::State<'_, Shared>) -> Result<PlayerSnapshot, String> {
    execute(state.inner().clone(), Action::Stop).await
}
#[tauri::command]
async fn player_seek(
    position_ms: f64,
    state: tauri::State<'_, Shared>,
) -> Result<PlayerSnapshot, String> {
    execute(state.inner().clone(), Action::Seek(position_ms)).await
}
#[tauri::command]
async fn player_set_volume(
    volume: f64,
    state: tauri::State<'_, Shared>,
) -> Result<PlayerSnapshot, String> {
    execute(state.inner().clone(), Action::Volume(volume)).await
}
#[derive(serde::Serialize)]
struct BridgeReply {
    nonce: String,
    backend: &'static str,
    snapshot: PlayerSnapshot,
}
#[tauri::command]
async fn bridge_probe(
    nonce: String,
    state: tauri::State<'_, Shared>,
) -> Result<BridgeReply, String> {
    if nonce.len() > 128 {
        return Err("验证参数过长".into());
    }
    Ok(BridgeReply {
        nonce,
        backend: "rust",
        snapshot: execute(state.inner().clone(), Action::State).await?,
    })
}
#[tauri::command]
async fn player_configure(
    mode: Option<String>,
    offset: Option<i64>,
    path: Option<String>,
    target: Option<usize>,
    state: tauri::State<'_, Shared>,
) -> Result<PlayerSnapshot, String> {
    let action = if let Some(mode) = mode {
        Action::Mode(mode)
    } else if let Some(offset) = offset {
        Action::Offset(offset)
    } else if let Some(path) = path {
        if let Some(target) = target {
            Action::Reorder(path, target)
        } else {
            Action::Select(path)
        }
    } else {
        return Err("缺少操作参数".into());
    };
    execute(state.inner().clone(), action).await
}
#[tauri::command]
async fn player_lyrics(state: tauri::State<'_, Shared>) -> Result<Option<lyrics::Lyrics>, String> {
    let path = state.lock().map_err(|_| "播放器不可用")?.current_path();
    tauri::async_runtime::spawn_blocking(move || path.and_then(|p| lyrics::load_for_track(&p)))
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
async fn player_artwork(
    state: tauri::State<'_, Shared>,
    catalog: tauri::State<'_, Arc<catalog::Catalog>>,
    covers: tauri::State<'_, Arc<artwork::covers::Store>>,
) -> Result<Option<String>, String> {
    let path = state.lock().map_err(|_| "播放器不可用")?.current_path();
    let Some(path) = path else {
        return Ok(None);
    };
    let path_text = path.to_string_lossy().to_string();
    let catalog = catalog.inner().clone();
    let covers = covers.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let edge = 960;
        let cover = match catalog.track_by_path(&path_text)? {
            Some(track) => covers.for_track(track.id, std::path::Path::new(&track.path), edge),
            None => covers.for_playback(&path, edge),
        };
        cover.map_err(|e| e.code().to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}
pub fn run() {
    use tauri::Manager;
    let context = tauri::generate_context!();
    if let Some(root) = portable_root_from(context.config()) {
        if let Err(error) = ensure_writable(&root)
            .and_then(|_| ensure_writable(&root.join("caches")))
            .and_then(|_| ensure_writable(&root.join("logs")))
        {
            report_portable_error(&error);
            std::process::exit(1);
        }
    }
    match tauri::Builder::default()
        .setup(|app| {
            let directory = app.path().app_data_dir()?;
            let cache = app.path().app_cache_dir()?;
            let log = app.path().app_log_dir()?;
            if let Err(error) = ensure_writable(&directory)
                .and_then(|_| ensure_writable(&cache))
                .and_then(|_| ensure_writable(&log))
            {
                return Err(std::io::Error::other(error).into());
            }
            app.manage(Arc::new(catalog::Catalog::new(
                directory.join("library-tauri-v1.sqlite"),
            )));
            app.manage(Arc::new(artwork::covers::Store::new(cache.join("covers"))));
            if let Ok(mut controller) = app.state::<Shared>().lock() {
                if let Err(error) = controller.attach_session(directory.join("playback-v1.json")) {
                    eprintln!("{error}");
                }
            }
            platform::desktop::setup(app)?;
            platform::media::setup(app);
            let handle = app.handle().clone();
            // Queue progression follows engine EOF on Rust's worker even with the UI hidden.
            let player = app.state::<Shared>().inner().clone();
            std::thread::Builder::new()
                .name("player-controller".into())
                .spawn(move || {
                    let mut last_path = None;
                    loop {
                        if handle
                            .state::<platform::desktop::Desktop>()
                            .exiting
                            .load(std::sync::atomic::Ordering::SeqCst)
                        {
                            break;
                        }
                        let snapshot = player
                            .lock()
                            .ok()
                            .and_then(|mut c| c.execute(Action::State).ok());
                        if let Some(snapshot) = snapshot {
                            if snapshot.playback_state == "playing"
                                && snapshot.current_track.as_ref().map(|t| &t.path)
                                    != last_path.as_ref()
                            {
                                if let Some(track) = &snapshot.current_track {
                                    if let Err(e) = handle
                                        .state::<Arc<catalog::Catalog>>()
                                        .record_listen(&track.path)
                                    {
                                        eprintln!("记录播放失败：{e}");
                                    }
                                }
                                last_path = snapshot.current_track.as_ref().map(|t| t.path.clone());
                            }
                            if snapshot.playback_state == "idle" {
                                last_path = None;
                            }
                            platform::desktop::update(&handle, &snapshot);
                            platform::media::update(&handle, &snapshot);
                        }
                        std::thread::sleep(std::time::Duration::from_millis(200));
                    }
                })?;
            Ok(())
        })
        .on_window_event(platform::desktop::window_event)
        .manage(Arc::new(Mutex::new(Controller::new())))
        .invoke_handler(tauri::generate_handler![
            platform::desktop::desktop_preferences,
            platform::media::media_status,
            player_configure,
            player_lyrics,
            player_artwork,
            bridge_probe,
            player_get_state,
            player_open_file,
            player_play,
            player_pause,
            player_stop,
            player_seek,
            player_set_volume,
            commands::catalog::library_query,
            commands::catalog::library_summary,
            commands::catalog::library_home,
            commands::catalog::library_scan,
            commands::catalog::library_progress,
            commands::catalog::library_cancel,
            commands::catalog::library_track,
            commands::catalog::library_audio_info,
            commands::catalog::library_cover,
            commands::catalog::library_thumb,
            commands::catalog::library_queue_thumb,
            commands::catalog::library_cover_stats,
            commands::catalog::library_play,
            commands::catalog::library_favorite,
            commands::catalog::library_playlist,
            commands::catalog::library_playlist_edit,
            commands::catalog::library_import,
            commands::catalog::library_queue,
            commands::catalog::library_queue_tracks
        ])
        .build(context)
    {
        Ok(built) => built.run(|app, event| {
            if matches!(event, tauri::RunEvent::ExitRequested { .. }) {
                platform::media::shutdown(app);
                platform::desktop::shutdown(app);
            }
        }),
        Err(error) => {
            report_portable_error(&error.to_string());
            std::process::exit(1);
        }
    }
}

fn report_portable_error(error: &str) {
    let description = if error.contains("app-data") {
        format!(
            "无法写入程序旁边的 app-data 目录，Music Mine 没有启动。\n便携版不会改用 Windows 用户目录中的曲库或设置。\n请把整个文件夹放到可以写入的位置后重试。\n\n{error}"
        )
    } else {
        format!("Music Mine 没有启动。\n\n{error}")
    };
    let _ = rfd::MessageDialog::new()
        .set_title("Music Mine")
        .set_level(rfd::MessageLevel::Error)
        .set_description(description)
        .show();
}

fn portable_root_from(config: &tauri::Config) -> Option<std::path::PathBuf> {
    let configured = match config.app.app_directories_override.as_ref() {
        Some(tauri::utils::config::AppDirectoriesOverride::Root(path)) => path,
        _ => return None,
    };
    let exe = std::env::current_exe().ok()?;
    exe_relative_root(configured, exe.parent()?)
}

fn exe_relative_root(
    configured: &std::path::Path,
    exe_dir: &std::path::Path,
) -> Option<std::path::PathBuf> {
    if configured.is_absolute() {
        return None;
    }
    let text = configured.to_string_lossy();
    if text.starts_with('$') || text == "." || text == "./" || text == ".\\" {
        return None;
    }
    let relative = text.trim_start_matches("./").trim_start_matches(".\\");
    if relative.is_empty() {
        return None;
    }
    Some(exe_dir.join(relative))
}

fn ensure_writable(dir: &std::path::Path) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("无法创建 {}：{e}", dir.display()))?;
    let probe = dir.join(".write-probe");
    std::fs::write(&probe, b"1").map_err(|e| format!("无法写入 {}：{e}", dir.display()))?;
    let _ = std::fs::remove_file(&probe);
    Ok(())
}

#[cfg(test)]
mod portable_dir_tests {
    use super::{ensure_writable, exe_relative_root};
    use std::path::{Path, PathBuf};

    #[test]
    fn writable_probe_leaves_no_marker() {
        let dir = std::env::temp_dir().join(format!("music-mine-write-{}", std::process::id()));
        ensure_writable(&dir).expect("temp dir");
        assert!(!dir.join(".write-probe").exists());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn portable_root_is_beside_the_exe_and_rejects_the_exe_directory() {
        let exe_dir = Path::new(r"D:\Music Mine");
        assert_eq!(
            exe_relative_root(Path::new("./app-data"), exe_dir),
            Some(PathBuf::from(r"D:\Music Mine\app-data"))
        );
        assert_eq!(exe_relative_root(Path::new("."), exe_dir), None);
        assert_eq!(exe_relative_root(Path::new("./"), exe_dir), None);
        assert_eq!(
            exe_relative_root(Path::new(r"E:\absolute\app-data"), exe_dir),
            None
        );
    }
}
````

## File: src-tauri/src/main.rs
````rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {
    music_mine_desktop::run();
}
````

## File: src-tauri/tests/audio.rs
````rust
use music_mine_desktop::player::{analyze_file, PlaybackState, Player};
use std::path::PathBuf;
use std::time::{Duration, Instant};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn encoded_formats_decode_and_play_with_real_device_controls() {
    let player = Player::new(None);
    player.set_volume(0.0);
    player.synchronize().unwrap();
    for name in ["tone.mp3", "tone.flac", "tone.wav"] {
        let report = analyze_file(&fixture(name), 5.0).unwrap();
        assert!(!report.is_silent(), "{name}: {report:?}");
        assert!(report.peak > 0.35 && report.rms > 0.2);
        assert!((report.seconds - 3.0).abs() < 0.25);
        player.prepare(fixture(name), 3000, 0);
        let ready = player.synchronize().unwrap();
        assert_eq!(ready.state, PlaybackState::Paused, "{ready:?}");
        assert!(ready.error.is_none(), "{ready:?}");
        player.resume();
        player.synchronize().unwrap();
        let deadline = Instant::now() + Duration::from_secs(4);
        while player.status().position < Duration::from_millis(150) && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(20));
        }
        assert!(
            player.status().position >= Duration::from_millis(150),
            "{name}"
        );
        player.pause();
        let paused = player.synchronize().unwrap();
        std::thread::sleep(Duration::from_millis(120));
        assert_eq!(
            player.status().position,
            paused.position,
            "{name}: paused clock moved"
        );
        player.seek(1.0);
        let sought = player.synchronize().unwrap();
        assert!(
            (sought.position.as_secs_f64() - 1.0).abs() < 0.1,
            "{sought:?}"
        );
        player.set_volume(0.25);
        assert_eq!(player.synchronize().unwrap().volume, 0.25);
        player.set_volume(0.0);
        player.resume();
        player.synchronize().unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        while player.status().state != PlaybackState::Stopped && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(20));
        }
        assert_eq!(
            player.status().state,
            PlaybackState::Stopped,
            "{name}: did not end"
        );
        assert!(player.status().error.is_none());
        player.stop();
        let stopped = player.synchronize().unwrap();
        assert_eq!(stopped.position, Duration::ZERO);
    }
}

#[test]
fn damaged_file_reports_failure_without_progress() {
    let path = std::env::temp_dir().join(format!(
        "music-mine-tauri-broken-{}.mp3",
        std::process::id()
    ));
    std::fs::write(&path, b"not an audio file").unwrap();
    let player = Player::new(None);
    player.play(path.clone(), 3000);
    let status = player.synchronize().unwrap();
    assert_eq!(status.state, PlaybackState::Stopped);
    assert!(status.error.is_some());
    assert_eq!(status.position, Duration::ZERO);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn rapid_track_changes_leave_only_the_last_track() {
    let player = Player::new(None);
    player.set_volume(0.0);
    for name in ["tone.mp3", "tone.wav", "tone.flac"] {
        player.prepare(fixture(name), 3000, 0);
    }
    let status = player.synchronize().unwrap();
    assert_eq!(
        status.current_path,
        Some(fixture("tone.flac").to_string_lossy().into_owned())
    );
    assert_eq!(status.state, PlaybackState::Paused);
    assert_eq!(status.position, Duration::ZERO);
}
````

## File: src-tauri/tests/lyrics.rs
````rust
//! Lyrics loading from disk: sidecar `.lrc`, encodings, and embedded tags.
//! Uses temp copies of the fixtures; no original music is touched.

use std::path::{Path, PathBuf};

use music_mine_desktop::lyrics;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("music-mine-lyrics-{tag}"));
    let _ = std::fs::create_dir_all(&dir);
    dir
}

#[test]
fn sidecar_lrc_is_loaded_and_synced() {
    let dir = temp_dir("sidecar");
    let audio = dir.join("song.live.flac");
    std::fs::write(&audio, b"not really audio").unwrap(); // sidecar path only
    std::fs::write(
        dir.join("song.live.lrc"),
        "[ti:Demo]\n[offset:+100]\n[00:01.00]Line one\n[00:02.50]Line two\n",
    )
    .unwrap();

    let loaded = lyrics::load_for_track(&audio).expect("sidecar lyrics");
    assert_eq!(loaded.source, "song.live.lrc");
    assert!(loaded.synced);
    assert_eq!(loaded.lines.len(), 2);
    assert_eq!(loaded.lines[0].time_ms, 1_100);
    assert_eq!(loaded.lines[1].text, "Line two");
    assert_eq!(loaded.index_at(2_600), Some(1));
}

#[test]
fn gbk_encoded_sidecar_is_decoded() {
    let dir = temp_dir("gbk");
    let audio = dir.join("cjk.flac");
    std::fs::write(&audio, b"x").unwrap();
    let text = "[00:01.00]你好，世界\n[00:03.00]再见\n";
    let (gbk, _, _) = encoding_rs::GBK.encode(text);
    std::fs::write(dir.join("cjk.lrc"), gbk.as_ref()).unwrap();

    let loaded = lyrics::load_for_track(&audio).expect("gbk lyrics");
    assert_eq!(loaded.lines[0].text, "你好，世界");
    assert_eq!(loaded.lines[1].text, "再见");
}

#[test]
fn embedded_lyrics_are_read_from_tags() {
    let dir = temp_dir("embedded");
    let audio = dir.join("tagged.flac");
    std::fs::copy(fixture("tone.flac"), &audio).unwrap();

    {
        use lofty::config::WriteOptions;
        use lofty::file::{AudioFile, TaggedFileExt};
        use lofty::tag::ItemKey;
        let mut tagged = lofty::read_from_path(&audio).expect("read copy");
        let tag = tagged.primary_tag_mut().expect("flac has a tag");
        tag.insert_text(
            ItemKey::Lyrics,
            "[00:00.50]Embedded one\n[00:02.00]Embedded two".to_string(),
        );
        tagged
            .save_to_path(&audio, WriteOptions::default())
            .expect("write tag");
    }

    let loaded = lyrics::load_for_track(&audio).expect("embedded lyrics");
    assert_eq!(loaded.source, "内嵌歌词");
    assert!(loaded.synced);
    assert_eq!(loaded.lines.len(), 2);
    assert_eq!(loaded.lines[1].text, "Embedded two");
    assert_eq!(loaded.index_at(2_100), Some(1));
}

#[test]
fn sidecar_wins_over_embedded_and_unsynced_text_still_shows() {
    let dir = temp_dir("priority");
    let audio = dir.join("both.flac");
    std::fs::copy(fixture("tone.flac"), &audio).unwrap();
    {
        use lofty::config::WriteOptions;
        use lofty::file::{AudioFile, TaggedFileExt};
        use lofty::tag::ItemKey;
        let mut tagged = lofty::read_from_path(&audio).unwrap();
        let tag = tagged.primary_tag_mut().unwrap();
        tag.insert_text(ItemKey::Lyrics, "[00:00.20]embedded".to_string());
        tagged
            .save_to_path(&audio, WriteOptions::default())
            .unwrap();
    }
    std::fs::write(dir.join("both.lrc"), "only plain text\nsecond line\n").unwrap();

    let loaded = lyrics::load_for_track(&audio).expect("lyrics");
    assert_eq!(loaded.source, "both.lrc");
    assert!(!loaded.synced, "plain text has no timestamps");
    assert_eq!(loaded.lines.len(), 2);
    assert_eq!(loaded.index_at(1_000), None);
}

#[test]
fn a_track_without_lyrics_returns_none() {
    let dir = temp_dir("none");
    let audio = dir.join("empty.flac");
    std::fs::copy(fixture("tone.flac"), &audio).unwrap();
    let _ = std::fs::remove_file(dir.join("empty.lrc"));
    assert!(lyrics::load_for_track(&audio).is_none());
}

#[test]
fn a_large_lyrics_file_is_still_fast_to_parse() {
    // 6000 timestamped lines: parsing must stay well under a frame budget.
    let mut text = String::new();
    for i in 0..6000 {
        let total_ms = i * 100;
        let minutes = total_ms / 60_000;
        let seconds = (total_ms % 60_000) / 1000;
        let millis = total_ms % 1000;
        text.push_str(&format!(
            "[{minutes:02}:{seconds:02}.{millis:03}]line {i}\n"
        ));
    }
    let start = std::time::Instant::now();
    let parsed = lyrics::parse_lrc(&text);
    let elapsed = start.elapsed();
    assert_eq!(parsed.lines.len(), 6000);
    assert!(
        elapsed < std::time::Duration::from_millis(500),
        "parsing 6000 lines took {elapsed:?}"
    );
    assert_eq!(
        parsed
            .index_at(30_050)
            .map(|i| parsed.lines[i].text.clone()),
        Some("line 300".to_string())
    );
}
````

## File: src-tauri/build.rs
````rust
fn main() {
    tauri_build::build()
}
````

## File: src-tauri/Cargo.toml
````toml
[package]
name = "music-mine-desktop"
version = "0.1.0"
edition = "2021"

[lib]
name = "music_mine_desktop"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2", features = [] }

[features]
custom-protocol = ["tauri/custom-protocol"]

[dependencies]
tauri = { version = "2", features = ["tray-icon", "image-png"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
symphonia = { version = "0.5", features = ["mp3", "flac", "wav", "pcm"] }
cpal = "0.15"
lofty = "0.21"
rfd = "0.15"
rusqlite = { version = "0.32", features = ["bundled", "backup"] }
base64 = "0.22"
# JPEG decoding is not in the `image` crate already pulled in by Tauri's PNG feature.
image = { version = "0.25", default-features = false, features = ["jpeg", "png"] }

encoding_rs = "0.8"
chardetng = "0.1"

[target.'cfg(windows)'.dependencies]
windows = { version = "0.58", features = ["Media", "Foundation", "Win32_System_WinRT", "Win32_Foundation"] }
````

## File: src-tauri/tauri.conf.json
````json
{"$schema":"https://schema.tauri.app/config/2","productName":"Music Mine","version":"0.1.0","identifier":"local.musicmine.desktop","build":{"beforeBuildCommand":"pnpm build","beforeDevCommand":"pnpm dev","devUrl":"http://localhost:1420","frontendDist":"../dist"},"app":{"windows":[{"title":"Music Mine","width":1280,"height":850,"minWidth":900,"minHeight":640,"theme":"Dark","backgroundColor":"#121414"}],"security":{"csp":"default-src 'self'; img-src 'self' data:; style-src 'self' 'unsafe-inline'; connect-src ipc: http://ipc.localhost"}},"bundle":{"active":false}}
````

## File: src-tauri/tauri.portable.conf.json
````json
{
  "app": {
    "appDirectoriesOverride": "./app-data"
  }
}
````

## File: tests/harness/index.html
````html
<!doctype html>
<html lang="zh-CN" data-theme="dark">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width,initial-scale=1" />
    <meta name="theme-color" content="#121414" />
    <title>Music Mine layout harness</title>
    <script src="/theme-init.js"></script>
    <script src="/tests/harness/mock-ipc.js"></script>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
````

## File: tests/harness/mock-ipc.js
````javascript
// Browser-only layout harness. Loaded exclusively by tests/harness/index.html; the app never imports it.
// It answers Tauri commands with a synthetic in-memory library so lists, dialogs and the queue can be
// inspected in a normal browser. Playback state never advances on its own.
const TOTAL = Number(new URLSearchParams(location.search).get('tracks') ?? 471);
const tracks = Array.from({ length: TOTAL }, (_, i) => ({
  id: i + 1,
  path: `C:\\Fixture\\${String(i + 1).padStart(3, '0')}.flac`,
  title: `测试歌曲 ${i + 1}`,
  artist: `艺术家 ${i % 17}`,
  album: `专辑 ${i % 40}`,
  album_artist: '',
  genre: '',
  year: 2020,
  track_number: (i % 12) + 1,
  disc_number: null,
  duration_ms: 180000 + i * 997,
  added_at: TOTAL - i,
  album_source: 'tag',
}));
const playlists = [
  { id: 7, name: '本地', count: 0 },
  { id: 8, name: '夜晚', count: 2 },
];
const members = { 7: [], 8: [3, 5] };
function groups(kind) {
  const map = new Map();
  for (const t of tracks) {
    const name = kind === 'album' ? t.album : t.artist;
    const current = map.get(name);
    if (current) current.count++;
    else
      map.set(name, {
        key: name,
        name,
        artist: kind === 'album' ? t.artist : '',
        count: 1,
        cover_id: t.id,
      });
  }
  return [...map.values()];
}
const queue = tracks.slice(0, 12).map(({ path, title, artist, album, duration_ms }) => ({
  path,
  title,
  artist,
  album,
  duration_ms,
}));
let snapshot = {
  revision: 1,
  track_id: queue[0].path,
  current_track: queue[0],
  playback_state: 'paused',
  position_ms: 42000,
  duration_ms: queue[0].duration_ms,
  volume: 0.7,
  playback_mode: 'sequential',
  lyrics_offset_ms: 0,
  output_device_id: 'Fixture output',
  error: null,
  queue,
};
const prefs = { close_to_tray: false, queue_open: false, theme: null };
const log = [];
window.__mockLog = log;
window.__mockSnapshot = () => snapshot;
function page(q) {
  let list = tracks;
  if (q.playlist != null) list = (members[q.playlist] ?? []).map((id) => tracks[id - 1]);
  if (q.search) {
    const s = q.search.toLowerCase();
    list = list.filter((t) => (t.title + t.artist + t.album).toLowerCase().includes(s));
  }
  if (q.artist) list = list.filter((t) => t.artist === q.artist);
  if (q.album) list = list.filter((t) => t.album === q.album);
  if (q.sort === 'title')
    list = [...list].sort((a, b) => a.title.localeCompare(b.title) || a.id - b.id);
  const offset = q.offset ?? 0,
    limit = q.limit ?? 100;
  return { tracks: list.slice(offset, offset + limit), total: list.length, favorites: [] };
}
const handlers = {
  bridge_probe: (a) => ({ nonce: a.nonce, backend: 'rust', snapshot }),
  player_get_state: () => snapshot,
  desktop_preferences: (a) => {
    if (a.closeToTray != null) prefs.close_to_tray = a.closeToTray;
    if (a.queueOpen != null) prefs.queue_open = a.queueOpen;
    if (a.theme !== undefined && a.theme !== null) prefs.theme = a.theme;
    return { ...prefs };
  },
  media_status: () => 'Fixture',
  library_progress: () => ({
    running: false,
    visited: 0,
    imported: 0,
    current: '',
    cancelled: false,
    errors: [],
  }),
  library_summary: () => ({
    tracks: tracks.length,
    albums: groups('album'),
    artists: groups('artist'),
    folders: ['C:\\Fixture'],
    playlists: playlists.map((p) => ({ ...p, count: (members[p.id] ?? []).length })),
  }),
  library_query: (a) => page(a.query),
  library_home: () => ({
    recent: tracks.slice(0, 6),
    artists: groups('artist')
      .slice(0, 8)
      .map((g) => [g.name, g.count]),
  }),
  library_cover: () => null,
  library_thumb: () => null,
  library_queue_thumb: () => null,
  library_track: (a) => tracks[a.id - 1],
  library_audio_info: () => ({ format: 'Flac', sample_rate: 44100, channels: 2, bitrate: 900 }),
  library_playlist: (a) => {
    if (a.name) {
      if (playlists.some((p) => p.name === a.name.trim()))
        throw `已存在名为「${a.name.trim()}」的歌单`;
      const id = Math.max(...playlists.map((p) => p.id)) + 1;
      playlists.push({ id, name: a.name.trim(), count: 0 });
      members[id] = [];
      return id;
    }
    members[a.playlistId].push(a.trackId);
    return null;
  },
  library_playlist_edit: (a) => {
    if (a.action === 'delete') {
      const i = playlists.findIndex((p) => p.id === a.playlistId);
      if (i >= 0) playlists.splice(i, 1);
      delete members[a.playlistId];
    }
    if (a.action === 'rename') {
      const p = playlists.find((p) => p.id === a.playlistId);
      if (p && a.name) p.name = a.name;
    }
    if (a.action === 'remove_track' && members[a.playlistId])
      members[a.playlistId] = members[a.playlistId].filter((id) => id !== a.trackId);
    return null;
  },
  library_queue_tracks: () => snapshot,
  library_play: () => snapshot,
  library_queue: () => snapshot,
  player_lyrics: () => ({
    lines: Array.from({ length: 30 }, (_, i) => ({
      time_ms: i * 4000,
      text: `歌词第 ${i + 1} 行`,
    })),
    offset_ms: 0,
    synced: true,
    source: 'fixture.lrc',
  }),
  player_artwork: () => null,
  player_configure: () => snapshot,
  player_play: () => snapshot,
  player_pause: () => snapshot,
  player_stop: () => snapshot,
  player_open_file: () => snapshot,
  player_seek: () => snapshot,
  player_set_volume: async (a) => {
    await new Promise((r) => setTimeout(r, 40));
    snapshot = {
      ...snapshot,
      volume: Math.min(1, Math.max(0, a.volume)),
      revision: snapshot.revision + 1,
    };
    return snapshot;
  },
  library_scan: () => null,
  library_cancel: () => null,
  library_cover_stats: () => ({}),
  library_import: () => null,
};
let callbackId = 0;
window.isTauri = true;
window.__TAURI_INTERNALS__ = {
  metadata: {
    currentWindow: { label: 'main' },
    currentWebview: { windowLabel: 'main', label: 'main' },
  },
  transformCallback: (cb) => {
    const id = ++callbackId;
    window['_' + id] = cb;
    return id;
  },
  unregisterCallback: (id) => {
    delete window['_' + id];
  },
  invoke: async (cmd, args = {}) => {
    log.push({ cmd, args });
    const h = handlers[cmd];
    if (cmd.startsWith('plugin:')) return null;
    if (!h) throw `mock: unhandled command ${cmd}`;
    return structuredClone(await h(args));
  },
};
````

## File: tests/harness/primitives.html
````html
<!doctype html>
<html lang="zh-CN" data-theme="dark">
  <head>
    <meta charset="UTF-8" />
    <title>Interaction primitives harness</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/tests/harness/primitives.ts"></script>
  </body>
</html>
````

## File: tests/harness/primitives.ts
````typescript
import { mount } from 'svelte';
import '../../src/app.css';
import PrimitivesFixture from './PrimitivesFixture.svelte';
mount(PrimitivesFixture, { target: document.getElementById('app')! });
````

## File: tests/harness/PrimitivesFixture.svelte
````svelte
<script lang="ts">
  import ContextMenu from '../../src/lib/components/ContextMenu.svelte';
  import Modal from '../../src/lib/components/Modal.svelte';
  import Slider from '../../src/lib/components/Slider.svelte';
  import { tooltip } from '../../src/lib/components/tooltip';
  import { openMenu, ui } from '../../src/lib/stores/ui.svelte';
  let selected = $state('');
  let modal = $state(false);
  let busy = $state(false);
  let volume = $state(50);
  function menu(at: MouseEvent | HTMLElement) {
    openMenu(at, [
      { label: '播放', action: () => (selected = 'play') },
      { label: '不可用', disabled: true, action: () => (selected = 'disabled') },
      { label: '下一首播放', action: () => (selected = 'next') },
      { kind: 'separator' },
      { label: '加入播放队列', action: () => (selected = 'queue') },
      {
        label: '失败操作',
        action: () => {
          throw new Error('test action failed');
        },
      },
    ]);
  }
</script>

<main style="padding: 24px">
  <button oncontextmenu={menu} onclick={(e) => menu(e.currentTarget)}>操作</button>
  <button onclick={() => (modal = true)}>打开弹窗</button>
  <button>外部按钮</button>
  <Slider value={volume} label="测试音量" oninput={(v) => (volume = v)} />
  <output aria-label="选择结果">{selected}</output>
  <output aria-label="音量结果">{volume}</output>
  <output aria-label="错误">{ui.toast?.message ?? ''}</output>
</main>
<button style="position:fixed;right:8px;bottom:8px" oncontextmenu={menu} use:tooltip={'边缘提示'}
  >边缘操作</button
>
{#if modal}
  <Modal
    onclose={() => (modal = false)}
    dismissible={!busy}
    initialFocus="#fixture-input"
    aria-label="测试弹窗"
  >
    <div style="padding:24px;min-height:260px">
      <label for="fixture-input">名称</label><input
        id="fixture-input"
        class="input"
        value="原始内容"
      />
      <button onclick={() => (busy = !busy)}>切换忙碌</button>
      <button onclick={() => (modal = false)}>关闭</button>
    </div>
  </Modal>
{/if}
<ContextMenu />
````

## File: tests/harness/select.html
````html
<!doctype html>
<html lang="zh-CN" data-theme="dark">
  <head>
    <meta charset="UTF-8" />
    <title>Select behavior harness</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/tests/harness/select.ts"></script>
  </body>
</html>
````

## File: tests/harness/select.ts
````typescript
import { mount } from 'svelte';
import '../../src/app.css';
import SelectFixture from './SelectFixture.svelte';

mount(SelectFixture, { target: document.getElementById('app')! });
````

## File: tests/harness/SelectFixture.svelte
````svelte
<script lang="ts">
  import Select from '../../src/lib/components/Select.svelte';
  import Modal from '../../src/lib/components/Modal.svelte';
  let value = $state('');
  let changes = $state<string[]>([]);
  let dialog = $state(false);
  const options = [
    { value: '', label: '选择模式' },
    { value: 'sequence', label: '顺序播放' },
    { value: 'shuffle', label: '随机播放' },
    { value: 'repeat', label: '单曲循环' },
  ];
</script>

<main style="padding: 24px">
  <Select label="播放模式" bind:value {options} onchange={(v) => changes.push(v)} />
  <button onclick={() => (value = 'shuffle')}>外部更新</button>
  <Select label="禁用选择" disabled {options} />
  <Select label="空列表" options={[]} />
  <output aria-label="绑定值">{value || '(empty)'}</output>
  <output aria-label="回调">{JSON.stringify(changes)}</output>
  <button onclick={() => (dialog = true)}>打开详情</button>
</main>

<div style="position: fixed; bottom: 8px; right: 8px">
  <Select
    label="长列表"
    options={Array.from({ length: 60 }, (_, i) => ({ value: String(i), label: `项目 ${i}` }))}
  />
</div>

{#if dialog}
  <Modal onclose={() => (dialog = false)} aria-label="选择器弹窗">
    <div style="padding: 24px; min-height: 260px">
      <Select label="弹窗模式" bind:value {options} />
      <button onclick={() => (dialog = false)}>关闭详情</button>
    </div>
  </Modal>
{/if}
````

## File: tests/helpers/source-styles.mjs
````javascript
import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

/** Read actual CSS, including Svelte style blocks, independently of where a rule lives. */
export function sourceStyles({ excludeTokens = false } = {}) {
  function walk(dir) {
    return readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
      const path = join(dir, entry.name);
      if (entry.isDirectory()) return walk(path);
      if (excludeTokens && path.replaceAll('\\', '/') === 'src/lib/styles/tokens.css') return [];
      const source = readFileSync(path, 'utf8');
      if (path.endsWith('.css')) return [source];
      if (path.endsWith('.svelte'))
        return [...source.matchAll(/<style\b[^>]*>([\s\S]*?)<\/style>/g)].map((m) => m[1]);
      return [];
    });
  }
  return walk('src')
    .join('\n')
    .replace(/\/\*[\s\S]*?\*\//g, '');
}
````

## File: tests/perf/cdp-eval.mjs
````javascript
// Evaluates one expression in the running WebView over CDP: node tests/perf/cdp-eval.mjs <file-with-expression> [port]
import { readFileSync } from 'node:fs';
const [file, port = '9333'] = process.argv.slice(2);
const page = (await (await fetch(`http://127.0.0.1:${port}/json`)).json()).find(
  (t) => t.type === 'page',
);
const ws = new WebSocket(page.webSocketDebuggerUrl);
await new Promise((ok, fail) => {
  ws.onopen = ok;
  ws.onerror = fail;
});
ws.onmessage = ({ data }) => {
  const m = JSON.parse(data);
  if (m.id === 1) {
    console.log(
      JSON.stringify(m.result?.result?.value ?? m.result?.exceptionDetails ?? m.error, null, 1),
    );
    ws.close();
  }
};
ws.send(
  JSON.stringify({
    id: 1,
    method: 'Runtime.evaluate',
    params: { expression: readFileSync(file, 'utf8'), awaitPromise: true, returnByValue: true },
  }),
);
````

## File: tests/perf/compare-recover.mjs
````javascript
// Compares the visible rows' image hashes before (cover-<label>-RS.json) and after (cover-<label>-S.json) the recover step.
import { readFileSync } from 'node:fs';
for (const label of process.argv.slice(2)) {
  const pick = (file) =>
    JSON.parse(readFileSync(`tests/perf/out/cover-${label}-${file}.json`, 'utf8')).find((r) =>
      r.name.startsWith('S '),
    ).rows;
  const before = new Map(pick('RS').map((r) => [r.i, r.hash]));
  const after = pick('S');
  const changed = after
    .filter((r) => before.has(r.i) && before.get(r.i) !== r.hash)
    .map((r) => r.i);
  const same = after.filter((r) => before.get(r.i) === r.hash).map((r) => r.i);
  console.log(JSON.stringify({ label, changed, same }));
}
````

## File: tests/perf/queue-cover-profile.mjs
````javascript
// Queue cover loading measurements against a running portable exe on the generated corpus.
// Launch first with tests/perf/stage-cover-run.ps1, then:
//   node tests/perf/queue-cover-profile.mjs <label> <scenarios=A,B,C,D,E,F,H> [port=9333]
// Paths and image data never leave the page: rows are identified by the corpus index in the title.
import { writeFileSync, mkdirSync } from 'node:fs';
import { execFileSync } from 'node:child_process';

const [label = 'run', scenarioArg = 'A,B,C,D,E,F,H', port = '9333'] = process.argv.slice(2);
const targets = await (await fetch(`http://127.0.0.1:${port}/json`)).json();
const page = targets.find((t) => t.type === 'page');
const ws = new WebSocket(page.webSocketDebuggerUrl);
await new Promise((ok, fail) => {
  ws.onopen = ok;
  ws.onerror = fail;
});
let seq = 0;
const pending = new Map();
// Tauri IPC goes through fetch to http://ipc.localhost/<command>; the Network domain sees every call.
const net = [];
const byRequest = new Map();
const rowOf = (path) => {
  const m = /[\\/](\d{3}) [^\\/]*$/.exec(path ?? '');
  if (m) return Number(m[1]);
  const s = /same-name[\\/](\d\d)[\\/]/.exec(path ?? '');
  return s ? 'same' + s[1] : null;
};
function onEvent(m) {
  const p = m.params;
  if (
    m.method === 'Network.requestWillBeSent' &&
    p.request.url.startsWith('http://ipc.localhost/')
  ) {
    const cmd = decodeURIComponent(p.request.url.slice(21).split('?')[0]);
    if (!/thumb|artwork/.test(cmd)) return;
    let args = {};
    try {
      args = JSON.parse(p.request.postData ?? '{}');
    } catch {}
    const rec = {
      cmd,
      row: rowOf(args.path),
      t0: p.timestamp,
      t1: 0,
      outcome: 'pending',
      bytes: 0,
    };
    net.push(rec);
    byRequest.set(p.requestId, rec);
  } else if (m.method === 'Network.loadingFinished' && byRequest.has(p.requestId)) {
    const rec = byRequest.get(p.requestId);
    rec.t1 = p.timestamp;
    rec.bytes = p.encodedDataLength;
    send('Network.getResponseBody', { requestId: p.requestId })
      .then(({ body }) => {
        let v = null;
        try {
          v = JSON.parse(body);
        } catch {}
        const data = typeof v === 'string' ? v : v?.data;
        rec.outcome = data
          ? 'data'
          : v && typeof v === 'object'
            ? 'none:' + (v.status ?? '?')
            : 'null';
        rec.source = v?.source;
        rec.timing = v?.timing;
      })
      .catch(() => {
        rec.outcome = 'body?';
      });
  } else if (m.method === 'Network.loadingFailed' && byRequest.has(p.requestId)) {
    const rec = byRequest.get(p.requestId);
    rec.t1 = p.timestamp;
    rec.outcome = 'error';
  }
}
ws.onmessage = ({ data }) => {
  const m = JSON.parse(data);
  if (m.id && pending.has(m.id)) {
    const { ok, fail } = pending.get(m.id);
    pending.delete(m.id);
    m.error ? fail(new Error(m.error.message)) : ok(m.result);
  } else if (m.method) onEvent(m);
};
const send = (method, params = {}) =>
  new Promise((ok, fail) => {
    const id = ++seq;
    pending.set(id, { ok, fail });
    ws.send(JSON.stringify({ id, method, params }));
  });
async function js(expression) {
  const r = await send('Runtime.evaluate', { expression, awaitPromise: true, returnByValue: true });
  if (r.exceptionDetails)
    throw new Error(r.exceptionDetails.exception?.description ?? r.exceptionDetails.text);
  return r.result.value;
}
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const q = (s) => JSON.stringify(s);

// Process CPU time and working set of this run's exe and its WebView2 children.
function processStats() {
  const script = `$r=Get-CimInstance Win32_Process | ? { $_.Name -eq 'music-mine-desktop.exe' -or ($_.Name -eq 'msedgewebview2.exe' -and $_.CommandLine -like '*mm-cover-corpus*run-${label}*') }; $cpu=($r | % { $_.KernelModeTime + $_.UserModeTime } | Measure-Object -Sum).Sum/10000; $ws=($r | Measure-Object WorkingSetSize -Sum).Sum/1MB; "$([math]::Round($cpu)) $([math]::Round($ws,1)) $($r.Count)"`;
  const [cpuMs, wsMb, procs] = execFileSync('powershell', ['-NoProfile', '-Command', script], {
    encoding: 'utf8',
  })
    .trim()
    .split(' ')
    .map(Number);
  return { cpuMs, wsMb, procs };
}
function cacheFiles() {
  const script = `$d=Join-Path $env:TEMP 'mm-cover-corpus\\run-${label}\\app-data\\caches\\covers'; $f=Get-ChildItem $d -File -ErrorAction SilentlyContinue; "$(($f|?{$_.Extension -eq '.jpg'}).Count) $(($f|?{$_.Extension -eq '.none'}).Count) $(($f|?{$_.Extension -notin '.jpg','.none'}).Count)"`;
  const [jpg, none, other] = execFileSync('powershell', ['-NoProfile', '-Command', script], {
    encoding: 'utf8',
  })
    .trim()
    .split(' ')
    .map(Number);
  return { jpg, none, other };
}

async function install() {
  await send('Network.enable', { maxPostDataSize: 4096 });
  await js(`(()=>{if(window.__coverProbe)return;const P=window.__coverProbe={imgs:[],frames:[],on:true};
  const imgEvent=type=>e=>{const img=e.target;if(!(img instanceof HTMLImageElement)||!img.closest('.queue'))return;const row=img.closest('.queue-item');P.imgs.push({type,t:performance.now(),title:row?.querySelector('.queue-text strong')?.textContent});};
  document.addEventListener('load',imgEvent('load'),true);document.addEventListener('error',imgEvent('error'),true);
  const loop=t=>{if(P.on)P.frames.push(t);requestAnimationFrame(loop);};requestAnimationFrame(loop);
 })()`);
}
let windowStart = 0;
const reset = () => {
  windowStart = net.length;
  return js(
    `(()=>{const P=window.__coverProbe;P.imgs=[];P.frames=[];return performance.now();})()`,
  );
};
function maxInflight(calls) {
  const ev = [];
  for (const c of calls) {
    ev.push([c.t0, 1]);
    ev.push([c.t1 || Infinity, -1]);
  }
  ev.sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  let n = 0,
    max = 0;
  for (const [, d] of ev) {
    n += d;
    max = Math.max(max, n);
  }
  return max;
}
// Visible queue rows with their corpus index, kind and what they show right now.
const rows = () =>
  js(`(()=>{const list=document.querySelector('.queue-list');if(!list)return [];const box=list.getBoundingClientRect();
 return [...list.querySelectorAll('.queue-item')].filter(r=>{const b=r.getBoundingClientRect();return b.bottom>box.top+1&&b.top<box.bottom-1;}).map(r=>{const title=r.querySelector('.queue-text strong')?.textContent??'';const m=/Cover (\\d{3}) (\\S+)/.exec(title);const img=r.querySelector('.queue-cover img');
  let h=0;const src=img?.getAttribute('src')??'';for(let i=0;i<src.length;i+=97)h=(h*31+src.charCodeAt(i))|0;
  const cover=r.querySelector('.queue-cover [data-cover]');const art=r.querySelector('.queue-cover [data-image]');
  return {i:m?Number(m[1]):-1,kind:m?.[2]??'?',state:img?(img.classList.contains('loaded')?'shown':'loading'):'placeholder',cover:cover?.dataset.cover,code:cover?.dataset.coverCode,image:art?.dataset.image,hash:img?h:0};});})()`);
// Frontend loader counters (new build only) and Rust cover counters (new build only).
const loaderStats = () =>
  js(
    `globalThis.__musicMineCoverStats?JSON.parse(JSON.stringify(globalThis.__musicMineCoverStats)):null`,
  );
const rustStats = () =>
  js(`window.__TAURI_INTERNALS__.invoke('library_cover_stats').catch(()=>null)`);
function delta(a, b) {
  if (!a || !b) return null;
  const out = {};
  for (const k of Object.keys(b)) {
    if (typeof b[k] === 'number') {
      const d = k.endsWith('Max') || k.startsWith('max') ? b[k] : b[k] - (a[k] ?? 0);
      if (d) out[k] = Math.round(d);
    } else if (k === 'codes') {
      const c = {};
      for (const [x, v] of Object.entries(b.codes)) {
        const d = v - (a.codes?.[x] ?? 0);
        if (d) c[x] = d;
      }
      if (Object.keys(c).length) out.codes = c;
    }
  }
  return out;
}
const SHOULD_SHOW = new Set([
  'jpeg-small',
  'jpeg-3000',
  'png-1000',
  'png-3000-rgba',
  'png-noise-13mb',
  'same-name',
  'webp',
  'gif',
]);

/** Samples the visible rows every 50 ms for `ms`, and reports when each first showed its cover. */
async function watch(name, ms, started) {
  const first = new Map();
  let last = [];
  const t0 = Date.now();
  while (Date.now() - t0 < ms) {
    last = await rows();
    const now = await js('performance.now()');
    for (const r of last) if (r.state === 'shown' && !first.has(r.i)) first.set(r.i, now - started);
    await sleep(50);
  }
  const probe = await js(`JSON.parse(JSON.stringify(window.__coverProbe))`);
  await sleep(100);
  const calls = net.slice(windowStart);
  const done = calls.filter((c) => c.t1);
  const visible = last.length;
  const expected = last.filter((r) => SHOULD_SHOW.has(r.kind));
  const shown = last.filter((r) => r.state === 'shown');
  const times = [...first.values()].sort((a, b) => a - b);
  const missing = expected
    .filter((r) => r.state !== 'shown')
    .map((r) => {
      const c = calls.filter((c) => c.row === r.i && c.cmd === 'library_queue_thumb');
      return `${r.i}:${r.kind}:${r.state}:${r.cover ? r.cover + (r.code ? '/' + r.code : '') : 'requests=' + c.length}`;
    });
  const noCover = last
    .filter((r) => !SHOULD_SHOW.has(r.kind))
    .map((r) => `${r.i}:${r.kind}:${r.cover ?? r.state}${r.code ? '/' + r.code : ''}`);
  const stuckLoading = last.filter((r) => r.cover === 'loading' || r.state === 'loading').length;
  const gaps = probe.frames.slice(1).map((t, i) => t - probe.frames[i]);
  const durations = done.map((c) => (c.t1 - c.t0) * 1000).sort((a, b) => a - b);
  const outcomes = {};
  for (const c of calls)
    outcomes[c.cmd + ' ' + c.outcome] = (outcomes[c.cmd + ' ' + c.outcome] ?? 0) + 1;
  const sources = {};
  for (const c of done) if (c.source) sources[c.source] = (sources[c.source] ?? 0) + 1;
  const base = calls[0]?.t0 ?? 0;
  const detail = last.map((r) => {
    const c = calls.filter((c) => c.row === r.i && c.cmd === 'library_queue_thumb');
    return {
      i: r.i,
      kind: r.kind,
      state: r.state,
      shownMs: first.has(r.i) ? Math.round(first.get(r.i)) : null,
      requests: c.map((c) => ({
        at: Math.round((c.t0 - base) * 1000),
        ms: c.t1 ? Math.round((c.t1 - c.t0) * 1000) : null,
        outcome: c.outcome,
      })),
    };
  });
  return {
    name,
    detail,
    visible,
    expected: expected.length,
    shown: shown.length,
    firstShownMs: times[0] != null ? Math.round(times[0]) : null,
    allShownMs: times.length && shown.length === expected.length ? Math.round(times.at(-1)) : null,
    missing,
    noCover,
    stuckLoading,
    ipc: calls.length,
    outcomes,
    sources,
    maxInflight: maxInflight(calls),
    ipcP50: Math.round(durations[Math.floor(durations.length / 2)] ?? 0),
    ipcMax: Math.round(durations.at(-1) ?? 0),
    imgErrors: probe.imgs.filter((e) => e.type === 'error').length,
    maxFrameGap: Math.round(Math.max(0, ...gaps)),
    jank: gaps.filter((g) => g > 50).length,
    rows: last,
  };
}

const QUEUE = 'button[aria-label="播放队列"]';
const click = (sel) => js(`document.querySelector(${q(sel)}).click()`);
const queueOpen = () => js(`document.querySelector('.app').classList.contains('queue-open')`);
async function setQueue(open) {
  if ((await queueOpen()) !== open) {
    await click(QUEUE);
    await sleep(400);
  }
}
async function listTo(top) {
  await js(`document.querySelector('.queue-list').scrollTop=${top}`);
}
const results = [];
function log(r) {
  const { rows: _, detail: __, ...rest } = r;
  console.log(JSON.stringify(rest));
  if (process.env.DETAIL) for (const d of r.detail) console.log('   ', JSON.stringify(d));
  if (process.env.ORDER) {
    const c = net.slice(windowStart).filter((c) => c.cmd === 'library_queue_thumb');
    const b = c[0]?.t0 ?? 0;
    console.log('    order', c.map((x) => `${x.row}@${Math.round((x.t0 - b) * 1000)}`).join(' '));
  }
  results.push(r);
}
async function measure(name, action, ms = 5000) {
  const before = {
    proc: processStats(),
    cache: cacheFiles(),
    loader: await loaderStats(),
    rust: await rustStats(),
  };
  const started = await reset();
  await action();
  const r = await watch(name, ms, started);
  r.proc = { before: before.proc, after: processStats() };
  r.cache = { before: before.cache, after: cacheFiles() };
  r.loader = delta(before.loader, await loaderStats());
  r.rust = delta(before.rust, await rustStats());
  log(r);
  return r;
}

await install();
// A queue of every corpus track (336) in corpus order, so the first screen mixes small and 3000 px JPEGs.
// Rust owns the queue; this only issues the same IPC the UI uses. Library ids follow scan order from 1.
const snap = await js(`window.__TAURI_INTERNALS__.invoke('player_get_state')`);
if ((snap.queue?.length ?? 0) < 300) {
  await js(
    `window.__TAURI_INTERNALS__.invoke('library_queue_tracks',{ids:Array.from({length:336},(_,i)=>i+1),replace:true})`,
  );
  await sleep(800);
  const scanned = (await js(`window.__TAURI_INTERNALS__.invoke('player_get_state')`)).queue;
  const order = scanned
    .map((t, k) => ({ id: k + 1, n: Number(/Cover (\d{3})/.exec(t.title)?.[1] ?? 999) }))
    .sort((a, b) => a.n - b.n)
    .map((x) => x.id);
  await js(
    `window.__TAURI_INTERNALS__.invoke('library_queue_tracks',{ids:${q(order)},replace:true})`,
  );
  await sleep(800);
}
console.log(
  'queue length',
  (await js(`window.__TAURI_INTERNALS__.invoke('player_get_state')`)).queue.length,
);
await setQueue(false);
await sleep(1500);

const scenario = {
  A: () => measure('A cold first open', () => click(QUEUE), 6000),
  B: async () => {
    await setQueue(false);
    await sleep(600);
    return measure('B warm reopen', () => click(QUEUE), 2500);
  },
  C: async () => {
    await setQueue(true);
    await listTo(0);
    await sleep(300);
    const down = await measure(
      'C scroll to end',
      async () => {
        for (let y = 0; y < 60000; y += 400) {
          await listTo(y);
          await sleep(40);
        }
      },
      4000,
    );
    await measure(
      'C back to top',
      async () => {
        await listTo(0);
      },
      2500,
    );
    return down;
  },
  D: async () => {
    await setQueue(false);
    await js(`document.querySelector('button[aria-label="正在播放与歌词 (Ctrl+L)"]')?.click()`);
    await sleep(900);
    await measure('D lyrics page open', () => click(QUEUE), 3000);
    await setQueue(false);
    await js(`document.querySelector('button[aria-label="收起正在播放 (Esc)"]')?.click()`);
    await sleep(600);
  },
  E: async () => {
    await setQueue(true);
    await listTo(0);
    return measure(
      'E rapid next x10',
      async () => {
        for (let i = 0; i < 10; i++) {
          await js(`document.querySelector('button[aria-label="下一首"]')?.click()`);
          await sleep(150);
        }
      },
      3000,
    );
  },
  F: async () => {
    await setQueue(false);
    const r = await measure(
      'F theme switch then open',
      async () => {
        for (const label of ['浅色', '深色']) {
          await js(
            `[...document.querySelectorAll('.nav-item')].find(e=>e.textContent.includes('设置')).click()`,
          );
          await sleep(400);
          await js(`document.querySelector('button.select-trigger[aria-label="主题"]').click()`);
          await sleep(200);
          await js(
            `[...document.querySelectorAll('.select-panel>button')].find(b=>b.textContent.includes(${q(label)})).click()`,
          );
          await sleep(500);
        }
        await click(QUEUE);
      },
      3000,
    );
    return r;
  },
  H: async () => {
    await setQueue(true);
    // Row 250 is the first non-JPEG picture; rows 250-299 hold every failure kind of the corpus.
    return measure(
      'H failure kinds',
      async () => {
        await js(
          `(()=>{const rows=[...document.querySelectorAll('.queue-item')];const r=rows.find(r=>/Cover 250 /.test(r.textContent));document.querySelector('.queue-list').scrollTop=r.offsetTop-8;})()`,
        );
      },
      5000,
    );
  },
  H2: async () => {
    await setQueue(true);
    return measure(
      'H2 failure kinds lower',
      async () => {
        await js(
          `(()=>{const rows=[...document.querySelectorAll('.queue-item')];const r=rows.find(r=>/Cover 270 /.test(r.textContent));document.querySelector('.queue-list').scrollTop=r.offsetTop-8;})()`,
        );
      },
      5000,
    );
  },
  R: () => measure('R first open after restart', () => click(QUEUE), 4000),
  Q: async () => {
    await setQueue(true);
    await listTo(0);
    await sleep(300);
    return measure(
      'Q rapid close/reopen x10',
      async () => {
        for (let i = 0; i < 20; i++) {
          await click(QUEUE);
          await sleep(60);
        }
      },
      2500,
    );
  },
  S: async () => {
    await setQueue(true);
    await listTo(0);
    await sleep(300);
    return measure('S same rows after recover', async () => {}, 1500);
  },
};
for (const key of scenarioArg.split(',')) await scenario[key]();
mkdirSync('tests/perf/out', { recursive: true });
writeFileSync(
  `tests/perf/out/cover-${label}-${scenarioArg.replace(/,/g, '')}.json`,
  JSON.stringify(results, null, 1),
);
ws.close();
````

## File: tests/perf/queue-profile.mjs
````javascript
// Real WebView2 queue-toggle profiler. Drives the running desktop app over CDP and prints per-scenario metrics.
// Start the built exe with WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9333, then:
//   node tests/perf/queue-profile.mjs <label> [scenarios=C,A,B,D] [port=9333]
import { writeFileSync, mkdirSync } from 'node:fs';

const [label = 'run', scenarioArg = 'C,A,B,D', port = '9333'] = process.argv.slice(2);
const targets = await (await fetch(`http://127.0.0.1:${port}/json`)).json();
const page = targets.find((t) => t.type === 'page');
if (!page) throw new Error('no page target');
const ws = new WebSocket(page.webSocketDebuggerUrl);
await new Promise((ok, fail) => {
  ws.onopen = ok;
  ws.onerror = fail;
});
let seq = 0;
const pending = new Map();
let traceChunks = [];
let traceDone = null;
ws.onmessage = ({ data }) => {
  const msg = JSON.parse(data);
  if (msg.id && pending.has(msg.id)) {
    const { ok, fail } = pending.get(msg.id);
    pending.delete(msg.id);
    msg.error ? fail(new Error(msg.error.message)) : ok(msg.result);
  } else if (msg.method === 'Tracing.dataCollected') traceChunks.push(...msg.params.value);
  else if (msg.method === 'Tracing.tracingComplete') traceDone?.();
};
const send = (method, params = {}) =>
  new Promise((ok, fail) => {
    const id = ++seq;
    pending.set(id, { ok, fail });
    ws.send(JSON.stringify({ id, method, params }));
  });
async function js(expression) {
  const r = await send('Runtime.evaluate', { expression, awaitPromise: true, returnByValue: true });
  if (r.exceptionDetails)
    throw new Error(r.exceptionDetails.exception?.description ?? r.exceptionDetails.text);
  return r.result.value;
}
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const q = (s) => JSON.stringify(s);
if (process.env.RELOAD) {
  await send('Page.reload');
  await sleep(4000);
}

// In-page probes: frame intervals, long tasks, DOM mutations by region, layout reads and lyric scrolls.
async function installProbe() {
  await js(`(()=>{if(window.__probe)return;const p=window.__probe={frames:[],longtasks:[],mut:{lyrics:0,queue:0,other:0},reads:0,scrolls:{instant:0,smooth:0},transitions:0,on:false};
 document.addEventListener('transitionrun',()=>{if(p.on)p.transitions++;},true);
 const loop=t=>{if(p.on)p.frames.push(t);requestAnimationFrame(loop);};requestAnimationFrame(loop);
 try{new PerformanceObserver(l=>{if(p.on)for(const e of l.getEntries())p.longtasks.push(Math.round(e.duration));}).observe({type:'longtask',buffered:false});}catch{}
 new MutationObserver(list=>{if(!p.on)return;for(const m of list){const el=m.target.nodeType===1?m.target:m.target.parentElement;if(el?.closest('.lyric-lines'))p.mut.lyrics++;else if(el?.closest('.queue'))p.mut.queue++;else p.mut.other++;}}).observe(document.body,{subtree:true,childList:true,attributes:true,characterData:true});
 const wrapGet=(proto,key)=>{const d=Object.getOwnPropertyDescriptor(proto,key);Object.defineProperty(proto,key,{...d,get(){if(p.on)p.reads++;return d.get.call(this);}});};
 for(const k of ['offsetTop','offsetHeight','clientHeight','scrollHeight'])wrapGet(Element.prototype.hasOwnProperty(k)?Element.prototype:HTMLElement.prototype,k);
 const gbr=Element.prototype.getBoundingClientRect;Element.prototype.getBoundingClientRect=function(){if(p.on)p.reads++;return gbr.call(this);};
 const st=Element.prototype.scrollTo;Element.prototype.scrollTo=function(o){if(p.on&&this.classList?.contains('lyric-lines'))p.scrolls[o?.behavior==='smooth'?'smooth':'instant']++;return st.apply(this,arguments);};
})()`);
  // Isolation experiments: INJECT_CSS adds a temporary stylesheet to the running page (removed when the run ends).
  await js(`document.getElementById('__perf_css')?.remove()`);
  // DELETE_RULES removes matching rules from the loaded stylesheets for this page session only.
  if (process.env.DELETE_RULES)
    console.log(
      'deleted rules',
      await js(
        `(()=>{const re=new RegExp(${q(process.env.DELETE_RULES)});let n=0;const walk=list=>{for(let i=list.length-1;i>=0;i--){const r=list[i];if(r.cssRules&&!r.selectorText)walk(r.cssRules);else if(r.selectorText&&re.test(r.selectorText)){(r.parentRule??r.parentStyleSheet).deleteRule(i);n++;}}};for(const s of document.styleSheets)walk(s.cssRules);return n;})()`,
      ),
    );
  if (process.env.INJECT_CSS)
    await js(
      `(()=>{const s=document.createElement('style');s.id='__perf_css';s.textContent=${q(process.env.INJECT_CSS)};document.head.append(s);})()`,
    );
}
await installProbe();
const click = (sel) =>
  js(
    `(()=>{const el=document.querySelector(${q(sel)});if(!el)throw new Error('missing '+${q(sel)});el.click();return true;})()`,
  );
const clickText = (sel, text) =>
  js(
    `(()=>{const el=[...document.querySelectorAll(${q(sel)})].find(e=>e.textContent.includes(${q(text)}));if(!el)throw new Error('missing '+${q(text)});el.click();})()`,
  );
const state = () =>
  js(
    `(()=>{const a=document.querySelector('.app');return {w:innerWidth,h:innerHeight,dpr:devicePixelRatio,immersive:!!document.querySelector('.immersive'),queue:a.classList.contains('queue-open'),reduce:a.classList.contains('reduce-motion'),playing:document.querySelector('.pb-play')?.getAttribute('aria-label')==='暂停',lyricRows:document.querySelectorAll('.lyric-lines>button').length,queueRows:document.querySelectorAll('.queue-item').length,theme:document.documentElement.dataset.theme,ambient:!!document.querySelector('.immersive.has-ambient')};})()`,
  );
const QUEUE = 'button[aria-label="播放队列"]';
async function setQueue(open) {
  const s = await state();
  if (s.queue !== open) {
    await click(QUEUE);
    await sleep(700);
  }
}
async function setImmersive(on) {
  const s = await state();
  if (s.immersive !== on) {
    if (on) await click('button[aria-label="正在播放与歌词 (Ctrl+L)"]');
    else await click('button[aria-label="收起正在播放 (Esc)"]');
    await sleep(900);
  }
}
async function setPlaying(on) {
  const s = await state();
  if (s.playing !== on) {
    await click('.pb-play');
    await sleep(500);
  }
}
async function setReduce(on) {
  const s = await state();
  if (s.reduce === on) return;
  const wasImmersive = s.immersive;
  if (wasImmersive) await setImmersive(false);
  await clickText('.nav-item', '设置');
  await sleep(500);
  await js(
    `(()=>{const l=[...document.querySelectorAll('label.switch')].find(e=>e.textContent.includes('减少动态效果'));l.querySelector('input').click();})()`,
  );
  await sleep(300);
  await clickText('.nav-item', '歌曲');
  await sleep(600);
  if (wasImmersive) await setImmersive(true);
}

const CATS = [
  'devtools.timeline',
  'disabled-by-default-devtools.timeline',
  'disabled-by-default-devtools.timeline.frame',
  'blink.user_timing',
  'toplevel',
  'v8.execute',
  'benchmark',
  'rail',
].join(',');
async function record(fn) {
  traceChunks = [];
  await send('Tracing.start', { categories: CATS, transferMode: 'ReportEvents' });
  await sleep(200);
  await fn();
  await sleep(200);
  const done = new Promise((r) => (traceDone = r));
  await send('Tracing.end');
  await done;
  return traceChunks;
}

async function actions(kind) {
  const marks = [];
  const act = async (name, fn, wait) => {
    await js(
      `(()=>{const p=window.__probe;p.frames=[];p.longtasks=[];p.mut={lyrics:0,queue:0,other:0};p.reads=0;p.transitions=0;p.scrolls={instant:0,smooth:0};p.on=true;performance.mark('A:${name}:start');})()`,
    );
    await fn();
    await sleep(wait);
    const probe = await js(
      `(()=>{const p=window.__probe;p.on=false;performance.mark('A:${name}:end');return JSON.parse(JSON.stringify(p));})()`,
    );
    marks.push({ name, probe });
  };
  await act('idle', async () => {}, 700);
  for (let i = 1; i <= 3; i++) {
    await act('open' + i, () => click(QUEUE), 700);
    await act('close' + i, () => click(QUEUE), 700);
  }
  if (kind !== 'light')
    await act(
      'rapid',
      async () => {
        for (let i = 0; i < 8; i++) {
          await click(QUEUE);
          await sleep(60);
        }
      },
      900,
    );
  // Steady state with the drawer open: what each 200 ms player poll costs while the queue is visible.
  await click(QUEUE);
  await sleep(800);
  await act('openIdle', async () => {}, 1000);
  await click(QUEUE);
  await sleep(700);
  return marks;
}

function analyse(events, marks) {
  const main = events.find(
    (e) =>
      e.name === 'thread_name' &&
      e.args?.name === 'CrRendererMain' &&
      events.some((x) => x.pid === e.pid && x.name === 'A:idle:start'),
  );
  const pid = main?.pid,
    tid = main?.tid;
  const mk = Object.fromEntries(
    events.filter((e) => e.pid === pid && e.name?.startsWith('A:')).map((e) => [e.name, e.ts]),
  );
  const thread = events
    .filter((e) => e.pid === pid && e.tid === tid && e.ph === 'X')
    .sort((a, b) => a.ts - b.ts);
  const frames = events.filter(
    (e) => e.name === 'PipelineReporter' && e.ph === 'b' && e.pid === pid,
  );
  const SCRIPT = new Set([
    'FunctionCall',
    'EvaluateScript',
    'TimerFire',
    'FireAnimationFrame',
    'EventDispatch',
    'RunMicrotasks',
    'V8.Execute',
    'v8.callFunction',
    'FireIdleCallback',
  ]);
  return marks.map(({ name, probe }) => {
    const from = mk[`A:${name}:start`],
      to = mk[`A:${name}:end`];
    const inWin = thread.filter((e) => e.ts >= from && e.ts <= to);
    const sum = (n) => {
      const l = inWin.filter((e) => e.name === n);
      return {
        n: l.length,
        ms: +(l.reduce((s, e) => s + e.dur, 0) / 1000).toFixed(1),
        max: +(Math.max(0, ...l.map((e) => e.dur)) / 1000).toFixed(1),
      };
    };
    // Top-level script time only (nested script events are not double-counted); layouts inside script are forced.
    let script = 0,
      forced = 0;
    const stack = [];
    for (const e of inWin) {
      while (stack.length && stack.at(-1).ts + stack.at(-1).dur <= e.ts) stack.pop();
      const inScript = stack.some((s) => SCRIPT.has(s.name));
      if (SCRIPT.has(e.name) && !inScript) script += e.dur;
      if ((e.name === 'Layout' || e.name === 'UpdateLayoutTree') && inScript) forced++;
      stack.push(e);
    }
    const tasks = inWin.filter(
      (e) => e.name === 'RunTask' || e.name === 'ThreadControllerImpl::RunTask',
    );
    const fr = probe.frames;
    const gaps = fr.slice(1).map((t, i) => t - fr[i]);
    const states = {};
    for (const f of frames.filter((f) => f.ts >= from && f.ts <= to)) {
      const s = f.args?.chrome_frame_reporter?.state ?? '?';
      states[s] = (states[s] ?? 0) + 1;
    }
    return {
      name,
      layout: sum('Layout'),
      style: sum('UpdateLayoutTree'),
      paint: sum('Paint'),
      prePaint: sum('PrePaint'),
      layerize: sum('Layerize'),
      commit: sum('Commit'),
      scriptMs: +(script / 1000).toFixed(1),
      forcedLayouts: forced,
      longTasks: tasks.filter((t) => t.dur > 50000).length,
      maxTaskMs: +(Math.max(0, ...tasks.map((t) => t.dur)) / 1000).toFixed(1),
      longtaskApi: probe.longtasks,
      rafFrames: fr.length,
      maxGapMs: +Math.max(0, ...gaps).toFixed(1),
      janky: gaps.filter((g) => g > 25).length,
      fps: gaps.length ? +(1000 / (gaps.reduce((a, b) => a + b, 0) / gaps.length)).toFixed(1) : 0,
      pipeline: states,
      mutations: probe.mut,
      transitions: probe.transitions,
      layoutReads: probe.reads,
      lyricScrolls: probe.scrolls,
    };
  });
}

const scenarios = {
  C: async () => {
    await setImmersive(false);
    await setPlaying(true);
    await setReduce(false);
  },
  A: async () => {
    await setReduce(false);
    await setImmersive(true);
    await setPlaying(true);
  },
  B: async () => {
    await setReduce(true);
    await setImmersive(true);
    await setPlaying(true);
  },
  D: async () => {
    await setReduce(false);
    await setImmersive(true);
    await setPlaying(false);
  },
  // A and B again in the light theme; the stored theme choice is restored at the end of the run.
  AL: async () => {
    await setTheme('light');
    await setReduce(false);
    await setImmersive(true);
    await setPlaying(true);
  },
  BL: async () => {
    await setTheme('light');
    await setReduce(true);
    await setImmersive(true);
    await setPlaying(true);
  },
};
const savedTheme = await js(`localStorage.getItem('music-mine-theme')`);
async function setTheme(choice) {
  if ((await js(`document.documentElement.dataset.theme`)) === choice) return;
  await js(`localStorage.setItem('music-mine-theme',${q(choice)})`);
  await send('Page.reload');
  await sleep(4000);
  await installProbe();
}
const initial = await state();
console.log('initial', JSON.stringify(initial));
await setQueue(false);
const out = { label, initial, results: {} };
mkdirSync('tests/perf/out', { recursive: true });
for (const key of scenarioArg.split(',')) {
  await scenarios[key]();
  await sleep(800);
  const env = await state();
  let marks;
  const events = await record(async () => {
    marks = await actions();
  });
  writeFileSync(
    `tests/perf/out/${label}-${key}.trace.json`,
    JSON.stringify({ traceEvents: events }),
  );
  out.results[key] = { env, rows: analyse(events, marks) };
  console.log(`\n== ${key}`, JSON.stringify(env));
  for (const r of out.results[key].rows)
    console.log(
      [
        r.name.padEnd(6),
        `style ${r.style.ms}/${r.style.max}`,
        `layout ${r.layout.ms}`,
        `paint ${r.paint.ms}`,
        `script ${r.scriptMs}`,
        `forced ${r.forcedLayouts}`,
        `maxTask ${r.maxTaskMs}`,
        `long ${r.longTasks}`,
        `fps ${r.fps}`,
        `gap ${r.maxGapMs}`,
        `jank ${r.janky}`,
        `trans ${r.transitions}`,
        `mut L${r.mutations.lyrics}/Q${r.mutations.queue}/O${r.mutations.other}`,
        `reads ${r.layoutReads}`,
        `scroll ${r.lyricScrolls.instant}+${r.lyricScrolls.smooth}`,
      ].join(' | '),
    );
}
if ((await js(`localStorage.getItem('music-mine-theme')`)) !== savedTheme) {
  await js(
    savedTheme === null
      ? `localStorage.removeItem('music-mine-theme')`
      : `localStorage.setItem('music-mine-theme',${q(savedTheme)})`,
  );
  await send('Page.reload');
  await sleep(4000);
  await installProbe();
}
await setReduce(initial.reduce);
await setImmersive(initial.immersive);
await setPlaying(initial.playing);
await setQueue(initial.queue);
await js(`document.getElementById('__perf_css')?.remove()`);
writeFileSync(`tests/perf/out/${label}.json`, JSON.stringify(out, null, 1));
ws.close();
````

## File: tests/perf/run-restart-recover.ps1
````powershell
# Restart and replaced-cover checks for builds that already have isolation source records.
# Stops only the pid recorded for this label. It does not stop music-mine-desktop by name.
param([string[]]$Labels = @("before", "after"), [switch]$SkipWarm, [switch]$RecoverOnly)
$ErrorActionPreference = "Stop"
$repo = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$origin = (Get-Location).Path
Set-Location $repo
. (Join-Path $repo "scripts\isolation-lib.ps1")

function Stop-Label([string]$Label) {
    if ($Label -notmatch '^[A-Za-z0-9._-]+$') { throw "label must be a simple token" }
    $corpus = Join-Path $env:TEMP "mm-cover-corpus"
    if (-not (Test-Path -LiteralPath $corpus)) { return }
    if (-not (Test-PathUnderRoot $corpus (Get-FinalPath $env:TEMP))) { throw "corpus escapes TEMP" }
    $run = Join-Path $corpus "run-$Label"
    if (-not (Test-PathUnderRoot $run $corpus)) { throw "run directory escapes the corpus" }
    Stop-RecordedPlayer -RecordPath (Join-Path $run "process.json")
}

function Run-Label($Label, [switch]$Keep, $Scenarios) {
    Stop-Label $Label
    $exe = Join-Path $repo "tests\perf\out\bin\cover-$Label.exe"
    $stageArgs = @(
        "-NoProfile", "-ExecutionPolicy", "Bypass", "-File", (Join-Path $PSScriptRoot "stage-cover-run.ps1"),
        "-Label", $Label, "-Exe", $exe
    )
    if ($Keep) { $stageArgs += "-KeepData" }
    & powershell.exe @stageArgs
    if ($LASTEXITCODE -ne 0) { throw "stage-cover-run failed: $LASTEXITCODE" }
    Start-Sleep 6
    Write-Output "== $Label $Scenarios"
    & node (Join-Path $repo "tests\perf\queue-cover-profile.mjs") $Label $Scenarios
    if ($LASTEXITCODE -ne 0) { throw "queue-cover-profile failed: $LASTEXITCODE" }
}

$failed = $null
try {
    if (-not $RecoverOnly) {
        foreach ($label in $Labels) {
            if (-not $SkipWarm) { Run-Label $label -Scenarios "A,C" }
            Run-Label $label -Keep -Scenarios "R,S"
        }
    }
    $env:MM_COVER_CORPUS = Join-Path $env:TEMP "mm-cover-corpus"
    $env:MM_COVER_CORPUS_RECOVER = "1"
    Set-Location (Join-Path $repo "src-tauri")
    & cargo test --release --lib cover_corpus -- --ignored --nocapture
    if ($LASTEXITCODE -ne 0) { throw "cargo test failed: $LASTEXITCODE" }
    Set-Location $repo
    foreach ($label in $Labels) { Run-Label $label -Keep -Scenarios "S" }
} catch {
    $failed = $_
} finally {
    Set-Location $origin
    foreach ($label in $Labels) {
        try { Stop-Label $label } catch { Write-Output "STOP_FAILED $label $($_.Exception.Message)" }
    }
    Remove-Item Env:MM_COVER_CORPUS_RECOVER -ErrorAction SilentlyContinue
}
if ($failed) { throw $failed }
````

## File: tests/perf/scan-sampler.mjs
````javascript
// Samples queue covers and loader counters while a rescan runs (the scan itself is started through the UI).
// node tests/perf/scan-sampler.mjs [seconds=150]
// Starts the scan button click itself; the native folder dialog must then be answered by hand.
const seconds = Number(process.argv[2] ?? 150);
const page = (await (await fetch('http://127.0.0.1:9333/json')).json()).find(
  (t) => t.type === 'page',
);
const ws = new WebSocket(page.webSocketDebuggerUrl);
await new Promise((r) => (ws.onopen = r));
let id = 0;
const wait = new Map();
ws.onmessage = ({ data }) => {
  const m = JSON.parse(data);
  if (wait.has(m.id)) {
    wait.get(m.id)(m.result);
    wait.delete(m.id);
  }
};
const send = (method, params = {}) =>
  new Promise((r) => {
    wait.set(++id, r);
    ws.send(JSON.stringify({ id, method, params }));
  });
const js = async (e) =>
  (await send('Runtime.evaluate', { expression: e, returnByValue: true, awaitPromise: true }))
    .result?.value;
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
await js(
  `[...document.querySelectorAll('.nav-item')].find(e=>e.textContent.includes('设置')).click()`,
);
await sleep(500);
await js(
  `document.querySelector('.app').classList.contains('queue-open')||document.querySelector('button[aria-label="播放队列"]').click()`,
);
await sleep(600);
await js(`document.querySelector('.queue-list').scrollTop=0`);
const sample = () =>
  js(`(async()=>{const S=globalThis.__musicMineCoverStats;const p=await window.__TAURI_INTERNALS__.invoke('library_progress');
 const list=document.querySelector('.queue-list');const box=list.getBoundingClientRect();
 const vis=[...list.querySelectorAll('.queue-item')].filter(r=>{const b=r.getBoundingClientRect();return b.bottom>box.top+1&&b.top<box.bottom-1;});
 const kinds=vis.map(r=>{const m=/Cover (\\d{3}) (\\S+)/.exec(r.querySelector('.queue-text strong')?.textContent??'');return m?.[2];});
 const expect=vis.filter((r,i)=>!/no-cover|corrupt/.test(kinds[i]));
 return {running:p.running,visited:p.visited,top:Math.round(list.scrollTop),visible:vis.length,expected:expect.length,
  shown:expect.filter(r=>r.querySelector('img.loaded')).length,blank:expect.filter(r=>!r.querySelector('img')).length,
  loading:vis.filter(r=>r.querySelector('[data-cover="loading"]')).length,failed:vis.filter(r=>r.querySelector('[data-cover="failed"]')).length,
  started:S.started,stale:S.stale,handed:S.staleHandedOver,maxRunning:S.maxRunning,retries:S.retries,failedTotal:S.failed,pending:null};})()`);
const t0 = Date.now();
let sawScan = false,
  endAt = 0,
  clicked = false;
let last = '';
const log = [];
let maxBlankDuringScan = 0,
  maxRunningSeen = 0;
while (Date.now() - t0 < seconds * 1000) {
  if (!clicked) {
    clicked = true;
    void js(
      `document.querySelector('.settings-actions button.btn-primary, button.btn-primary')?.click()`,
    );
  }
  const s = await sample();
  if (s.running) {
    sawScan = true;
    maxBlankDuringScan = Math.max(maxBlankDuringScan, s.blank);
  }
  maxRunningSeen = Math.max(maxRunningSeen, s.maxRunning);
  // Scroll back and forth during the scan so covers are in flight when the scan clears the loader.
  if (s.running)
    await js(
      `(()=>{const l=document.querySelector('.queue-list');l.scrollTop=(l.scrollTop+560)%(l.scrollHeight-l.clientHeight);})()`,
    );
  if (sawScan && !s.running && !endAt) {
    endAt = Date.now();
    await js(`document.querySelector('.queue-list').scrollTop=0`);
  }
  const key = JSON.stringify({ ...s, top: undefined, visited: undefined });
  if (key !== last) {
    last = key;
    const line = { t: Date.now() - t0, ...s };
    log.push(line);
    console.log(JSON.stringify(line));
  }
  if (endAt && Date.now() - endAt > 6000) break;
  await sleep(200);
}
console.log(JSON.stringify({ summary: { sawScan, maxBlankDuringScan, final: log.at(-1) } }));
ws.close();
````

## File: tests/perf/stage-cover-run.ps1
````powershell
# Stage a portable exe next to a fresh copy of the generated cover corpus and launch it.
# The exe must have a portable-relative isolation source record. This script does not
# accept an arbitrary build, and it does not stop processes by name.
param([Parameter(Mandatory)][string]$Label, [Parameter(Mandatory)][string]$Exe, [switch]$KeepData, [int]$Port = 9333)
$ErrorActionPreference = "Stop"
$repo = Resolve-Path (Join-Path $PSScriptRoot "..\..")
. (Join-Path $repo "scripts\isolation-lib.ps1")

if ($Label -notmatch '^[A-Za-z0-9._-]+$') { throw "label must be a simple token" }
if (-not (Test-Path -LiteralPath $Exe)) { throw "exe not found: $Exe" }
[void](Assert-PortableLaunchSource -Repo $repo -ExePath $Exe)

$corpus = Join-Path $env:TEMP "mm-cover-corpus"
$tempFinal = Get-FinalPath $env:TEMP
if (-not (Test-Path -LiteralPath $corpus)) { throw "corpus missing: run the cover_corpus test first" }
if (-not (Test-PathUnderRoot $corpus $tempFinal)) { throw "corpus escapes TEMP" }
$pristine = Join-Path $corpus "app-data"
if (-not (Test-Path (Join-Path $pristine "library-tauri-v1.sqlite"))) { throw "corpus missing: run the cover_corpus test first" }

$run = Join-Path $corpus "run-$Label"
if (-not (Test-PathUnderRoot $run $corpus)) { throw "run directory escapes the corpus" }
if (-not $KeepData -and (Test-Path -LiteralPath $run)) {
    Assert-DeletableUnderRoot $run $corpus
    Remove-Item -LiteralPath $run -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $run | Out-Null
$stagedExe = Join-Path $run "music-mine-desktop.exe"
Copy-Item -LiteralPath $Exe -Destination $stagedExe -Force
if ((Get-Sha256Hex $stagedExe) -ne (Get-Sha256Hex $Exe)) { throw "staged exe hash does not match the build source" }
if (-not (Test-Path (Join-Path $run "app-data"))) { Copy-Item $pristine (Join-Path $run "app-data") -Recurse }
if (-not (Test-PathUnderRoot (Join-Path $run "app-data") $corpus)) { throw "staged app-data escapes the corpus" }

$env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = "--remote-debugging-port=$Port"
$process = Start-Process $stagedExe -WorkingDirectory $run -PassThru
Write-RunProcessRecord -RecordPath (Join-Path $run "process.json") -ProcessId $process.Id -ExePath $stagedExe
Write-Output "RUN_DIR=$run"
Write-Output "PID=$($process.Id)"
````

## File: tests/perf/transient-cover.mjs
````javascript
// Transient failure check on the generated corpus only: hides one corpus file, reloads the page, scrolls to its
// queue row and reports the row state over time; then restores the file and checks the cover comes back.
import { renameSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
const row = Number(process.argv[2] ?? 150);
const dir = join(process.env.TEMP, 'mm-cover-corpus', 'music', 'jpeg-small');
const file = readdirSync(dir).find((n) => n.startsWith(String(row).padStart(3, '0') + ' '));
const page = (await (await fetch('http://127.0.0.1:9333/json')).json()).find(
  (t) => t.type === 'page',
);
const ws = new WebSocket(page.webSocketDebuggerUrl);
await new Promise((r) => (ws.onopen = r));
let id = 0;
const wait = new Map();
ws.onmessage = ({ data }) => {
  const m = JSON.parse(data);
  if (wait.has(m.id)) {
    wait.get(m.id)(m.result);
    wait.delete(m.id);
  }
};
const send = (method, params = {}) =>
  new Promise((r) => {
    wait.set(++id, r);
    ws.send(JSON.stringify({ id, method, params }));
  });
const js = async (e) =>
  (await send('Runtime.evaluate', { expression: e, returnByValue: true, awaitPromise: true }))
    .result?.value;
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
async function visit(label, ms) {
  await send('Page.reload');
  for (let i = 0; i < 200; i++) {
    if (
      await js(
        `!!document.querySelector('.queue-list .queue-item')&&!!globalThis.__musicMineCoverStats`,
      )
    )
      break;
    await sleep(50);
  }
  await js(
    `document.querySelector('.app').classList.contains('queue-open')||document.querySelector('button[aria-label="播放队列"]').click()`,
  );
  await sleep(500);
  await js(
    `(()=>{const r=[...document.querySelectorAll('.queue-item')].find(r=>/Cover ${row} /.test(r.textContent));document.querySelector('.queue-list').scrollTop=r.offsetTop-8;})()`,
  );
  const out = [];
  for (let t = 0; t < ms; t += 500) {
    await sleep(500);
    out.push(
      await js(
        `(()=>{const r=[...document.querySelectorAll('.queue-item')].find(r=>/Cover ${row} /.test(r.textContent));const c=r.querySelector('[data-cover]');const S=globalThis.__musicMineCoverStats;return {t:${t + 500},cover:c.dataset.cover,code:c.dataset.coverCode??null,img:!!r.querySelector('img.loaded'),retries:S.retries,failed:S.failed,codes:S.codes};})()`,
      ),
    );
  }
  console.log(label);
  for (const s of out) console.log('  ', JSON.stringify(s));
}
renameSync(join(dir, file), join(dir, file + '.hidden'));
try {
  await visit('hidden', 5000);
} finally {
  renameSync(join(dir, file + '.hidden'), join(dir, file));
}
await visit('restored', 1500);
ws.close();
````

## File: tests/ui/primitives.spec.ts
````typescript
import { expect, test } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  await page.goto('/tests/harness/primitives.html');
});

test('menu navigates disabled items, wraps, searches, ignores IME and restores Escape/Tab focus', async ({
  page,
}) => {
  const trigger = page.getByRole('button', { name: '操作', exact: true });
  await trigger.click();
  const play = page.getByRole('menuitem', { name: '播放', exact: true });
  await page.keyboard.press('Home');
  await expect(play).toBeFocused();
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('menuitem', { name: '下一首播放' })).toBeFocused();
  await page.keyboard.press('End');
  await expect(page.getByRole('menuitem', { name: '失败操作' })).toBeFocused();
  await page.keyboard.press('ArrowDown');
  await expect(play).toBeFocused();
  await play.dispatchEvent('keydown', { key: '加' });
  const queue = page.getByRole('menuitem', { name: '加入播放队列' });
  await expect(queue).toBeFocused();
  await queue.dispatchEvent('keydown', { key: 'ArrowDown', isComposing: true });
  await expect(queue).toBeFocused();
  await page.keyboard.press('Escape');
  await expect(page.getByRole('menu')).toHaveCount(0);
  await expect(trigger).toBeFocused();
  await trigger.click();
  await expect(play).toBeFocused();
  await page.keyboard.press('Tab');
  await expect(page.getByRole('menu')).toHaveCount(0);
  await expect(trigger).toBeFocused();
});

test('menu handles point/rectangle anchors, collisions, replacement, outside clicks and action errors', async ({
  page,
}) => {
  await page.setViewportSize({ width: 900, height: 640 });
  const trigger = page.getByRole('button', { name: '操作', exact: true });
  await trigger.click();
  await page.getByRole('menuitem', { name: '加入播放队列' }).click();
  await expect(page.getByLabel('选择结果')).toHaveText('queue');
  await expect(trigger).toBeFocused();
  const edge = page.getByRole('button', { name: '边缘操作' });
  await edge.click({ button: 'right' });
  const menu = page.getByRole('menu');
  const box = await menu.boundingBox();
  expect(box).toBeTruthy();
  expect(box!.x).toBeGreaterThanOrEqual(8);
  expect(box!.y).toBeGreaterThanOrEqual(8);
  expect(box!.x + box!.width).toBeLessThanOrEqual(892);
  expect(box!.y + box!.height).toBeLessThanOrEqual(632);
  await page.getByRole('button', { name: '外部按钮' }).click();
  await expect(menu).toHaveCount(0);
  await trigger.click();
  await page.getByRole('menuitem', { name: '失败操作' }).click();
  await expect(page.getByLabel('错误')).toContainText('test action failed');
});

test('native Dialog preserves initial selection, focus trapping, drag-out and busy/IME dismissal guards', async ({
  page,
}) => {
  const trigger = page.getByRole('button', { name: '打开弹窗' });
  await trigger.click();
  const dialog = page.getByRole('dialog');
  const input = page.getByRole('textbox', { name: '名称' });
  await expect(input).toBeFocused();
  expect(
    await input.evaluate((el: HTMLInputElement) => [el.selectionStart, el.selectionEnd]),
  ).toEqual([0, 4]);
  await input.press('Shift+Tab');
  await expect(dialog.getByRole('button', { name: '关闭', exact: true })).toBeFocused();
  await input.focus();
  await input.dispatchEvent('keydown', { key: 'Escape', isComposing: true });
  await expect(dialog).toBeVisible();
  const box = await input.boundingBox();
  await page.mouse.move(box!.x + 8, box!.y + 8);
  await page.mouse.down();
  await page.mouse.move(4, 4);
  await page.mouse.up();
  await expect(dialog).toBeVisible();
  await dialog.getByRole('button', { name: '切换忙碌' }).click();
  await page.keyboard.press('Escape');
  await page.mouse.click(4, 4);
  await expect(dialog).toBeVisible();
  await dialog.getByRole('button', { name: '切换忙碌' }).click();
  await page.mouse.click(4, 4);
  await expect(dialog).toHaveCount(0);
  await expect(trigger).toBeFocused();
});

test('native Slider and Floating UI tooltip retain keyboard and viewport behavior', async ({
  page,
}) => {
  const slider = page.getByRole('slider', { name: '测试音量' });
  await slider.focus();
  await page.keyboard.press('ArrowRight');
  await expect(page.getByLabel('音量结果')).toHaveText('51');
  await page.keyboard.press('End');
  await expect(page.getByLabel('音量结果')).toHaveText('100');
  await page.getByRole('button', { name: '边缘操作' }).hover();
  const tip = page.getByRole('tooltip');
  await expect(tip).toHaveText('边缘提示');
  await expect(tip).toBeInViewport();
  await page.mouse.move(4, 4);
  await expect(tip).toBeHidden();
});
````

## File: tests/ui/reduce-motion.spec.ts
````typescript
import { expect, test } from '@playwright/test';

const HARNESS = '/tests/harness/index.html';

test('reduce motion does not start a transition on every element', async ({ page }) => {
  await page.goto(HARNESS);
  await expect(page.getByRole('button', { name: '设置', exact: true })).toBeVisible();
  await page.getByRole('button', { name: '设置', exact: true }).click();
  // The checkbox itself is opacity 0; the label is the visible control.
  await page.locator('label.switch', { hasText: '减少动态效果' }).click();
  await expect(page.locator('.app')).toHaveClass(/reduce-motion/);
  await expect(page.locator('.nav-item').first()).toHaveCSS('transition-property', 'none');

  await page.evaluate(() => {
    const probe = window as unknown as { __transitions: number };
    probe.__transitions = 0;
    document.addEventListener(
      'transitionrun',
      () => {
        probe.__transitions += 1;
      },
      true,
    );
  });
  await page.locator('.player-bar').getByRole('button', { name: '播放队列' }).click();
  await expect(page.locator('.app')).toHaveClass(/queue-open/);
  const opened = await page.evaluate(() => {
    const probe = window as unknown as { __transitions: number };
    return { transitions: probe.__transitions, elements: document.querySelectorAll('*').length };
  });
  expect(opened.elements).toBeGreaterThan(50);
  expect(opened.transitions).toBeLessThan(12);
  await expect(page.locator('.queue')).toHaveCSS('transition-property', 'none');
});
````

## File: tests/ui/select.spec.ts
````typescript
import { expect, test } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  await page.goto('/tests/harness/select.html');
});

test('Select delegates navigation, IME, Escape and Tab without changing the value', async ({
  page,
}) => {
  const trigger = page.getByRole('combobox', { name: '播放模式', exact: true });
  const highlighted = page.locator('.select-panel [data-highlighted]');
  await trigger.focus();
  await page.keyboard.press('ArrowDown');
  await expect(highlighted).toHaveText('选择模式');
  await page.keyboard.press('ArrowUp');
  await expect(highlighted).toHaveText('单曲循环');
  await page.keyboard.press('Home');
  await expect(highlighted).toHaveText('选择模式');
  await page.keyboard.press('End');
  await expect(highlighted).toHaveText('单曲循环');
  await page.keyboard.press('ArrowDown');
  await expect(highlighted).toHaveText('选择模式');
  await trigger.dispatchEvent('keydown', { key: 'ArrowDown', isComposing: true });
  await expect(highlighted).toHaveText('选择模式');
  await trigger.dispatchEvent('keydown', { key: '随' });
  await expect(highlighted).toHaveText('随机播放');
  await page.keyboard.press('Escape');
  await expect(page.getByRole('listbox')).toHaveCount(0);
  await expect(trigger).toBeFocused();
  await trigger.press('ArrowDown');
  await page.keyboard.press('Tab');
  await expect(page.getByRole('listbox')).toHaveCount(0);
  await expect(page.getByRole('button', { name: '外部更新' })).toBeFocused();
  await expect(page.getByLabel('绑定值')).toHaveText('(empty)');
  await expect(page.getByLabel('回调')).toHaveText('[]');
  await trigger.press('ArrowDown');
  await page.keyboard.press('Home');
  await page.keyboard.press('ArrowDown');
  await page.keyboard.press('Enter');
  await expect(page.getByLabel('绑定值')).toHaveText('sequence');
  await expect(page.getByLabel('回调')).toHaveText('["sequence"]');
});

test('Select preserves binding, callbacks, empty values, disabled state and both themes', async ({
  page,
}) => {
  const trigger = page.getByRole('combobox', { name: '播放模式', exact: true });
  await expect(page.getByRole('combobox', { name: '禁用选择' })).toBeDisabled();
  await page.getByRole('button', { name: '外部更新' }).click();
  await expect(trigger).toHaveText('随机播放');
  await expect(page.getByLabel('回调')).toHaveText('[]');
  for (const theme of ['dark', 'light']) {
    await page.locator('html').evaluate((el, theme) => (el.dataset.theme = theme), theme);
    await trigger.click();
    const panel = page.getByRole('listbox', { name: '播放模式' });
    await expect(panel).toBeVisible();
    const colors = await panel.evaluate((el) => {
      const style = getComputedStyle(el);
      const probe = document.createElement('div');
      probe.style.background = 'var(--surface-overlay)';
      el.append(probe);
      const expected = getComputedStyle(probe).backgroundColor;
      probe.remove();
      return { actual: style.backgroundColor, expected };
    });
    expect(colors.actual).toBe(colors.expected);
    await page.getByRole('option', { name: '顺序播放' }).click();
    await expect(page.getByLabel('绑定值')).toHaveText('sequence');
    await expect(trigger).toBeFocused();
    await trigger.click();
    await page.getByRole('option', { name: '选择模式' }).click();
    await expect(page.getByLabel('绑定值')).toHaveText('(empty)');
  }
  await expect(page.getByLabel('回调')).toHaveText('["sequence","","sequence",""]');
  await trigger.click();
  await page.getByRole('button', { name: '外部更新' }).click();
  await expect(page.getByRole('listbox')).toHaveCount(0);
  await page.getByRole('combobox', { name: '空列表' }).click();
  await expect(page.getByRole('option')).toHaveCount(0);
  await page.keyboard.press('Escape');
});

test('Select stays usable inside a native dialog and dismisses before its parent', async ({
  page,
}) => {
  await page.setViewportSize({ width: 900, height: 640 });
  await page.getByRole('button', { name: '打开详情' }).click();
  const dialog = page.getByRole('dialog');
  const trigger = dialog.getByRole('combobox', { name: '弹窗模式' });
  await trigger.click();
  const panel = dialog.getByRole('listbox');
  await expect(panel).toBeVisible();
  await page.getByRole('option', { name: '随机播放' }).click();
  await expect(trigger).toHaveText('随机播放');
  await expect(trigger).toBeFocused();
  await trigger.press('ArrowDown');
  await page.keyboard.press('Escape');
  await expect(panel).toHaveCount(0);
  await expect(dialog).toBeVisible();
  await trigger.click();
  await page.mouse.click(4, 4);
  await expect(panel).toHaveCount(0);
  await expect(dialog).toBeVisible();
  await page.keyboard.press('Escape');
  await expect(dialog).toHaveCount(0);
});

test('Select flips at viewport edges and scrolls long lists with the keyboard', async ({
  page,
}) => {
  await page.setViewportSize({ width: 900, height: 640 });
  const trigger = page.getByRole('combobox', { name: '长列表' });
  await trigger.click();
  const panel = page.getByRole('listbox', { name: '长列表' });
  const box = await panel.boundingBox();
  expect(box).toBeTruthy();
  expect(box!.x).toBeGreaterThanOrEqual(8);
  expect(box!.x + box!.width).toBeLessThanOrEqual(892);
  expect(box!.y).toBeGreaterThanOrEqual(8);
  expect(box!.y + box!.height).toBeLessThanOrEqual(632);
  expect(box!.height).toBeLessThanOrEqual(320);
  await page.keyboard.press('End');
  await expect(page.locator('[data-highlighted]')).toHaveText('项目 59');
  await expect(page.getByRole('option', { name: '项目 59', exact: true })).toBeInViewport();
  await page.keyboard.press('Enter');
  await expect(trigger).toHaveText('项目 59');
  await expect(trigger).toBeFocused();
});
````

## File: tests/ui/shell.spec.ts
````typescript
import { expect, test, type Locator, type Page } from '@playwright/test';

const HARNESS = '/tests/harness/index.html';

async function openApp(page: Page) {
  await page.addInitScript(() => {
    const fixed = Date.parse('2026-11-14T15:30:00+08:00');
    const Original = Date;
    function FakeDate(this: Date, ...args: ConstructorParameters<typeof Date>) {
      if (args.length === 0) return new Original(fixed);
      return new Original(...args);
    }
    FakeDate.prototype = Original.prototype;
    Object.assign(FakeDate, { now: () => fixed, parse: Original.parse, UTC: Original.UTC });
    window.Date = FakeDate as DateConstructor;
  });
  await page.goto(HARNESS);
  await expect(page.getByRole('button', { name: 'Music Mine 音乐空间' })).toBeVisible();
  await expect(page.locator('.pb-volume-value')).toHaveText('70%');
  await expect(page.getByRole('alert')).toHaveCount(0);
  await expect(page.getByRole('heading', { name: '下午好' })).toBeVisible();
}

function side(page: Page) {
  return page.locator('.sidebar');
}

function playlists(page: Page) {
  return page.getByRole('list', { name: '我的歌单' });
}

function mainHeading(page: Page, name: string) {
  return page.locator('#main-content').getByRole('heading', { name, exact: true });
}

async function openQueue(page: Page) {
  await page.locator('.player-bar').getByRole('button', { name: '播放队列' }).click();
  const queue = page.getByRole('complementary', { name: '播放队列' });
  await expect(queue).toBeVisible();
  await page.waitForFunction(() => {
    const el = document.querySelector('.queue');
    if (!el) return false;
    const style = getComputedStyle(el);
    return style.visibility === 'visible' && Number(style.opacity) === 1;
  });
  return queue;
}

async function tokenColor(locator: Locator, name: string) {
  return locator.evaluate((el, token) => {
    const probe = document.createElement('div');
    probe.style.background = `var(${token})`;
    el.append(probe);
    const color = getComputedStyle(probe).backgroundColor;
    probe.remove();
    return color;
  }, name);
}

test.beforeEach(async ({ page }) => {
  await openApp(page);
});

test('sidebar opens the main pages', async ({ page }) => {
  await expect(page).toHaveScreenshot('home-dark.png');

  await side(page).getByRole('button', { name: /^歌曲/ }).click();
  await expect(mainHeading(page, '歌曲')).toBeVisible();
  await expect(page.locator('.track-row').first()).toBeVisible();
  await expect(side(page).getByRole('button', { name: /^歌曲/ })).toHaveAttribute(
    'aria-current',
    'page',
  );
  await expect(page).toHaveScreenshot('songs-dark.png');

  const main = page.locator('.main');
  const head = page.locator('.track-head');
  await main.evaluate((el) => {
    el.scrollTop = 900;
  });
  const headBox = await head.boundingBox();
  const mainBox = await main.boundingBox();
  expect(headBox).toBeTruthy();
  expect(mainBox).toBeTruthy();
  expect(headBox!.y).toBeGreaterThanOrEqual(mainBox!.y - 1);
  expect(headBox!.y).toBeLessThan(mainBox!.y + 20);

  await side(page).getByRole('button', { name: /^专辑/ }).click();
  await expect(mainHeading(page, '专辑')).toBeVisible();
  await expect(page.locator('.card').first()).toBeVisible();
  await expect(page).toHaveScreenshot('albums-dark.png');

  await side(page).getByRole('button', { name: '设置' }).click();
  await expect(mainHeading(page, '设置')).toBeVisible();
  await expect(page.getByRole('heading', { name: '外观' })).toBeVisible();
  await expect(page).toHaveScreenshot('settings-dark.png');

  await page.getByRole('button', { name: 'Music Mine 音乐空间' }).click();
  await expect(page.getByRole('heading', { name: '下午好' })).toBeVisible();
});

test('player bar shows the real volume and does not invent progress', async ({ page }) => {
  const bar = page.locator('.player-bar');
  await expect(bar.getByRole('button', { name: '测试歌曲 1' })).toBeVisible();
  await expect(bar.locator('.pb-time').first()).toHaveText('0:42');
  const slider = bar.getByRole('slider', { name: '音量' });
  await expect(slider).toBeEnabled();
  await expect(slider).toHaveAttribute('aria-valuetext', '70%');

  await slider.fill('15');
  await expect(page.locator('.pb-volume-value')).toHaveText('15%');
  await expect(slider).toHaveAttribute('aria-valuetext', '15%');

  await bar.locator('.pb-volume').getByRole('button').click();
  await expect(page.locator('.pb-volume-value')).toHaveText('0%');
  await expect(slider).toHaveAttribute('aria-valuetext', '0%，已静音');
  await bar.locator('.pb-volume').getByRole('button').click();
  await expect(page.locator('.pb-volume-value')).toHaveText('15%');

  await bar.getByRole('button', { name: '播放', exact: true }).click();
  await page.waitForTimeout(500);
  await expect(bar.locator('.pb-time').first()).toHaveText('0:42');
  await expect(bar.getByRole('button', { name: '播放', exact: true })).toBeVisible();
});

test('queue overlays the page and closes from the button, Escape and the scrim', async ({
  page,
}) => {
  const main = page.locator('.main');
  const before = await main.evaluate((el) => el.getBoundingClientRect().width);
  const queue = await openQueue(page);
  const after = await main.evaluate((el) => el.getBoundingClientRect().width);
  expect(after).toBe(before);
  await expect(queue).toContainText('测试歌曲 1');
  const transition = await page.locator('.queue').evaluate((el) =>
    getComputedStyle(el)
      .transitionProperty.split(',')
      .map((part) => part.trim())
      .sort(),
  );
  expect(transition).toEqual(['opacity', 'transform', 'visibility']);
  await expect(page).toHaveScreenshot('queue-dark.png');

  await queue.getByRole('button', { name: '关闭队列 (Esc)' }).click();
  await expect(page.locator('.app')).not.toHaveClass(/queue-open/);

  await openQueue(page);
  await page.keyboard.press('Escape');
  await expect(page.locator('.app')).not.toHaveClass(/queue-open/);

  await page.setViewportSize({ width: 960, height: 700 });
  await openQueue(page);
  await expect(page.locator('.queue-scrim')).toBeVisible();
  await page.locator('.queue-scrim').click();
  await expect(page.locator('.app')).not.toHaveClass(/queue-open/);
});

test('theme switch and a narrower window keep the shell usable', async ({ page }) => {
  await side(page).getByRole('button', { name: '设置' }).click();
  await page.getByRole('combobox', { name: '主题' }).click();
  await page.getByRole('option', { name: '浅色' }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
  await expect(page.locator('.pb-volume-value')).toHaveText('70%');
  await page.getByRole('button', { name: 'Music Mine 音乐空间' }).click();
  await expect(page).toHaveScreenshot('home-light.png');

  await side(page).getByRole('button', { name: '设置' }).click();
  await page.getByRole('combobox', { name: '主题' }).click();
  await page.getByRole('option', { name: '深色' }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');

  await page.setViewportSize({ width: 960, height: 700 });
  await expect(page.locator('.pb-volume-value')).toBeHidden();
  await expect(page.locator('.pb-volume').getByRole('button')).toBeVisible();
  await side(page).getByRole('button', { name: /^歌曲/ }).click();
  await expect(page.locator('.track-row').first()).toBeVisible();
  await expect(page.locator('.player-bar')).toBeVisible();
  await expect(page).toHaveScreenshot('songs-narrow.png');

  await page.setViewportSize({ width: 1280, height: 800 });
  await expect(page.locator('.pb-volume-value')).toBeVisible();
});

test('dialogs cancel without changing playlists and confirm a real create', async ({ page }) => {
  const lists = playlists(page);
  await page.getByRole('button', { name: '新建歌单' }).click();
  const dialog = page.getByRole('dialog');
  await expect(dialog.getByRole('heading', { name: '新建歌单' })).toBeVisible();
  const box = await dialog.boundingBox();
  expect(box).toBeTruthy();
  expect(Math.abs(box!.x + box!.width / 2 - 640)).toBeLessThan(2);
  await expect(page).toHaveScreenshot('dialog-dark.png');
  await page.keyboard.press('Escape');
  await expect(dialog).toHaveCount(0);
  await expect(page.getByRole('button', { name: '新建歌单' })).toBeFocused();
  await expect(lists.getByRole('button', { name: /雨天/ })).toHaveCount(0);

  await page.getByRole('button', { name: '新建歌单' }).click();
  await page.getByRole('textbox', { name: '歌单名称' }).fill('雨天');
  await page.getByRole('button', { name: '创建' }).click();
  await expect(lists.getByRole('button', { name: /雨天/ })).toBeVisible();

  await lists.getByRole('button', { name: /夜晚/ }).click({ button: 'right' });
  await page.getByRole('menuitem', { name: '删除歌单…' }).click();
  await expect(page.getByRole('dialog')).toContainText('不删除本地音乐文件');
  await page.getByRole('button', { name: '取消' }).click();
  await expect(lists.getByRole('button', { name: /夜晚/ })).toBeVisible();
});

test('right-click menus differ for songs, the built-in collection, a playlist, an album and the queue', async ({
  page,
}) => {
  await side(page).getByRole('button', { name: /^歌曲/ }).click();
  await page.locator('.track-row', { hasText: '测试歌曲 3' }).click({ button: 'right' });
  await expect(page.getByRole('menuitem', { name: '播放', exact: true })).toBeVisible();
  await page.keyboard.press('Escape');

  const lists = playlists(page);
  await lists
    .getByRole('button', { name: /本地/ })
    .filter({ hasNotText: '自建' })
    .click({ button: 'right' });
  await expect(page.getByRole('menuitem', { name: '播放全部' })).toBeVisible();
  await expect(page.getByRole('menuitem', { name: '随机播放' })).toBeVisible();
  await expect(page.getByRole('menuitem', { name: /重命名|删除/ })).toHaveCount(0);
  await page.keyboard.press('Escape');

  await lists.getByRole('button', { name: /夜晚/ }).click({ button: 'right' });
  await expect(page.getByRole('menuitem', { name: '重命名…' })).toBeVisible();
  await page.keyboard.press('Escape');

  await side(page).getByRole('button', { name: /^专辑/ }).click();
  await page.locator('.card').first().click({ button: 'right' });
  await expect(page.getByRole('menu')).toBeVisible();
  await page.keyboard.press('Escape');

  await openQueue(page);
  await page.locator('.queue-item').nth(2).click({ button: 'right' });
  await expect(page.getByRole('menuitem', { name: '从队列移除' })).toBeVisible();
});

test('hover and selection use the semantic fills, and the queue stacks above Now Playing', async ({
  page,
}) => {
  const songs = side(page).getByRole('button', { name: /^歌曲/ });
  await songs.hover();
  await expect(songs).toHaveCSS('background-color', await tokenColor(songs, '--hover'));
  await songs.click();
  await expect(songs).toHaveCSS('background-color', await tokenColor(songs, '--selected'));

  const row = page.locator('.track-row').nth(1);
  await row.hover();
  await expect(row).toHaveCSS('background-color', await tokenColor(row, '--hover'));
  await row.click();
  await expect(row).toHaveCSS('background-color', await tokenColor(row, '--selected'));

  const border = await page.locator('.search').evaluate((el) => {
    const style = getComputedStyle(el);
    return `${style.borderTopWidth} ${style.borderTopStyle}`;
  });
  expect(border).toBe('1px solid');

  await page.locator('.player-bar').getByRole('button', { name: '打开正在播放' }).click();
  await expect(page.getByRole('region', { name: '正在播放' })).toBeVisible();
  await expect(page).toHaveScreenshot('now-playing-dark.png');
  const queue = await openQueue(page);
  const item = queue.locator('.queue-item').first();
  await expect(item).toHaveCSS('background-color', await tokenColor(item, '--selected'));

  const layers = await page.evaluate(() => {
    const read = (selector: string) =>
      Number(getComputedStyle(document.querySelector(selector)!).zIndex);
    return {
      queue: read('.queue'),
      scrim: read('.queue-scrim'),
      immersive: read('.immersive-layer'),
      player: read('.player-bar'),
    };
  });
  expect(layers.queue).toBeGreaterThan(layers.scrim);
  expect(layers.scrim).toBeGreaterThan(layers.immersive);
  expect(layers.immersive).toBeGreaterThan(layers.player);

  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.reload();
  await expect(page.locator('.pb-volume-value')).toHaveText('70%');
  await openQueue(page);
  await expect
    .poll(() =>
      page
        .locator('.queue')
        .evaluate((el) => getComputedStyle(el).getPropertyValue('--drawer-shift').trim()),
    )
    .toBe('0px');
});
````

## File: tests/ipc-surface.test.mjs
````javascript
import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';
import assert from 'node:assert/strict';

const root = dirname(dirname(fileURLToPath(import.meta.url)));
const expectedUnused = ['library_cover', 'library_favorite'];

function extractInvokeNames(source) {
  const names = new Set();
  const length = source.length;
  let i = 0;
  let mode = 'code';
  while (i < length) {
    const char = source[i];
    const next = source[i + 1];
    if (mode === 'code') {
      if (char === '/' && next === '/') {
        mode = 'line';
        i += 2;
        continue;
      }
      if (char === '/' && next === '*') {
        mode = 'block';
        i += 2;
        continue;
      }
      if (char === "'") {
        mode = 'sq';
        i++;
        continue;
      }
      if (char === '"') {
        mode = 'dq';
        i++;
        continue;
      }
      if (char === '`') {
        mode = 'tpl';
        i++;
        continue;
      }
      if (
        source.startsWith('invoke', i) &&
        (i === 0 || !/[A-Za-z0-9_]/.test(source[i - 1])) &&
        !/[A-Za-z0-9_]/.test(source[i + 6] ?? '')
      ) {
        const name = readInvokeName(source, i + 6);
        if (name) names.add(name);
      }
      i++;
      continue;
    }
    if (mode === 'line') {
      if (char === '\n') mode = 'code';
      i++;
      continue;
    }
    if (mode === 'block') {
      if (char === '*' && next === '/') {
        mode = 'code';
        i += 2;
        continue;
      }
      i++;
      continue;
    }
    if (mode === 'sq' || mode === 'dq' || mode === 'tpl') {
      const quote = mode === 'sq' ? "'" : mode === 'dq' ? '"' : '`';
      if (char === '\\') {
        i += 2;
        continue;
      }
      if (char === quote) mode = 'code';
      i++;
    }
  }
  return names;
}

function readInvokeName(source, start) {
  let j = start;
  while (/\s/.test(source[j] ?? '')) j++;
  if (source[j] === '<') {
    let depth = 0;
    for (; j < source.length; j++) {
      if (source[j] === '<') depth++;
      else if (source[j] === '>') {
        depth--;
        if (depth === 0) {
          j++;
          break;
        }
      }
    }
    while (/\s/.test(source[j] ?? '')) j++;
  }
  if (source[j] !== '(') return '';
  j++;
  while (/\s/.test(source[j] ?? '')) j++;
  const quote = source[j];
  if (quote !== "'" && quote !== '"') return '';
  const end = source.indexOf(quote, j + 1);
  if (end < 0) return '';
  const name = source.slice(j + 1, end);
  return /^[a-z0-9_]+$/.test(name) ? name : '';
}

function stripComments(source) {
  let out = '';
  let i = 0;
  let mode = 'code';
  while (i < source.length) {
    const char = source[i];
    const next = source[i + 1];
    if (mode === 'code') {
      if (char === '/' && next === '/') {
        mode = 'line';
        i += 2;
        continue;
      }
      if (char === '/' && next === '*') {
        mode = 'block';
        i += 2;
        continue;
      }
      if (char === "'") {
        mode = 'sq';
        out += char;
        i++;
        continue;
      }
      if (char === '"') {
        mode = 'dq';
        out += char;
        i++;
        continue;
      }
      if (char === '`') {
        mode = 'tpl';
        out += char;
        i++;
        continue;
      }
      out += char;
      i++;
      continue;
    }
    if (mode === 'line') {
      if (char === '\n') {
        mode = 'code';
        out += char;
      }
      i++;
      continue;
    }
    if (mode === 'block') {
      if (char === '*' && next === '/') {
        mode = 'code';
        i += 2;
        continue;
      }
      i++;
      continue;
    }
    out += char;
    const quote = mode === 'sq' ? "'" : mode === 'dq' ? '"' : '`';
    if (char === '\\') {
      out += source[i + 1] ?? '';
      i += 2;
      continue;
    }
    if (char === quote) mode = 'code';
    i++;
  }
  return out;
}

function extractRegistered(source) {
  const names = new Set();
  const text = stripComments(source);
  const pattern = /generate_handler!\s*\[([\s\S]*?)\]/g;
  let match;
  while ((match = pattern.exec(text))) {
    for (const part of match[1].split(',')) {
      const token = part.trim();
      if (!token) continue;
      const name = token.split('::').pop().trim();
      if (/^[A-Za-z0-9_]+$/.test(name)) names.add(name);
    }
  }
  return names;
}

function extractHandlerKeys(source) {
  const text = stripComments(source);
  const at = text.indexOf('const handlers');
  if (at < 0) throw new Error('handlers object not found');
  let i = text.indexOf('{', at);
  let depth = 0;
  const keys = new Set();
  let mode = 'code';
  for (; i < text.length; i++) {
    const char = text[i];
    const next = text[i + 1];
    if (mode === 'code') {
      if (char === "'") {
        mode = 'sq';
        continue;
      }
      if (char === '"') {
        mode = 'dq';
        continue;
      }
      if (char === '`') {
        mode = 'tpl';
        continue;
      }
      if (char === '{') {
        depth++;
        continue;
      }
      if (char === '}') {
        depth--;
        if (depth === 0) break;
        continue;
      }
      if (depth === 1 && /[A-Za-z_]/.test(char)) {
        const found = /^[A-Za-z_][A-Za-z0-9_]*/.exec(text.slice(i));
        const name = found[0];
        let j = i + name.length;
        while (/\s/.test(text[j] ?? '')) j++;
        if (text[j] === ':') keys.add(name);
        i = j - 1;
      }
      continue;
    }
    const quote = mode === 'sq' ? "'" : mode === 'dq' ? '"' : '`';
    if (char === '\\') {
      i++;
      continue;
    }
    if (char === quote) mode = 'code';
  }
  return keys;
}

function extractFence(markdown, info) {
  const pattern = new RegExp('```' + info + '\\r?\\n([\\s\\S]*?)```');
  const match = pattern.exec(markdown);
  assert.ok(match, `missing fence ${info}`);
  return new Set(
    match[1]
      .split(/\r?\n/)
      .map((line) => line.trim())
      .filter((line) => line.length > 0),
  );
}

function rustSources(dir, found = []) {
  for (const name of readdirSync(dir)) {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) rustSources(path, found);
    else if (name.endsWith('.rs')) found.push(path);
  }
  return found;
}

function diff(left, right) {
  return [...left].filter((name) => !right.has(name)).sort();
}

test('invoke extraction ignores comments, strings, and repeated calls', () => {
  const sample = `
    // invoke('not_a_command')
    /* invoke("also_no") */
    invoke<Record<string, number>>(
      'library_cover_stats'
    );
    invoke('library_cover_stats');
    const sample = "invoke('inside_string')";
  `;
  assert.deepEqual([...extractInvokeNames(sample)], ['library_cover_stats']);
});

test('registration extraction ignores comments and keeps the command name', () => {
  const sample = `
    generate_handler![
      desktop::desktop_preferences,
      // player_fake,
      player_get_state,
    ]
  `;
  assert.deepEqual([...extractRegistered(sample)].sort(), [
    'desktop_preferences',
    'player_get_state',
  ]);
});

test('ipc names stay aligned across rust, frontend, harness, and the contract lists', () => {
  const registered = new Set();
  for (const path of rustSources(join(root, 'src-tauri', 'src'))) {
    for (const name of extractRegistered(readFileSync(path, 'utf8'))) registered.add(name);
  }
  const frontend = extractInvokeNames(
    readFileSync(join(root, 'src', 'lib', 'services', 'music.ts'), 'utf8'),
  );
  const harness = extractHandlerKeys(
    readFileSync(join(root, 'tests', 'harness', 'mock-ipc.js'), 'utf8'),
  );
  const contract = readFileSync(join(root, 'docs', 'ipc-contract.md'), 'utf8');
  const documented = extractFence(contract, 'ipc-commands');
  const unused = extractFence(contract, 'ipc-unused');
  const unusedActual = diff(registered, frontend);

  assert.deepEqual(diff(frontend, registered), [], 'frontend command is not registered');
  assert.deepEqual(diff(frontend, harness), [], 'frontend command has no harness handler');
  assert.deepEqual(
    diff(documented, registered),
    [],
    'contract lists a command that is not registered',
  );
  assert.deepEqual(
    diff(registered, documented),
    [],
    'registered command is missing from the contract list',
  );
  assert.deepEqual(unusedActual, [...expectedUnused].sort());
  assert.deepEqual([...unused].sort(), [...expectedUnused].sort());
});
````

## File: tests/isolation-guard.test.mjs
````javascript
import { spawnSync } from 'node:child_process';
import { mkdtempSync, mkdirSync, writeFileSync, rmSync, symlinkSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';
import assert from 'node:assert/strict';

const repo = dirname(dirname(fileURLToPath(import.meta.url)));
const cli = join(repo, 'scripts', 'isolation-cli.ps1');

function run(args) {
  return spawnSync(
    'powershell.exe',
    ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', cli, ...args],
    {
      encoding: 'utf8',
      cwd: repo,
    },
  );
}

function status(args) {
  const result = run(args);
  assert.equal(result.error, undefined, String(result.error));
  return result;
}

test('path checks reject traversal, similar prefixes, and junctions', () => {
  const base = mkdtempSync(join(tmpdir(), 'mm-path-'));
  const corpus = join(base, 'mm-cover-corpus');
  const evil = join(base, 'mm-cover-corpus-evil');
  const outside = join(base, 'outside');
  mkdirSync(join(corpus, 'sub'), { recursive: true });
  mkdirSync(evil, { recursive: true });
  mkdirSync(outside, { recursive: true });
  writeFileSync(join(corpus, 'sub', 'file.txt'), 'inside');
  writeFileSync(join(evil, 'file.txt'), 'evil');
  writeFileSync(join(outside, 'file.txt'), 'outside');
  symlinkSync(outside, join(corpus, 'link'), 'junction');
  try {
    assert.equal(
      status([
        '-Action',
        'test-path',
        '-Candidate',
        join(corpus, 'sub', 'file.txt'),
        '-Root',
        corpus,
      ]).status,
      0,
    );
    assert.equal(
      status(['-Action', 'test-path', '-Candidate', join(evil, 'file.txt'), '-Root', corpus])
        .status,
      1,
    );
    assert.equal(
      status([
        '-Action',
        'test-path',
        '-Candidate',
        `${corpus}\\sub\\..\\..\\outside`,
        '-Root',
        corpus,
      ]).status,
      1,
    );
    assert.equal(
      status([
        '-Action',
        'test-path',
        '-Candidate',
        join(corpus, 'link', 'file.txt'),
        '-Root',
        corpus,
      ]).status,
      1,
    );
    assert.equal(
      status(['-Action', 'test-delete', '-Candidate', join(corpus, 'sub'), '-Root', corpus]).status,
      0,
    );
    const blocked = status(['-Action', 'test-delete', '-Candidate', corpus, '-Root', corpus]);
    assert.equal(blocked.status, 1, blocked.stdout);
    assert.match(blocked.stdout, /reparse point/);
  } finally {
    rmSync(base, { recursive: true, force: true });
  }
});

test('descendant collection includes grandchildren and skips other trees', () => {
  const dir = mkdtempSync(join(tmpdir(), 'mm-proc-'));
  const table = join(dir, 'procs.json');
  writeFileSync(
    table,
    JSON.stringify([
      { ProcessId: 1, ParentProcessId: 0 },
      { ProcessId: 2, ParentProcessId: 1 },
      { ProcessId: 3, ParentProcessId: 2 },
      { ProcessId: 4, ParentProcessId: 9 },
    ]),
  );
  try {
    const result = status(['-Action', 'descendants', '-Table', table, '-ProcessId', '1']);
    assert.equal(result.status, 0, result.stdout + result.stderr);
    assert.deepEqual(result.stdout.trim().split(/\r?\n/).filter(Boolean), ['2', '3']);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

function fakeRepo() {
  const fake = mkdtempSync(join(tmpdir(), 'mm-src-'));
  const built = join(fake, 'src-tauri', 'target', 'debug');
  mkdirSync(built, { recursive: true });
  mkdirSync(join(fake, 'src-tauri'), { recursive: true });
  writeFileSync(join(fake, 'src-tauri', 'tauri.conf.json'), '{"productName":"Music Mine"}\n');
  return { fake, built };
}

test('launch source requires a hash-bound record and checks the two data-dir kinds separately', () => {
  const { fake, built } = fakeRepo();
  const exe = join(built, 'music-mine-desktop.exe');
  const other = join(built, 'other-desktop.exe');
  writeFileSync(exe, 'portable-bytes');
  writeFileSync(other, 'absolute-bytes');
  const portable = join(fake, 'src-tauri', 'tauri.portable.conf.json');
  writeFileSync(portable, '{"app":{"appDirectoriesOverride":"./app-data"}}\n');
  const testRoot = join(fake, 'empty-root');
  const data = join(testRoot, 'data');
  mkdirSync(data, { recursive: true });
  const outside = join(fake, 'outside');
  mkdirSync(outside, { recursive: true });
  const absoluteConfig = join(fake, 'override.json');
  writeFileSync(absoluteConfig, JSON.stringify({ app: { appDirectoriesOverride: data } }));
  const escapeConfig = join(fake, 'escape.json');
  writeFileSync(
    escapeConfig,
    JSON.stringify({
      app: { appDirectoriesOverride: join(testRoot, 'data', '..', '..', 'outside') },
    }),
  );
  try {
    const missing = status(['-Action', 'check-source', '-Repo', fake, '-Exe', exe]);
    assert.equal(missing.status, 1);
    assert.match(missing.stdout, /MISSING_RECORD/);

    const outsideExe = join(fake, 'stray.exe');
    writeFileSync(outsideExe, 'stray');
    const stray = status([
      '-Action',
      'write-source',
      '-Repo',
      fake,
      '-BuiltExe',
      outsideExe,
      '-Config',
      'src-tauri/tauri.portable.conf.json',
      '-Command',
      'test',
      '-Kind',
      'portable-relative',
      '-Expected',
      './app-data',
    ]);
    assert.equal(stray.status, 1, stray.stdout);

    const wrote = status([
      '-Action',
      'write-source',
      '-Repo',
      fake,
      '-BuiltExe',
      exe,
      '-Config',
      'src-tauri/tauri.portable.conf.json',
      '-Command',
      'pnpm exec tauri build --config src-tauri/tauri.portable.conf.json',
      '-Kind',
      'portable-relative',
      '-Expected',
      './app-data',
    ]);
    assert.equal(wrote.status, 0, wrote.stdout + wrote.stderr);
    assert.equal(status(['-Action', 'check-source', '-Repo', fake, '-Exe', exe]).status, 0);

    writeFileSync(portable, '{"app":{"appDirectoriesOverride":"C:\\\\somewhere"}}\n');
    const changed = status(['-Action', 'check-source', '-Repo', fake, '-Exe', exe]);
    assert.equal(changed.status, 1, changed.stdout);
    assert.match(changed.stdout, /CONFIG_HASH_MISMATCH|PORTABLE_OVERRIDE_NOT_RELATIVE/);

    writeFileSync(portable, '{"app":{"appDirectoriesOverride":"./app-data"}}\n');
    const absoluteOverride = status([
      '-Action',
      'write-source',
      '-Repo',
      fake,
      '-BuiltExe',
      exe,
      '-Config',
      'src-tauri/tauri.portable.conf.json',
      '-Command',
      'test',
      '-Kind',
      'portable-relative',
      '-Expected',
      './app-data',
    ]);
    assert.equal(absoluteOverride.status, 0, absoluteOverride.stdout + absoluteOverride.stderr);
    writeFileSync(portable, '{"app":{"appDirectoriesOverride":"C:\\\\abs"}}\n');
    const refusedKind = status([
      '-Action',
      'write-source',
      '-Repo',
      fake,
      '-BuiltExe',
      other,
      '-Config',
      'src-tauri/tauri.portable.conf.json',
      '-Command',
      'test',
      '-Kind',
      'portable-relative',
      '-Expected',
      './app-data',
    ]);
    assert.equal(refusedKind.status, 1, refusedKind.stdout);
    writeFileSync(portable, '{"app":{"appDirectoriesOverride":"./app-data"}}\n');

    const absolute = status([
      '-Action',
      'write-source',
      '-Repo',
      fake,
      '-BuiltExe',
      other,
      '-Config',
      absoluteConfig,
      '-Command',
      'pnpm exec tauri build --debug --no-bundle --config override.json',
      '-Kind',
      'absolute-override',
      '-Expected',
      data,
      '-TestRoot',
      testRoot,
    ]);
    assert.equal(absolute.status, 0, absolute.stdout + absolute.stderr);
    assert.equal(status(['-Action', 'check-source', '-Repo', fake, '-Exe', other]).status, 0);

    const escaped = status([
      '-Action',
      'write-source',
      '-Repo',
      fake,
      '-BuiltExe',
      other,
      '-Config',
      escapeConfig,
      '-Command',
      'test',
      '-Kind',
      'absolute-override',
      '-Expected',
      join(testRoot, 'data', '..', '..', 'outside'),
      '-TestRoot',
      testRoot,
    ]);
    assert.equal(escaped.status, 1, escaped.stdout);
    assert.match(escaped.stdout, /escapes the test root/);
  } finally {
    rmSync(fake, { recursive: true, force: true });
  }
});
````

## File: tests/lyrics-view.test.mjs
````javascript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import {
  displayParts,
  groupLines,
  activeLine,
  scrollTarget,
} from '../src/lib/utils/lyrics-view.ts';
test('verified thin-space bilingual separator preserves all content', () => {
  const text = '너 같은 남자는 No No No\u2009像你这种男人 不不不';
  assert.equal(displayParts(text).join('\u2009'), text);
  assert.equal(displayParts(text).length, 2);
  assert.deepEqual(displayParts('한국어 中文 English 日本語'), ['한국어 中文 English 日本語']);
});
test('equal timestamps remain separate display lines in one active group', () => {
  const rows = groupLines(
    [
      { time_ms: 100, text: '원문' },
      { time_ms: 100, text: '翻译' },
      { time_ms: 200, text: 'next' },
    ],
    true,
  );
  assert.equal(rows.length, 2);
  assert.deepEqual(rows[0].parts, ['원문', '翻译']);
  assert.equal(activeLine(rows, 99), -1);
  assert.equal(activeLine(rows, 100), 0);
  assert.equal(activeLine(rows, 200), 1);
  assert.equal(activeLine(rows, 100), 0);
});
test('plain lyrics never group at the sentinel timestamp', () =>
  assert.equal(
    groupLines(
      [
        { time_ms: -1, text: 'a' },
        { time_ms: -1, text: 'b' },
      ],
      false,
    ).length,
    2,
  ));
test('scroll target clamps to independent container bounds', () => {
  assert.equal(scrollTarget(0, 40, 400, 1200), 0);
  assert.equal(scrollTarget(600, 40, 400, 1200), 460);
  assert.equal(scrollTarget(1190, 40, 400, 1200), 800);
});
````

## File: tests/player-stability.test.mjs
````javascript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync, unlinkSync } from 'node:fs';
import { compileModule } from 'svelte/compiler';
import { execFileSync } from 'node:child_process';

test('real Svelte effects keep cover stable through progress, pause, volume and reject stale results', () => {
  const app = readFileSync('src/App.svelte', 'utf8');
  const start = app.search(/let artwork\s*=\s*\$state/);
  const end = app.search(/async\s+function\s+connect\s*\(/);
  assert.ok(start >= 0 && end > start, 'cover effect is present in App');
  const declaration = app.slice(start, end).replace(/<string\s*\|\s*null>/g, '');
  const code = `import {flushSync} from 'svelte';
 let snapshot=$state(null); const desktopAvailable=true;const diagnostics=null; let pending=[]; let calls=0; const getArtwork=()=>{calls++;return new Promise(r=>pending.push(r));};
 export async function run(){let dispose= $effect.root(()=>{${declaration}
 globalThis.read=()=>({artwork,calls});});
 const update=(v)=>flushSync(()=>snapshot=v);
 update({track_id:'a'});pending.shift()('cover-a');await Promise.resolve();flushSync();
 for(let i=0;i<1500;i++)update({track_id:'a',position_ms:i*200,volume:i%100,playback_state:i%2?'playing':'paused'});
 if(calls!==1||globalThis.read().artwork!=='cover-a')throw Error('progress reloaded cover '+calls);
 update({track_id:'b'});const old=pending.shift(); update({track_id:'c'});pending.shift()('cover-c');await Promise.resolve();old('cover-b');await Promise.resolve();flushSync();
 if(globalThis.read().artwork!=='cover-c'||calls!==3)throw Error('stale cover');dispose(); }
 `;
  const path = 'tests/.cover-runtime.mjs';
  try {
    writeFileSync(
      path,
      compileModule(code, { filename: 'cover.svelte.ts', generate: 'client' }).js.code +
        '\nawait run();',
    );
    execFileSync(process.execPath, ['--conditions=browser', path], { stdio: 'pipe' });
  } finally {
    try {
      unlinkSync(path);
    } catch {}
  }
});

test('player bar never resets the cover or the seek draft on progress-only snapshots', () => {
  const bar = readFileSync('src/lib/layout/PlayerBar.svelte', 'utf8');
  // Seek draft resets only when the track identity changes, not on each 200 ms snapshot.
  assert.match(
    bar,
    /let\s+trackId\s*=\s*\$derived\(\s*s\s*\?\.\s*track_id\s*\)\s*;?\s*\$effect\(\s*\(\s*\)\s*=>\s*\{\s*trackId\s*;\s*dragging\s*=\s*null\s*;?\s*\}\s*\)\s*;?/,
  );
  // The bar renders the App-level artwork and does not request covers itself.
  assert.doesNotMatch(bar, /getArtwork|trackCover/);
  const now = readFileSync('src/lib/views/NowPlaying.svelte', 'utf8');
  assert.doesNotMatch(
    now,
    /getArtwork\(/,
    'Now Playing reuses the App cover instead of a second IPC request',
  );
});
````

## File: tests/reduce-motion.test.mjs
````javascript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { sourceStyles } from './helpers/source-styles.mjs';

const css = sourceStyles();

/** Rules whose selector list targets every element, including the reduce-motion variant. */
function universalRules(source) {
  const rules = [];
  for (const match of source.matchAll(/([^{}]+)\{([^{}]*)\}/g)) {
    const selectors = match[1].split(',').map((part) => part.trim());
    const universal = selectors.some(
      (selector) =>
        selector === '*' ||
        selector === '*::before' ||
        selector === '*::after' ||
        selector === '.reduce-motion *' ||
        selector === '.reduce-motion *::before' ||
        selector === '.reduce-motion *::after',
    );
    if (universal) rules.push({ selectors, body: match[2] });
  }
  return rules;
}

test('a universal reduce-motion duration does not transition every property', () => {
  const rules = universalRules(css);
  const timed = rules.filter((rule) => /transition-duration\s*:/.test(rule.body));
  assert.ok(
    timed.length >= 2,
    'both the media query and the in-app class set a universal duration',
  );
  for (const rule of timed) {
    assert.match(
      rule.body,
      /transition-property\s*:\s*none\s*!important/,
      rule.selectors.join(', '),
    );
  }
});
````

## File: tests/shortcuts.test.mjs
````javascript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { shortcut } from '../src/lib/utils/shortcuts.ts';
const key = (value, overrides = {}) => ({
  key: value,
  ctrlKey: false,
  altKey: false,
  metaKey: false,
  shiftKey: false,
  repeat: false,
  isComposing: false,
  defaultPrevented: false,
  ...overrides,
});
test('window actions map to requested keys', () => {
  for (const [value, action] of [
    ['f', 'search'],
    ['ArrowLeft', 'previous'],
    ['ArrowRight', 'next'],
    ['l', 'lyrics'],
    ['o', 'open'],
  ])
    assert.equal(shortcut(key(value, { ctrlKey: true }), false, false, false), action);
  assert.equal(shortcut(key(' '), false, false, false), 'toggle');
  assert.equal(shortcut(key('Escape'), true, false, true), 'escape');
});
test('editing, native controls and dialogs preserve their keyboard behavior', () => {
  assert.equal(shortcut(key(' '), true, false, false), null);
  assert.equal(shortcut(key(' '), false, true, false), null);
  for (const value of ['ArrowLeft', 'ArrowRight', 'l', 'o'])
    assert.equal(shortcut(key(value, { ctrlKey: true }), true, false, false), null);
  assert.equal(shortcut(key('f', { ctrlKey: true }), true, false, false), 'search');
  assert.equal(shortcut(key(' '), false, false, true), null);
});
test('repeat, composition, handled events and extra modifiers do not dispatch', () => {
  for (const flag of ['repeat', 'isComposing', 'defaultPrevented', 'altKey', 'metaKey', 'shiftKey'])
    assert.equal(shortcut(key(' ', { [flag]: true }), false, false, false), null);
  assert.equal(shortcut(key('k', { ctrlKey: true }), false, false, false), null);
});
````

## File: tests/ui-refinements.test.mjs
````javascript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

test('playlist deletion is confirmed, addresses the playlist by ID and states files are kept', () => {
  const actions = readFileSync('src/lib/actions.ts', 'utf8');
  const body = actions.slice(actions.indexOf('export async function deletePlaylistDialog'));
  assert.match(body, /confirmAction\(/);
  assert.match(body, /只删除歌单，不删除本地音乐文件/);
  assert.match(body, /await deletePlaylist\(playlist\.id\)/);
  assert.ok(
    body.indexOf('confirmAction(') < body.indexOf('await deletePlaylist('),
    'confirmation happens before deletion',
  );
  assert.match(
    body,
    /catch\s*\(\s*e\s*\)\s*\{\s*notifyError\s*\(\s*e\s*\)\s*;?\s*\}/,
    'failures surface the real error',
  );
  const service = readFileSync('src/lib/services/music.ts', 'utf8');
  assert.match(
    service,
    /deletePlaylist\s*=\s*\(\s*playlistId\s*:\s*number\s*\)\s*=>\s*invoke\s*<\s*void\s*>\s*\(\s*'library_playlist_edit'\s*,\s*\{\s*action\s*:\s*'delete'\s*,\s*playlistId\b/,
  );
});
````

## File: tests/v09-utils.test.mjs
````javascript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { allowsNativeMenu, isBrowserAccelerator } from '../src/lib/utils/native-menu.ts';
import { playlistNameError, suggestPlaylistName } from '../src/lib/utils/playlist-name.ts';
import { createHistory } from '../src/lib/utils/history.ts';
import { visibleRange, pagesFor, scrollToRow } from '../src/lib/utils/virtual-range.ts';
import { ambientColor } from '../src/lib/utils/color.ts';
import { duration, initial, nameHue } from '../src/lib/utils/format.ts';

const el = (editable) => ({
  closest: (selector) => (editable && selector.includes('textarea') ? {} : null),
});

test('native WebView menu only for text fields, or Shift+right click in development', () => {
  assert.equal(allowsNativeMenu(el(false), false, false), false);
  assert.equal(allowsNativeMenu(el(true), false, false), true);
  assert.equal(allowsNativeMenu(el(false), true, true), true, 'dev keeps Inspect via Shift');
  assert.equal(allowsNativeMenu(el(false), false, true), false, 'release ignores Shift');
  assert.equal(allowsNativeMenu(null, false, false), false);
  const k = (key, mods = {}) => ({
    key,
    ctrlKey: false,
    shiftKey: false,
    altKey: false,
    metaKey: false,
    ...mods,
  });
  assert.equal(isBrowserAccelerator(k('F5')), true);
  assert.equal(isBrowserAccelerator(k('r', { ctrlKey: true })), true);
  assert.equal(isBrowserAccelerator(k('P', { ctrlKey: true, shiftKey: true })), true);
  assert.equal(
    isBrowserAccelerator(k('f', { ctrlKey: true })),
    false,
    'Ctrl+F stays the app search shortcut',
  );
  assert.equal(isBrowserAccelerator(k('l', { ctrlKey: true })), false);
  assert.equal(isBrowserAccelerator(k(' ')), false);
});

test('playlist names: blank, too long and case-insensitive duplicates rejected', () => {
  const lists = [
    { id: 1, name: '本地' },
    { id: 2, name: 'Night Drive' },
  ];
  assert.equal(playlistNameError('   ', lists), '请输入歌单名称');
  assert.match(playlistNameError(' 本地 ', lists), /已存在/);
  assert.match(playlistNameError('night drive', lists), /已存在/);
  assert.equal(playlistNameError('Night Drive', lists, 2), null, 'renaming to itself is fine');
  assert.equal(playlistNameError('新しい 🎵', lists), null);
  assert.match(playlistNameError('x'.repeat(101), lists), /最多/);
  assert.equal(suggestPlaylistName([]), '新建歌单');
  assert.equal(suggestPlaylistName([{ name: '新建歌单' }, { name: '新建歌单 2' }]), '新建歌单 3');
});

test('history: back/forward, no duplicates, forgetting a deleted playlist', () => {
  const h = createHistory({ view: 'home' });
  h.push({ view: 'songs' });
  h.push({ view: 'songs' });
  h.push({ view: 'playlist', id: 7 });
  h.push({ view: 'albums' });
  assert.equal(h.canBack, true);
  h.back();
  assert.deepEqual(h.current, { view: 'playlist', id: 7 });
  h.back();
  assert.deepEqual(h.current, { view: 'songs' });
  h.forward();
  assert.deepEqual(h.current, { view: 'playlist', id: 7 });
  h.forget((r) => r.view === 'playlist' && r.id === 7, { view: 'songs' });
  assert.deepEqual(h.current, { view: 'songs' });
  h.forward();
  assert.deepEqual(h.current, { view: 'albums' });
  h.back();
  h.back();
  assert.deepEqual(h.current, { view: 'home' });
  assert.equal(h.canBack, false);
});

test('virtual list range, page loading and keyboard scrolling', () => {
  assert.deepEqual(visibleRange(0, 560, 100, 56, 1000, 4), { start: 0, end: 13 });
  assert.deepEqual(visibleRange(5600, 560, 100, 56, 1000, 4), { start: 94, end: 113 });
  assert.deepEqual(visibleRange(999999, 560, 0, 56, 30, 4), { start: 30, end: 30 });
  assert.deepEqual(visibleRange(0, 560, 0, 56, 0), { start: 0, end: 0 });
  assert.deepEqual(pagesFor(95, 215, 100), [0, 1, 2]);
  assert.deepEqual(pagesFor(5, 5, 100), []);
  assert.equal(scrollToRow(0, 500, 560, 100, 56), 100);
  assert.equal(scrollToRow(20, 0, 560, 100, 56), 100 + 21 * 56 - 560);
  assert.equal(scrollToRow(3, 0, 560, 100, 56), null);
});

test('ambient colour is muted and readable, ignoring transparent and extreme pixels', () => {
  const red = [];
  for (let i = 0; i < 64; i++) red.push(230, 30, 40, 255);
  const [r, g, b] = ambientColor(red);
  assert.ok(r > g && r > b, 'keeps the hue');
  const max = Math.max(r, g, b) / 255,
    min = Math.min(r, g, b) / 255;
  assert.ok((max + min) / 2 <= 0.43, 'lightness capped');
  assert.equal(ambientColor([0, 0, 0, 255, 255, 255, 255, 255, 10, 200, 10, 0]), null);
});

test('formatting helpers', () => {
  assert.equal(duration(0), '0:00');
  assert.equal(duration(61500), '1:01');
  assert.equal(duration(3723000), '1:02:03');
  assert.equal(initial('  ailee'), 'A');
  assert.equal(initial('周杰伦'), '周');
  assert.equal(initial(''), '?');
  assert.equal(nameHue('IU'), nameHue('IU'));
  assert.ok(nameHue('IU') >= 0 && nameHue('IU') < 360);
});
````

## File: tests/v091-collections.test.mjs
````javascript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join } from 'node:path';
import { LOCAL_COLLECTION, localQuery, shadowsBuiltIn } from '../src/lib/utils/collections.ts';
import { playlistNameError } from '../src/lib/utils/playlist-name.ts';

const read = (f) => readFileSync(f, 'utf8');
function sources(dir) {
  return readdirSync(dir).flatMap((n) => {
    const p = join(dir, n);
    return statSync(p).isDirectory() ? sources(p) : /\.(svelte|ts)$/.test(n) ? [p] : [];
  });
}

test('built-in 本地 is a stable route over the whole library, not a stored playlist', () => {
  assert.equal(LOCAL_COLLECTION.view, 'local');
  const q = localQuery('title', '周');
  assert.deepEqual(q, { sort: 'title', search: '周' });
  assert.ok(
    !('playlist' in q) && !('favorite' in q),
    'no membership filter: imports and rescans show up automatically',
  );
  const nav = read('src/lib/stores/nav.svelte.ts');
  assert.match(nav, /\{\s*view\s*:\s*'local'\s*\}/);
  const sidebar = read('src/lib/layout/Sidebar.svelte');
  assert.match(sidebar, /go\(\s*\{\s*view\s*:\s*'local'\s*\}\s*\)/, 'opened by route kind');
  assert.match(
    sidebar,
    /library\.summary\.tracks\s*\}\s*<\/span>/,
    'count is the library size from the summary',
  );
  const songs = read('src/lib/views/SongsView.svelte');
  assert.match(songs, /case\s+'local'\s*:\s*return\s+localQuery\(\s*sort\s*,\s*''\s*\)/);
  assert.match(
    songs,
    /route\.view\s*===\s*'songs'\s*\|\|\s*route\.view\s*===\s*'local'\s*\|\|\s*route\.view\s*===\s*'artist'/,
    'sortable',
  );
});

test('built-in collection can be played and shuffled but never renamed or deleted', () => {
  const actions = read('src/lib/actions.ts');
  const menu = actions.slice(
    actions.indexOf('export function localMenu'),
    actions.indexOf('export async function newPlaylist'),
  );
  assert.match(menu, /播放全部/);
  assert.match(menu, /随机播放/);
  assert.doesNotMatch(menu, /deletePlaylist|renamePlaylist|删除|重命名/);
  const services = read('src/lib/services/music.ts');
  assert.doesNotMatch(services, /'local'/, 'no IPC ever addresses the built-in collection');
});

test('a user playlist named 本地 keeps its data; the name is reserved only for new names', () => {
  const lists = [
    { id: 7, name: '本地' },
    { id: 8, name: '夜晚' },
  ];
  assert.equal(shadowsBuiltIn(' 本地 '), true);
  assert.equal(shadowsBuiltIn('本地音乐'), false);
  assert.equal(
    playlistNameError('本地', lists, 7),
    null,
    'existing same-name playlist can be saved unchanged',
  );
  assert.match(
    playlistNameError('本地', [{ id: 8, name: '夜晚' }]),
    /系统歌单/,
    'cannot create a new one',
  );
  assert.match(
    playlistNameError('本地', [{ id: 8, name: '夜晚' }], 8),
    /系统歌单/,
    'cannot rename into it',
  );
  assert.equal(playlistNameError('夜晚 2', lists), null, 'ordinary playlists unaffected');
  const sidebar = read('src/lib/layout/Sidebar.svelte');
  assert.match(
    sidebar,
    /\{\s*#each\s+library\.summary\.playlists\s+as\s+list\s+\(\s*list\.id\s*\)\s*\}/,
    'user playlists still listed by ID',
  );
  assert.match(sidebar, /playlistMenu\(list\)/, 'and keep rename/delete');
});

test('favourites removed from every UI surface without touching stored data', () => {
  for (const file of sources('src')) {
    const text = read(file);
    assert.doesNotMatch(
      text,
      /收藏|favoriteTrack|setFavorite|favoritesOnly|library_favorite|\bHeart\b/,
      file,
    );
  }
  const songs = read('src/lib/views/SongsView.svelte');
  assert.doesNotMatch(songs, /favorite/);
  const rust = read('src-tauri/src/catalog/collections.rs');
  assert.match(rust, /CREATE|favorites/, 'database layer and stored favourites are left in place');
  const db = read('src-tauri/src/catalog/db.rs');
  assert.match(db, /favorites/, 'schema keeps the favourites table for compatibility');
});
````

## File: tests/v091-layers.test.mjs
````javascript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createDismissal, outsideBox } from '../src/lib/utils/modal.ts';
import { escapeLayer } from '../src/lib/utils/layers.ts';
import { visibleRange, scrollToRow } from '../src/lib/utils/virtual-range.ts';

const box = { left: 420, right: 860, top: 280, bottom: 520 };
function harness({ dismissible = true, popover = false } = {}) {
  const node = { getBoundingClientRect: () => box };
  const inner = {};
  let closed = 0;
  const d = createDismissal(node, {
    canDismiss: () => dismissible,
    popoverOpen: () => popover,
    dismiss: () => closed++,
  });
  const at = (target, x, y) => ({ target, clientX: x, clientY: y, button: 0 });
  return { node, inner, d, at, closed: () => closed };
}

test('modal: backdrop click closes, clicks inside never do', () => {
  const h = harness();
  h.d.pointerdown(h.at(h.inner, 500, 300));
  h.d.click(h.at(h.inner, 500, 300));
  assert.equal(h.closed(), 0, 'content click');
  h.d.pointerdown(h.at(h.node, 500, 300));
  h.d.click(h.at(h.node, 500, 300));
  assert.equal(h.closed(), 0, 'dialog box itself (padding/gap) is inside');
  h.d.pointerdown(h.at(h.node, 40, 40));
  h.d.click(h.at(h.node, 40, 40));
  assert.equal(h.closed(), 1, 'backdrop');
  assert.equal(outsideBox(box, 419, 300), true);
  assert.equal(outsideBox(box, 420, 300), false);
});

test('modal: drag out of an input or a light-dismissed popover does not close', () => {
  const h = harness();
  h.d.pointerdown(h.at(h.inner, 500, 300));
  h.d.click(h.at(h.node, 40, 40));
  assert.equal(h.closed(), 0, 'selection drag that ends on the backdrop');
  const p = harness({ popover: true });
  p.d.pointerdown(p.at(p.node, 40, 40));
  p.d.click(p.at(p.node, 40, 40));
  assert.equal(p.closed(), 0, 'first click only closes the open select/menu');
  const right = harness();
  right.d.pointerdown({ ...right.at(right.node, 40, 40), button: 2 });
  right.d.click(right.at(right.node, 40, 40));
  assert.equal(right.closed(), 0, 'secondary button');
});

test('modal component: dialogs share Modal and it restores focus', () => {
  const modal = readFileSync('src/lib/components/Modal.svelte', 'utf8');
  assert.match(modal, /showModal\(\)/);
  assert.match(modal, /previous\.focus\(/);
  for (const file of [
    'src/lib/components/Dialog.svelte',
    'src/lib/components/TrackDetails.svelte',
  ]) {
    const source = readFileSync(file, 'utf8');
    assert.match(source, /<Modal\b/, file);
    assert.doesNotMatch(source, /<dialog|showModal/, file + ' must not manage its own <dialog>');
  }
});

test('escape closes only the topmost layer: queue above Now Playing above toast', () => {
  assert.equal(escapeLayer({ queue: true, immersive: true, toast: true }), 'queue');
  assert.equal(escapeLayer({ queue: false, immersive: true, toast: true }), 'immersive');
  assert.equal(escapeLayer({ queue: false, immersive: false, toast: true }), 'toast');
  assert.equal(escapeLayer({ queue: false, immersive: false, toast: false }), null);
});

test('queue is an app-level layer drawn above Now Playing', () => {
  const app = readFileSync('src/App.svelte', 'utf8');
  const contentStart = app.search(/<div\b[^>]*\bclass\s*=\s*["']content["']/);
  const contentEnd = app.search(/<\/main\s*>/);
  assert.ok(contentStart >= 0 && contentEnd > contentStart, 'page content is present');
  const content = app.slice(contentStart, contentEnd);
  assert.doesNotMatch(content, /<Queue\b/, 'not nested in the page content');
  const immersive = app.search(/class\s*=\s*["']immersive-layer["']/);
  assert.ok(
    immersive >= 0 && app.search(/<Queue\b/) > immersive,
    'rendered after the immersive layer',
  );
  const tokens = readFileSync('src/lib/styles/tokens.css', 'utf8');
  const z = (name) => Number(tokens.match(new RegExp(`--z-${name}:\\s*(\\d+)`))[1]);
  assert.ok(
    z('queue') > z('queue-scrim') &&
      z('queue-scrim') > z('immersive') &&
      z('immersive') > z('player'),
  );
});

test('song table rows 1/50/200/460/471 stay inside the virtual window', () => {
  const ROW = 56,
    TOTAL = 471,
    listTop = 180,
    viewport = 620,
    head = 36;
  const maxScroll = listTop + TOTAL * ROW + 32 - viewport;
  for (const n of [1, 50, 200, 460, 471]) {
    const index = n - 1;
    const target = Math.min(
      maxScroll,
      Math.max(0, scrollToRow(index, 0, viewport, listTop, ROW, head) ?? 0),
    );
    const range = visibleRange(target, viewport, listTop, ROW, TOTAL);
    assert.ok(
      range.start <= index && index < range.end,
      `row ${n} rendered at scrollTop ${target}`,
    );
    assert.ok(
      listTop + index * ROW + ROW <= target + viewport,
      `row ${n} bottom inside the viewport`,
    );
  }
  assert.deepEqual(
    visibleRange(maxScroll, viewport, listTop, ROW, TOTAL).end,
    TOTAL,
    'last row rendered at the very bottom',
  );
  assert.deepEqual(
    visibleRange(0, viewport, listTop, ROW, TOTAL).start,
    0,
    'back at the top after a fast scroll',
  );
});
````

## File: tests/v091-theme.test.mjs
````javascript
import { sourceStyles } from './helpers/source-styles.mjs';
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import vm from 'node:vm';
import { parseTheme, resolveTheme, THEME_KEY } from '../src/lib/utils/theme.ts';

const read = (f) => readFileSync(f, 'utf8');

/** Run public/theme-init.js against a fake document, as WebView2 does before the bundle loads. */
function boot(stored, systemLight) {
  const root = { dataset: {}, style: {} };
  const store = stored === undefined ? {} : { [THEME_KEY]: stored };
  const context = {
    localStorage: { getItem: (k) => (k in store ? store[k] : null) },
    matchMedia: (q) => ({
      matches: q === '(prefers-color-scheme: light)' ? systemLight : !systemLight,
    }),
    document: { documentElement: root },
  };
  context.window = context;
  vm.runInNewContext(read('public/theme-init.js'), context);
  return root;
}

test('existing users and unknown values default to dark', () => {
  for (const v of [null, undefined, '', 'Dark', 'blue']) assert.equal(parseTheme(v), 'dark');
  assert.equal(parseTheme('light'), 'light');
  assert.equal(parseTheme('system'), 'system');
  assert.equal(
    boot(undefined, true).dataset.theme,
    'dark',
    'no saved choice stays dark even on a light system',
  );
});

test('resolved theme follows the choice, and the system only for 跟随系统', () => {
  assert.equal(resolveTheme('dark', false), 'dark');
  assert.equal(resolveTheme('light', true), 'light');
  assert.equal(resolveTheme('system', true), 'dark');
  assert.equal(resolveTheme('system', false), 'light');
});

test('pre-render script agrees with resolveTheme for every saved choice (no flash)', () => {
  for (const choice of ['dark', 'light', 'system'])
    for (const systemLight of [true, false]) {
      const root = boot(choice, systemLight);
      const expected = resolveTheme(choice, !systemLight);
      assert.equal(
        root.dataset.theme,
        expected,
        `${choice} / system ${systemLight ? 'light' : 'dark'}`,
      );
      assert.equal(root.style.colorScheme, expected);
    }
  const html = read('index.html');
  const init = html.indexOf('<script src="/theme-init.js"></script>');
  assert.ok(
    init > 0 && init < html.indexOf('</head>') && init < html.indexOf('type="module"'),
    'classic script in <head>, before the bundle',
  );
  assert.doesNotMatch(html, /<script>[^<]/, 'no inline script: CSP is default-src self');
});

test('choice persists locally and in desktop preferences, and system changes are followed', () => {
  const prefs = read('src/lib/stores/preferences.ts');
  assert.match(prefs, /localStorage\.setItem\(\s*THEME_KEY\s*,\s*choice\s*\)/);
  assert.match(
    prefs,
    /desktopPreferences\(\s*null\s*,\s*null\s*,\s*choice\s*\)/,
    'Rust applies it to the window on the next start',
  );
  assert.match(prefs, /addEventListener\(\s*'change'\s*,\s*onMedia\s*\)/);
  assert.match(
    prefs,
    /onWindowThemeChanged\(\s*\(\s*t\s*\)\s*=>\s*\{\s*if\s*\(\s*ui\.theme\s*===\s*'system'\s*\)\s*paint\(\s*t\s*\)\s*;?\s*\}\s*\)/,
  );
  assert.doesNotMatch(
    prefs,
    /reduceMotion\s*=.*theme|theme.*reduceMotion\s*=/,
    'theme never touches reduce-motion',
  );
  const services = read('src/lib/services/music.ts');
  assert.match(
    services,
    /getCurrentWindow\(\)\.setTheme\(\s*choice\s*===\s*'system'\s*\?\s*null\s*:\s*choice\s*\)/,
  );
  assert.match(read('src-tauri/capabilities/default.json'), /"core:window:allow-set-theme"/);
  assert.match(read('src-tauri/tauri.conf.json'), /"theme":"Dark","backgroundColor":"#121414"/);
  const desktop = read('src-tauri/src/platform/desktop.rs');
  assert.match(desktop, /w\.set_theme\(window_theme\(&preferences\.theme\)\)\?/);
  const settings = read('src/lib/views/SettingsView.svelte');
  assert.match(settings, /<h3 id="set-appearance">外观<\/h3>/);
  for (const label of ['深色', '浅色', '跟随系统'])
    assert.match(settings, new RegExp(`label\\s*:\\s*'${label}'`));
});

test('both themes share one set of tokens and components', () => {
  const tokens = read('src/lib/styles/tokens.css');
  const [dark, light] = tokens.split(/:root\[data-theme=['"]?light['"]?\]/);
  const names = (block) => new Set([...block.matchAll(/(--[\w-]+):/g)].map((m) => m[1]));
  const darkNames = names(dark);
  for (const name of names(light))
    assert.ok(darkNames.has(name), `${name} overrides an existing token`);
  for (const [name, value] of [
    ['--bg', '#f3f5f1'],
    ['--bg-sidebar', '#e9ede7'],
    ['--surface', '#fafbf8'],
    ['--surface-hover', '#e2e8e0'],
    ['--text', '#29342c'],
    ['--accent', '#52705a'],
  ])
    assert.match(light, new RegExp(`${name}:\\s*${value};`, 'i'));
  const css = sourceStyles({ excludeTokens: true });
  const colours = [...css.matchAll(/#[0-9a-f]{3,8}\b|rgba?\((?!var\()[^)]*\)/gi)].map((m) =>
    m[0].replace(/\s+/g, ''),
  );
  const allowed = /^(?:#000|rgba\(0,0,0,0?\.[45]\))$/;
  assert.deepEqual(
    colours.filter((c) => !allowed.test(c)),
    [],
    'only masks and shadows over artwork stay literal',
  );
});
````

## File: tests/v092-queue-motion.test.mjs
````javascript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { retainQueue, sameTracks } from '../src/lib/utils/queue.ts';

const tokens = readFileSync('src/lib/styles/tokens.css', 'utf8');
const track = (path, title = 't') => ({ path, title, artist: 'a', album: 'b', duration_ms: 1 });

test('drawer timing: ~240ms decelerating in, ~180ms accelerating out, no overshoot', () => {
  const ms = (name) => Number(tokens.match(new RegExp(`${name}:\\s*(\\d+)ms`))[1]);
  assert.ok(ms('--dur-drawer-in') >= 220 && ms('--dur-drawer-in') <= 250);
  assert.ok(ms('--dur-drawer-out') >= 160 && ms('--dur-drawer-out') <= 200);
  for (const name of ['--ease-enter', '--ease-exit']) {
    const [x1, y1, x2, y2] = tokens
      .match(new RegExp(`${name}:\\s*cubic-bezier\\(([^)]+)\\)`))[1]
      .split(',')
      .map(Number);
    for (const v of [x1, y1, x2, y2])
      assert.ok(v >= 0 && v <= 1, `${name} control points stay in [0,1]: no spring or bounce`);
  }
});

test('identical queue polls keep the same array, so rows are not re-diffed', () => {
  const rendered = [track('a', '一'), track('b', '二')];
  const poll = [track('a', '一'), track('b', '二')];
  assert.equal(sameTracks(rendered, poll), true);
  assert.equal(retainQueue(rendered, poll), rendered);
  assert.equal(retainQueue(rendered, rendered), rendered);
});

test('a real queue change replaces the array', () => {
  const rendered = [track('a'), track('b')];
  const renamed = [track('a', '新标题'), track('b')];
  const reordered = [track('b'), track('a')];
  const shorter = [track('a')];
  assert.equal(retainQueue(rendered, renamed), renamed);
  assert.equal(retainQueue(rendered, reordered), reordered);
  assert.equal(retainQueue(rendered, shorter), shorter);
  assert.equal(retainQueue([], rendered), rendered);
});

test('queue scrim stays light enough to read the page underneath', () => {
  const alphas = [...tokens.matchAll(/--scrim-soft:\s*rgba\([^)]*,\s*([.\d]+)\s*\)/g)].map((m) =>
    Number(m[1]),
  );
  assert.ok(alphas.length >= 2);
  for (const alpha of alphas) assert.ok(alpha <= 0.35, String(alpha));
});
````

## File: tests/v092-visual.test.mjs
````javascript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const read = (f) => readFileSync(f, 'utf8');
const tokens = read('src/lib/styles/tokens.css').replace(/\/\*[\s\S]*?\*\//g, '');
const [darkBlock, lightBlock] = tokens.split(/:root\[data-theme=['"]?light['"]?\]/);
const parse = (block) =>
  Object.fromEntries([...block.matchAll(/(--[\w-]+):([^;]+);/g)].map((m) => [m[1], m[2].trim()]));
const dark = parse(darkBlock);
const light = { ...dark, ...parse(lightBlock) };

function rgba(value) {
  const short = value.match(/^#([0-9a-f])([0-9a-f])([0-9a-f])$/i);
  if (short)
    value =
      '#' +
      short
        .slice(1)
        .map((c) => c + c)
        .join('');
  const hex = value.match(/^#([0-9a-f]{6})$/i);
  if (hex) return [0, 2, 4].map((i) => parseInt(hex[1].slice(i, i + 2), 16)).concat(1);
  const fn = value.match(/^rgba?\(([^)]+)\)$/);
  if (fn) {
    const p = fn[1].split(',').map(Number);
    return [p[0], p[1], p[2], p[3] ?? 1];
  }
  throw new Error(`not a literal colour: ${value}`);
}
/** Composite a (possibly translucent) colour over an opaque base. */
function over(top, base) {
  const [r, g, b, a] = rgba(top);
  const [R, G, B] = rgba(base);
  return [r * a + R * (1 - a), g * a + G * (1 - a), b * a + B * (1 - a), 1];
}
function luminance([r, g, b]) {
  const c = (v) => {
    v /= 255;
    return v <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4;
  };
  return 0.2126 * c(r) + 0.7152 * c(g) + 0.0722 * c(b);
}
function contrast(a, b) {
  const [x, y] = [luminance(a), luminance(b)].sort((p, q) => q - p);
  return (x + 0.05) / (y + 0.05);
}

for (const [name, t] of [
  ['dark', dark],
  ['light', light],
]) {
  test(`${name}: text meets 4.5:1 on every surface it sits on`, () => {
    const surfaces = {
      bg: t['--bg'],
      sidebar: t['--bg-sidebar'],
      player: t['--bg-player'],
      card: t['--surface'],
      overlay: t['--surface-overlay'],
      drawer: t['--bg-drawer'],
    };
    surfaces['hover on bg'] = over(t['--hover'], t['--bg']).slice(0, 3).map(Math.round);
    surfaces['selected on sidebar'] = over(t['--selected'], t['--bg-sidebar'])
      .slice(0, 3)
      .map(Math.round);
    for (const [where, surface] of Object.entries(surfaces)) {
      const base = Array.isArray(surface) ? [...surface, 1] : rgba(surface);
      for (const text of ['--text', '--text-2', '--text-3']) {
        const ratio = contrast(rgba(t[text]), base);
        assert.ok(ratio >= 4.5, `${text} on ${where}: ${ratio.toFixed(2)}`);
      }
    }
  });
  test(`${name}: accent and danger stay legible; primary button text contrasts with the accent`, () => {
    assert.ok(
      contrast(rgba(t['--accent']), rgba(t['--bg'])) >= 4.5,
      'accent text (current song) on bg',
    );
    assert.ok(
      contrast(rgba(t['--accent-ink']), rgba(t['--accent'])) >= 4.5,
      'primary button label',
    );
    assert.ok(
      contrast(rgba(t['--danger']), rgba(t['--surface-overlay'])) >= 4.5,
      'danger menu items',
    );
    assert.ok(
      contrast(rgba(t['--on-danger']), rgba(t['--danger-bg'])) >= 4.5,
      'danger button label',
    );
    assert.ok(
      contrast(rgba(t['--focus']), rgba(t['--bg'])) >= 3,
      'focus ring is a visible non-text indicator',
    );
  });
}

test('light theme uses the soft warm palette with no pure-white surfaces', () => {
  for (const token of [
    '--bg',
    '--bg-sidebar',
    '--bg-player',
    '--surface',
    '--surface-overlay',
    '--bg-drawer',
    '--toast-bg',
  ])
    assert.notEqual(light[token].toLowerCase(), '#ffffff', `${token} is not pure white`);
  assert.equal(light['--bg-player'], light['--bg-sidebar'], 'player bar blends with the sidebar');
  const [r, g, b] = rgba(light['--accent']);
  const max = Math.max(r, g, b),
    min = Math.min(r, g, b);
  assert.ok((max - min) / max < 0.3, 'brand green is low-saturation');
});

test('dark theme keeps its core palette', () => {
  for (const [token, value] of [
    ['--bg', '#121414'],
    ['--bg-sidebar', '#090b0c'],
    ['--surface', '#1c2220'],
    ['--accent', '#b9d69b'],
    ['--text', '#e8ece7'],
  ])
    assert.equal(dark[token], value);
  assert.equal(dark['--hover'], dark['--surface'], 'hover fill unchanged from V0.9.1');
  assert.equal(dark['--selected'], dark['--surface-hover'], 'selected fill unchanged from V0.9.1');
});
````

## File: tests/v092-volume.test.mjs
````javascript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { toPercent, toGain, muteTarget, latest, DEFAULT_PERCENT } from '../src/lib/utils/volume.ts';

test('percent is a presentation of the same 0.0–1.0 gain Rust uses (no new curve)', () => {
  for (const p of [0, 1, 35, 50, 99, 100]) assert.equal(toPercent(toGain(p)), p, 'round-trips');
  assert.equal(toPercent(0.35), 35);
  assert.equal(toPercent(0.354), 35);
  assert.equal(toPercent(0.355), 36);
  assert.equal(toPercent(1.4), 100);
  assert.equal(toPercent(-1), 0);
  assert.equal(toPercent(Number.NaN), 0);
  assert.equal(toGain(150), 1);
  assert.equal(toGain(-5), 0);
});

test('mute keeps the previous level and unmute restores it', () => {
  assert.equal(muteTarget(35, 35), 0, 'audible -> mute');
  assert.equal(muteTarget(0, 35), 35, 'muted -> previous level');
  assert.equal(
    muteTarget(0, 0),
    DEFAULT_PERCENT,
    'nothing remembered -> sensible default, never stays silent',
  );
});

/** Fake backend: each call resolves only when the test says so, like a slow IPC round-trip. */
function backend() {
  const calls = [];
  const waiting = [];
  return {
    calls,
    send: (v) => {
      calls.push(v);
      return new Promise((r) => waiting.push(() => r({ volume: v / 100 })));
    },
    async flush() {
      while (waiting.length) {
        waiting.shift()();
        await new Promise((r) => setImmediate(r));
      }
    },
  };
}

test('slider drags coalesce: one call in flight, the final value always wins', async () => {
  const b = backend();
  const results = [];
  let settled = 0;
  const sync = latest(b.send, {
    result: (r) => results.push(r.volume),
    error: assert.fail,
    settled: () => settled++,
  });
  for (const v of [10, 20, 30, 40, 55]) sync.push(v);
  assert.deepEqual(b.calls, [10], 'first value goes out immediately');
  assert.equal(sync.busy, true);
  await b.flush();
  assert.deepEqual(b.calls, [10, 55], 'intermediate values dropped, last value sent');
  assert.deepEqual(results, [0.1, 0.55], 'every reply is applied, ending at the real final volume');
  assert.equal(settled, 1);
  assert.equal(sync.busy, false);
});

test('repeated identical values in a burst are sent once; a later burst can resend', async () => {
  const b = backend();
  const sync = latest(b.send, { result: () => {}, error: assert.fail, settled: () => {} });
  sync.push(40);
  sync.push(40);
  await b.flush();
  sync.push(40);
  await b.flush();
  assert.deepEqual(
    b.calls,
    [40, 40],
    'oninput+onchange with the same value collapse; a new interaction is not swallowed',
  );
});

test('a failed call reports the error and still settles so the UI falls back to the real state', async () => {
  let errors = 0,
    settled = 0;
  const sync = latest(() => Promise.reject(new Error('device lost')), {
    result: assert.fail,
    error: () => errors++,
    settled: () => settled++,
  });
  sync.push(30);
  await new Promise((r) => setImmediate(r));
  assert.equal(errors, 1);
  assert.equal(settled, 1);
  assert.equal(sync.busy, false);
});
````

## File: tests/v0953-cover-loader.test.mjs
````javascript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { createCoverLoader, coverErrorCode } from '../src/lib/utils/cover-loader.ts';

// Manual clock and timers so retries and timeouts run deterministically.
function harness(options = {}) {
  let t = 0;
  const timers = [];
  const schedule = (fn, ms) => {
    timers.push({ at: t + ms, fn });
  };
  const advance = async (ms) => {
    t += ms;
    for (const timer of timers.filter((x) => x.at <= t)) {
      timers.splice(timers.indexOf(timer), 1);
      timer.fn();
    }
    await flush();
  };
  const calls = [];
  const pending = new Map();
  const load = (key) => () => {
    calls.push(key);
    return new Promise((resolve, reject) => pending.set(key, { resolve, reject }));
  };
  const loader = createCoverLoader({ now: () => t, schedule, ...options });
  return { loader, load, calls, pending, advance };
}
const flush = () => new Promise((r) => setTimeout(r, 0));

test('visible and current rows start before preloads; concurrency is bounded', async () => {
  const { loader, load, calls } = harness({ concurrency: 2 });
  const seen = [];
  loader.request('pre-1', 2, load('pre-1'), (o) => seen.push(o));
  loader.request('pre-2', 2, load('pre-2'), () => {});
  loader.request('vis', 1, load('vis'), () => {});
  loader.request('cur', 0, load('cur'), () => {});
  // The first two preloads started immediately; the rest wait in priority order.
  assert.deepEqual(calls, ['pre-1', 'pre-2']);
  assert.deepEqual(loader.pending(), { queued: 2, running: 2, waiting: 4 });
  assert.equal(loader.stats.maxRunning, 2);
});

test('waiting requests are ordered current, visible, preload', async () => {
  const { loader, load, calls, pending } = harness({ concurrency: 1 });
  loader.request('busy', 1, load('busy'), () => {});
  loader.request('pre', 2, load('pre'), () => {});
  loader.request('vis', 1, load('vis'), () => {});
  loader.request('cur', 0, load('cur'), () => {});
  pending.get('busy').resolve('data:a');
  await flush();
  pending.get('cur').resolve('data:b');
  await flush();
  pending.get('vis').resolve(null);
  await flush();
  assert.deepEqual(calls, ['busy', 'cur', 'vis', 'pre']);
});

test('a row that scrolls away cancels its queued request; identical requests share one load', async () => {
  const { loader, load, calls, pending } = harness({ concurrency: 1 });
  loader.request('busy', 1, load('busy'), () => {});
  const away = loader.request('gone', 1, load('gone'), () => {});
  const a = [],
    b = [];
  loader.request('same', 1, load('same'), (o) => a.push(o));
  loader.request('same', 1, load('same'), (o) => b.push(o));
  away.cancel();
  assert.equal(loader.stats.coalesced, 1);
  pending.get('busy').resolve('data:x');
  await flush();
  pending.get('same').resolve('data:y');
  await flush();
  assert.deepEqual(calls, ['busy', 'same']);
  assert.equal(a[0].src, 'data:y');
  assert.equal(b[0].src, 'data:y');
});

test('fast scrolling through hundreds of rows leaves no backlog in front of the visible rows', async () => {
  const { loader, load, calls, pending } = harness({ concurrency: 4 });
  for (let i = 0; i < 300; i++) {
    const ticket = loader.request('row' + i, 1, load('row' + i), () => {});
    if (i < 290) ticket.cancel();
  }
  // Four requests were already running; the other passed-over rows were cancelled before they started.
  assert.equal(calls.length, 4);
  for (const key of [...calls]) pending.get(key).resolve(null);
  await flush();
  assert.deepEqual(calls.slice(4), ['row290', 'row291', 'row292', 'row293']);
  assert.ok(loader.pending().queued <= 6);
});

test('null is cached as "no cover"; transient errors retry a bounded number of times and are not cached as null', async () => {
  const { loader, load, calls, pending, advance } = harness({
    retryDelays: [100, 200],
    failedTtlMs: 1000,
  });
  const none = [];
  loader.request('plain', 1, load('plain'), (o) => none.push(o));
  pending.get('plain').resolve(null);
  await flush();
  assert.deepEqual(none, [{ status: 'none', src: null }]);
  assert.equal(loader.peek('plain').status, 'none');
  const out = [];
  loader.request('busy', 1, load('busy'), (o) => out.push(o));
  pending.get('busy').reject('cover-unreadable');
  await flush();
  assert.equal(out.length, 0);
  await advance(100);
  pending.get('busy').reject('cover-unreadable');
  await flush();
  await advance(200);
  pending.get('busy').reject(new Error('IPC broke'));
  await flush();
  assert.equal(calls.filter((c) => c === 'busy').length, 3);
  assert.deepEqual(out, [{ status: 'failed', src: null, code: 'ipc-error' }]);
  assert.equal(loader.stats.retries, 2);
  // Failures are remembered briefly, then requested again instead of staying "no cover" forever.
  assert.equal(loader.peek('busy').status, 'failed');
  await advance(1001);
  assert.equal(loader.peek('busy'), undefined);
});

test('a terminal error is not retried; a hung request times out instead of loading forever', async () => {
  const { loader, load, calls, pending, advance } = harness({
    terminal: (c) => c === 'cover-not-in-library',
    retryDelays: [],
    timeoutMs: 500,
  });
  const a = [],
    b = [];
  loader.request('stray', 1, load('stray'), (o) => a.push(o));
  pending.get('stray').reject('cover-not-in-library');
  await flush();
  assert.deepEqual(a, [{ status: 'failed', src: null, code: 'cover-not-in-library' }]);
  loader.request('hang', 1, load('hang'), (o) => b.push(o));
  await advance(500);
  assert.deepEqual(b, [{ status: 'failed', src: null, code: 'cover-timeout' }]);
  assert.equal(loader.stats.timeouts, 1);
  assert.equal(calls.length, 2);
});

test('finished covers are available synchronously; clear() forgets them', async () => {
  const { loader, load, pending } = harness();
  loader.request('k', 1, load('k'), () => {});
  pending.get('k').resolve('data:z');
  await flush();
  const hit = [];
  loader.request('k', 1, load('k'), (o) => hit.push(o));
  assert.equal(hit[0].src, 'data:z');
  assert.equal(loader.stats.peekHits, 1);
  loader.clear();
  assert.equal(loader.peek('k'), undefined);
});

test('the queued set stays bounded by dropping the newest preloads', async () => {
  const { loader, load } = harness({ concurrency: 1, maxQueued: 3 });
  const dropped = [];
  loader.request('run', 1, load('run'), () => {});
  for (let i = 0; i < 6; i++)
    loader.request('p' + i, 2, load('p' + i), (o) => dropped.push([i, o.code]));
  loader.request('v', 1, load('v'), () => {});
  assert.ok(loader.pending().queued <= 3);
  assert.ok(dropped.every(([, code]) => code === 'dropped'));
  assert.ok(loader.stats.dropped >= 3);
});

test('error codes never carry paths', () => {
  assert.equal(coverErrorCode('cover-file-missing'), 'cover-file-missing');
  assert.equal(coverErrorCode('C:\\Music\\a.flac: denied'), 'ipc-error');
  assert.equal(coverErrorCode(new Error('boom')), 'ipc-error');
});
````

## File: tests/v0955-cover-generation.test.mjs
````javascript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { createCoverLoader } from '../src/lib/utils/cover-loader.ts';

// Every load() call gets its own manually settled promise; timers run only when advanced.
function harness(options = {}) {
  let t = 0;
  const timers = [];
  const schedule = (fn, ms) => {
    timers.push({ at: t + ms, fn });
  };
  const advance = async (ms) => {
    t += ms;
    for (const timer of timers.filter((x) => x.at <= t)) {
      timers.splice(timers.indexOf(timer), 1);
      timer.fn();
    }
    await flush();
  };
  const calls = [];
  const load = (key) => () =>
    new Promise((resolve, reject) => calls.push({ key, resolve, reject }));
  const loader = createCoverLoader({
    now: () => t,
    schedule,
    retryDelays: [100],
    timeoutMs: 1e9,
    ...options,
  });
  const open = (key) => calls.filter((c) => c.key === key);
  return { loader, load, calls, open, advance };
}
const flush = () => new Promise((r) => setTimeout(r, 0));
const sink = () => {
  const got = [];
  return { got, done: (o) => got.push(o) };
};

test('an old request that succeeds after clear() reaches only its own rows and is not cached', async () => {
  const { loader, load, open } = harness();
  const old = sink(),
    fresh = sink();
  loader.request('a', 1, load('a'), old.done);
  loader.clear();
  loader.request('a', 1, load('a'), fresh.done);
  // The new generation does not join the old in-flight load: a second load for the same key starts.
  assert.equal(open('a').length, 2);
  open('a')[0].resolve('data:old');
  await flush();
  assert.deepEqual(old.got, [{ status: 'ready', src: 'data:old' }]);
  assert.deepEqual(fresh.got, []);
  assert.equal(loader.peek('a'), undefined);
  open('a')[1].resolve('data:new');
  await flush();
  assert.deepEqual(fresh.got, [{ status: 'ready', src: 'data:new' }]);
  assert.equal(loader.peek('a').src, 'data:new');
  assert.equal(loader.stats.stale, 1);
  assert.deepEqual(loader.pending(), { queued: 0, running: 0, waiting: 0 });
});

test('an old request finishing after the new one cannot overwrite the new result', async () => {
  const { loader, load, open } = harness();
  loader.request('a', 1, load('a'), () => {});
  loader.clear();
  const fresh = sink();
  loader.request('a', 1, load('a'), fresh.done);
  open('a')[1].resolve(null);
  await flush();
  open('a')[0].resolve('data:old');
  await flush();
  assert.deepEqual(fresh.got, [{ status: 'none', src: null }]);
  assert.deepEqual(loader.peek('a'), { status: 'none', src: null, code: undefined });
});

test('an old request that fails after clear() hands its rows to a current request instead of failing them', async () => {
  const { loader, load, open } = harness();
  const old = sink();
  loader.request('a', 1, load('a'), old.done);
  loader.clear();
  open('a')[0].reject('cover-unreadable');
  await flush();
  assert.deepEqual(old.got, []);
  assert.equal(open('a').length, 2, 'a current-generation load replaces the failed old one');
  assert.equal(loader.peek('a'), undefined, 'the old failure is not cached');
  open('a')[1].resolve('data:new');
  await flush();
  assert.deepEqual(old.got, [{ status: 'ready', src: 'data:new' }]);
  assert.equal(loader.peek('a').src, 'data:new');
  assert.equal(loader.stats.staleHandedOver, 1);
  assert.equal(loader.stats.failed, 0);
  assert.deepEqual(loader.pending(), { queued: 0, running: 0, waiting: 0 });
});

test('a hand-over joins a current request for the same key rather than starting another', async () => {
  const { loader, load, open } = harness();
  const old = sink(),
    fresh = sink();
  loader.request('a', 1, load('a'), old.done);
  loader.clear();
  loader.request('a', 1, load('a'), fresh.done);
  open('a')[0].reject(new Error('IPC broke'));
  await flush();
  assert.equal(open('a').length, 2);
  open('a')[1].resolve('data:new');
  await flush();
  assert.equal(old.got[0].src, 'data:new');
  assert.equal(fresh.got[0].src, 'data:new');
});

test('old in-flight loads count toward the concurrency limit, and new rows start as soon as they finish', async () => {
  const { loader, load, calls, open } = harness({ concurrency: 2 });
  loader.request('a', 1, load('a'), () => {});
  loader.request('b', 1, load('b'), () => {});
  loader.clear();
  const c = sink(),
    d = sink();
  loader.request('c', 1, load('c'), c.done);
  loader.request('d', 1, load('d'), d.done);
  assert.equal(calls.length, 2, 'no new load starts while two old ones still run');
  assert.deepEqual(loader.pending(), { queued: 2, running: 2, waiting: 4 });
  open('a')[0].resolve('data:a');
  await flush();
  assert.deepEqual(
    calls.map((x) => x.key),
    ['a', 'b', 'c'],
  );
  open('b')[0].reject('cover-unreadable');
  await flush();
  assert.deepEqual(
    calls.map((x) => x.key),
    ['a', 'b', 'c', 'd'],
  );
  open('c')[0].resolve('data:c');
  open('d')[0].resolve(null);
  await flush();
  assert.equal(c.got[0].src, 'data:c');
  assert.equal(d.got[0].status, 'none');
  assert.equal(loader.stats.maxRunning, 2);
  // The old row of b was handed over and still waits for a current load, which starts now.
  assert.equal(open('b').length, 2);
  open('b')[1].resolve('data:b');
  await flush();
  assert.deepEqual(
    [...['a', 'b', 'c', 'd']].map((k) => loader.peek(k)?.src ?? null),
    [null, 'data:b', 'data:c', null],
  );
  assert.equal(loader.peek('d').status, 'none');
  assert.deepEqual(loader.pending(), { queued: 0, running: 0, waiting: 0 });
});

test('queued and retrying requests survive clear() and still notify their rows', async () => {
  const { loader, load, open, advance } = harness({ concurrency: 1 });
  const a = sink(),
    b = sink();
  loader.request('a', 1, load('a'), a.done);
  loader.request('b', 1, load('b'), b.done);
  open('a')[0].reject('cover-unreadable');
  await flush();
  // a waits for its retry, b is running; clear() now.
  loader.clear();
  open('b')[0].resolve('data:b');
  await flush();
  // b started before clear(): its row gets the picture, but the result is not cached for the new generation.
  assert.deepEqual(b.got, [{ status: 'ready', src: 'data:b' }]);
  assert.equal(loader.peek('b'), undefined);
  await advance(100);
  assert.equal(open('a').length, 2, 'the retry of a still runs after clear()');
  open('a')[1].resolve('data:a');
  await flush();
  assert.equal(a.got[0].src, 'data:a');
  assert.equal(loader.peek('a').src, 'data:a');
});

test('after clear() the current track and visible rows still go before preloads', async () => {
  const { loader, load, calls, open } = harness({ concurrency: 1 });
  loader.request('busy', 1, load('busy'), () => {});
  loader.clear();
  loader.request('pre', 2, load('pre'), () => {});
  loader.request('vis', 1, load('vis'), () => {});
  loader.request('cur', 0, load('cur'), () => {});
  open('busy')[0].resolve('data:x');
  await flush();
  open('cur')[0].resolve('data:c');
  await flush();
  open('vis')[0].resolve('data:v');
  await flush();
  assert.deepEqual(
    calls.map((x) => x.key),
    ['busy', 'cur', 'vis', 'pre'],
  );
});

test('repeated clear() during long loads keeps the number of loads bounded', async () => {
  const { loader, load, calls } = harness({ concurrency: 4 });
  for (let round = 0; round < 10; round++) {
    for (let i = 0; i < 20; i++) loader.request('k' + i, 1, load('k' + i), () => {});
    loader.clear();
  }
  assert.equal(calls.length, 4, 'nothing finished, so only the first four loads ever started');
  assert.equal(loader.pending().running, 4);
  assert.ok(loader.pending().queued <= 20);
  for (let n = 0; n < 40 && calls.some((c) => !c.settled); n++) {
    for (const c of calls.filter((c) => !c.settled)) {
      c.settled = true;
      c.resolve(null);
    }
    await flush();
  }
  assert.ok(calls.length <= 4 + 20, `${calls.length} loads for 20 keys`);
  assert.deepEqual(loader.pending(), { queued: 0, running: 0, waiting: 0 });
});
````

## File: tests/worktree-fingerprint.test.mjs
````javascript
import { spawnSync } from 'node:child_process';
import { mkdtempSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';
import assert from 'node:assert/strict';

const script = join(
  dirname(dirname(fileURLToPath(import.meta.url))),
  'scripts',
  'worktree-fingerprint.ps1',
);

function git(repo, args) {
  const result = spawnSync('git', args, { cwd: repo, encoding: 'utf8' });
  assert.equal(result.status, 0, result.stderr || result.stdout);
}

function fingerprint(repo, extra = []) {
  const result = spawnSync(
    'powershell.exe',
    ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', script, '-Repo', repo, ...extra],
    { encoding: 'utf8' },
  );
  assert.equal(result.status, 0, result.stdout + result.stderr);
  return result.stdout.trim();
}

test('worktree fingerprint covers staged, unstaged, and included untracked files', () => {
  const repo = mkdtempSync(join(tmpdir(), 'mm-fp-'));
  try {
    git(repo, ['init']);
    git(repo, ['config', 'user.email', 'fingerprint@example.com']);
    git(repo, ['config', 'user.name', 'fingerprint']);
    git(repo, ['config', 'core.autocrlf', 'false']);
    writeFileSync(join(repo, '.gitignore'), '*.log\n');
    writeFileSync(join(repo, 'file.txt'), 'one\n');
    git(repo, ['add', '.gitignore', 'file.txt']);
    git(repo, ['commit', '-m', 'init']);
    const clean = fingerprint(repo);
    assert.equal(fingerprint(repo), clean);

    writeFileSync(join(repo, 'file.txt'), 'two\n');
    const unstaged = fingerprint(repo);
    assert.notEqual(unstaged, clean);
    git(repo, ['add', 'file.txt']);
    const staged = fingerprint(repo);
    assert.notEqual(staged, unstaged);

    writeFileSync(join(repo, 'noise.log'), 'ignored\n');
    assert.equal(fingerprint(repo), staged);
    writeFileSync(join(repo, 'keep.txt'), 'keep\n');
    const withKeep = fingerprint(repo);
    assert.notEqual(withKeep, staged);
    assert.equal(fingerprint(repo, ['-Exclude', 'keep.txt']), staged);
    const excluded = fingerprint(repo, ['-Exclude', 'file.txt,keep.txt']);
    writeFileSync(join(repo, 'file.txt'), 'three\n');
    writeFileSync(join(repo, 'keep.txt'), 'changed\n');
    assert.equal(fingerprint(repo, ['-Exclude', 'file.txt,keep.txt']), excluded);
    git(repo, ['add', 'file.txt']);
    assert.equal(fingerprint(repo, ['-Exclude', 'file.txt,keep.txt']), excluded);
    assert.notEqual(fingerprint(repo), withKeep);
  } finally {
    rmSync(repo, { recursive: true, force: true });
  }
});
````

## File: .gitattributes
````
* text=auto eol=lf
*.ps1 text eol=crlf
*.cmd text eol=crlf
````

## File: .gitignore
````
node_modules/
dist/
dist-portable/
src-tauri/target/
.tools/
.checkpoints/
.pnpm-store/
tests/perf/out/
test-results/
playwright-report/
blob-report/
build-activity.log
craft.json
feed.json
usage.json
inbox/
investigation.cany/
capture/
````

## File: .node-version
````
24.14.0
````

## File: .prettierignore
````
node_modules/
dist*/
src-tauri/
docs/
pnpm-lock.yaml
````

## File: .prettierrc
````
{
  "plugins": ["prettier-plugin-svelte"],
  "printWidth": 100,
  "tabWidth": 2,
  "useTabs": false,
  "singleQuote": true,
  "endOfLine": "auto",
  "trailingComma": "all",
  "overrides": [
    {
      "files": "*.svelte",
      "options": {
        "parser": "svelte"
      }
    }
  ]
}
````

## File: .repomixignore
````
docs/history/**
docs/tasks/done/**
src-tauri/tests/fixtures/**
src-tauri/gen/**
tests/ui/snapshots/**
tests/perf/out/**
pnpm-lock.yaml
src-tauri/Cargo.lock
````

## File: AGENTS.md
````markdown
# Music Mine — AI 开发约定

适用于本仓库。这是工程约束，不是产品需求。开工前看 `git status` 和 HEAD。事实优先级：当前代码 > `docs/current-state.md` > `docs/architecture.md`。`docs/tasks/done/` 与 `docs/history/` 仅在任务点名时打开，并标明历史基线。`docs/RFC-v2.0.md` 只描述目标。

技术栈：Rust、Tauri v2、Svelte 5 Runes、TypeScript、Vite、Tailwind CSS v4。Windows 10/11 本地播放器。产品版本以 `package.json` 和 `src-tauri/tauri.conf.json` 的 `0.1.0` 为准。

## 安全边界

- 只改这个仓库。保留旧播放器项目。只有单独授权的迁移任务才能从旧项目提取音频代码。
- 禁止打开、迁移实验或改写用户正在使用的曲库数据库、音乐、歌词和设置。数据在 Tauri `app_data_dir()`：`library-tauri-v1.sqlite`、`playback-v1.json`、`desktop-v1.json`。
- Windows 上进程环境变量 `APPDATA` 不会改变这个目录。需要隔离时按 `docs/isolation.md` 用 CLI 合并配置，不修改 `src-tauri/tauri.conf.json`。没有与 exe 哈希绑定的构建来源记录，就不把那个 exe 当作隔离构建。
- 播放状态以 Rust 快照为准。前端通过 `src/lib/stores/player.svelte.ts` 的 `accept()` 接收带 `revision` 的快照。禁止用 UI 定时器推进进度，或把 Mock 成功说成 Rust 播放成功。
- 前端 `invoke` 只放在 `src/lib/services/music.ts`。
- 产品范围不接入 AI 服务，不重写解码器。这里的范围边界沿用 RFC 的 V1.0，不是 `package.json` 里的 `0.1.0`。格式支持以 `src-tauri/Cargo.toml` 里启用的 Symphonia feature 和已有 fixture 为准。
- 音频回调里不做磁盘、数据库、阻塞锁、无界分配或高频日志。UI 任务不改音频引擎和数据库。细则见 `docs/rust-guidelines.md`。
- 未经当前任务明确授权，不 `git commit`、不 `git push`、不 `git reset --hard`、不 `git clean -fd`。先给出 diff 和检查结果。

## 每个任务

1. 记录分支、HEAD 和未提交改动，保留用户工作。
2. 只读与任务相关的代码和文档。跨音频、数据库、IPC 或大范围 UI 时，先给短计划。
3. 做最小、可回退的修改。不顺手格式化或重构无关文件。
4. 跑与改动相关的检查。三条证据分开写：自动测试、浏览器验证、Windows Tauri 实机。没跑的写 `NOT RUN`。
5. 桌面启动验收使用按 `docs/isolation.md` 构建出的 exe。普通 `pnpm tauri build --debug --no-bundle` 的产物使用系统数据目录，不能用来做会写入曲库或设置的验收。Craft Anything 里的持久开发服务器、`pnpm dev` 和 Playwright harness 都不能代替构建出的 exe。
6. 有架构、契约或决策变化时，只更新对应文档。

按任务打开：Rust / 数据库 `docs/rust-guidelines.md`；IPC `docs/ipc-contract.md`；界面 `docs/frontend-guidelines.md`；验收 `docs/testing.md`；现状 `docs/current-state.md`；取舍 `docs/decisions.md`。

## 组件库

shadcn-svelte 和 Bits UI 没有安装。没有单独的迁移任务时，继续用仓库里的手写组件。禁止在功能任务里运行会覆盖 `src/app.css` 的初始化命令。

## 改这份约定

同一类失误重复出现时，补一条可执行规则或一个测试。改本文件前说明旧规则、新规则和原因。
````

## File: CLAUDE.md
````markdown
# Claude Code — Music Mine

@AGENTS.md

规则只维护在根目录 `AGENTS.md`。按任务再读 `docs/rust-guidelines.md`、`docs/ipc-contract.md`、`docs/frontend-guidelines.md`、`docs/testing.md`。不要把那些内容复制进本文件。
````

## File: index.html
````html
<!doctype html><html lang="zh-CN" data-theme="dark"><head><meta charset="UTF-8"/><meta name="viewport" content="width=device-width,initial-scale=1"/><meta name="theme-color" content="#121414"/><title>Music Mine</title><script src="/theme-init.js"></script></head><body><div id="app"></div><script type="module" src="/src/main.ts"></script></body></html>
````

## File: npm.cmd
````batch
@echo off
setlocal
rem Node is provided by fnm. NODE_HOME wins, then node on PATH, then fnm's default alias.
if defined NODE_HOME if exist "%NODE_HOME%\npm.cmd" (
  set "MUSIC_MINE_NODE=%NODE_HOME%"
  goto run
)
for /f "delims=" %%P in ('where node 2^>nul') do (
  echo %%P | findstr /I /C:"WindowsApps" >nul
  if errorlevel 1 (
    set "MUSIC_MINE_NODE=%%~dpP"
    goto run
  )
)
if defined FNM_DIR if exist "%FNM_DIR%\aliases\default\npm.cmd" (
  set "MUSIC_MINE_NODE=%FNM_DIR%\aliases\default"
  goto run
)
echo Node.js could not be found. Install it with fnm or set NODE_HOME to its folder. 1>&2
exit /b 1
:run
set "PATH=%MUSIC_MINE_NODE%;%PATH%"
call "%MUSIC_MINE_NODE%\npm.cmd" %*
exit /b %errorlevel%
````

## File: package.json
````json
{
  "name": "music-mine",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "check": "svelte-check --tsconfig ./tsconfig.json",
    "format": "prettier --write src/ public/ tests/",
    "format:check": "prettier --check src/ public/ tests/",
    "check:all": "pwsh -File scripts/check.ps1",
    "tauri": "tauri",
    "test": "node --experimental-strip-types --test tests/*.test.mjs",
    "test:ui": "playwright test",
    "test:ui:update": "playwright test --update-snapshots",
    "portable:debug": "tauri build --debug --no-bundle --config src-tauri/tauri.portable.conf.json",
    "portable": "tauri build --no-bundle --config src-tauri/tauri.portable.conf.json"
  },
  "dependencies": {
    "@floating-ui/dom": "^1.8.0",
    "@tauri-apps/api": "^2.0.0",
    "bits-ui": "^2.21.0",
    "lucide-svelte": "^0.468.0"
  },
  "devDependencies": {
    "@playwright/test": "^1.65.1",
    "@sveltejs/vite-plugin-svelte": "^6.0.0",
    "@tailwindcss/vite": "^4.0.0",
    "@tauri-apps/cli": "^2.0.0",
    "prettier": "^3.10.1",
    "prettier-plugin-svelte": "^4.1.1",
    "svelte": "^5.0.0",
    "svelte-check": "^4.0.0",
    "tailwindcss": "^4.0.0",
    "typescript": "~5.9.3",
    "vite": "^7.0.0"
  },
  "packageManager": "pnpm@12.21.0"
}
````

## File: playwright.config.ts
````typescript
import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: 'tests/ui',
  timeout: 30_000,
  fullyParallel: false,
  workers: 1,
  retries: 0,
  snapshotPathTemplate: '{testDir}/snapshots/{arg}{ext}',
  expect: {
    timeout: 10_000,
    toHaveScreenshot: { animations: 'disabled', caret: 'hide', maxDiffPixels: 0, threshold: 0 },
  },
  use: {
    baseURL: 'http://127.0.0.1:4173',
    viewport: { width: 1280, height: 800 },
    locale: 'zh-CN',
    timezoneId: 'Asia/Shanghai',
    deviceScaleFactor: 1,
    colorScheme: 'dark',
  },
  webServer: {
    command: 'pnpm exec vite --host 127.0.0.1 --port 4173 --strictPort',
    url: 'http://127.0.0.1:4173/tests/harness/index.html',
    reuseExistingServer: false,
    timeout: 60_000,
  },
});
````

## File: pnpm-workspace.yaml
````yaml
allowBuilds:
  esbuild: true
````

## File: README.md
````markdown
# Music Mine

Windows 本地音乐播放器。技术栈是 Rust、Tauri v2、Svelte 5、TypeScript、Vite 和 Tailwind CSS v4。

当前程序的源码包含本地扫描、歌单、播放、歌词、队列，以及深色、浅色、跟随系统。产品版本在 `package.json` 和 `src-tauri/tauri.conf.json` 里是 `0.1.0`。`docs/tasks/done/` 里的 V0.9.x 是历史任务编号，不是安装包版本。

播放进度只显示 Rust 快照里的位置，界面不用计时器推进播放。哪一层检查已经跑过，见 [docs/current-state.md](docs/current-state.md)。进度只看那一页指向的当前任务。源码里有的能力，不等于自动测试、浏览器或 Windows 实机都已通过。

## 检查

安装依赖后可以运行：

```powershell
pnpm install --frozen-lockfile
pnpm check
pnpm test
pnpm test:ui
pnpm build
pwsh -File scripts/check.ps1
```

`scripts/check.ps1` 在仓库根目录运行 `pnpm check` 和 `pnpm test`，再进入 `src-tauri` 运行 `cargo fmt`、`cargo clippy` 和 `cargo test`。失败时恢复原来的工作目录，并返回失败命令的退出码。它不跑浏览器，也不启动 exe。

统一检查的自动部分由 `.github/workflows/check.yml` 跑。`windows-latest` 只是运行镜像。CI 和本仓库使用 Node `24.14.0`、Rust `1.99.0`（含 rustfmt 和 clippy）和 pnpm `12.21.0`。仓库不使用 `rust-toolchain.toml`。Node 版本写在 `.node-version`，pnpm 版本写在 `package.json` 的 `packageManager`。

本地 Node 用 fnm 安装，当前 default 别名是 `24.14.0`。pnpm 用 Scoop 安装。仓库里的 `npm.cmd` 先看 `NODE_HOME`，再看 PATH 上的 `node`，最后用 `%FNM_DIR%\aliases\default`。pnpm 12 默认不运行依赖的安装脚本；`esbuild` 写在 `pnpm-workspace.yaml` 的 `allowBuilds` 里。本地还需要 Windows 10/11、WebView2 和 Visual Studio 生成工具。依赖升级另开任务。

三条结果不能混写：

- `pnpm check` 和 `pnpm test` 是自动测试。`pnpm test` 只运行 `tests/*.test.mjs`，检查歌词、音量、主题、弹层、队列比较、IPC 名字和隔离路径。
- `pnpm test:ui` 是浏览器验证。Playwright 打开 `tests/harness/index.html` 的内存曲库，覆盖侧边栏、播放栏、队列、菜单、弹窗、主题切换和窗口变窄。基准截图在 `tests/ui/snapshots/`，是 Windows 上 1 倍缩放的 Chromium 画面。这个夹具会假装桌面环境存在，所以页面上的连接指示不是 Rust 引擎。
- 构建出的 exe 才是 Windows Tauri 实机。浏览器和 Playwright 都不能代替它。

更新基准截图：

```powershell
pnpm test:ui:update
```

## 桌面窗口

```powershell
pnpm tauri build --debug --no-bundle
```

会写入曲库或设置的桌面验收按 [docs/isolation.md](docs/isolation.md) 构建和启动。上面这条普通 debug 命令使用系统数据目录，不能代替隔离构建。只改 `APPDATA` 也不会换掉 Tauri 的数据目录。设置页可以确认 Rust 已连接。浏览器预览和 Playwright harness 都不能代替构建出的 exe。

## 文档

- Agent 约束：[AGENTS.md](AGENTS.md)。Claude Code 从 [CLAUDE.md](CLAUDE.md) 引入同一份。
- 验收记录：[docs/current-state.md](docs/current-state.md)。架构：[docs/architecture.md](docs/architecture.md)。
- 产品目标：[docs/RFC-v2.0.md](docs/RFC-v2.0.md)。其中「cargo test + Vitest」已经过时，当前前端测试命令以上面的脚本为准。
- 已完成任务在 [docs/tasks/done/](docs/tasks/done/)。初始化与迁移交接在 [docs/history/](docs/history/)。它们不描述最新行为。新任务用 [docs/tasks/TEMPLATE.md](docs/tasks/TEMPLATE.md)。
````

## File: svelte.config.js
````javascript
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
export default { preprocess: vitePreprocess() };
````

## File: tsconfig.json
````json
{"compilerOptions":{"target":"ES2022","module":"ESNext","moduleResolution":"bundler","strict":true,"allowJs":true,"checkJs":true,"isolatedModules":true,"noEmit":true,"allowImportingTsExtensions":true,"verbatimModuleSyntax":true,"resolveJsonModule":true,"skipLibCheck":true,"lib":["ES2022","DOM","DOM.Iterable"]},"include":["src/**/*.ts","src/**/*.svelte","vite.config.ts"]}
````

## File: vite.config.ts
````typescript
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import tailwindcss from '@tailwindcss/vite';
export default defineConfig({plugins:[svelte(),tailwindcss()],clearScreen:false,server:{strictPort:true,port:1420},envPrefix:['VITE_','TAURI_ENV_']});
````
