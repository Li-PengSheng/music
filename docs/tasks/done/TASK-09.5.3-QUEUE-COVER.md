# 09.5.3 播放队列封面加载专项修复

分支 `main`，HEAD `decbefb4`，未提交。开始时唯一的未提交文件是 `portable/README.txt`（用户修改），本轮未改动它。

## 结论

大队列下封面慢和“永远不显示”有两个独立原因，都不是 Rust 解码本身慢：

1. **请求积压。** 旧 `QueueCover` 行一进入视口就发请求，发出后不撤销，也没有并发上限。快速滚过 336 首的队列时，同时在途的封面请求达到 213–278 个，停下后可见行要排在它们后面，IPC 中位耗时 6.0–7.4 s，最长 9.6 s。观察窗内 10 个应有封面的可见行只显示 4–7 个。
2. **把失败当成“没有封面”并永久缓存。** 下列情况在 Rust 里都返回 `None`，写入永久 `.none` 文件，前端又把 `null` 缓存整个会话：
   - WebP/GIF；
   - 超过 8 MB 的图片；
   - 解码内存超过 32 MB（3000×3000 RGBA PNG 需要 36 MB）；
   - 标签读取 I/O 错误。

   前端的 `.catch(() => {})` 吞掉错误，不重试。这些行永远停在占位图。

6 首测试通过，是因为请求数从未超过 6，不会积压；而且样本里只有小 JPEG，触发不到格式和尺寸限制。

## 根因与代码位置（修复前）

| 位置 | 问题 |
|---|---|
| `src/lib/QueueCover.svelte` | IntersectionObserver 用隐式视口作根，`rootMargin` 不能越过 `.queue-list` 的裁剪，所以没有预取；第一次相交就 `disconnect`，请求不能取消；`$effect` 依赖 track 对象，`coverEpoch` 变化时先把 `src` 置空；`.catch(() => {})` |
| `src/lib/services/music.ts` `queueCover` / `trackThumb` | Promise 缓存把 resolved `null` 存一整个会话；并发不受限 |
| `src-tauri/src/covers.rs` `encode_bytes` / `write_disk` | 只认 JPEG/PNG；`MAX_SOURCE_BYTES` 为 8 MB、`MAX_DECODE_ALLOC` 为 32 MB；所有失败都写 `.none`；Triangle 缩放 |
| `src-tauri/src/metadata.rs` `read_cover` | 读取错误变成 `None`；取第一张图片而不是封面（CoverFront） |
| `src-tauri/src/catalog.rs` `track_by_path` / `track_present` | 每个封面请求 `db::open` 都要跑一次 `init()`：建表检查加 `UPDATE tracks ... WHERE added_at=0` 写语句，每次都争写锁 |
| `src/lib/Cover.svelte`、`src/App.svelte` | 同样 `.catch(() => {})`，没有重试 |

## 修改

**Rust**

- `covers.rs`：结果分三类，规则如下。
  - **确定没有**（无图、格式不支持、超限、损坏）：缓存 `.none`，直到文件的 mtime 或长度变化。
  - **暂时失败**：返回 `CoverError::FileMissing` 或 `Unreadable`，不写磁盘，也不进内存缓存。
  - **磁盘文件名**：加 `v2-` 前缀。启动时在后台线程删除旧前缀的文件，因为旧策略把失败写成了 `.none`；这一步不是为了掩盖问题。
  - **WebP/GIF**：不超过 512 KB 时原样返回，带正确的 MIME。
  - **尺寸上限**：源图上限调到 16 MB，解码内存上限调到 80 MB；解码槽仍是 2 个。
  - **缩放**：改用 `thumbnail`（面积采样）。
  - **计数**：新增 `Stats.snapshot()`，记录请求、命中、解码、各失败原因、解码耗时和槽位等待。
- `metadata.rs`：新增 `read_cover_checked`，先取 CoverFront，区分 I/O 错误和标签错误。`read_cover` 的行为保持不变。
- `db.rs` / `catalog.rs`：封面查询改用只读连接 `open_read`，不做 schema 检查，没有写语句，`busy_timeout` 为 2 s。连接打不开时退回原来的 `connection()`。
- `catalog_commands.rs` / `lib.rs`：三个封面命令把暂时失败返回为不含路径的错误码。`library_queue_thumb` 遇到不在曲库的路径时返回 `cover-not-in-library`（以前是 `null`）。新增 `library_cover_stats`，只返回数字。
- 没有新增 crate，Cargo.lock 未变，capabilities 未变，也没有开放任意路径读取或 asset 协议。

