# Music Mine — RFC v2.0 迁移任务 01 审查

日期：2026-11-13。状态：仅完成现有架构审查，尚未初始化 Tauri 工程。

## 边界与基线

当前写权限仅覆盖 `E:\Code\Project\music-mine`，无法创建指定的同级目录
`E:\Code\Project\music-mine-tauri`。依据任务“无权创建时先报告，不擅自修改原工程结构”，本次只新增此审查文档，不在旧工程内另建替代迁移目录。
未找到 RFC v2.0 正文；以下依据任务中列出的要求和实际代码，不声称已经符合未提供的 RFC 细节。

旧版检查点为 `208225f`，已有 `v0.5.2` 标签；未移动标签或覆盖提交。
工作区原有 Cargo.toml/Cargo.lock 修改及未跟踪的 src/ai、src/commands.rs、截图和验证文档均保留。
原库未打开或迁移，471 首记录的数量与收藏、歌单内容本次未重新核验。

## 模块与目标架构对应

| 现有模块 | 可复用程度 | 目标职责与必要处理 |
| --- | --- | --- |
| player.rs + logging.rs | 无 egui 类型依赖，复用解码、重采样和输出算法 | 后端唯一 Player；包装状态 DTO、错误、命令生命周期，不直接整库搬入 |
| library.rs | Track、分组、排序、扫描可复用 | 后端曲库；扫描放工作线程，保留 album_source 与分组规则 |
| metadata.rs | lofty 读取逻辑可复用 | 只读音频标签，封面字节通过受控接口提供 |
| db.rs | SQL 与映射可复用，但 open 有写入副作用 | 新库连接与副本策略独立；不能用 open 读取原库统计 |
| collections.rs | SQL 事务可复用 | 下一阶段保留 ID、外键和歌单 position，本轮不开放编辑 |
| session.rs | 序列化、解析、路径恢复可复用 | 保留 settings.session_v2 全部字段；启动 prepare 后暂停 |
| import.rs | 分类、合并队列算法可复用 | 后端队列操作；本轮不迁移拖放导入 |
| lyrics.rs | 解析、解码、时间定位无 UI 依赖 | 后续复用；本轮不迁歌词页面 |
| selection.rs / shortcuts.rs | 纯规则可复用 | 后续适配浏览器事件与输入焦点 |
| app.rs | 业务与 egui 混合，不能直接迁入 | 抽出队列、当前曲目、循环随机、结束事件、会话保存为 Rust 控制器 |
| lyrics_ui.rs / theme.rs | egui 专用 | 后续以 Svelte/CSS 重建展示；不迁 TextureHandle |
| media_keys.rs | Windows 通道实现可参考 | 后续统一进入控制器，避免旧新版争抢热键 |
| config.rs / main.rs / lib.rs | 入口和路径需调整 | 新 app-data 路径独立；旧启动参数及 screenshot 能力继续保留 |

证据：player.rs 的 Player 持有 mpsc Receiver，status 返回 Mutex 快照；app.rs 的
start_index/play_visible/advance/toggle_play/save_session 维护队列和会话，
update 路径消费 Finished/Failed。虽然音频算法已独立，完整播放业务仍在 UI 内。
lib.rs 无条件引入 app/theme/lyrics_ui，直接依赖旧 crate 会同时引入 egui/eframe；
应在独立工作目录提取最小核心 crate 或加入经过测试的 UI feature 边界。

## 首条链路与接口设计（待实现）

前端统一 `src/lib/services/music.ts` 调用 invoke；组件使用 Svelte 5 Runes，
只保存显示快照、加载和错误状态。禁止本地定时器推进播放位置或维护另一套队列。

| Command | 输入 | 结果与约束 |
| --- | --- | --- |
| library_stats | 无 | 实际曲目、收藏、歌单数量；专辑数量使用既有分组算法 |
| list_tracks | 分页参数 | 真实 Track DTO；稳定排序与明确错误 |
| player_status | 无 | state、track_id、position_ms、duration_ms、volume、output、error |
| play_track | track_id | 后端查库得到路径，进入唯一控制器；不接受前端任意文件路径 |
| pause / resume | 无 | 发送既有 Player 命令，随后查询真实状态；调用成功不等于设备已播放 |

