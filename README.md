# Music Mine

Windows 本地音乐播放器。技术栈是 Rust、Tauri v2、Svelte 5、TypeScript、Vite 和 Tailwind CSS v4。

当前程序的源码包含本地扫描、歌单、播放、歌词、队列，以及深色、浅色、跟随系统。产品版本在 `package.json` 和 `src-tauri/tauri.conf.json` 里是 `0.1.0`。`docs/TASK-*.md` 的 V0.9.x 是历史任务编号，不是安装包版本。

播放进度只显示 Rust 快照里的位置，界面不用计时器推进播放。哪一层检查已经跑过，见 [docs/current-state.md](docs/current-state.md)。源码里有的能力，不等于自动测试、浏览器或 Windows 实机都已通过。

## 检查

安装依赖后可以运行：

```powershell
pnpm install
pnpm check
pnpm test
pnpm test:ui
pnpm build
```

三条结果不能混写：

- `pnpm check` 和 `pnpm test` 是自动测试。`pnpm test` 只运行 `tests/*.test.mjs`，检查歌词、音量、主题、弹层和队列比较。
- `pnpm test:ui` 是浏览器验证。Playwright 打开 `tests/harness/index.html` 的内存曲库，覆盖侧边栏、播放栏、队列、菜单、弹窗、主题切换和窗口变窄。基准截图在 `tests/ui/snapshots/`，是 Windows 上 1 倍缩放的 Chromium 画面。这个夹具会假装桌面环境存在，所以页面上的连接指示不是 Rust 引擎。
- 构建出的 exe 才是 Windows Tauri 实机。浏览器和 Playwright 都不能代替它。

更新基准截图：

```powershell
pnpm test:ui:update
```

## 桌面窗口

```powershell
pnpm tauri build --debug --no-bundle
```

用构建出的 `src-tauri/target/debug/music-mine-desktop.exe` 做桌面验收。在 Windows 上，只改进程的 `APPDATA` 不会换掉 Tauri 的数据目录，程序仍会打开正在使用的曲库。需要隔离时，在 `src-tauri/tauri.conf.json` 的 `app` 里临时写入 `appDirectoriesOverride`，指向一个空的临时目录，重新构建后再启动。验收结束后删掉这项并再次构建，不要把覆盖路径留在配置里。设置页可以确认 Rust 已连接。浏览器预览和 Playwright harness 都不能代替这一步。

## 文档

- Agent 约束：[AGENTS.md](AGENTS.md)。Claude Code 从 [CLAUDE.md](CLAUDE.md) 引入同一份。
- 验收记录：[docs/current-state.md](docs/current-state.md)。架构：[docs/architecture.md](docs/architecture.md)。
- 产品目标：[docs/RFC-v2.0.md](docs/RFC-v2.0.md)。其中「cargo test + Vitest」已经过时，当前前端测试命令以上面的脚本为准。
- [docs/INITIALIZATION.md](docs/INITIALIZATION.md) 和 `docs/TASK-*.md` 是历史交接，不描述最新行为。