**前端**

- 新增 `src/lib/utils/cover-loader.ts`，一个有上限的优先级调度器，没有 IPC：
  - 并发上限 4；
  - 优先级依次为当前曲目、可见行、预取；
  - 相同 key 的请求合并；
  - 行离开时，尚未开始的请求取消；
  - 排队数上限 48，超出时丢弃最新的预取；
  - `null` 作为“没有封面”缓存，最多 600 条；
  - 暂时失败按 0.7 s、2.5 s 重试两次，单次超时 15 s，失败结果 30 s 后过期；
  - `peek` 同步返回已完成的结果。
- 新增 `src/lib/utils/row-visibility.ts`：每个列表共用两个 IntersectionObserver，根是 `.queue-list`。一个判断“可见”，一个带 360 px 预取边距判断“临近”。观察对象是 `.queue-item` 行本身，因为行内容在屏幕外时会被 `content-visibility` 跳过。
- `QueueCover.svelte` 的改动：
  - 只依赖路径字符串；
  - 已完成的结果同步显示；
  - 重新扫描时保留旧图，直到新结果到达；
  - 当前曲目用最高优先级；
  - 状态写在 `data-cover` / `data-cover-code` 上，不含路径。
- `Queue.svelte`：给 QueueCover 传入 `current`。
- `Cover.svelte`（歌曲列表、专辑网格、详情）：走同一个 loader，离开即取消，不再吞错误。
- `Artwork.svelte`：新增 `data-image`（empty / loading / loaded / error），用来区分 WebView 解码失败。
- `music.ts`：`invoke` 仍然只在这个文件里。`getArtwork` 遇到暂时失败重试一次。`globalThis.__musicMineCoverStats` 只有计数，Release 中也存在，不写日志，不含路径或 Base64。

**为什么不改队列整体或做全局缓存框架。** 积压的根源是“发出就不撤销”，加上 Rust 端每次请求都打开数据库并写一次。调度器只管封面请求，没有改动队列渲染、虚拟化或动画。

## 隔离测试数据

`src-tauri/src/cover_corpus.rs` 是 `#[cfg(test)]` 加 `#[ignore]` 的生成器，用 `MM_COVER_CORPUS=%TEMP%\mm-cover-corpus` 运行。它复制仓库自带的 `tone.flac` / `tone.mp3`，嵌入程序生成的图片，再扫描进独立的 `app-data`。共 336 首，队列为 336 项：

| 类型 | 数量 | 最大图片 |
|---|---|---|
| jpeg-small（300/500/800 px） | 194 | 384 KB |
| jpeg-3000 | 50 | 5.2 MB |
| png-1000 | 20 | 2.0 MB |
| png-3000-rgba | 8 | 187 KB（解码需要 36 MB） |
| png-noise-13mb | 4 | 12.96 MB |
| webp | 10 | — |
| gif | 5 | — |
| corrupt-jpeg | 5 | — |
| no-cover | 30 | — |
| same-name（不同文件夹的同名文件） | 10 | — |

`MM_COVER_CORPUS_RECOVER=1` 会在同名文件里换掉前 5 首的图片。全程没有读取、扫描或复制用户曲库。

测试运行方式：

- 便携版 exe 复制到 `%TEMP%\mm-cover-corpus\run-<label>`，配一份全新的 `app-data`，用 WebView2 远程调试端口 9333 启动。
- `tests/perf/queue-cover-profile.mjs` 通过 CDP 记录：
  - 封面 IPC 的时间（Network 域）；
  - 每 50 ms 一次可见行状态采样；
  - rAF 帧间隔；
  - 进程 CPU 和工作集；
  - 缓存文件数；
  - 新版的 loader 计数与 Rust 计数。
- CDP 响应体的结果分类（data/null）不可靠：直接调用确认 Rust 对 no-cover 行返回 `null`，CDP 却标成 data。所以逐行的结果判断以 DOM 状态为准。

## 修复前后数据（同一语料，同样的场景脚本）

