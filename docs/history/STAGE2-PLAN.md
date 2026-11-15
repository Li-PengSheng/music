# 第二阶段施工图：健壮性、初始化性能与可编辑性

基线：HEAD `ed191d48c7af4c9bf66c4f8accee308897860af6`。本复核临时去掉 `src-tauri/src/catalog/mod.rs` 顶部 3 个 `#[allow(dead_code)]`，运行了 `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`，取到完整诊断后已把该文件还原。业务代码工作区无残留。开始时已有 `docs/architecture.md`、`docs/current-state.md` 修改及未跟踪 `.repomixignore`、`repomix-output.xml`，全部保留。下列行号来自当前工作区，实施时重新定位函数。没有读取用户曲库、音乐、歌词或设置。

“零破坏”是验收目标。专项 1、3、4 的代码修改尚未实施。除上面这次已还原的 clippy 取证外，自动测试、浏览器验证、Windows Tauri 实机均为 **NOT RUN**。

## 专项 1：一次迁移，轻量连接

### 已确认的调用链

`catalog/mod.rs:175–176` 的 `Catalog::connection` 每次调用 `db::open`。`catalog/db.rs:7–13` 每次 `init`。`init:32` 起建表、三次 `ensure_column`，并执行 `UPDATE tracks SET added_at = strftime('%s','now') WHERE added_at = 0`。`init` 是私有函数，只由 `open` 和 `open_in_memory` 调用。`open_read:17–24` 只设置 2 秒 `busy_timeout`，不建表、不设 `foreign_keys`。`foreign_keys` 与 `busy_timeout` 都是连接级 pragma，不写进库文件。

| 方法（catalog/mod.rs） | 当前行为 | 最小改法 |
|---|---|---|
| `new:167`；`connection:175` | 构造不失败，打开时迁移 | 保留 `new -> Self`。首次连接懒初始化 |
| `home:182`、`track:198`、`tracks_by_ids:238`、`query:253`、`summary:329` | SELECT，但经完整 `open` | `ensure_initialized` 一次后走 `open_light` |
| `track_present:208`、`track_by_path:213`、`lookup:224` | 先 `open_read`，失败回退 `connection()` | 首次 `ensure_initialized` 后仍可 `open_read`；回退改为 `open_light`，禁止回退再跑迁移 |
| `record_listen:178`、`scan:376`；`commands/catalog.rs` 歌单与收藏写入 | 写入 | 初始化后的轻量读写连接 |
| `import_copy:443–522` | 448 行打开目标库并计数；482 行 `db::open(&temp)` 迁移副本；516 行 `rename` 覆盖 `self.path` | 见下方导入裁决 |

### 初始化标记裁决

`Catalog` 增加实例级 `Mutex<bool>`，`new` 设为 `false`。`ensure_initialized` 持锁执行一次 `db::open`，成功并关闭该连接后才置 `true`。失败保持 `false`，允许重试。锁毒化返回明确 `String`。查询不在这把锁里运行。

禁止无互斥的 `AtomicBool` 先检查再迁移。禁止 `OnceLock` 缓存永久失败。禁止进程级或按路径缓存。多个 `Catalog` 指向同一文件时各初始化一次。契约是每实例一次。

`db::open` 继续调用 `init`。`catalog_tests.rs` 和迁移测试直接调用 `db::open` / `open_in_memory`，这些调用不看 `Catalog` 标记。`open_in_memory` 保留自己的 `init`。

`import_copy` 的安全性：

