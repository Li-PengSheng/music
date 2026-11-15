# Task 05.1：歌词与响应式修复

## 检查点与范围

修改前检查点 `5d6ceed`，独立 Git 仓库 `.checkpoints/task051/.git`（环境将主 `.git` 设为只读）。应用保持 Craft 的 `app_in_root=true` 结构，没有另建重复 app/。

修改文件：`src/lib/NowPlaying.svelte`、`src/lib/lyrics-view.ts`、`src/app.css`、`tests/lyrics-view.test.mjs`、`package.json`、本报告。

## 修复

- 只读检查真实 `E:\LMY-Music\音乐库\Ailee\Ailee - No No No.lrc`：原文及译文处在同一时间戳文本中，以 U+2009 分隔。展示层仅按该明确分隔符换行；普通空格或语言转换不拆分。原始文件未修改，也未复制完整歌词进项目。
- 同时间戳的独立歌词行归为同一显示组，全部同时高亮，避免原文/译文争抢当前行。
- 默认字号 22，允许 16–32 调整，使用本地界面偏好保存；随窗口宽度适配。歌词最大 36rem、自然换行、混排字体回退、当前行更突出。
- 独立歌词容器限制纵向滚动并禁止横向溢出，移除祖先 scrollIntoView；高亮行定位在容器约 40% 处。手动滚轮、触摸、键盘、滚动条交互暂停跟随，可恢复；点击歌词调用真实 Seek，快照决定高亮，不伪造成功。取消平滑滚动以避免 Seek/切歌动画竞态；ResizeObserver 调整定位并在销毁时释放。
- 沉浸层固定在底栏上方，外层不滚动；宽屏左右布局，1000 CSS px 以下上下布局，保留小封面和歌曲信息；封面保持正方形。
- 保留歌曲多列表格，列允许缩小、标题内部省略；已有标题、艺术家、专辑 title 提示保留，菜单未扩充。
- 未修改 Rust、数据库结构、队列与恢复机制或原始音频/LRC。

## 验证

- 新增 4 项 Node 测试通过：分隔符内容完整保留、不猜测语言拆分、同时间戳分组及边界/回退 Seek、非同步歌词分行、容器滚动边界。滚动测试验证布局定位计算，不代表浏览器几何验收。
- `pnpm exec svelte-check`：0 errors / 0 warnings。
- `pnpm build`、`pnpm test`、Craft `rebuild_check`：通过。
- `cargo test --offline`：56 项通过；`cargo check --offline`、`cargo fmt --check`、严格 all-target clippy：通过。
- 最终 `cargo build --offline --features custom-protocol`：通过，EXE 嵌入最终 dist；Cargo 报已有 lib/bin PDB 名称碰撞警告，不影响退出成功。
- pnpm 使用本机版本参数 `--pm-on-fail=ignore --config.verify-deps-before-run=false`，未改锁文件、未宣称干净安装成功。

## 尚待桌面验收

使用 computer-use 尝试启动 built EXE 被自动审批拒绝：`Computer Use was not approved to use music-mine-desktop`。没有通过 shell 或其他工具绕过拒绝，也未用网页截图替代桌面验收。

因此 1280×800、1366×768、1920×1080 的实际窗口布局、Windows 高 DPI、Ailee 双语可读性、手动滚动恢复、暂停/Seek/切歌、真实曲库 IPC 与底栏可见性仍需人工验证。自动化检查不证明这些项目已完成。
