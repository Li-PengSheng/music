# 当前事实与验收记录

进度只看本页。四个入口：

- [当前任务](#当前任务)
- [已知缺陷](#已知缺陷)
- [验收缺口](#验收缺口)
- [发布决定](#发布决定)

## 核验基线

- 核验 Commit：HEAD `93000e39257d45d2c39aeee272021f14d9d02d07`，分支 `main`；本次 Select 试点尚未提交。保留原有未跟踪文件 `repomix-output.md`。
- 工作区指纹：`1c6a7755e25fefa3ca3501c2b9e4c7a8531ea866c19292284f13e869e9099e94`，验收前后相同；命令为 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/current-state.md`。
- 工作区内容：已安装 `bits-ui 2.21.0` 并迁移 `Select.svelte`，Props 和调用点保持原样。保留 tokens、触发器与选项按钮样式；库负责浮层定位、键盘导航和关闭，原生 dialog 与 IME 做小范围兼容。没有运行样式初始化脚本，没有改 Rust 或数据库。
- 自动测试：`pnpm format`、`pnpm test`、`pwsh -File scripts/check.ps1` 退出码均为 0。格式检查、类型检查（0 error / 0 warning）、74 项前端测试、Rust fmt / clippy 均通过；Rust 单元 78 通过、1 ignored，音频与歌词集成分别 3 / 6 通过，使用 `MUSIC_MINE_NULL_OUTPUT=1`。既有 Cargo PDB 文件名冲突警告仍在。旧 Select 源码抽取测试已迁移到浏览器行为测试，因此前端单测从 75 项变为 74 项，覆盖保留并扩展。
- 浏览器验证：`pnpm test:ui` 12/12 通过、退出码 0；原有 8 项及其全部基准截图零像素差异，快照无修改；另增 4 项 Select 测试覆盖双向绑定、回调、空值、禁用、键盘/IME、深浅主题、900×640 视口边缘与长列表、原生 dialog 的 Escape 和遮罩关闭顺序。测试在沙箱外运行以正常清理服务器。harness 不证明 Rust 播放。Select 从 150 行缩减至 71 行。
- Windows Tauri 实机：NOT RUN。完成 Windows 隔离窗口验收前不扩展到其他组件。

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
| 源码可见 | `src/app.css` 3147 行。Select 已使用 Bits UI，其余通用组件仍为手写 |
| 源码可见 | `package.json` 有 `@playwright/test` 和 `bits-ui`。没有 shadcn-svelte、Vitest、WebdriverIO、tauri-specta、TanStack Virtual |
| 源码可见 | Symphonia feature 为 mp3、flac、wav、pcm |
| 源码可见 | 数据库演进是 `db.rs` 的建表和 `ensure_column` |
| 源码可见 | 队列用 `retainQueue`。宽屏队列覆盖页面 |

`src/app.css` 的行数已在本次全量格式化后重数。

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
| P2 | 全局 CSS 责任 | 只在改到的区域把独有规则移出 `app.css`，并有浏览器对比 |
| P2 | Select 组件库试点 | `bits-ui 2.21.0` 已安装，Select 已迁移。Windows 隔离窗口验收 NOT RUN，通过后再决定下一组件 |

表中事项仍未完成。Rust 自动检查记录已从本表移出，结果在核验基线。
