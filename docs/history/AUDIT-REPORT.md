# 仓库治理与重构施工图

审计日期：2026-11-15。基线：`main`，HEAD `2e13dc0f536c1938a635edd478840af7d994c8ab`。未跟踪文件 `repo-context.txt` 保留不动。本文只是待执行方案。红队已对照当前源码修正原草稿，未改业务代码。

自动测试 **NOT RUN**。浏览器验证 **NOT RUN**。Windows Tauri 实机 **NOT RUN**。

## 红队结论

以下按当前文件核对，执行时以此节和各 Phase 的【注意事项】为准。

### 搬迁不会打断的两个表面测试

`tests/ipc-surface.test.mjs` 不按文件名找 Rust。`rustSources`（157–163 行）递归读取 `src-tauri/src` 下全部 `.rs`。`extractRegistered` 只取 `generate_handler!` 里 `::` 的最后一段，所以 `commands::catalog::library_query` 仍得到 `library_query`。它写死的路径只有三处，本方案都不动：

- `src/lib/services/music.ts`（199 行）
- `tests/harness/mock-ipc.js`（200 行）
- `docs/ipc-contract.md` 的围栏 `ipc-commands` 与 `ipc-unused`（201–204 行）

`tests/isolation-guard.test.mjs` 不读 Rust 或前端源码。它写死 `scripts/isolation-cli.ps1`（10 行），并在临时仓库里使用 `src-tauri/tauri.conf.json` 与 `src-tauri/tauri.portable.conf.json`。这些路径保持不动。

### 原草稿里会编译失败或测挂的点

1. `include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/icons/icon.png"))` 不能编译。`include_bytes!` 只接受字符串字面量。`desktop.rs:237–238` 现在是 `include_bytes!("../icons/icon.png")`，相对 `src-tauri/src/desktop.rs`，指向 `src-tauri/icons/icon.png`。文件改到 `src-tauri/src/platform/desktop.rs` 后，字面量必须是 `include_bytes!("../../icons/icon.png")`。
2. 私有子模块对外不可见。父模块能看见自己的私有子模块；堂兄弟模块看不见。因此 `platform::desktop`、`platform::media`、`artwork::covers`、`commands::catalog`、`playback::controller`、`infrastructure::logging`，以及 `catalog` 里被外部使用的 `library`、`collections`、`metadata`，都必须是 `pub(crate)`。只写 `mod desktop` 时，`lib.rs` 里的 `platform::desktop::setup` 编译失败。
3. `catalog/catalog.rs` 会把类型路径变成 `catalog::catalog::Catalog`。当前 `lib.rs`、`covers.rs:793`、`cover_corpus.rs:237` 都是 `catalog::Catalog`。旧文件正文放到 `catalog/mod.rs`，这些路径保持不变。
4. `catalog.rs:519–521` 的 `#[path = "catalog_tests.rs"]` 相对**所在源文件目录**，不是 crate 根。`catalog/mod.rs` 与 `catalog/catalog_tests.rs` 放在同一目录，属性原文不动。`catalog_tests.rs` 只有 `use super::*`，没有 `crate::` 路径。
5. 集成测试是外部 crate。`src-tauri/Cargo.toml` 的库名是 `music_mine_desktop`。`src-tauri/tests/audio.rs:1` 使用 `music_mine_desktop::player::{analyze_file, PlaybackState, Player}`。`src-tauri/tests/lyrics.rs:6` 使用 `music_mine_desktop::lyrics`，并调用 `load_for_track` 与公开字段 `source`、`synced`、`lines`、`time_ms`、`text`、`index_at`。这些项现在都是 `pub`。根上必须 `pub use audio::player; pub use playback::lyrics;`。`pub(crate) use` 对集成测试不可见，也不能把 `player` / `lyrics` 降成 `pub(crate)`。`player.rs:1271–1272` 的 `#[cfg(test)] pub mod tests` 不进入集成测试，不要导出它。`main.rs:3` 的 `music_mine_desktop::run()` 依赖 `lib.rs` 里现有的 `pub fn run`，`main.rs` 不改。
6. 下面三个 Node 测试按旧文件名 `readFileSync`，搬迁后是 ENOENT。`ipc-surface` 与 `isolation-guard` 不在此列。
   - `tests/v091-theme.test.mjs:60`：`src-tauri/src/desktop.rs`
   - `tests/v091-collections.test.mjs:54`：`src-tauri/src/collections.rs`
   - `tests/v091-collections.test.mjs:56`：`src-tauri/src/db.rs`
