# 当前事实与验收记录

进度只看本页。四个入口：

- [当前任务](#当前任务)
- [已知缺陷](#已知缺陷)
- [验收缺口](#验收缺口)
- [发布决定](#发布决定)

## 核验基线

- 核验 Commit：HEAD `b2c35e73d6814677639ac413cdb4370ccac26716`，分支 `main`。TASK-24在此前未提交工作上增量修改Console输入区；TASK-22/23源码/测试/归档及npm.cmd删除均保留，无自动提交。
- 工作区指纹：`6d653ca07fc2f456548f4223e11f1b77298c36101feacf128a2e594b203d6b7d`。命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-24-CONSOLE-INPUT-DISTINCTION.md,docs/tasks/done/TASK-24-CONSOLE-INPUT-DISTINCTION.md,docs/current-state.md`，最终验收前后及归档前后相同。
- 工作区内容：Console日志页显示Terminal/本地短命令/说明、等宽输入与执行次按钮；AI页显示Sparkles/AI对话指令/当前模型或未启用徽章、正文输入与accent-line边框、发送/发送中主按钮。长模型名截断并保留完整title。复用2秒诊断轮询只读AI配置，关闭后忽略响应；日志自然语言自动切AI，AI页统一发送对话。旧短命令执行、工具闭环和取消机制保留。Rust/数据库/权限/依赖/主配置未改，未访问真实用户数据；此前旧版导入全链路删除和AI/ListenBrainz实现均保留。
- 自动测试：pnpm format、pwsh -File scripts/check.ps1均退出0。Svelte 0 errors/0 warnings，前端95/95，Rust fmt/clippy、单元114 passed/1 ignored、音频集成3/3、歌词集成6/6，Null输出。本轮未重现TASK-23记录的暂停时钟单次失败，未改音频代码/断言，不称为已修复。既有Cargo PDB警告保留。
- 浏览器验证：pnpm test:ui 62/62退出0，既有快照零像素差异，无更新。新增3项输入区回归先失败再通过，覆盖标签/占位符/字体/边框/按钮/模型和未启用状态、发送中/自动切AI及工具执行；1280×800/900×640、浅深主题和超长模型名均通过，窄窗截图已查看。原AI/Console定向14/14，其他既有用例全部通过。
- Windows Tauri 实机：本轮NOT RUN，纯前端变更未运行构建exe；不使用旧来源产物验证当前修改。TASK-23已记录debug exe占用导致隔离构建失败，当前仍保留该验收缺口，不打扰已有进程。此前TASK-22的较早指纹模拟验收不能代替本轮；真实WebView2/DPI/硬件和外部服务未验证。

历史任务 09.5.3、09.5.5 的指纹仍为“未记录，无法重建”。更早任务的指纹不回填。

## 当前任务

TASK-24已完成并归档：[TASK-24-CONSOLE-INPUT-DISTINCTION](tasks/done/TASK-24-CONSOLE-INPUT-DISTINCTION.md)。三项指定门禁全部0、既有快照未改，无自动提交；当前无后续指定任务。Windows实机和此前未覆盖项继续保留为NOT RUN。

封面加载单元已经提交：

- 状态：完成
- 提交：`4352fc6`
- 记录：[09.5.5](tasks/done/TASK-09.5.5-RC-FREEZE.md)，依赖 [09.5.3](tasks/done/TASK-09.5.3-QUEUE-COVER.md)
- 下一步：没有指定的下一任务
- 遗留事项的任务 ID：库外打开、WAV 经界面打开还没有单独的任务 ID。托盘菜单和任务栏按钮是 TASK-17，实机仍是 NOT RUN

## 已知缺陷

这些是任务记录里已经写明的行为，不是 NOT RUN。

- 会话中替换文件里的封面、又没有重新扫描时，页面会继续显示旧图，直到重新扫描或重启。Rust 按 mtime 和长度失效。见 09.5.3。
- 超过 512 KB 的 WebP/GIF，以及 BMP、TIFF、AVIF、HEIC，仍显示为没有封面。见 09.5.3。

## 验收缺口

NOT RUN 放在这里。它不是已知缺陷，也不表示可以放行。

- 库外 `player_open_file`、WAV 经界面打开：09.5.5 写明未做。保持 NOT RUN。
- 托盘菜单和任务栏缩略图按钮已有代码和单元测试。真实托盘、任务栏按钮、DPI 和系统媒体键仍是 NOT RUN。
- 当前工作区自动检查和浏览器已通过；TASK-23 Windows实机因已有debug进程占用无法构建，旧入口/IPC移除尚未在隔离exe确认。首轮Null暂停时钟单次失败根因未定位，后续复跑通过不等同修复。AI真实厂商/Key/外网TLS/非标准扩展、ListenBrainz真实账户/TLS/上传/限流/推荐、ClearType主观锐度、跨WebView2字体同步、硬件音频、原生选择/扫描与DPI仍NOT RUN。标题栏颜色不随色系改变；历史无指纹记录不作为当前验收。
- 浏览器 harness 不是 Rust 播放。

## 发布决定

这是 2026-11-15 这次文档整理做出的决定，不是 09.5.3 或 09.5.5 当时已经写完的验收结论。

库外打开、WAV 经界面打开，以及托盘和任务栏按钮的实机确认，仍是 NOT RUN。这些不阻塞 `4352fc6` 里的封面加载。它们仍阻塞把当前程序称为完整发布。

## 源码事实

| 状态 | 事实 |
|---|---|
| 源码可见 | Svelte 5、TypeScript、Vite、Tailwind v4、Tauri v2、Rust |
| 源码可见 | 产品版本 `0.1.0`。`docs/tasks/done/` 里的 V0.9.x 是任务编号，不是安装包版本 |
| 源码可见 | IPC 在 `src/lib/services/music.ts`。播放快照有 `revision` |
| 源码可见 | `src/app.css` 1461 行。Select / ContextMenu / Modal 使用 Bits UI，Slider 使用原生 range |
| 源码可见 | `package.json` 有 `@playwright/test` 和 `bits-ui`。没有 shadcn-svelte、Vitest、WebdriverIO、tauri-specta、TanStack Virtual |
| 源码可见 | Symphonia feature 为 mp3、flac、wav、pcm |
| 源码可见 | 数据库演进是 `db.rs` 的建表和 `ensure_column` |
| 源码可见 | 设置页旧版迁移与library_import全链路已删除；当前库schema升级保留；rusqlite仅启用bundled，未保留backup feature |
| 源码可见 | ListenBrainz 默认关闭；desktop 偏好保存凭证；scrobble_outbox 按用户名持久化待传记录；后台采集与网络调度位于 platform/scrobble.rs |
| 源码可见 | AI助手默认关闭，Rust platform/ai_agent.rs代理单轮Chat Completions，Console前端最多5轮执行六类允许工具；凭证不返回WebView，AI上下文不含本地路径 |
| 源码可见 | 队列用 `retainQueue`。宽屏队列覆盖页面 |
| 源码可见 | 桌面歌词窗口标签 `lyrics-overlay`，地址带 `?window=lyrics`。主窗口 capability 未扩大 |
| 源码可见 | 配色 `palette` 与明暗 `theme` 分开。默认 `celadon`。首屏脚本设置 `data-palette` |
| 源码可见 | 字体 `font` 与主题/配色独立，默认 `harmony`。`music-mine-font` 保存本机字体预设，首屏设置 `data-font`，已打开歌词窗口监听 storage 更新 |
| 源码可见 | 未保存主题时默认浅色；初始音量为 20%。已有主题选择和播放会话音量继续恢复 |
| 源码可见 | 托盘菜单含曲目、播放控制、桌面歌词和播放模式。Windows 任务栏缩略图按钮 ID 为 1001–1004 |

`src/app.css` 的行数已在本次组件 CSS 搬迁后重数。

## 已记录的验证

这些是 `da5fe14` 提交前的结果。指纹：未记录，无法重建。不能用来勾掉 `decbefb` 及更晚提交。

| 层次 | 结果 |
|---|---|
| 自动测试 | `pnpm test`：50 通过，0 失败。`pnpm check`：0 error，0 warning |
| 浏览器验证 | `pnpm test:ui`：7 通过。覆盖侧栏页面、播放栏、队列、弹窗、主题和 960×700 窄窗。截图与 `tests/ui/snapshots/` 一致 |
| Windows Tauri 实机 | 部分通过。临时 `appDirectoriesOverride` 指向空目录后构建 debug exe。空曲库中确认：连接指示在线、主题切换、新建歌单取消与创建、播放栏显示「未在播放」、队列可 Escape 关闭、980×720 时音量百分比隐藏并在 1280×850 恢复。随后覆盖项已删除并重新构建 |
| 未跑 | `cargo fmt`、`cargo clippy`、`cargo test`。实机没有播放真实音频，没有扫描文件夹，没有测 DPI、托盘或系统媒体键 |

浏览器 harness 使用内存夹具。实机用的是隔离空库。两者都没有打开用户正在使用的 `library-tauri-v1.sqlite` 做实验。曾有一次未隔离启动把桌面主题偏好写成深色，随后已按原文件恢复。以后的实机检查按 `docs/isolation.md`。

上面的实机做法是当时的记录：改主配置再删掉重编。这个做法已经停止，见 ADR-008。

## 文档与代码不一致

- `docs/RFC-v2.0.md` 写自动化测试是 cargo test + Vitest。当前命令是 `pnpm test` 和 `pnpm test:ui`。RFC 仍是产品目标，测试一行过时。
- RFC 写前端用单调时钟对播放进度做平滑插值。当前界面只显示快照里的 `position_ms`，拖动时另有松开前的草稿。约定禁止用定时器推进进度。
- `docs/tasks/done/TASK-V0.9.2-VISUAL-POLISH.md` 写宽屏正在播放会为队列让出宽度。当前 CSS 是队列覆盖，`min-width: 1200px` 时非沉浸页隐藏遮罩。
- `docs/history/` 和 `docs/tasks/done/` 是历史交接，可能落后于后续提交。其中没有进行中的任务。
- 规范包 `music-mine-ai-guidelines-v3-project-wide/` 基于 `2d8d8d2`，其中「测试仍因格式化失败」「尚未安装 Playwright」「README 仍是 Task 02」已过时。以本仓库文档为准。该目录未纳入版本管理。

## 后续优先级

| 优先级 | 事项 | 完成标志 |
|---|---|---|
| P1 | 实机播放与扫描 | 隔离目录里播放一条 fixture 或扫描一个临时文件夹，并记录 IPC 结果 |
| P1 | 实机窗口外观 | DPI 125% 或 150%，以及托盘行为有记录 |
| P2 | 全局 CSS 责任 | 本轮十个指定组件区块已搬迁，规则逐块对比一致；共享样式仍由全局管理 |
| P2 | Select 组件库试点 | Select、ContextMenu 和 Modal 已接入 `bits-ui 2.21.0`；本次范围已完成，Windows 隔离窗口验收仍为 NOT RUN |

P2 全局 CSS 责任已完成本轮十组件范围；实机相关缺口仍未完成。自动检查结果在核验基线。