1. 448 行 `self.connection()` 会先把**当前目标文件**迁移完，并把标记置为 `true`。这发生在替换之前。
2. 482 行必须继续是无条件的 `db::open(&temp)`。它迁移的是 `importing.db`，不读取 `Catalog` 标记，也不改成 `open_light`。标记已是 `true` 时仍要执行这行。漏掉它会把未迁移的备份换进 `self.path`。
3. 516 行 `rename` 成功后，同一实例的标记保持 `true`。这只因为新字节已在 482 行迁移完。成功后禁止把标记改回 `false`，否则并发查询会再跑一次迁移。失败时 `rename` 没有发生，原文件已由 448 行迁移完，同样禁止改回 `false`。
4. `catalog_tests.rs:80` 的 `db::open(&dest.path)` 在标记之外，会再执行一次 `init`。`CREATE TABLE IF NOT EXISTS` 与 `added_at = 0` 的更新是幂等的，保持这条直接调用。同函数第 90 行第二次 `import_copy` 必须仍能数到表并因曲库非空失败；计数可以用已打开的连接或 `open_light`，不能靠跳过 schema 来假装空库。
5. 当前测试都是新建 `Catalog`。没有“删掉 `self.path` 再对同一实例调用查询、并期望自动重建”的路径。本阶段不支持运行中外部换库。若以后要支持，必须在路径身份变化时作废标记；不能靠现在这个 `true` 去迁移一个新文件。
6. 目标文件尚不存在时，第一次打开仍是 `db::open`（建目录、建文件、`init`）。`open_light` 使用 `READ_WRITE` 且不带 `CREATE`，不建目录、不执行 DDL。已初始化文件被删掉后，轻量打开应失败，不能建出一个没有 schema 的空库。

每个新连接，包括 `open`、`open_light`、`open_read`，都经过 `configure_connection`：`foreign_keys=ON` 与 2 秒 `busy_timeout`。跳过 `init` 不能丢掉这两项。

本阶段不改 `journal_mode`，不加连接池，不跨线程共享 `rusqlite::Connection`。五个纯查询先用轻量读写。若以后改只读，单独验证空库首次查询、旧 schema、读权限与写权限的失败差异，不和这次去重 DDL 绑在一起。

### 验收补充

只用临时目录。旧 schema 补列后，旧记录、歌单、设置仍在。迁移失败修复后可重试。并发首次访问不返回缺表。成功后反复 `query` / `summary` / `home` / `track` / `tracks_by_ids` 不再执行 DDL。测试内按**实例**计数 `init` 入口。插入 `added_at = 0` 的记录后，热查询不得把它回填；下一次显式 `db::open` 仍回填。断言 `foreign_keys=1`、`busy_timeout=2000`。保留 `catalog_tests` 原断言、WAL 备份源数据不变、导入后同实例查询、`scan` / `cancel`。性能只记录固定临时数据集的冷热查询与初始化次数。

## 专项 2：按编译器清单撤销模块级豁免

取证命令就是上面的 clippy，退出码 1。库目标（无 `cfg(test)`）16 条 `dead_code`。库的 test 目标只剩 2 条。没有 `unused_imports`。`metadata` 不在豁免下，也没有被报出。诊断结束后 `catalog/mod.rs` 已恢复这 3 个模块级 `#[allow(dead_code)]`。实施时再删它们，并按本表逐项处理。禁止把豁免放回模块，禁止放宽 CI。

库目标 16 条，以及 test 目标仍在的 2 条：

