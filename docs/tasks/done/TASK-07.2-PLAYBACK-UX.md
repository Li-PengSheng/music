# TASK 07.2 — 正在播放与歌曲详情 UI/UX

## 范围与检查点

- 开发前 Git 检查点：`57d6e5c`，Git 目录沿用 `.checkpoints/task061/.git`，工作树为本项目。
- Craft 配置为 `app_in_root: true`，本项目根目录就是应用目录，没有创建重复的 `app/`。
- 遵循 `docs/RFC-v2.0.md`，保留深绿主题与现有业务；未开发 07.3。
- 没有修改数据库结构、真实音乐/歌词文件、旧项目或 Rust 音频引擎。未对真实音乐库执行迁移实验。

## 修改文件与行为

| 文件 | 修改 |
| --- | --- |
| `src/App.svelte` | 播放栏读取真实 `player_artwork`；封面与歌曲信息可打开正在播放；标题截断并提供完整标题提示；音量拖动显示百分比，提交后以 Rust 返回状态为准；进度取消拖动/失焦时清理预览；歌单名称校验、保存状态和就地错误反馈。 |
| `src/lib/Artwork.svelte` | 统一真实封面、缺失封面和图片失败占位，供播放栏、正在播放、详情与曲库复用。 |
| `src/lib/Cover.svelte` | 保留按需加载和封面缓存，改用统一封面组件。 |
| `src/lib/NowPlaying.svelte` | 三种布局明确为封面与歌词、纯歌词、大封面；统一封面、标题提示、未知专辑回退及歌词空状态布局。保留已有 LRC 解析、双语独立行、Rust 进度高亮、歌词真实 seek、手动滚动和恢复自动跟随。 |
| `src/lib/TrackDetails.svelte` | 独立歌曲信息组件，原生模态 dialog 与焦点恢复、Escape 关闭；歌曲信息与音频信息分区；路径省略、完整提示、只读选择和复制；播放、收藏、加入队列及歌单使用现有 Service；异步操作反馈、错误、禁用状态与小窗内部滚动。 |
| `src/lib/Library.svelte` | 接入统一详情；搜索无结果文案；设置快捷键改为紧凑的键位/说明双列表格；移除旧详情实现。 |
| `src/lib/services/music.ts` | 集中新增只读 `library_audio_info` IPC 与类型，组件没有直接 invoke。 |
| `src/app.css` | 固定播放栏与时间/音量排版、歌词强调和空状态、小窗上下布局、模态最大尺寸、侧栏滚动、统一间距/圆角/焦点反馈。 |
| `src-tauri/src/catalog_commands.rs` | 仅新增按曲库 ID 在后台只读调用既有 lofty 的音频信息接口；实际读取格式、采样率、声道与音频码率，未知值不展示，无默认参数猜测；新增测试验证 WAV 头属性、失败路径和读取后文件字节不变。 |
| `src-tauri/src/lib.rs` | 注册只读音频信息命令。 |

播放按钮、队列、模式、进度、音量仍使用现有 Rust 命令与状态。前端只保留拖动期间的控件预览，没有 UI 定时器推进进度或模拟播放成功。现有 Windows SMTC、托盘、快捷键和窗口状态恢复未改写。

## 自动验证结果

- `pnpm test`：7 项通过，包含双语歌词、时间分组、滚动边界和快捷键。
- `pnpm exec svelte-check`：0 错误、0 警告。
- `pnpm build`：通过，最终 JS 105.41 kB（gzip 38.77 kB），CSS 41.15 kB（gzip 10.19 kB）。
- `cargo test --offline --manifest-path src-tauri/Cargo.toml`：63 项通过（54 单元、3 音频集成、6 歌词集成）。音频集成测试使用工程夹具与真实输出设备，不等同于桌面 IPC 验收。
- `cargo check --offline --manifest-path src-tauri/Cargo.toml`：通过。
- `cargo fmt --manifest-path src-tauri/Cargo.toml --check`：通过。
- `cargo clippy --offline --manifest-path src-tauri/Cargo.toml -- -D warnings`：通过。
- `git diff --check`：通过。
- 最终 `rebuild_check`：build 与 own-tests 均通过。
- pnpm 使用现有环境兼容参数 `--pm-on-fail=ignore --config.verify-deps-before-run=false`，复用已安装依赖，未增加依赖或 UI 框架。本轮没有重新运行 pnpm install，不宣称重新完成基础工程全部验证。
- 初次 Rust 编译缺少 `TaggedFileExt` 导入，已修复；最终检查通过。
- 构建嵌入静态资源的 Windows 程序：`cargo rustc --offline --manifest-path src-tauri/Cargo.toml --features custom-protocol --bin music-mine-desktop -- -o target/debug/music-mine-task072.exe`；产物 `src-tauri/target/debug/music-mine-task072.exe`。保留正在运行的默认 EXE，不覆盖或终止其进程。Cargo 仍报告原有 bin/lib PDB 同名告警，以及指定独立输出文件名产生的输出参数告警。

## 桌面验收限制与人工清单

Computer Use 能列出现有 Music Mine 窗口，但读取窗口时自动审批拒绝：`Computer Use was not approved to use music-mine-desktop`。未绕过该拒绝，也没有使用开发服务器替代真实 Tauri 验收。最终新 EXE 已构建，尚未完成其窗口启动、IPC 与交互验收。

请在方便时保存当前播放状态并退出旧实例，再启动上述新 EXE，确认：

- 真实 471 首歌曲、收藏、歌单、歌词与队列完整；正常播放、暂停、停止、上一首/下一首和播放模式。
- 有/无封面歌曲在播放栏、正在播放、详情中的封面一致；快速切歌不显示旧封面；长标题截断且悬停可见全名。
- 播放栏固定，点击封面/歌曲信息进入正在播放；点击/拖动进度后跳转准确，音量数值和听感一致；切页后播放不中断。
- 三种正在播放布局、双语及多语混排、高亮、手动滚动后恢复跟随、歌词点击跳转、无歌词空状态；仅歌词区域滚动。
- 详情封面、元数据和实际音频参数；参数未知/文件失效时反馈；长路径省略与复制；播放、收藏、队列和添加歌单；Escape、Tab 焦点约束及关闭后焦点恢复。
- 搜索无结果文案，侧栏创建歌单的空名称/失败/成功反馈，设置快捷键表格。
- 1280×800、1366×768、1920×1080，Windows 100%/125%/150%：无横向滚动，弹窗不越界，控件可达，播放栏不遮挡内容，歌词上下布局合理。响应式样式已经实现，但这些组合尚未实机验证。
- 现有 Ctrl+F、Space、Ctrl+方向键、Ctrl+L、Ctrl+O、Escape、SMTC、托盘和窗口状态恢复回归。

本轮交付实现与自动检查，不将上述受限的桌面项目标记为已验收。