- 修复前 exe：`cover-before.exe`（当前源码修改前构建），SHA-256 `0D5C100B…D5EF`。
- 修复后 exe：先用 `cover-after.exe`（`F8367558…B856`），之后用最终 ZIP 里的 exe（`251714C0…69F7`）复测，见下一节。两者源码只差 `music.ts` 的空白格式。
- 所有运行的最大帧间隔都是 17–20 ms，超过 50 ms 的帧数均为 0。修复前后都看不出滚动卡顿；这个指标是页面内 rAF，不能代表合成线程。

| 场景 | 修复前 | 修复后 |
|---|---|---|
| A 冷启动首次打开 | 4 次中 3 次 12/12 在 10–32 ms 内显示（队列关闭时预挂载已加载）；1 次（新 exe 首次启动）12/12，最后一张在 4859 ms，窗口内 60 个封面 IPC | 6 次中 5 次（含 ZIP 版）12/12 在 6–8 ms；1 次（新 exe 首次启动）13/13，最后一张在 5576 ms，窗口内 234 个 IPC，在途最多 4 个 |
| B 再次打开 | 12/12，13 ms | 12/12，7–9 ms |
| C 滚到底（150 步 × 400 px，停 4 s） | 3 次：可见 10 张中显示 7、4、4 张；IPC 280–340 个；在途最多 213–278 个；IPC 中位 5985–7367 ms，最长 8903–9642 ms；进程 CPU 6.2–8.3 s | 3 次：10/10、10/10、10/10；IPC 63–272 个；在途最多 7 个；IPC 中位 6–14 ms，最长 123–144 ms；排队最长 150–180 ms；进程 CPU 3.7–5.5 s |
| C 回到顶部 | 12/12，2–10 ms | 12/12，2–3 ms，0 个 IPC |
| D 歌词页打开队列 | 13/13，28 ms | 13/13，6 ms |
| E 连按下一首 10 次 | 12/12（时间含点击 1.5 s） | 12/12 |
| F 浅色、深色切换后打开 | 12/12，封面 IPC 只有 3 个（都是 player_artwork） | 12/12，3 个，没有批量重载 |
| H 251–262 行（png-1000） | 第 1 次 10/12（257、258 有返回但显示占位，原因未查明，第 2 次没有复现）；第 2 次 12/12，249 ms | 12/12，67–115 ms |
| H2 271–282 行（3000 RGBA PNG、13 MB PNG、WebP） | 0/12，全部当成“没有封面” | 12/12 |
| G 重启后首次打开 | 12/12，7 ms，0 个 IPC（磁盘缓存命中） | 12/12，6 ms；ZIP 版 13/13，9 ms |
| 同名文件换图后重启 | 第 0–4 行图片变化，第 5–11 行不变 | 第 0–4 行变化，第 5–11 行不变 |
| 暂时失败（隐藏语料第 150 首后重载页面） | 未测（修复前会写 `.none`，见代码） | 重试 2 次，约 3.5 s 后状态为 failed / `cover-file-missing`，不会一直显示加载中；文件恢复后 500 ms 内显示 |

修复后在 C 场景中，Rust 计数（ZIP 版）如下：

- 封面请求 259 次，其中解码 224 次、原样返回 5 次；
- 确定无图 30 次，损坏 5 次；
- 解码合计 1945 ms，单次最长 12 ms；
- 槽位等待合计 1413 ms。

前端 loader 计数：

- 请求 303 次，实际发出 256 次，取消 63 次；
- 排队最多 30 个，同时运行最多 4 个，排队等待最长 180 ms。

工作集在各场景是 490–690 MB（7 个进程，大部分属于 WebView2），修复前后在同一范围。

**两次首次启动的异常值。** 两个版本各有一次，都出现在新复制的 exe 第一次启动时，而且队列 IPC 落在了打开窗口之内，当时没有记录请求顺序。之后加了顺序记录，连续复测都没有复现。原因未查明，可能与首次启动时的系统扫描导致预挂载推迟有关，但这只是推测。

## 真实 Windows Release EXE