| 编译器位置 | test 目标 | 处置 |
|---|---|---|
| `collections.rs:26` `is_favorite` | 消失 | `#[cfg(test)]` |
| `collections.rs:50` `toggle_favorite` | 消失 | `#[cfg(test)]` |
| `collections.rs:58` `set_favorites` | 消失 | `#[cfg(test)]` |
| `collections.rs:213` `add_many_to_playlist` | 消失 | `#[cfg(test)]` |
| `collections.rs:268` `move_in_playlist` | 消失 | `#[cfg(test)]` |
| `db.rs:26` `open_in_memory` | 消失 | `#[cfg(test)]` |
| `db.rs:208` `remove_folder` | 消失 | `#[cfg(test)]` |
| `db.rs:227` `set_setting` | 消失 | `#[cfg(test)]` |
| `library.rs:56` `struct Library` | **仍在** | 结构体上单独 `#[allow(dead_code)]`。两个 target 都没有构造者，`#[cfg(test)]` 消不掉。不删字段以外的扫描函数来凑这个结构 |
| `library.rs:65` `scan_folder` | 消失 | `#[cfg(test)]` |
| `library.rs:73` `walk` | 消失 | `#[cfg(test)]` |
| `library.rs:178` `AlbumGroup.is_compilation` | 消失（测试 514、547 行读取） | `#[cfg_attr(not(test), allow(dead_code))]`。`group_albums:344–349` 仍在生产构造里赋值 |
| `library.rs:179` `AlbumGroup.source` | **仍在** | 字段上无条件 `#[allow(dead_code)]`。`group_albums:350` 仍赋值，全仓库没有读取。`#[cfg(test)]` 和 `cfg_attr(not(test))` 都会在 test 目标留下警告。不补一条假读取 |
| `library.rs:182` `AlbumGroup.disc_count` | 消失（测试 624 行读取） | `#[cfg_attr(not(test), allow(dead_code))]`。生产 `group_albums:352` 仍赋值 |
| `library.rs:358` `album_artist_label` | 消失 | `#[cfg(test)]`。它只被 `library.rs:464` 的单测调用，不是生产保留项 |
| `library.rs:371` `SortKey` | 消失 | `#[cfg(test)]` |
| `library.rs:380` `sort_track_indices` | 消失 | `#[cfg(test)]` |
| `library.rs:413` `sort_tracks` | 消失 | `#[cfg(test)]` |

178 行在库目标上是一条诊断，同时点名 `is_compilation`、`source`、`disc_count`。三个字段的处置不同，按上表分开写属性。

取证阶段与实施阶段的差异：

- `collections.rs:93` `Playlist.created_at`。`Playlist` 派生了 `PartialEq`，比较会读取该字段；`AlbumGroup` 只派生 `Clone, Debug`，编译器不把这两种派生当字段读取。不加 `allow`。
- `library.rs` 的 `use super::metadata;`：最初仅移除模块豁免时没有 `unused_imports`；实施时将 `walk` 等函数标为 `#[cfg(test)]` 后，生产构建不再使用此导入，clippy 报 unused_imports。经用户明确授权，同样为该导入添加 `#[cfg(test)]`，不修改 metadata 模块或其生产调用。

生产保留、禁止套上 `#[cfg(test)]`：`set_favorite:35`（已有 IPC）、`playlist_track_ids:180`、`renumber:296`（`remove_from_playlist:252` 调用它）、`load_tracks`、`load_folders`、`get_setting`、`upsert_tracks`、`now_secs`、`add_folder`，以及 `Track`、`AlbumSource`、`fill_album_fallbacks`、`normalize_key`、`album_name`、`group_albums`、`is_supported`。`favorite_ids`、`list_playlists` 和常规歌单 CRUD 同样保留。收藏表和已注册命令不因界面隐藏而删除。

验收：两个 target 同时没有 `dead_code` / `unused_imports`。测试数量不因门控减少。允许项只有 `Library` 结构体、`AlbumGroup.source`，以及测试才读取的 `is_compilation` / `disc_count`。

## 专项 3：只去掉同一次 Open 里的第二次屏障

用户稿里的 344 / 392 行对不上当前文件。当前事实：

- `Start:317–319`：`execute(Open)?` 然后 `return execute(Play)`。
- `Replace:333–342`：改队列后 `return execute(Start)`。外层 `Replace` 在进入 `Start` 前返回，自己不加第三次 `persist`。
- `Select:311–315`、`State` 的 EOF `298–306` 同样 `return execute(Start)`。
- `Next` / `Previous:351–357` 与 `Start` 相同：`execute(Open)?` 然后 `return execute(Play)`。
- `Open:393–439` 在 429 行 `prepare`，430 行 `let status = self.engine.synchronize()?`。这个 `status` 只活在 match 臂内，用来按 `current_path` 接受或拒绝 `self.track`。臂末没有 `return`。
- 公共尾部 `469` 行再次 `self.engine.synchronize()?`，随后按 `replaces_transport` 排空 EOF，`474` 行 `revision += 1`，`493–495` 行按节流 `persist`。
- `Play:440–448` 用的是 `engine.status()`（互斥量克隆），不是屏障；确认发生在它落入的 469 行。