7. `src/lib/actions.ts` 调用 services 和 stores，留在 `src/lib/actions.ts`。`tests/v091-collections.test.mjs:27` 与 `tests/ui-refinements.test.mjs:39` 因此不用改。

### 已从方案中删除

- `.ai-context-ignore`、`check.ps1` 的 `-Mode` / `-Area`、CI 缓存键、生成器 `export_ipc`、`ipc.generated.ts`、`ipc-schema.json`、全命令 `CommandError` 迁移。
- 把 `actions.ts` 放进 `utils/`。
- 按版本号重命名 `tests/v09*.test.mjs`。`package.json` 的 `tests/*.test.mjs` 已经能发现它们。
- 原草稿里的全量行数表和 import 转储。搬迁表就是文件清单。`tests/perf/out/` 是本地忽略产物，不参与替换。

`scripts/check.ps1` 不改。它由 `pnpm check:all` 通过 `pwsh -File` 启动，第 17 行设置的 `MUSIC_MINE_NULL_OUTPUT` 只活在该子进程里。

## 1. 文档一句规则

旧规则（`AGENTS.md` 开头的事实优先级）：当前代码，然后 `docs/current-state.md`、`docs/architecture.md`、`docs/tasks/done/`、`docs/RFC-v2.0.md`；同时又说 done 与 history 不是当前行为。

新规则，只改这一处：当前代码，然后 `docs/current-state.md`、`docs/architecture.md`。`docs/tasks/done/` 与 `docs/history/` 仅在任务点名时打开，并标明历史基线。`docs/RFC-v2.0.md` 只描述目标。原因：避免两套顺序。不新增忽略文件，不改 RFC 正文。

默认阅读：`AGENTS.md`、`docs/current-state.md`，再加任务点名的一份专项文档（Rust `docs/rust-guidelines.md`，IPC `docs/ipc-contract.md`，界面 `docs/frontend-guidelines.md`，验收 `docs/testing.md`，隔离 `docs/isolation.md`）。

## 2. Rust 搬迁

先搬文件并改路径，不拆函数，不改音频线程、锁、缓存和数据库。`src-tauri/src` 现有 17 个 `.rs`：两个入口留在根上，其余 15 个按下表移动。

| 当前文件 | 新路径 | 模块 |
|---|---|---|
| `lib.rs` | 留在根 | crate 根 |
| `main.rs` | 留在根 | 二进制，不改 |
| `player.rs` | `audio/player.rs` | `audio::player`，`pub` |
| `controller.rs` | `playback/controller.rs` | `playback::controller`，`pub(crate)` |
| `lyrics.rs` | `playback/lyrics.rs` | `playback::lyrics`，`pub` |
| `catalog.rs` | `catalog/mod.rs` | `catalog`，正文仍是这个模块 |
| `library.rs` | `catalog/library.rs` | `pub(crate)` |
| `collections.rs` | `catalog/collections.rs` | `pub(crate)` |
| `db.rs` | `catalog/db.rs` | 私有。外部无人引用 |
| `metadata.rs` | `catalog/metadata.rs` | `pub(crate)` |
| `catalog_tests.rs` | `catalog/catalog_tests.rs` | 仍由 `#[path = "catalog_tests.rs"]` 挂到 catalog |
| `catalog_commands.rs` | `commands/catalog.rs` | `commands::catalog`，`pub(crate)` |
| `covers.rs` | `artwork/covers.rs` | `pub(crate)` |
| `cover_corpus.rs` | `test_support/cover_corpus.rs` | 仅测试编译 |
| `desktop.rs` | `platform/desktop.rs` | `pub(crate)` |
| `media.rs` | `platform/media.rs` | `pub(crate)` |
| `logging.rs` | `infrastructure/logging.rs` | `pub(crate)` |