Player 的 Receiver 不是 Sync，不能未经验证直接放入 Tauri 共享状态。
建议后端控制器线程拥有 Player 和数据库连接，Command 通过通道请求控制器；
同一线程持续消费播放事件，处理 Finished/Failed，不依赖 UI 轮询推进队列。
状态输出用毫秒 DTO，不暴露 Duration。生命周期需验证窗口关闭后音频线程退出。
数据库/扫描任务不进入音频回调；保持当前设备交付帧计数作为唯一进度来源。

## 数据兼容与回滚方案（待实施验证）

1. 原数据库路径来自 MUSIC_MINE_DATA_DIR 或用户 roaming data/music-mine/library.db，不能猜测当前账户就是旧版用户。
2. 原连接以 SQLite read-only 打开，不调用 db::open：该函数会建表、ALTER TABLE，并更新 added_at。
3. 使用 SQLite 一致性备份机制产生新应用独立目录内的副本；有 WAL 时不能只复制主文件。备份目的路径必须与原路径不同且不存在，不覆盖旧副本。
4. 备份前后核对 integrity_check、foreign_key_check、tracks 完整记录与 ID/path、folders、favorites、playlists、playlist_tracks（含顺序）及 settings 的逐项值。471 是预期基线，实际不符应报告，不补造数据。
5. 保留不可变的迁移前副本，仅对另一份工作副本执行旧 init。现有 init 多条 DDL 不在统一事务内，不能假设整体原子；未来迁移加事务及故障回滚测试，不对原库试验。
6. session_v2 保留路径顺序、current_index、position_ms、volume、repeat、shuffle、view、selected_album、filter、sort、sort_ascending、playlist_id。缺失音频仅在恢复视图中报告并跳过，不改音频或备份。
7. 不迁移收藏/歌单/歌词 UI 不代表丢弃其数据。保留全部表与设置；旁置 LRC、内嵌歌词保持只读。回滚为关闭新版、弃用工作副本并继续运行旧版，原库无需降级。

现有迁移测试只证明单条 V0.2 记录和 session 值升级保留，不足以证明真实 471 首及全部关联数据安全；须补充真实副本前后比较与重复迁移/注入失败测试。

## 最小目标目录（方案，未创建）

```text
music-mine-tauri/
  package.json / pnpm-lock.yaml / vite.config.ts / tsconfig.json
  index.html
  src/App.svelte / src/app.css / src/main.ts
  src/lib/services/music.ts
  src-tauri/Cargo.toml / build.rs / tauri.conf.json
  src-tauri/capabilities/default.json
  src-tauri/src/main.rs / lib.rs / commands.rs / controller.rs
  crates/music-core/Cargo.toml
  crates/music-core/src/{lib,player,logging,library,metadata,db,session}.rs
```

初始化使用 Tauri v2、Svelte 5、TypeScript、Vite、pnpm、Tailwind CSS、CSS Variables、lucide-svelte；
只实现左导航、歌曲列表和底栏，不创建未使用模块。版本与配置在真正初始化时核对官方资料并锁定。
首要技术验证：WebView2 可用性、Windows 工具链和离线依赖、控制器 Send/Sync、状态错误传播、真实设备播放暂停进度、复制库隔离和迁移回滚。

## 本轮验证

- `cargo build --offline`：成功。
- `cargo test --offline`：79 项通过（64 单元、2 引擎、3 导入、6 歌词、4 真实文件）；既有示例 dead_code 与歌词测试 unused_import 警告仍存在。
- 测试覆盖真实输出设备交付进度和 MP3/FLAC/WAV 非静音解码；不等同人工听音。
- Tauri Rust 编译、Svelte 类型检查/构建、新窗口启动、Rust ↔ TS 通信与真实副本统计：未实现、未验证。
- `rebuild_check`：build passed、app-screenshot passed（1280×800），结果为 “This would pass”；旧版检查通过不能视为迁移成功。

下一批：获得 RFC 正文与可写独立工作目录后，先建立安全快照与曲库统计读取，再提取核心/后端控制器并运行独立测试，最后接入最小 Svelte 页面及真实播放暂停进度。收藏、歌单、歌词和媒体键留待后续批次。
