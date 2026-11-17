# Rust、音频与数据库约定

适用于 `src-tauri/`。全项目安全规则在根目录 `AGENTS.md`。本页规则来自代码边界，这次文档整理没有重审音频回调的每一行。

## 模块

- `lib.rs`：Tauri 启动、命令、应用数据目录、播放控制线程。
- `audio/player.rs`：Symphonia 解码、cpal 输出、音频线程。
- `playback/controller.rs`：播放控制、队列、会话文件。
- `platform/media.rs`：Windows 媒体集成。`platform/desktop.rs`：窗口、主题、托盘、桌面歌词窗口、`desktop-v1.json`。`platform/thumbbar.rs`：Windows 任务栏缩略图按钮。`ITaskbarList3` 只留在主线程；按钮点击经 `spawn_blocking` 进入控制器，不进音频回调。
- `catalog/library.rs`、`catalog/mod.rs`、`commands/catalog.rs`、`catalog/collections.rs`：曲库、扫描、歌单。
- `catalog/db.rs`：SQLite 连接、`CREATE TABLE IF NOT EXISTS`、`ensure_column`。
- `playback/lyrics.rs`、`catalog/metadata.rs`：歌词和标签。`artwork/covers.rs`：封面缩略图、内存和磁盘缓存。`infrastructure/logging.rs`：日志。

没有按 RFC 假设尚未存在的模块或 crate。

Catalog 每实例首次连接完成 schema 初始化；后续轻量连接不执行 DDL，每个连接仍设置 `foreign_keys=ON` 与 2 秒 `busy_timeout`。TASK-23 已删除旧版数据库导入及备份路径；当前库的 schema 升级与数据保留测试继续保留。

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
- SMTC 封面在后台媒体更新中复用 Store 的 320px JPEG 输出，按完整播放路径缓存 WinRT 引用（含无封面/失败结果）；暂停/继续只复用引用，停止时清空。封面失败不跳过文本 Update，不在音频回调或 Controller 锁内读封面或创建流。GIF/WebP 透传因尺寸未解码约束而不送入 SMTC。
- 断音、暂停、切歌、Seek、EOF、播放模式、设备拔插、休眠恢复，有对应测试；真实设备不可用时写 `NOT RUN`。
- 格式只认已启用的 Symphonia feature：`mp3`、`flac`、`wav`、`pcm`。现有 fixture 是 `tone.mp3`、`tone.flac`、`tone.wav`。
- UI 任务不重写播放控制器。

## 错误、权限与依赖

- Windows 的 `WebviewWindowBuilder::build()` 不得在同步 IPC 或窗口/托盘事件回调里执行。桌面歌词命令通过异步 IPC + `spawn_blocking` 创建窗口，以独立操作锁串行化创建；事件回调也进入该异步路径。浏览器 mock 无法验证这个约束，需在隔离 exe 中检查打开后主窗口 IPC 和歌词仍更新。

- 可预期的文件、设备和数据库错误不用 `unwrap` 或 `expect` 吞掉。
- 现有命令返回 `Result<T, String>`。不为了统一风格改全部签名。
- 主窗口 `capabilities/default.json` 除 `core:default`、`allow-set-theme` 外，TASK-14 授权加入窗口 minimize / toggle-maximize / close / start-dragging 四项权限；不授予文件系统或 shell 权限。桌面歌词窗口使用单独的 `capabilities/lyrics-overlay.json`，只允许 `core:window:allow-start-dragging`。路径和数值在 Rust 侧校验。
- 新增 crate 前说明现有依赖为何不够、Windows 目标、feature、构建成本和替代方案。依赖变更带上 `Cargo.lock`。任务栏按钮复用已有 `windows` 0.58，不升级版本，不新增 crate。补充的 feature 是 `Win32_UI_Shell`、`Win32_UI_WindowsAndMessaging`、`Win32_Graphics_Gdi`，以及 `Win32_System_Com`（`CoCreateInstance` 不在 Shell feature 中）。
- Rust 改动按范围运行 `cargo fmt --check`、`cargo clippy --all-targets -- -D warnings` 和相关 `cargo test`。没跑就写 `NOT RUN`。
