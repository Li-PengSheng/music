# 当前事实与验收记录

进度只看本页。四个入口：

- [当前任务](#当前任务)
- [已知缺陷](#已知缺陷)
- [验收缺口](#验收缺口)
- [发布决定](#发布决定)

## 核验基线

- 核验 Commit：HEAD `7a768e37ffd40c84deaed6a1d6a3f73dd81d1831`，分支 `main`；开工时工作区干净，本次改动未提交。
- 工作区指纹：`fa8806f1694d6756aa432e1054e22e9eedbb67506f69b05ab5b7bd9571f83cac`，验收前后相同；命令为 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/current-state.md,docs/architecture.md`。
- 工作区内容：ContextMenu 已用 Bits UI DropdownMenu 替换手写键盘循环、坐标算法与全局监听；Modal 以 Bits UI Dialog 管理焦点和 Escape，保留原生 dialog 顶层及最小遮罩命中适配，Dialog 业务接口保持原样。tooltip 使用 Floating UI 定位；Slider 保留浏览器原生 range，没有自建拖动或键盘算法。旧菜单坐标工具与源码函数提取器已删除，覆盖移至真实浏览器测试。
- CSS 责任：已完成本次指定的 Sidebar、TopBar、PlayerBar、Queue、NowPlaying、SettingsView、TrackDetails 七个区块搬迁。`src/app.css` 本次实际从 3147 行降到 1926 行（任务描述的旧基线为 3125 行）；共享 tokens、基础样式和跨组件响应式/减少动画规则继续留在全局。迁入组件的规则使用 `:global` 保持选择器权重与跨子组件语义，七个原始区块与迁入规则的选择器及声明逐块一致。
- 自动测试：`pnpm format`、`pnpm format:check`、`pnpm check`、`pnpm test`、`pwsh -File scripts/check.ps1` 退出码均为 0。前端单测 71/71（旧菜单导航/坐标及 Modal 键盘单测迁至浏览器）；Rust fmt、clippy 均通过，Rust 单元测试 78 通过、1 ignored，音频/歌词集成测试 3/6 通过；设置 `MUSIC_MINE_NULL_OUTPUT=1`，不证明真实设备出声。既有 Cargo PDB 文件名冲突警告仍在。CSS 搬迁后 Svelte CSS 检查会对原有 `-webkit-line-clamp` 提示缺少标准 `line-clamp`（1 warning）；为保持原视觉声明，未添加新属性。CSS 源码测试扫描全部 CSS 与 Svelte style 块，主题调色板单独验证。
- 浏览器验证：`pnpm test:ui` 16/16 通过，退出码 0；原有快照全部零像素差异，`tests/ui/snapshots/` 无改动。新增 4 项原语浏览器用例覆盖菜单循环/搜索/IME/焦点、点/矩形锚点与视口碰撞、动作失败、Modal 初始选区/焦点约束/拖出/忙碌保护、原生 range 键盘与 tooltip。覆盖深浅主题、1280×800、960×700、900×640。harness 不证明 Rust 播放。
- Windows Tauri 实机：NOT RUN。

历史任务 09.5.3、09.5.5 的指纹仍为“未记录，无法重建”；本次结果不回填历史验收。

## 当前任务

没有进行中的任务。新任务从 `docs/tasks/TEMPLATE.md` 写起，完成后把记录放到 `docs/tasks/done/`。

封面加载单元已经提交：

- 状态：完成
- 提交：`4352fc6`
- 记录：[09.5.5](tasks/done/TASK-09.5.5-RC-FREEZE.md)，依赖 [09.5.3](tasks/done/TASK-09.5.3-QUEUE-COVER.md)
- 下一步：没有指定的下一任务
- 遗留事项的任务 ID：库外打开、WAV 经界面打开、托盘三态还没有单独的任务 ID

## 已知缺陷

这些是任务记录里已经写明的行为，不是 NOT RUN。

- 会话中替换文件里的封面、又没有重新扫描时，页面会继续显示旧图，直到重新扫描或重启。Rust 按 mtime 和长度失效。见 09.5.3。
- 超过 512 KB 的 WebP/GIF，以及 BMP、TIFF、AVIF、HEIC，仍显示为没有封面。见 09.5.3。

## 验收缺口

NOT RUN 放在这里。它不是已知缺陷，也不表示可以放行。

- 库外 `player_open_file`、WAV 经界面打开、托盘三态：09.5.5 写明未做。保持 NOT RUN。
- 当前工作区的 Rust 自动检查和浏览器 harness 已记在核验基线。真实音频、扫描、DPI、托盘和系统媒体键的实机仍是 NOT RUN。历史无指纹记录不作为当前验收。
- 浏览器 harness 不是 Rust 播放。

## 发布决定

这是 2026-11-15 这次文档整理做出的决定，不是 09.5.3 或 09.5.5 当时已经写完的验收结论。

库外打开、WAV 经界面打开、托盘三态这三项 NOT RUN，不阻塞 `4352fc6` 里的封面加载。它们仍阻塞把当前程序称为完整发布。

## 源码事实

| 状态 | 事实 |
|---|---|
| 源码可见 | Svelte 5、TypeScript、Vite、Tailwind v4、Tauri v2、Rust |
| 源码可见 | 产品版本 `0.1.0`。`docs/tasks/done/` 里的 V0.9.x 是任务编号，不是安装包版本 |
| 源码可见 | IPC 在 `src/lib/services/music.ts`。播放快照有 `revision` |
| 源码可见 | `src/app.css` 1926 行。Select / ContextMenu / Modal 使用 Bits UI，Slider 使用原生 range |
| 源码可见 | `package.json` 有 `@playwright/test` 和 `bits-ui`。没有 shadcn-svelte、Vitest、WebdriverIO、tauri-specta、TanStack Virtual |
| 源码可见 | Symphonia feature 为 mp3、flac、wav、pcm |
| 源码可见 | 数据库演进是 `db.rs` 的建表和 `ensure_column` |
| 源码可见 | 队列用 `retainQueue`。宽屏队列覆盖页面 |

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
| P2 | 全局 CSS 责任 | 本次指定七组件区块已搬迁，规则逐块对比一致；其余共享样式仍由全局管理 |
| P2 | Select 组件库试点 | Select、ContextMenu 和 Modal 已接入 `bits-ui 2.21.0`；本次范围已完成，Windows 隔离窗口验收仍为 NOT RUN |

P2 全局 CSS 责任已完成本次指定范围；实机相关缺口仍未完成。自动检查结果在核验基线。