`player.rs:131–137` 的 `synchronize` 发送 `Cmd::Synchronize` 并最多等 15 秒。引擎在 `807–809` 行收下回复槽，`937–951` 行发布状态后回复。`status()` 不是屏障。

一次成功的 `Start` 今天是 3 次屏障、2 次 `revision`、2 次 `persist` 调用。`persist:567–587` 在 `session_path` 为空时直接返回，内容相同时不写盘。调用次数不是磁盘写次数。

### 为什么 430 行之后还有 469 行

430 到 469 之间，`Open` 只改控制器内存里的 `self.track` 和可能的队列。`execute` 是 `&mut self`，中间没有另一条控制器命令。`Prepare`（`player.rs:675–708`）把状态留在 `Paused`。`fill_output:545–549` 在 `paused` 时直接写静音并返回，不增加 `frames_played`。因此同一次 `Open` 的第二次屏障读到的是同一份已发布状态：暂停、同一路径、同一位置、同一错误。它存在，是因为 430 行的局部变量在臂结束时丢掉了，尾部又无条件发了一次新命令。

### 保留的去重

同一次 `Open` 把 430 行已经取到的 `PlayerStatus` 交给尾部生成快照。尾部在这次 `Open` 上不再调用 `synchronize`。EOF 排空、`revision += 1`、一次 `persist` 都留在尾部。`Play` 以及 `Pause` / `Stop` / `Seek` / `Volume` 等没有提前屏障的动作，尾部仍各调用一次 `synchronize`。

去重之后，成功的 `Start` / `Next` / `Previous` / `Replace` / `Select` / EOF 切歌仍是两次 `execute`：`Open` 一次屏障（430 行，复用于 `Open` 的快照）加 `Play` 尾部一次屏障。`revision` 仍加 2，`persist` 仍调用两次。单独的 `Open` 仍在返回前写好暂停会话。`Open` 返回 `Ok` 之后若 `Play` 失败，暂停会话和已经增加的 `revision` 保持不变，错误继续向外传。

`session_restores_queue_position_mode_and_pauses` 依赖单独 `Open` 写出暂停会话。`replace_queue_starts_first_real_file` 与 `queue_uses_existing_engine_for_selection_next_and_previous` 断言复合动作结束时是 `"playing"`。现有 `controller.rs` 测试 16–205 行的断言保持原样。

### 已裁决不实施

把 `Start` 收成不写盘的 `prepare_track` + `apply_play` + 一次最终 `persist`，会改变失败顺序。今天 `Open` 的 `?` 保证：中间 `persist` 失败时 `Play` 不会执行；`Open` 成功后暂停会话已经落盘，然后才 `Play`。合成一次最终保存后，可能先出声再返回保存失败。这与现有 `?` 顺序冲突。稳定性优先，这处优化砍掉。不改 `Cmd`、解码器、线程布局，不把成功复合动作的 `revision` 从 +2 改成 +1，不让 `attach_session:537` 与 `flush_session` 进入这次去重。

### Poison

`audio/player.rs` 里 `.lock().unwrap()` 改为 `.lock().unwrap_or_else(|e| e.into_inner())`：`status:141`；`Shared::clear_ring:239`、`fill:243`；`begin_track:489`；`fill_output:561`；`engine_loop:848`、`937`、`951`；`output_selftest` 的 ring `1225`、`1240` 与 capture `1217`、`1254`；测试 `1369`、`1387`、`1400`。`cpal_gate:267` 已经恢复。`554` 的 `map` / `unwrap_or(false)` 与 `586` 的 `if let Ok` 保持原样。`synchronize().unwrap` 和解码 `unwrap` 不在这次替换里。

