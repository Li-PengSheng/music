# Task 06.1 — Windows 托盘与窗口状态

## 实现与边界

- Git 检查点：`.checkpoints/task061/.git`，提交 `0a4c445`（应用修改前）。Craft 的 `app_in_root: true` 已将本目录配置为应用根目录，本轮沿用根目录，没有复制第二套 app/。
- Tauri v2 官方 Tray API；仅在 setup 创建一个图标、菜单及处理器。左键恢复/聚焦，右键显示菜单：显示窗口、实际状态与曲名、播放/暂停、上一首、下一首、退出。没有新全局快捷键。
- 菜单操作直接调用已有 Controller 的 Action；状态来自同一个真实快照，已有 Rust 200ms 队列工作线程兼顾菜单更新，未添加第二套播放状态或 UI 播放计时器。窗口反复隐藏不注册额外事件、菜单或线程。
- 设置增加“关闭主窗口时”：默认退出，可选关闭到托盘；成功保存后展示操作提示。隐藏只操作原窗口，音乐和 EOF 推进继续由原控制器执行。托盘退出使用 app.exit，不受关闭到托盘选项阻拦。
- 退出先保存当前队列/进度，再通过音频命令队列发送 Shutdown；音频线程释放 decoder 和输出资源后确认，再结束进程。仅增添生命周期命令，没有改解码、输出回调、DSP 或播放算法。
- 官方 Window State 插件优先尝试，但本机无缓存且 crates.io 下载不可达；为保持可构建，采用 Tauri 窗口 API + 应用数据目录 `desktop-v1.json` 回退实现。保存大小、位置、最大化及队列展开/关闭选项，临时文件替换写入；不保存隐藏状态，重启正常显示。
- 启动、托盘恢复、DPI 事件检查监视器工作区；离屏/过大窗口回到主屏可视范围，支持负坐标监视器，适当缩小最小尺寸以适应较小工作区；最大化窗口按中心点判断监视器，保留系统标题栏。
- 主题、歌词、收藏、库结构与音乐文件未修改；没有实际打开用户音乐库做迁移实验。

## 修改文件

- `src-tauri/src/desktop.rs`：托盘、偏好 IPC、窗口生命周期/显示范围与 5 项测试。
- `src-tauri/src/lib.rs`：一次性注册与原队列工作线程状态更新、统一退出处理。
- `src-tauri/src/controller.rs`：只读状态字段与资源释放入口。
- `src-tauri/src/player.rs`：Shutdown 命令，释放现有资源后确认。
- `src-tauri/Cargo.toml`、`Cargo.lock`：官方 tray-icon/image-png 功能及对应依赖。
- `src/lib/services/music.ts`：集中 desktop_preferences IPC。
- `src/App.svelte`：队列展开偏好读取/保存。
- `src/lib/Library.svelte`：关闭选项及保存错误提示。

## 自动检查

- `pnpm exec svelte-check` 与 `pnpm check`：0 errors、0 warnings。
- `pnpm test`：4 项通过。
- `pnpm build`：通过；Craft `rebuild_check` 的 build/own-tests 均通过。
- `cargo test --offline`：61 项通过（52 单元 + 3 音频 + 6 歌词）；合计 65 项自动测试。
- `cargo check --offline`、`cargo fmt --check`、`cargo clippy --offline --all-targets -- -D warnings`：通过。
- `cargo build --offline --features custom-protocol`：通过，构建嵌入 dist 的实际 Windows EXE。现有 lib/bin PDB 文件名碰撞警告仍存在，不影响成功。
- pnpm 命令使用本机参数 `--pm-on-fail=ignore --config.verify-deps-before-run=false`；尝试 frozen/offline install 因无 TTY 拒绝重建 node_modules，未强制删除现有依赖，不能宣称干净安装已验证。

## 桌面限制与人工验收

使用 computer-use 技能启动已构建 `src-tauri/target/debug/music-mine-desktop.exe` 被自动审批拒绝：`Computer Use was not approved to use music-mine-desktop`。未绕过拒绝；没有以网页构建代替托盘验收。

下列全部仍待真实 Windows 验收：托盘显示/左右键菜单、IPC、隐藏/恢复、后台播放和 EOF、真正退出、重启恢复、多显示器断开、高 DPI，以及重复操作的内存/监听器稳定性。

1. 直接运行上述 EXE（无需开发服务器）。确认绿色主题、原标题栏和真实曲库可用；右键系统托盘，空队列时控制禁用。
2. 播放真实音频，核对托盘曲名、状态和播放/暂停文字；使用上一首、下一首并比较主界面同步结果。检查歌词、收藏和队列行为。
3. 保持默认“退出程序”，点击 X；任务管理器确认进程消失，重启保留队列/进度并暂停。
4. 启用“最小化到系统托盘”，播放后点击 X；确认声音持续且曲终可进入下一首，左键托盘恢复并聚焦；原窗口最小化后也应恢复。
5. 连续隐藏/恢复至少 50 次，确认仅一个托盘图标、一个窗口，菜单操作不重复，任务管理器内存不会持续攀升。
6. 隐藏时右键托盘“退出”，确认声音结束、图标消失、进程退出；重启检查会话与偏好。
7. 调整窗口位置和大小、展开队列，退出重启检查恢复；再分别测试最大化和收起队列。
8. 将窗口置于副屏，隐藏或退出并断开副屏，恢复/重启应回到可见区域；覆盖左侧负坐标屏幕、不同分辨率以及 100%/150%/200% DPI 切换。确认标题栏和关闭/退出入口始终可达。

本轮只交付上述范围；桌面集成验收尚未完成。
