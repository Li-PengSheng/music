# Task 07.1 — 首页与音乐库

Git 检查点：11b18f3，位于 `.checkpoints/task061/.git`，工作树为项目根目录。项目 `craft.json` 设置 `app_in_root: true`，沿用现有根目录应用，没有新建重复的 app/ 工程。

## 修改文件

- `src/App.svelte`：默认音乐空间首页、导航入口、清除搜索、首页搜索转入完整音乐库。
- `src/lib/Home.svelte`：真实最近播放歌曲与专辑信息、最近添加、收藏入口、常听艺术家、歌单入口、继续播放以及空/加载/失败状态。
- `src/lib/Library.svelte`：保留三视图；歌曲每页 50 首，专辑/艺术家每页 36 项；歌曲选中、双击或 Enter 播放、右键操作、独立收藏按钮、当前曲目标记；分组名称/数量排序、查询防抖与请求过期保护。
- `src/lib/Cover.svelte`、`src/lib/services/music.ts`：集中 IPC，按需加载、异步解码、最多 100 项封面 Promise 缓存（失败可重试，导入时清除）。
- `src/app.css`：深绿品牌首页、紧凑内容卡片、响应式网格、焦点与当前歌曲样式。
- `src-tauri/src/db.rs`、`catalog.rs`、`catalog_commands.rs`、`lib.rs`：仅增加首页所需最小播放记录表和查询；现有后台状态观察在确认 playing 且路径变化时记录，在音频锁释放之后写入。暂停/恢复不重复累计；不代表完整聆听次数，同曲连续循环和 200ms 内极短播放可能无法统计。没有历史回填。
- `src-tauri/src/catalog_tests.rs`：新增空历史、未知路径、重复记录及持久化测试，使用临时测试库与工程音频夹具。

## 验证

- pnpm test：7 项通过。
- pnpm exec svelte-check：0 错误，0 警告。
- pnpm build：通过；JS 约 99.17 kB（gzip 36.68 kB），CSS 33.92 kB（gzip 8.73 kB）。
- cargo test --offline：62 项通过（53 单元、3 音频、6 歌词）。
- cargo check --offline、cargo fmt、cargo clippy --offline -- -D warnings：通过。
- rebuild_check：前端 build 与 own-tests 通过。
- 使用现有 pnpm 兼容参数 `--pm-on-fail=ignore --config.verify-deps-before-run=false`，未重新安装依赖。
- 默认 EXE 输出被运行中的旧进程锁住。使用 `cargo rustc --offline --manifest-path src-tauri/Cargo.toml --features custom-protocol --bin music-mine-desktop -- -o target/debug/music-mine-task071.exe` 构建独立嵌入资源的程序，产物位于 `src-tauri/target/debug/`。Cargo 仍有原有 bin/lib PDB 同名告警。

## 性能与人工验收

静态资源体积与渲染上限已确认：歌曲 50 行，封面网格 36 项，首页最近播放最多 12 首、最近添加 8 首。摘要保留在 Library 组件中，普通页面导航不重新查询完整摘要；首页仅查询有上限的数据。封面仅在接近视口时请求。尚未测量真实曲库的帧率、内存、解码耗时或数据库延迟，不能将结构性优化视为性能实测通过。

Computer Use 找到已运行的 Music Mine，但访问窗口被自动审批拒绝：`Computer Use was not approved to use music-mine-desktop`。没有绕过拒绝，没有用开发服务器替代桌面验收。因此仍需人工启动新构建 EXE 验证以下事项：

- 首页真实 SQLite 数据、首次使用/空库/无历史、导入后首页更新、播放记录与常听艺术家积累。
- 471 首歌曲、417 张专辑、301 位艺术家的实际数量及封面；歌曲/专辑/艺术家切换，详情进入与返回。
- 搜索、清除、名称/数量排序、分页、长标题提示、键盘焦点及 Enter 播放。
- 单击选中、双击播放、右键菜单、收藏成功/失败、歌单入口及添加歌曲。
- 播放期间切换页面，确认进度、音量、上一首/下一首、队列、歌词与已有托盘/SMTC行为。
- 导入文件夹选择、取消、扫描状态、失败反馈；不得用真实库做迁移实验。
- 1280×800、1366×768、1920×1080；Windows 100%/125%/150% 缩放，检查横向溢出、滚动流畅度、封面请求数量、帧率与内存。

本轮没有完成真实桌面窗口与 IPC 验收，不宣称基础工程或本轮全部人工验收通过；未开发 07.2/07.3。
