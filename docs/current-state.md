# 当前事实与验收记录

进度只看本页。四个入口：

- [当前任务](#当前任务)
- [已知缺陷](#已知缺陷)
- [验收缺口](#验收缺口)
- [发布决定](#发布决定)

## 核验基线

- 核验 Commit：HEAD `ef60136108699cd30a7cb1d00057b3fa1ee0bada`，分支 `main`。两轮 TASK-18、TASK-19 与本次 TASK-20 均未提交；保留原有根目录两张图标、repomix-output.md、Android/iOS 图标删除。
- 工作区指纹：`10772dec321f8ea34658cf04d195ab8e5b9f066568a38839280fcd1d0b570cb8`。命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-20-SHARPEN-AND-LAYOUT-CLEANUP.md,docs/tasks/done/TASK-20-SHARPEN-AND-LAYOUT-CLEANUP.md,docs/current-state.md`，最终验收前后及归档后相同。
- 工作区内容：body 移除 antialiased，启用 optical sizing auto，保留 synthesis none/optimizeLegibility；标题与歌词使用正向微字距及行高 token。正在播放文字分组，在窄窗封面右侧有序排列，歌名间距收紧，规格改为单行 11px 胶囊、保留完整 title。正在播放/详情移除次数，AudioInfo 仅六个文件技术字段，专用次数事件/订阅已清理；首页艺术家显示库内歌曲数，零数量无副标题。内部收听历史/最近播放/排行规则与 schema 不变。此前字体切换、侧栏 Console 全部保留，未访问真实用户数据。
- 自动测试：`pnpm format` 与 `pwsh -File scripts/check.ps1` 退出 0。Prettier、Svelte 0 error / 0 warning、前端 85/85（新增字体单元，相关 8/8）、Rust fmt / clippy、Rust 单元 98 passed / 1 ignored、音频集成 3/3 与歌词集成 6/6。检查使用空音频输出。既有 Cargo PDB 文件名冲突警告仍在。`Cargo.lock` 未变。
- 浏览器验证：`pnpm test:ui` 49/49，退出 0。新增 4 项锐化设置/首页数量/长文本布局测试，删除已无功能的次数事件测试；此前字体与 Console 用例保留。1280×800 / 900×640、鸿蒙浅色/宋体深色、封面与歌词/纯封面、零/非零曲目数均通过。首轮仅 5 个截图用例因变化失败；查看布局后更新 9 张基线，普通复跑零像素差异。
- Windows Tauri 实机：限定范围 PASS。按 docs/isolation.md 构建并核对哈希的隔离 exe，在临时三首仓库 fixture 库确认真实 AudioInfo 六项展示、无次数、单行技术胶囊，以及内部 9001 次收听的艺术家显示 3 首歌曲；WebView2 的 smoothing/optical sizing 为 auto，synthesis 为 none。1280×850 / 100%，空音频输出，实例已关闭。exe SHA-256 `31E9EF93EC566A7535C685999F981694ACDB9840142860A0F2F5348CB0DB3B6D`；来源与产物见 TASK-20。ClearType 主观锐度、125%/150% DPI、实际硬件出声仍 NOT RUN，不把 CSS 属性测试说成这些已通过。

历史任务 09.5.3、09.5.5 的指纹仍为“未记录，无法重建”。更早任务的指纹不回填。

## 当前任务

TASK-20 已完成并按要求归档：[TASK-20-SHARPEN-AND-LAYOUT-CLEANUP](tasks/done/TASK-20-SHARPEN-AND-LAYOUT-CLEANUP.md)。无自动提交，任务完成后停止。此前 TASK-18 两轮和 TASK-19 仍保留原记录与未提交工作；播放次数字段/事件由本轮明确移除，不再沿用其旧展示验收。TASK-17 已在当前 HEAD 中。

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
- 当前工作区自动检查、浏览器和 TASK-20 隔离 exe 的限定 IPC/展示验收已记在核验基线。ClearType 主观锐度、跨 WebView2 字体实时同步、真实硬件音频、原生选择/扫描与 DPI 仍 NOT RUN；本轮实机没有覆盖这些项目。标题栏颜色不随色系改变。历史无指纹记录不作为当前验收。
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