恢复 guard 只避免毒化后再 panic。它不证明被保护的数据完整，也不能救活已经退出的引擎线程。本次不重写 ring，不在回调里加日志、IO 或新分配。用独立线程在持锁时 panic，再验证 `status` 读取、`clear` / `fill`、回调填充和状态发布不再二次 panic。真实设备另按 `docs/isolation.md` 验收。null output 通过不是出声证据。

## 专项 4：展开格式前先改掉单行正则

`package.json` 未声明 Prettier 或 `prettier-plugin-svelte`。实施时锁定一套兼容的 devDependency 与 lockfile。禁止 `prettier --write .`。不格式化 Rust、历史文档、fixture、构建产物，也不格式化 CSS。本轮不格式化 `App.svelte`、`Dialog.svelte`、`Select.svelte`、`Queue.svelte`、`tokens.css`、`app.css`。

先改测试并在**未格式化**源码上通过，再格式化。用 Svelte / TypeScript AST 提取声明、副作用和调用，保留现有运行时断言。不删断言，不改回压缩源码，不用更新快照来变绿。

第一批白名单：`src/lib/views/NowPlaying.svelte`、`src/lib/components/TrackDetails.svelte`、`Artwork.svelte`、`Cover.svelte`。随后：`src/lib/actions.ts`、`services/music.ts`、`stores/preferences.ts`、`stores/nav.svelte.ts`、`utils/lyrics-view.ts`、`utils/shortcuts.ts`、`layout/PlayerBar.svelte`、`Sidebar.svelte`、`views/SongsView.svelte`、`HomeView.svelte`、`GridView.svelte`、`SettingsView.svelte`、`components/ContextMenu.svelte`。

### 会因换行或空格失败、必须先改的匹配

| 测试 | 匹配 | 触到的白名单文件 |
|---|---|---|
| `tests/player-stability.test.mjs:31` | `/let trackId=\$derived\(s\?\.track_id\);\s*\$effect\(\(\)=>\{trackId;dragging=null;\}\);/`。`\s*` 只在两条语句之间；效果内部 `trackId;dragging=null` 不能有空格 | 格式化 `PlayerBar.svelte` 之前 |
| `tests/ui-refinements.test.mjs:45` | `/catch\(e\)\{notifyError\(e\);\}/`，括号与花括号内侧零空白 | 格式化 `actions.ts` 之前 |
| `tests/ui-refinements.test.mjs:47` | `/deletePlaylist=\(playlistId:number\)=>invoke<void>\('library_playlist_edit',\{action:'delete',playlistId/`，`=>`、冒号和对象内零空白 | 格式化 `music.ts` 之前 |
| `tests/ui-refinements.test.mjs:21` | `indexOf(' function key(')`，再精确替换 `function key(e:KeyboardEvent)` 与 `document.activeElement as HTMLButtonElement` | 格式化 `ContextMenu.svelte` 之前 |
| `tests/v091-collections.test.mjs:17` | `/{view:'local'}/` | 格式化 `nav.svelte.ts` 之前 |
| 同文件 19 | `/go\(\{view:'local'\}\)/` | 格式化 `Sidebar.svelte` 之前 |
| 同文件 20 | `/library\.summary\.tracks\}<\/span>/`，`}` 前不能有空格 | 同上 |
| 同文件 22 | `/case 'local':return localQuery\(sort,''\)/`，冒号后零空白 | 格式化 `SongsView.svelte` 之前 |
| 同文件 23 | `/route\.view==='songs'\|\|route\.view==='local'\|\|route\.view==='artist'/`，`===` 与 `\|\|` 两侧零空白 | 同上 |
| 同文件 43 | `/{#each library\.summary\.playlists as list \(list\.id\)\}/` | 格式化 `Sidebar.svelte` 之前 |
| `tests/v091-layers.test.mjs:58` | `/<Modal /`，空格是字面量，不是 `\s`。同一断言还读 `Dialog.svelte` | 格式化 `TrackDetails.svelte` 之前。`Dialog.svelte` 本轮不格式化 |

