# 当前事实与验收记录

进度只看本页。四个入口：

- [当前任务](#当前任务)
- [已知缺陷](#已知缺陷)
- [验收缺口](#验收缺口)
- [发布决定](#发布决定)

## 核验基线

- 核验 Commit：HEAD `2af3b3c66d6449edf6d48e482f042a9b3d31cc67`，分支 `main`。TASK-21 开工时工作区干净，本轮改动未提交；此前文档的未提交状态已落后于当前 HEAD。
- 工作区指纹：`84ba08ac6188cb4e2c5f56b443babd2fbeb82bfb5cb9cc2b1dd324065870b295`。命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-21-LISTENBRAINZ.md,docs/tasks/done/TASK-21-LISTENBRAINZ.md,docs/current-state.md`，最终门禁前后、隔离实机后及归档后相同。
- 工作区内容：ListenBrainz 默认关闭，设置页可保存验证凭证、开启/关闭、清除凭证；开关走本地持久化，关闭停止新采集和新请求，已在途请求可能完成。Token 只保存在后端设置文件，不经配置查询或 desktop_preferences 返回明文，日志写入前脱敏。独立采集/网络线程，暂停与 Seek 不计，半首/4分钟达标入 SQLite outbox，按用户名隔离、成功删除、失败退避、重启恢复。ureq 2.12.1 + Rustls，无 OpenSSL，Cargo.lock 已更新。统一限流 1.1 秒并遵守 Reset-In/Retry-After；同会话重复保存/切换开关保持去重。推荐 MBID 经官方元数据解析后按艺术家+曲名匹配当前本地库，候选缓存 10 分钟，未生成时回退公开近期收听；仅命中时显示可播放/右键操作的推荐架。此前字体、技术参数、Console 均保留，未访问真实用户数据。
- 自动测试：`pnpm format` 与 `pwsh -File scripts/check.ps1` 退出 0。Prettier、Svelte 0 error / 0 warning、前端 85/85、Rust fmt / clippy、Rust 单元 109 passed / 1 ignored（本轮新增 10 项 Scrobble 与 1 项日志测试）、音频集成 3/3、歌词集成 6/6。检查使用空音频输出，不发真实 ListenBrainz 请求；既有 Cargo PDB 文件名冲突警告仍在。只读复审无未解决 Critical/Important。
- 浏览器验证：`pnpm test:ui` 53/53，退出 0。新增 4 项 ListenBrainz 默认关闭、凭证错误/保存、键盘开关、推荐播放/右键菜单、空/失败状态用例；字段和推荐架在 1280×800 / 900×640 检查无挤压。首轮 51/52 仅 settings-dark 副标题文字差异，查看后只更新这一张快照，最终普通复跑零差异；此前字体、Console 与布局用例全部通过。
- Windows Tauri 实机：限定范围 PASS。CLI 合并隔离配置、核对 exe 哈希与来源，临时旧 schema/旧偏好和三首仓库 fixture 上验证新增 outbox、默认关闭、真实配置查询/禁用/清除持久化、缺凭证不能启用、非法 Token 在 HTTP 前拒绝、禁用推荐为空、Console 无测试凭证，以及密码框。1280×850 / 100%，Null 输出，实例已关闭。exe SHA-256 `C259D4FDC8FD7451435EAE0E180B75CBD947A3DE2D07A587EE51C1B2188C8C30`，来源和产物见 TASK-21。真实账户验证、外网上传与在线推荐命中 NOT RUN；本轮不声称真实服务成功。

历史任务 09.5.3、09.5.5 的指纹仍为“未记录，无法重建”。更早任务的指纹不回填。

## 当前任务

TASK-21 已完成并按要求归档：[TASK-21-LISTENBRAINZ](tasks/done/TASK-21-LISTENBRAINZ.md)。无自动提交，任务完成后停止。真实账户/外网端到端验收仍未覆盖；不把模拟上传与浏览器夹具成功称为真实 ListenBrainz 服务成功。此前任务保留原历史记录，旧未提交说明不能代替当前 HEAD 基线。

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
- 当前工作区自动检查、浏览器和 TASK-21 隔离 exe 的限定配置/迁移验收已记在核验基线。ListenBrainz 真实账户验证、真实 TLS/上传/限流与在线推荐命中、ClearType 主观锐度、跨 WebView2 字体实时同步、硬件音频、原生选择/扫描与 DPI 仍 NOT RUN；本轮实机没有覆盖这些项目。标题栏颜色不随色系改变。历史无指纹记录不作为当前验收。
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
| 源码可见 | ListenBrainz 默认关闭；desktop 偏好保存凭证；scrobble_outbox 按用户名持久化待传记录；后台采集与网络调度位于 platform/scrobble.rs |
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