- 旧 ZIP：`AA1629BCCCBFFE0A2637678F0133F936DFBDD09B3D80978AD7368EDE4841A622`（16:53 记录）。
- 新 ZIP：`dist-portable\Music-Mine-Portable-x64.zip`，`9FC7815B91E7C4FA375FFAB86102E08B0186B066B51E7F22112F6E3F23BD8845`，5,278,584 字节。
- 解压到 `%TEMP%\mm-zip-extract`，里面的 exe 哈希 `251714C0…69F7`，与本次构建产物相同。README 来自用户修改后的 `portable/README.txt`。
- 用这个 exe 配新的 `app-data` 跑了 A、B、C、D、E、F、H、H2，以及重启 R，全部 0 个缺失、0 个一直加载、0 个图片 onerror，数值见上表“修复后”一栏。
- 标准版数据 `%APPDATA%\local.musicmine.desktop` 的三个文件最后写入时间是 13:46–13:47，早于本任务，没有变化。

## 检查

| 检查 | 结果 |
|---|---|
| pnpm check | 0 错误，0 警告 |
| pnpm test | 60/60（新增 `tests/v0953-cover-loader.test.mjs` 9 项） |
| pnpm test:ui | 8/8，截图未更新。沙箱里需要设 `PLAYWRIGHT_BROWSERS_PATH=%LOCALAPPDATA%\ms-playwright`，不设时浏览器找不到，8 项全部失败 |
| pnpm build | 通过 |
| cargo fmt --all -- --check | 通过 |
| cargo clippy --all-targets -- -D warnings | 通过 |
| cargo test | lib 72 通过、1 忽略（语料生成器），集成测试 3 + 6 通过；新增 3 个封面测试：结果分类、暂时失败不缓存、旧版本清理与 CoverFront 优先 |
| Windows Tauri 实机 | 便携 Release exe，隔离数据目录，真实 IPC，见上节 |
| pnpm install | NOT RUN（依赖未变） |

## 修改的文件

修改：

- `docs/ipc-contract.md`
- `src-tauri/src/{catalog.rs, catalog_commands.rs, covers.rs, db.rs, lib.rs, metadata.rs}`
- `src/lib/{Artwork.svelte, Cover.svelte, Queue.svelte, QueueCover.svelte, services/music.ts}`

新增：

- `src/lib/utils/{cover-loader.ts, row-visibility.ts}`
- `src-tauri/src/cover_corpus.rs`（只在测试中编译）
- `tests/v0953-cover-loader.test.mjs`
- `tests/perf/{queue-cover-profile.mjs, stage-cover-run.ps1, run-restart-recover.ps1, compare-recover.mjs, transient-cover.mjs, cdp-eval.mjs}`

测量输出在 `tests/perf/out/`，已被 git 忽略。完整 diff 用 `git diff` 查看。

## 未解决和兼容性问题

- **会话内换图。** 文件里的封面在会话中被替换、又没有重新扫描时，页面 loader 按路径缓存，会继续显示旧图，直到重新扫描或重启。Rust 端按 mtime/长度失效，没有这个问题。修复前也是这样。
- **仍显示为“没有封面”的格式。** 超过 512 KB 的 WebP/GIF，以及 BMP、TIFF、AVIF、HEIC。动图 GIF 会动。
- **JPEG 变体。** `image` 解不了的 JPEG（例如算术编码）会归为“损坏”，即使浏览器可能显示得了。
- **合同变化。** `library_queue_thumb` 对不在曲库的路径改为返回错误码 `cover-not-in-library`，原来是 `null`。界面显示一样，都是占位图。
- **升级后首次运行。** 会删除旧格式的磁盘缓存并重新解码一遍。
- **内存。** 两个解码槽同时处理超大图时，峰值临时内存约 160 MB。详情页 720 px 的结果也进入 loader 的 600 条 LRU，原来是 400 条 Promise 缓存。
- **观察窗以外的情况。** 可见行之外、离开预取区的行不会请求封面。队列关闭时，第一屏和预取区仍会在后台加载，与修复前的行为相同。
- **未查明的异常值。** 两个版本各一次的首次启动异常值，见上文。

## 来源

这次整理只补充本文件已经写明的事实，没有补算指纹。

- 任务 ID：09.5.3
- 时间：未记录，无法重建
- HEAD：文件开头写的是 `decbefb4`，未提交
- 工作区指纹：未记录，无法重建
- 工具版本：未记录，无法重建
- 构建配置：文件写了便携 Release exe 和隔离数据目录。配置文件哈希未记录
- 产物哈希：文件里的 ZIP 哈希保留原样，没有重新计算
- 三层结果：见上文检查表。这次没有重跑
