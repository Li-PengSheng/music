# 任务 07.2.1 验证记录

范围：Craft 的 app_in_root=true，app 为当前项目根目录；不创建第二套 app。修复前检查点 a3ab132（.checkpoints/task061/.git）。

## 根因与代码证据

App.svelte 原封面 effect 读取 snapshot?.track_id。refresh 每次从 Rust 取回快照后 accept 替换 snapshot（200ms 间隔加 IPC 耗时），导致 effect 每轮执行 artwork=null 和 player_artwork IPC。即使 Base64 字符串内容相同，异步间隙也先卸载 img 再重建，产生闪烁。不是 CSS 动画，也不是不稳定 key；无 Blob URL 生命周期。通过真实 Svelte 编译器和 effect runtime 重现：还原原始依赖后测试报 progress reloaded cover，修复后 1500 次更新只有一次请求。

## 文件和函数

- src/App.svelte：artworkTrackId 使用 $derived 标量；封面 effect 只订阅它。相同 track_id 不重置 artwork，不重复生成 Base64、不请求 IPC；原 ticket 防止旧请求覆盖新歌曲。Seek draft 同样按歌曲 ID 重置。
- src/lib/Artwork.svelte：保留稳定失败占位，增加开发期挂载、加载、URL 变化计数。
- src/lib/cover-diagnostics.ts：仅 DEV 暴露 globalThis.__musicMineDiagnostics；记录快照、歌曲变化、封面请求、URL 变化、图片挂载/加载/错误及开始时间。不存歌曲 ID、URL 或路径，不写日志；计数覆盖所有 Artwork 实例。
- src/lib/Library.svelte：歌曲 each 使用稳定 t.id，菜单安装 songMenu action。
- src/lib/song-menu.ts：createMenuOwner 和 songMenu。原来每行独立 details 且右键只打开本行，互不关闭。现在按歌曲 ID 管理唯一所有者；按钮/右键打开时关闭旧菜单；外部 pointerdown、Escape、滚动、resize、销毁均关闭。浮层 fixed 定位并约束视口，避免底部裁剪；滚动关闭避免错位。操作仍使用对应 keyed 行的 t.id，未改变队列和收藏服务。
- tests/player-stability.test.mjs：真实 Runes effect 测试及菜单 DOM action 测试。

## 自动验证

- pnpm test：10 项通过；覆盖同歌 1500 次进度/暂停恢复/音量、切歌、乱序结果、菜单唯一所有者、外部点击、Escape、滚动、底部定位。
- pnpm exec svelte-check：0 errors / 0 warnings。
- pnpm build：通过。
- pnpm 使用现有兼容参数 --pm-on-fail=ignore --config.verify-deps-before-run=false，未增依赖。本轮未重跑 install，不宣称基础工程全量验收。
- cargo test --offline：首轮真实 MP3 暂停时钟断言相差 10ms；cargo test --offline -- --test-threads=1 重跑 63 项全部通过，包含 MP3/FLAC/WAV 真输出设备测试。未修改音频代码或放宽断言；该时序波动如再现需独立定位。
- cargo check、cargo fmt --check、cargo clippy --offline -- -D warnings：通过。
- cargo build --offline --features custom-protocol --bin music-mine-desktop：通过，src-tauri/target/debug/music-mine-desktop.exe 嵌入最终 dist；原有 bin/lib PDB 输出重名告警仍存在。
- rebuild_check：最终结果见 Craft 检查记录。

## 未验证与人工验收

本轮未操作真实 Tauri 窗口或完成窗口 IPC 验收，不使用持久开发服务器替代。会话已有窗口操作审批拒绝记录（Computer Use was not approved to use music-mine-desktop），未绕过。自动化 1500 次状态更新不等于五分钟实机播放。

请退出旧实例后启动最终 EXE：同歌播放至少五分钟；暂停、恢复、Seek、音量；连续切换至少十首；歌词/音乐库/首页往返；最小化恢复；不同歌曲按钮及右键菜单互斥、外部点击/Escape、底部菜单与滚动；队列、收藏；托盘、SMTC；重启封面。均尚未人工确认。本轮只修复 07.2.1，不进入 07.3。