这些文件本轮保持原样，对应匹配才能继续成立：

- `App.svelte`：`player-stability.test.mjs:10` 的 `indexOf('async function connect')` 要求 `function` 与 `connect` 留在同一片段；`v091-layers.test.mjs:73–74` 的 `<Queue`、`'<Queue '` 和 `class="immersive-layer"` 也读它。
- `Select.svelte`：`ui-refinements.test.mjs:7–9` 已用 `\s*`，仍不纳入本轮白名单。
- `tokens.css`：`v091-layers.test.mjs:76` 的 `` `--z-${name}:(\\d+)` `` 要求冒号后紧跟数字。`v092-visual`、`v092-queue-motion` 的色值和时间正则同样依赖当前 CSS 拼写。
- `app.css`：`reduce-motion.test.mjs` 读取媒体规则。本轮不改 CSS 测试实现。

`player-stability.test.mjs:9` 的 `/let artwork\s*=\s*\$state/`，以及 33–35 行的 `getArtwork` / `trackCover` 调用形，普通换行还能过。`v091-collections.test.mjs:28–30` 的 `indexOf('export function localMenu')` 与 `export async function newPlaylist` 在函数名仍紧挨关键字时还能过。`ipc-surface.test.mjs` 已按注释、字符串和泛型扫描，格式化后补一条等价输入即可，不改命令名。

## Phase 1～4

每阶段先记录 HEAD 与工作区指纹，检查后再取指纹。临时数据只用隔离目录。失败只修本阶段。不自动提交。

| Phase | 修改文件 | 验证与退出条件 |
|---|---|---|
| 1 数据库初始化 | `catalog/db.rs`、`catalog/mod.rs`、`catalog/catalog_tests.rs`；`docs/rust-guidelines.md` 补一句连接级 pragma | `MUSIC_MINE_NULL_OUTPUT=1`。`cargo test --manifest-path src-tauri/Cargo.toml catalog::`。`cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`。`cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`。`pnpm test`。并发、重试、导入、迁移和热路径零重复 DDL 都有证据。`import_copy` 的 482 行仍是 `db::open` |
| 2 死代码门禁 | `catalog/mod.rs`、`library.rs`、`collections.rs`、`db.rs` | 去掉 3 个模块级 allow 后按专项 2 的表处理。然后 fmt、clippy `-D warnings`、`cargo test`，均带 `--manifest-path src-tauri/Cargo.toml` 与空输出变量。`pnpm test`。无新的模块级 allow，原测试不被跳过 |
| 3 控制器与 Poison | `playback/controller.rs`、`audio/player.rs` 及两者内联测试；`docs/rust-guidelines.md` | 只做专项 3 的 Open 状态复用和 Poison 替换。先 Poison，再 Open 去重，两步可单独回退。空输出下 `cargo test --manifest-path src-tauri/Cargo.toml`。`pwsh -File scripts/check.ps1`。隔离 exe 实机切歌、暂停、恢复、退出持久化另做。**NOT RUN 不得称为完整音频验收** |
| 4 可读性 | 专项 4 白名单；上表点名的测试；`tests/helpers/source-ast.mjs`；`package.json`、`pnpm-lock.yaml`、Prettier 配置；`docs/testing.md` | 未格式化前测试适配通过。白名单 `prettier --check`。`pnpm check`、`pnpm test`、`pnpm build`、`pnpm test:ui`、`pwsh -File scripts/check.ps1` 退出码 0，不更新快照 |

Phase 3 不实施“成功只 `persist` 一次”或内部不写盘的 `start_track`。那会改变 `Open` 成功、`Play` 失败时的既有顺序。