根 `lib.rs` 开头换成：

```rust
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
```

各 `mod.rs` 只有声明：

```rust
// audio/mod.rs
pub mod player;

// playback/mod.rs
pub(crate) mod controller;
pub mod lyrics;

// commands/mod.rs
pub(crate) mod catalog;

// artwork/mod.rs
pub(crate) mod covers;

// platform/mod.rs
pub(crate) mod desktop;
pub(crate) mod media;

// infrastructure/mod.rs
pub(crate) mod logging;

// test_support/mod.rs
mod cover_corpus;
```

`catalog/mod.rs` 在旧 `catalog.rs` 正文之前写入下面四行，并删掉原来的 `use crate::{collections, db, library, metadata};`。`use library::Track;` 与文件末尾的 `#[cfg(test)] #[path = "catalog_tests.rs"] mod tests;` 保持原文。

```rust
#[allow(dead_code)]
pub(crate) mod collections;
#[allow(dead_code)]
mod db;
#[allow(dead_code)]
pub(crate) mod library;
pub(crate) mod metadata;
```

`#[allow(dead_code)]` 今天只在 `lib.rs` 的 `collections`、`db`、`library` 三条上。属性跟着这三条走。不要加到根上的 `mod catalog`，否则豁免会扩到 catalog 正文和 metadata。

`#[cfg(test)]` 今天只包 `cover_corpus`（`lib.rs:6–7`）。改由根上的 `#[cfg(test)] mod test_support;` 包住。`cover_corpus.rs` 里的 `#[ignore]` 保持，常规 `cargo test` 不跑它。

路径替换，只改这些前缀：

| 文件 | 替换 |
|---|---|
| `catalog/db.rs` | `crate::library` → `super::library`（5、240、352 行）；`crate::metadata` → `super::metadata`（353 行） |
| `catalog/collections.rs` | `crate::db` → `super::db`（12、311 行）；`crate::library` → `super::library`（312 行） |
| `catalog/library.rs` | `crate::metadata` → `super::metadata`（6 行） |
| `catalog/metadata.rs` | `crate::library` → `super::library`（6 行） |
| `commands/catalog.rs` | 首行改为 `use crate::catalog::{collections, library, metadata};` 加上原有的 `execute`、`Action`、`PlayerSnapshot`、`Shared`。`crate::covers::` → `crate::artwork::covers::`。`crate::library::` → `crate::catalog::library::`。`crate::controller::` → `crate::playback::controller::` |
| `playback/controller.rs` | `crate::player::` → `crate::audio::player::` |
| `audio/player.rs` | `crate::logging::` → `crate::infrastructure::logging::` |
| `artwork/covers.rs` | `crate::metadata::` → `crate::catalog::metadata::`。`crate::catalog::Catalog` 不动 |
| `test_support/cover_corpus.rs` | `crate::catalog::Catalog` 不动 |
| `platform/desktop.rs` | `controller::Action` → `playback::controller::Action`。`crate::controller::` → `crate::playback::controller::`。`crate::player::` → `crate::audio::player::`（404 行）。`include_bytes!` 改为 `"../../icons/icon.png"`。`crate::Shared` 不动 |
| `platform/media.rs` | `crate::controller::` → `crate::playback::controller::`。`crate::Shared` 不动 |
| `lib.rs` 其余正文 | `covers::` → `artwork::covers::`。`desktop::` → `platform::desktop::`。`media::` → `platform::media::`。`catalog_commands::` → `commands::catalog::`。`catalog::` 与 `lyrics::` 保持。`generate_handler!` 只改路径前缀，命令函数名不变 |

`CARGO_MANIFEST_DIR` 拼 `tests/fixtures` 的代码不改。库名 `music_mine_desktop` 不改。

同一 Phase 只更新仍被当作当前说明的路径：`docs/architecture.md` 中的 `src-tauri/src/*.rs` 树，`docs/rust-guidelines.md` 的模块清单。不改 `docs/history/` 与 `docs/tasks/done/`。

同一 Phase 改三处测试读取路径：

