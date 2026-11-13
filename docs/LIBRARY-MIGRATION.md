# 任务 04：真实曲库迁移记录

## 修改与使用

本项目的 Craft 配置是 `app_in_root: true`，应用源码位于根目录的 `src/` 与 `src-tauri/`，没有另建 app/。
从旧工程 `E:/Code/Project/music-mine/app/src/` 只读复用 `db.rs`、`library.rs`、`metadata.rs`、`collections.rs`，保留其表结构、元数据回退、专辑分组及歌单顺序语义。没有修改旧工程、访问真实用户数据库进行实验，也没有修改已验收的 `player.rs` 音频引擎。

新增 `catalog.rs` 和 `catalog_commands.rs`：后台递归扫描 MP3/FLAC/WAV，扫描快照、取消、路径与错误报告、按路径 upsert、分页、搜索、排序、歌曲详情、专辑/歌手分组、内嵌 JPEG/PNG 封面、收藏和歌单命令。文件和标签只读；跳过链接以避免递归循环。取消保留已经导入的歌曲；错误明细最多 100 条。

`src/lib/services/music.ts` 是唯一的前端 IPC 入口。`Library.svelte` / `Cover.svelte` 取代演示数据，保留绿色主题与三栏布局，封面按可见区域延迟读取，歌曲每页 50 条，支持双击播放、单独的详情/播放/入队按钮、空曲库导入引导、创建歌单及添加歌曲。没有浏览器假数据或假 IPC 成功。

`controller.rs` 在现有 Player 外增加 Rust 队列、入队/移除、上下曲和引擎 EOF 驱动的顺序推进。Rust 后台工作线程处理 EOF，界面计时器只查询真实状态，不推动播放。队列快照由 Rust 返回。

## 数据库兼容性与安全

新版使用 Tauri `app_data_dir()/library-tauri-v1.sqlite`，不会直接使用旧版数据库作为可写库。

设置页可选择旧版 SQLite 数据库，仅在新版歌曲、收藏、歌单、文件夹、设置均为空时导入，已有新版数据不会被覆盖。使用只读 SQLite 源连接和 SQLite online backup API 制作 `*.importing.db` 一致性副本；不直接复制数据库主文件，因此包含 WAL 已提交数据。只升级副本，原库可继续由旧版管理，不存在两个应用共同写源库的流程。

副本必须通过 integrity_check、外键检查、歌曲模型读取、迁移前后数据逐项比较后才替换空的新库。核对歌曲 ID/路径/主要标签/时长、收藏、歌单、成员和位置、文件夹、所有设置；不兼容或关联损坏会明确报错并拒绝导入。其他未使用的表和设置也随整库备份保留。旧版缺少的 disc_number、album_source、added_at 使用原有补列迁移；旧 added_at=0 依旧版规则赋予时间，不删除或重建歌曲。

自动化迁移只用了测试创建的数据库，包含保持源连接打开的 WAL 数据库与旧版 V0.2 合成模型。已验证迁移后歌曲数量、ID、收藏、歌单顺序、文件夹与设置保持一致。没有对用户的真实旧库做兼容性验收，不能将合成模型测试等同于真实用户库验收。

## 验证

- Svelte 类型检查：0 errors / 0 warnings。
- `pnpm build`：通过（使用本机可用 pnpm，参数见下）。
- `cargo fmt`、`cargo check --offline`、`cargo clippy --offline --all-targets -- -D warnings`：通过。
- `cargo test --offline -- --test-threads=1`：36 项通过（33 单元、3 音频集成）。覆盖真实 MP3/FLAC/WAV 扫描/元数据与设备播放、递归、损坏文件报告、音乐字节不变、去重、取消、字面量搜索转义、分页、V0.2 补列、WAL 一致性备份、收藏/歌单 CRUD 与顺序，以及队列控制复用原引擎。
- `cargo build --offline --features custom-protocol`：生成内嵌生产前端的 Windows EXE；未以常驻开发服务器做桌面启动测试。
- `rebuild_check`：build passed；工具没有 JS test 脚本可运行，不能代替上面的 Rust 测试。
- `pnpm --pm-on-fail=ignore --config.verify-deps-before-run=false install --offline --frozen-lockfile --ignore-scripts`：被 `ERR_PNPM_ABORTED_REMOVE_MODULES_DIR_NO_TTY` 中止。现有依赖上的 check/build 可用，不能宣称干净安装通过；保持声明的 packageManager 与锁文件。
- Cargo 1.99 仍报告已有 bin/lib 同名 PDB 输出警告，不影响构建和严格 Clippy。

Computer Use 尝试启动构建后的 `src-tauri/target/debug/music-mine-desktop.exe`，明确返回 `Computer Use was not approved to use music-mine-desktop`。没有通过其他启动/自动化方式绕过该拒绝。因此本轮尚未验证实际可见窗口、原生文件夹选择器、端到端 IPC、双击播放和可听输出；任务 03 用户此前人工验收不替代新增曲库流程验收。

## 尚未迁移及下一步

- 旧版 `settings.session_v2` 原样保留，导入报告会明确提示：旧队列、上次曲目/进度、随机/循环及视图设置尚未自动恢复。当前 Rust 队列在会话内工作，重启后的队列/播放记录恢复仍待迁移。方案是从副本解析版本化 session_v2，以路径解析现有歌曲，逐项报告缺失文件与不支持字段，转换为新会话模型后暂停加载，验证后再启用恢复，不自动发声。
- 歌单顺序已保持并按原位置显示；完整的重命名、删除、移除歌曲及拖动排序界面尚待接入，旧版对应 Rust 实现与测试已复用。
- 暂未增加文件监控增量扫描、FTS 索引、封面磁盘缓存与超大曲库分组缓存；当前为 SQL 参数化搜索和分页、分组按需读取，封面限制 8 MiB。
- 真实用户库的导入验收及 Windows UI/IPC 需要在允许访问该应用的桌面环境中执行。先在空的新库上导入旧库副本，核对导入报告、收藏及歌单顺序；或者导入一个已知文件夹，核对 MP3/FLAC/WAV 标签与封面，再双击歌曲验证暂停/恢复、拖动进度、上下曲、扫描取消和错误显示。

本轮没有加入 AI、语音、EQ、在线音乐、云同步或新的音频解码能力。
