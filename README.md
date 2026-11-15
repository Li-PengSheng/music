# Music Mine

Windows 本地音乐播放器。技术栈是 Rust、Tauri v2、Svelte 5、TypeScript、Vite 和 Tailwind CSS v4。

当前程序的源码包含本地扫描、歌单、播放、歌词、队列，以及深色、浅色、跟随系统。产品版本在 `package.json` 和 `src-tauri/tauri.conf.json` 里是 `0.1.0`。`docs/tasks/done/` 里的 V0.9.x 是历史任务编号，不是安装包版本。

播放进度只显示 Rust 快照里的位置，界面不用计时器推进播放。哪一层检查已经跑过，见 [docs/current-state.md](docs/current-state.md)。进度只看那一页指向的当前任务。源码里有的能力，不等于自动测试、浏览器或 Windows 实机都已通过。

## 检查

安装依赖后可以运行：

```powershell
pnpm install --frozen-lockfile
pnpm check
pnpm test
pnpm test:ui
pnpm build
pwsh -File scripts/check.ps1
```

`scripts/check.ps1` 在仓库根目录运行 `pnpm check` 和 `pnpm test`，再进入 `src-tauri` 运行 `cargo fmt`、`cargo clippy` 和 `cargo test`。失败时恢复原来的工作目录，并返回失败命令的退出码。它不跑浏览器，也不启动 exe。

统一检查的自动部分由 `.github/workflows/check.yml` 跑。`windows-latest` 只是运行镜像。CI 和本仓库使用 Node `24.14.0`、Rust `1.99.0`（含 rustfmt 和 clippy）和 pnpm `12.21.0`。仓库不使用 `rust-toolchain.toml`。Node 版本写在 `.node-version`，pnpm 版本写在 `package.json` 的 `packageManager`。

本地 Node 用 fnm 安装，当前 default 别名是 `24.14.0`。pnpm 用 Scoop 安装。仓库里的 `npm.cmd` 先看 `NODE_HOME`，再看 PATH 上的 `node`，最后用 `%FNM_DIR%\aliases\default`。pnpm 12 默认不运行依赖的安装脚本；`esbuild` 写在 `pnpm-workspace.yaml` 的 `allowBuilds` 里。本地还需要 Windows 10/11、WebView2 和 Visual Studio 生成工具。依赖升级另开任务。

三条结果不能混写：

- `pnpm check` 和 `pnpm test` 是自动测试。`pnpm test` 只运行 `tests/*.test.mjs`，检查歌词、音量、主题、弹层、队列比较、IPC 名字和隔离路径。
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

会写入曲库或设置的桌面验收按 [docs/isolation.md](docs/isolation.md) 构建和启动。上面这条普通 debug 命令使用系统数据目录，不能代替隔离构建。只改 `APPDATA` 也不会换掉 Tauri 的数据目录。设置页可以确认 Rust 已连接。浏览器预览和 Playwright harness 都不能代替构建出的 exe。

## 文档

- Agent 约束：[AGENTS.md](AGENTS.md)。Claude Code 从 [CLAUDE.md](CLAUDE.md) 引入同一份。
- 验收记录：[docs/current-state.md](docs/current-state.md)。架构：[docs/architecture.md](docs/architecture.md)。
- 产品目标：[docs/RFC-v2.0.md](docs/RFC-v2.0.md)。其中「cargo test + Vitest」已经过时，当前前端测试命令以上面的脚本为准。
- 已完成任务在 [docs/tasks/done/](docs/tasks/done/)。初始化与迁移交接在 [docs/history/](docs/history/)。它们不描述最新行为。新任务用 [docs/tasks/TEMPLATE.md](docs/tasks/TEMPLATE.md)。
