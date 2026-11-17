# Music Mine

Windows 本地音乐播放器。技术栈是 Rust、Tauri v2、Svelte 5、TypeScript、Vite 和 Tailwind CSS v4。

产品版本在 `package.json` 和 `src-tauri/tauri.conf.json` 里是 `0.1.0`。`docs/tasks/done/` 里的 V0.9.x 是历史任务编号，不是安装包版本。

当前源码能力：

- 本地 MP3 / FLAC / WAV 扫描、搜索、专辑/艺术家分组、歌单与顺序管理、播放队列、输出设备选择与会话恢复。移除曲库文件夹仅删除库内记录，不删除音乐；旧版数据库导入入口已移除。
- 本地/内嵌歌词、双语分组、偏移与字号、正在播放布局、桌面悬浮歌词及鼠标穿透；封面缩略图缓存。
- Windows SMTC、托盘与任务栏缩略图按钮；无边框窗口。
- 四套色系（celadon / cinnabar / azure / amber）、深浅/系统主题与五套本机字体预设（未安装时回退），减少动画。
- Console 脱敏日志与诊断、本地短命令，以及默认关闭的 AI 对话。AI 使用用户配置的标准 Chat Completions 服务，最多五轮、六类允许工具，无 shell 或任意命令执行。
- 默认关闭的 ListenBrainz Scrobble、账户绑定的持久待传队列和本地曲库推荐匹配；Last.fm 由用户在 ListenBrainz 网站自行关联，没有独立 Last.fm API 集成。

AI 与 ListenBrainz 需用户显式开启并填写凭证，真实账户、外网 TLS、限流和厂商兼容性属于外部依赖验收。凭证保存在本机 `desktop-v1.json`（尚未使用系统凭据库），配置查询不回传密钥；推荐 HTTPS 或本机 AI 服务。AI 工具只投影必要曲库元数据，用户自行输入的对话也会发往所配置服务。

播放进度只显示 Rust 快照里的位置，界面不用计时器推进播放。哪一层检查已经跑过，见 [docs/current-state.md](docs/current-state.md)。进度只看那一页指向的当前任务。源码里有的能力，不等于自动测试、浏览器或 Windows 实机都已通过。

## 检查

安装依赖后可以运行：

```powershell
pnpm install --frozen-lockfile
pnpm exec playwright install chromium
pnpm format
pnpm check
pnpm test
pnpm test:ui
pnpm build
pwsh -File scripts/check.ps1
```

`scripts/check.ps1` 依次运行 `pnpm format:check`、`pnpm check`、`pnpm test`，再进入 `src-tauri` 运行 `cargo fmt --all -- --check`、`cargo clippy --all-targets -- -D warnings` 和 `cargo test`（Null 音频输出）。失败时恢复工作目录并返回失败码。它不跑浏览器，也不启动 exe。

统一检查的自动部分由 `.github/workflows/check.yml` 跑。`windows-latest` 只是运行镜像。CI 和本仓库使用 Node `24.14.0`、Rust `1.99.0`（含 rustfmt 和 clippy）和 pnpm `12.21.0`。仓库不使用 `rust-toolchain.toml`。Node 版本写在 `.node-version`，pnpm 版本写在 `package.json` 的 `packageManager`。

准备与 CI 一致的 Node、pnpm、Rust（含 rustfmt/clippy），以及 Windows 10/11、WebView2、Visual Studio C++ 生成工具和 Windows SDK。仓库不再含 `npm.cmd` 包装；使用 PATH 中的工具。pnpm 12 默认不运行依赖安装脚本；`esbuild` 写在 `pnpm-workspace.yaml` 的 `allowBuilds` 里。依赖升级另开任务。

三条结果不能混写：

- `pnpm check` 和 `pnpm test` 是自动测试。`pnpm test` 只运行 `tests/*.test.mjs`，检查歌词、音量、主题、弹层、队列比较、IPC 名字和隔离路径。
- `pnpm test:ui` 是浏览器验证。Playwright 打开 `tests/harness/index.html` 的内存曲库，覆盖侧边栏、播放栏、队列、菜单、弹窗、主题切换和窗口变窄。基准截图在 `tests/ui/snapshots/`，是 Windows 上 1 倍缩放的 Chromium 画面。这个夹具会假装桌面环境存在，所以页面上的连接指示不是 Rust 引擎。
- 构建出的 exe 才是 Windows Tauri 实机。浏览器和 Playwright 都不能代替它。

只有明确授权调整视觉基准时才更新截图；审查门禁要求既有基准零像素差异：

```powershell
pnpm test:ui:update
```

## 桌面窗口

浏览器预览（不连接本地曲库）：

```powershell
pnpm dev
```

普通桌面构建（使用系统数据目录）：

```powershell
pnpm tauri build --debug --no-bundle
```

会写入曲库或设置的桌面验收按 [docs/isolation.md](docs/isolation.md) 构建和启动。上面这条普通 debug 命令使用系统数据目录，不能代替隔离构建。只改 `APPDATA` 也不会换掉 Tauri 的数据目录。设置页可以确认 Rust 已连接。浏览器预览和 Playwright harness 都不能代替构建出的 exe。

便携构建及哈希绑定来源记录：

```powershell
pwsh -File scripts/build-portable.ps1 -Debug
# 发布便携包省略 -Debug
```

数据在 exe 旁的 `app-data`，包输出至 `dist-portable/`。隔离空库验收使用 `scripts/build-isolated.ps1`，具体合并配置和启动核验见隔离文档；不修改主 `tauri.conf.json`。

## 文档

- Agent 约束：[AGENTS.md](AGENTS.md)。Claude Code 从 [CLAUDE.md](CLAUDE.md) 引入同一份。
- 验收记录：[docs/current-state.md](docs/current-state.md)。架构：[docs/architecture.md](docs/architecture.md)。
- 产品目标：[docs/RFC-v2.0.md](docs/RFC-v2.0.md)。其中「cargo test + Vitest」已经过时，当前前端测试命令以上面的脚本为准。
- 已完成任务在 [docs/tasks/done/](docs/tasks/done/)。初始化与迁移交接在 [docs/history/](docs/history/)。它们不描述最新行为。新任务用 [docs/tasks/TEMPLATE.md](docs/tasks/TEMPLATE.md)。
