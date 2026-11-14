# Music Mine

Windows 本地音乐播放器。技术栈是 Rust、Tauri v2、Svelte 5、TypeScript、Vite 和 Tailwind CSS v4。

当前程序可以扫描本地音乐、管理歌单、播放、显示歌词和队列，并在深色、浅色、跟随系统之间切换主题。播放进度只跟随 Rust 返回的快照，界面不用计时器假装播放在前进。浏览器里打开页面不能代替桌面里的原生命令。

## 检查

安装依赖后可以运行：

```powershell
pnpm install
pnpm check
pnpm test
pnpm test:ui
pnpm build
```

`pnpm test` 检查歌词、音量、主题、弹层和队列比较这些不需要窗口的行为。`pnpm test:ui` 用 Playwright 打开 `tests/harness/index.html`。这个页面使用内存里的模拟曲库，覆盖侧边栏、播放栏、队列、菜单、弹窗、主题切换和窗口变窄。基准截图在 `tests/ui/snapshots/`，是 Windows 上 1 倍缩放的 Chromium 画面。模拟曲库不是用户的音乐库。

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

产品范围和技术约束见 [docs/RFC-v2.0.md](docs/RFC-v2.0.md)。[docs/INITIALIZATION.md](docs/INITIALIZATION.md) 只保留最初 Task 02 的交接记录，不描述现在的播放器和曲库。