- `tests/v091-theme.test.mjs:60` → `src-tauri/src/platform/desktop.rs`
- `tests/v091-collections.test.mjs:54` → `src-tauri/src/catalog/collections.rs`
- `tests/v091-collections.test.mjs:56` → `src-tauri/src/catalog/db.rs`

### 【注意事项】

- `pub use` 两条必须留在 crate 根，可见性是 `pub`。少了它们，`cargo test` 里的 `tests/audio.rs` 与 `tests/lyrics.rs` 找不到 `music_mine_desktop::player` 和 `music_mine_desktop::lyrics`。
- 被 crate 根或其他领域调用的子模块用 `pub(crate)`。`db` 与 `cover_corpus` 保持私有。`player` 与 `lyrics` 保持 `pub`，已有的 `pub fn` / `pub struct` / `pub enum` 不降级。
- `#[path = "catalog_tests.rs"]` 不要改成带 `catalog/` 的路径。
- `include_bytes!` 使用 `../../icons/icon.png` 这一条字面量。
- `ipc-surface` 与 `isolation-guard` 不改。
- library 与 metadata 的现有双向 `use` 保持，用 `super::`。本阶段不引入 trait，不改表。

## 3. 前端搬迁

`services/`、`stores/`、`styles/`、现有 `components/`、`layout/`、`views/` 保持。`src/lib/actions.ts` 不移动。`src/lib/services/music.ts` 不移动。

| 现路径 `src/lib/` | 新路径 `src/lib/` |
|---|---|
| `Artwork.svelte`、`Cover.svelte`、`QueueCover.svelte`、`Select.svelte`、`TrackDetails.svelte` | `components/` 下同名 |
| `NowPlaying.svelte` | `views/NowPlaying.svelte` |
| `Queue.svelte` | `layout/Queue.svelte` |
| `cover-diagnostics.ts`、`lyrics-view.ts`、`shortcuts.ts` | `utils/` 下同名 |

需要改 import 的应用文件：

| 文件 | 新 import |
|---|---|
| `src/App.svelte` | `./lib/utils/shortcuts`；`./lib/utils/cover-diagnostics`；`./lib/layout/Queue.svelte`；`./lib/views/NowPlaying.svelte`；`./lib/components/TrackDetails.svelte` |
| `components/TrackList.svelte` | `./Cover.svelte`。`../actions` 不动 |
| `layout/PlayerBar.svelte` | `../components/Artwork.svelte` |
| `views/SongsView.svelte` | `../components/Select.svelte`、`Cover.svelte`、`Artwork.svelte`。`../actions` 不动 |
| `views/SettingsView.svelte` | `../components/Select.svelte`；`../utils/shortcuts`。`../actions` 不动 |
| `views/HomeView.svelte` | `../components/Cover.svelte`、`Artwork.svelte`。`../actions` 不动 |
| `views/GridView.svelte` | `../components/Cover.svelte`、`Select.svelte`。`../actions` 不动 |
| `components/Artwork.svelte` | `../utils/cover-diagnostics` |
| `components/Cover.svelte` | `./Artwork.svelte` 不动；`./services`、`./stores` 改为 `../services`、`../stores` |
| `views/NowPlaying.svelte` | `../components/Select.svelte`、`Artwork.svelte`、`IconButton.svelte`、`EmptyState.svelte`；`../services/music`；`../utils/lyrics-view`；`../utils/color`；`../stores/...` |
| `layout/Queue.svelte` | `../components/QueueCover.svelte`、`IconButton.svelte`、`EmptyState.svelte`、`tooltip`；`../services/music`；`../stores/...`；`../utils/format`、`../utils/queue` |
| `components/QueueCover.svelte` | `./Artwork.svelte` 不动；`../services/music`；`../stores/library.svelte`；`../utils/row-visibility` |
| `components/TrackDetails.svelte` | `./Artwork.svelte`、`./Select.svelte`、`./IconButton.svelte`、`./Modal.svelte`；`../services/music`；`../stores/...`；`../utils/format` |

`layout/Sidebar.svelte`、`layout/TopBar.svelte`、`actions.ts` 的 import 不动。

测试只改读取路径或 import 路径，不断言内容：

