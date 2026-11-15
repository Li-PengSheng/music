# 任务 05：歌词与完整播放体验

## 实现

- 项目沿用根目录应用（craft.json 的 app_in_root=true），没有创建重复 app/。
- 修改前 Git 检查点：33d031c，位于 `.checkpoints/task05/.git`。主仓库 `.git` 写入被环境权限拒绝，所以使用项目内独立 Git 仓库保存可恢复的源码检查点。
- Rust Controller 统一管理完整队列、拖动排序/键盘上移下移、选曲、下一首插入、末尾添加与移除。Svelte 仅显示服务返回的版本化快照，没有第二套真实队列。
- 顺序、随机、单曲循环、列表循环；EOF 由 Rust 200ms 后台任务消费真实引擎事件。单曲循环只作用于自然结束，手动下一首仍跳至下一曲。
- 独立 `app_data/playback-v1.json` 保存队列、当前歌曲、进度、音量、模式、歌词偏移；临时文件替换写入，播放期间每两秒保存变化，操作后及窗口关闭时保存。异常退出可能丢失最近两秒进度。重启 prepare 为暂停；丢失的当前音乐文件不自动播放，队列保留。
- 复用旧版 `app/src/lyrics.rs` 与 `app/tests/lyrics.rs`，读取同名 LRC 和内嵌 Lyrics 标签，支持多时间戳、LRC offset、纯文本、UTF-8/UTF-16、GBK/GB18030、Shift_JIS、Big5 自动识别。自动识别对极短的歧义文本不能保证完全正确。
- 修正旧模块多点文件名 LRC 查找、无 BOM UTF-16 检测顺序和极大时间戳溢出；普通 comment 标签不再被当作歌词。
- 歌词及封面通过 spawn_blocking 读取，仅允许读取当前引擎选中的歌曲，IPC 集中在 music.ts。
- 正在播放页面：大封面、歌曲/歌手/专辑、同步歌词高亮、自动跟随/手动浏览、点击 Seek、持久化 ±60秒偏移（正数延后），封面/歌词/双栏切换，空状态，保留底部控制；Escape/返回按钮关闭覆盖层，原曲库组件和滚动位置保留。
- 移除曲库大横幅；歌曲/艺术家/专辑/时长分列，次要操作收进菜单，队列滚动与窄窗口底栏布局修正。

## 数据保护

没有改动数据库结构、旧版数据库或用户音乐文件；没有为实验打开真实 471 首曲库数据库。player.rs 音频引擎未修改，无 AI 依赖。测试仅使用生成数据及测试音频的临时副本；内嵌标签写入只发生于临时测试副本。旧版源码可访问且只读复用。

## 验证结果（2026-11-13）

- cargo test --offline：56 项通过（47 单元 + 3 音频集成 + 6 歌词集成），含真实设备解码播放/暂停/Seek/切歌，四模式 EOF、队列排序插入、Windows 重复会话替换及重启暂停恢复。
- cargo check --offline、cargo fmt --check、cargo clippy --offline --all-targets -- -D warnings：通过。
- pnpm check / pnpm exec svelte-check：0 错误、0 警告；pnpm build：通过。
- 本机 pnpm 为 11.23.0，项目声明版本为 10.28.2；普通 pnpm 尝试切换版本时网络 fetch 失败。检查与构建使用 `pnpm --pm-on-fail=ignore --config.verify-deps-before-run=false ...`，保留现有锁文件及 packageManager 声明。
- pnpm install --offline --frozen-lockfile --ignore-scripts（同上参数）：ERR_PNPM_ABORTED_REMOVE_MODULES_DIR_NO_TTY，未强制删除现有 node_modules；干净安装未验证。
- 嵌入 dist 的 Windows 可执行程序使用 `cargo build --offline --features custom-protocol` 构建；没有把持久开发服务器作为桌面启动测试。
- rebuild_check：构建通过；报告 package.json 无 test 脚本，因此该工具没有执行自动化测试，Rust 测试另行完成。
- 实际启动编译后的 EXE 被 Computer Use 自动审批明确拒绝：`Computer Use was not approved to use music-mine-desktop`。没有换用 shell 启动绕过拒绝；pnpm exec tauri dev 的窗口启动亦未继续。

## 未验收及下一步

还需允许桌面访问后，在真实 Windows 窗口执行实际 471 首曲库 IPC、LRC 高亮与 Seek、暂停/切歌、拖拽、关闭/重开恢复及不同缩放比例验收；本轮自动化测试不替代这些交互验收。下一阶段先完成此人工/桌面验收与干净依赖安装，再考虑托盘、媒体键和播放历史。旧版导入 session_v2 仍只保留，当前恢复采用新版独立会话，不擅自转换旧会话。
