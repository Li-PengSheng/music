# 09.5.5 RC 前封面竞态修复与构建冻结

分支 `main`，HEAD `decbefb4f7eaf4a23210793264b4cddfa36adb9c`。未自动提交或推送。

## A. 工作区

- 保留未提交的 `portable/README.txt`，本轮未改。
- 09.5.4 格式化漂移已用 `git checkout HEAD --` 恢复（含 NowPlaying / PlayerBar / TrackList / Modal / ContextMenu / Dialog / EmptyState / IconButton / Slider / tooltip.ts / history.ts）。备份在 `tests/perf/out/format-backup-0955/`。
- 未提交产物与用户数据不在提交范围：`build-activity.log`、`capture/`、`inbox/`、`usage.json`、ZIP、测试曲库。

## B. 封面 generation 竞态

**位置：** `src/lib/utils/cover-loader.ts`

**原则：**

1. `clear()` 只抬高 `generation` 并清空结果缓存；进行中的 `running` job 从 `jobs` 摘掉，但**仍计入 concurrency 4**，直到结束。
2. 旧 generation 成功：只通知该 job 自己的 waiters，**不** `remember` 进新缓存。
3. 旧 generation 失败：`handOver` 把 waiters 交给当前 generation 的同 key job（或新建），避免永久挂起。
4. 排队 / 重试中的 job 在 `clear()` 后把 `generation` 抬到当前值，继续可被调度。
5. 优先级 0/1/2 与上限 4 不变；旧请求写回不会污染新缓存。

**测试：** `tests/v0955-cover-generation.test.mjs`（8）+ 更新后的 `tests/v0953-cover-loader.test.mjs` → loader **17/17 PASS**。无固定延时掩盖。

## C. 库外文件封面

- 隔离文件已备：`%TEMP%\mm-rc-extra\`（`outside-tone.wav` / `outside-cover.flac` / `outside-tone.lrc`）。
- **通过现有 UI 打开（native `player_open_file`）：NOT RUN**（本轮改为 CDP-only，避免审批抢焦点误入真实曲库路径）。
- 库内 FLAC 的 `player_artwork` 实机：返回 `data:image`（约 89 KB）PASS。
- 读错误不永久缓存：沿用 09.5.3 单元 / Rust 路径；本轮 UI 复现 NOT RUN（无生产后门）。

## D. 格式化 diff

已清掉可证明为纯格式化的文件；其余为 09.5.3/09.5.5 功能改动。

## E. 自动检查（本树重跑，不复用旧结果）

| 检查 | 结果 |
|---|---|
| `pnpm check` | PASS |
| `pnpm test` | PASS 68/68 |
| `pnpm test:ui` | PASS 8/8 |
| `pnpm build` | PASS |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --all-targets -- -D warnings` | PASS |
| `cargo test` | PASS（lib 72 + 1 ignored；integration 3 + 6） |

浏览器 Playwright harness ≠ 桌面 exe 验收。

## F. Release ZIP

- 脚本：`scripts\build-portable.ps1`（Release + `tauri.portable.conf.json`，`--no-bundle`）。
- ZIP `dist-portable\Music-Mine-Portable-x64.zip`  
  SHA-256 `AB70F76F2EBB8072ACAFD2EEEA946A99C8097D2A78DEE677C33E9BBF705B088E`（5,277,769 B）
- 内含 / 构建 exe 同哈希  
  `1A77E81331DB7452D26873ED6D17BE319E5A0F5DE63B2A5BF2A1C6E74F81DB57`（16,090,624 B）
- 与 09.5.3 ZIP / 更早 ZIP 哈希均不同；未与旧包混用。

## G. 隔离 Windows 实机（ZIP 解出的 exe，`run-rc`，corpus 336）