| 测试 | 改动 |
|---|---|
| `tests/lyrics-view.test.mjs:3` | `../src/lib/utils/lyrics-view.ts` |
| `tests/shortcuts.test.mjs:3` | `../src/lib/utils/shortcuts.ts` |
| `tests/player-stability.test.mjs:34` | `src/lib/views/NowPlaying.svelte`。同文件的 `src/App.svelte` 与 `src/lib/layout/PlayerBar.svelte` 不动 |
| `tests/ui-refinements.test.mjs:6` | `src/lib/components/Select.svelte`。39 行的 `src/lib/actions.ts` 不动 |
| `tests/v091-layers.test.mjs:56` | 数组里只把 `src/lib/TrackDetails.svelte` 改为 `src/lib/components/TrackDetails.svelte`。`Dialog.svelte` 不动 |

同一 Phase 更新 `docs/architecture.md` 里 Select、Queue、NowPlaying、QueueCover、Artwork、Cover 的路径，以及 `docs/decisions.md:34` 的 `src/lib/Select.svelte`。Select 移入 `components/` 后，删掉“不在 components 里”这句。

### 【注意事项】

- `music.ts` 留在 `src/lib/services/music.ts`，否则 `ipc-surface.test.mjs:199` 失败。
- `actions.ts` 留在 `src/lib/actions.ts`。所有 `../actions` 保持。
- `v091-collections.test.mjs:9` 的 `sources('src')` 已经递归，收藏文案扫描不用改。
- 不改 `tests/perf/out/`。
- 不新增路径别名，不新增 barrel `index.ts`。

## 4. 本轮不做

IPC 仍以 `docs/ipc-contract.md` 和 `tests/ipc-surface.test.mjs` 的命令名对齐为准。错误类型继续是 `String`（`docs/decisions.md` 的 ADR-007）。封面、快照和 mock 的形状差异留在原地，另开任务再改，并且那时要同时改 `music.ts`、`mock-ipc.js`、`cover-loader.ts`。本轮不加工具生成类型，不改 `scripts/check.ps1`，不改 `.github/workflows/check.yml`，不重命名版本化测试，不按行数拆 `player.rs`、`controller.rs`、`covers.rs`。

## 5. 分阶段执行

每阶段单独 diff。不自动 commit。下一阶段开始前，上一阶段的验证命令要通过。

### Phase 1：AGENTS 一句优先级

修改文件：`AGENTS.md` 的事实优先级那一句。

验证：`git diff --check`。没有代码变化，不跑测试。

【注意事项】不要顺手改 RFC、current-state、architecture。不要新增忽略文件或脚本参数。

### Phase 2：Rust 机械归位

修改文件：第 2 节的 15 个搬迁文件、`lib.rs`、7 个新 `mod.rs`、三个 Node 测试的 Rust 路径、`docs/architecture.md`、`docs/rust-guidelines.md`。`main.rs` 不改。

验证：

```powershell
$env:MUSIC_MINE_NULL_OUTPUT = '1'
cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
pnpm test
```

完成条件：上述命令退出码为 0。`tests/audio.rs` 与 `tests/lyrics.rs` 能编译。`cover_corpus` 的 ignore 测试没有被跑。`pnpm test` 里 `ipc-surface` 与 `isolation-guard` 仍通过。

【注意事项】见第 2 节。可见性、`pub use`、`include_bytes!` 字面量、`#[path]` 四项缺一就会在 `cargo test` 或 `cargo clippy -D warnings` 失败。

### Phase 3：前端归位

修改文件：第 3 节列出的搬迁文件、那里列出的 import、五个测试路径、`docs/architecture.md` 的对应行、`docs/decisions.md:34`。

验证：

```powershell
pnpm check
pnpm test
pnpm build
```

`pnpm test:ui` 作为界面证据，单独记录。本阶段不改标记和文案，快照应保持。

【注意事项】见第 3 节。`music.ts` 与 `actions.ts` 的路径是测试硬编码，移走就会挂。

Phase 2 与 Phase 3 都改 `docs/architecture.md` 时，各改各的路径段，避免后一阶段覆盖前一阶段。