| # | 项 | 结果 |
|---|---|---|
| 1 | MP3 / FLAC 播放 | PASS（Rust 快照 `playing`，position 前进；WAV 打开需 picker → **NOT RUN**） |
| 2 | 双语歌词 | PASS（IPC 含 U+2009；DOM `.lyric-secondary` ×3：第一行/第二行/第三行） |
| 3 | 336 队列滚动 | PASS（30 页 blankPages=0） |
| 4 | 冷/热首屏封面 | PASS（先前 profile A/B 12/12；本轮重启后顶栏 12/12） |
| 5 | 滚到底再回顶 | PASS |
| 6 | 队列快关快开 | PASS（先前 Q；本轮 reopen 12/12） |
| 7 | 扫描中/后缓存失效 | PASS（扫 corpus；扫后 walk 0 blank；maxRunning≤4；观测窗口内 stale/handed 常为 0——旧请求多在 clear 前结束；单测覆盖写回禁令） |
| 8 | 同名不同封面 | PASS（corpus `same-name` 行在滚动中显示） |
| 9 | 主题切换 | PASS（`desktop_preferences` dark→light→dark） |
| 10 | 减少动态效果 | PASS（开关切换，`.app.reduce-motion` 同步） |
| 11 | 托盘隐藏/恢复/退出 | **NOT RUN**（CDP-only） |
| 12 | 库外文件封面 | **NOT RUN**（需 native open-file） |
| 13 | 标准数据 SHA-256 | PASS 未变（见下） |

**标准数据** `%APPDATA%\local.musicmine.desktop`（回归前后一致）：

- `desktop-v1.json` `597E7C43728D6A09D8E160A4D6EB20285D1C5C339B1C2384E4E2F4BAC3F68314`
- `library-tauri-v1.sqlite` `7B615BEABE5809ECB5AF36DE6CD16AC62960EA4BCAD4AD7473661F53BF42DBE3`
- `playback-v1.json` `AEAABA6C89B046D50B2CD5278391BC2C9B9BFF781FE2F646B44F2269EC2C65AE`

**度量摘要：** 首屏应有封面均显示；无持久空白；`maxRunning` 峰值 4；清空后旧成功不进新缓存（单测）；重试有界。

## H. 剩余问题

| 级别 | 项 |
|---|---|
| P0 | 无 |
| P1 | 无（库外打开 / 托盘未在本轮实机点过，属验收缺口而非已知缺陷） |
| P2 | 扫描中 `stale` 实机计数常为 0：说明该 corpus 上 clear 时少有仍在飞的旧请求；竞态正确性依赖单测 |
| P3 | 验收脚本与 `tests/perf/out/*` 证据文件勿提交 |

## 建议提交范围与说明

**纳入：**

- `src/lib/utils/cover-loader.ts`（新）
- `src/lib/utils/row-visibility.ts`（新，09.5.3）
- `src/lib/{Artwork,Cover,Queue,QueueCover}.svelte`、`src/lib/services/music.ts`
- `src-tauri/src/{catalog,catalog_commands,covers,db,lib,metadata}.rs`、`cover_corpus.rs`
- `docs/ipc-contract.md`、`docs/TASK-09.5.3-QUEUE-COVER.md`、`docs/TASK-09.5.5-RC-FREEZE.md`
- `tests/v0953-cover-loader.test.mjs`、`tests/v0955-cover-generation.test.mjs`
- 可选：`tests/perf/*.mjs` / `stage-cover-run.ps1`（验收工具）

**不要纳入：** `portable/README.txt`（用户保留）、`build-activity.log`、`capture/`、`inbox/`、`usage.json`、ZIP、曲库与 `tests/perf/out/`。

建议信息：

```
fix(covers): generation-aware loader clear so stale results cannot poison cache

Keep in-flight pre-clear loads inside the concurrency cap, hand failed waiters
to the current generation, and freeze the portable RC build hashes.
```

## RC 技术冻结评估

**有条件满足：** 竞态修复 + 全量自动检查 + 新 ZIP 哈希已固定 + 隔离实机核心路径（播放 / 封面 / 扫描失效 / 主题 / 减动效 / 双语歌词 / 标准数据未动）已过。

**未满足实机项：** 库外 open-file 封面、WAV 经 UI 打开、托盘三态——需一次人工点选后即可宣称完全冻结。

完成后停止，等待授权提交。

## 来源

这次整理只补充本文件已经写明的事实，没有补算指纹。

- 任务 ID：09.5.5
- 时间：未记录，无法重建
- HEAD：文件开头写的是 `decbefb4f7eaf4a23210793264b4cddfa36adb9c`。未自动提交或推送
- 工作区指纹：未记录，无法重建
- 工具版本：未记录，无法重建
- 构建配置：文件写了 Release 便携配置 `tauri.portable.conf.json`。配置文件哈希未记录
- 产物哈希：文件里的 ZIP 和 exe 哈希保留原样，没有重新计算
- 三层结果：见上文检查表。库外打开、WAV 经 UI 打开、托盘三态仍是未验证。这次没有重跑
