# 当前事实与验收记录

进度只看本页。四个入口：

- [当前任务](#当前任务)
- [已知缺陷](#已知缺陷)
- [验收缺口](#验收缺口)
- [发布决定](#发布决定)

## 核验基线

- 下面「已记录的验证」属于提交 `da5fe14`（2026-11-14）。那次没有保存工作区指纹：未记录，无法重建。
- 此后尚未纳入那次验证的变更：`decbefb`（便携模式与封面缓存），以及 `decbefb` 之上仍未提交的封面加载改动。
- [09.5.3](TASK-09.5.3-QUEUE-COVER.md) 和 [09.5.5](TASK-09.5.5-RC-FREEZE.md) 写过后来的检查结果。两份记录的工作区指纹都是：未记录，无法重建。不能把其中的通过结果写成当前工作区已经验收。

## 当前任务

未提交的封面加载改动是一个验收单元。任务记录是 [09.5.5](TASK-09.5.5-RC-FREEZE.md)，它依赖同一工作区里的 [09.5.3](TASK-09.5.3-QUEUE-COVER.md)。两轮改过同一个 loader 和测试，不拆成没有独立验收的两次提交。`portable/README.txt` 是用户留下的未提交修改，09.5.5 写明本轮未改，不放进这个单元。

- 状态：待审阅
- 依赖：09.5.3 的队列封面改动已经叠在同一工作区里
- 下一步：由用户决定是否提交这一单元
- 提交：待提交
- 遗留事项的任务 ID：库外打开、WAV 经界面打开、托盘三态还没有单独的任务 ID

## 已知缺陷

这些是任务记录里已经写明的行为，不是 NOT RUN。

- 会话中替换文件里的封面、又没有重新扫描时，页面会继续显示旧图，直到重新扫描或重启。Rust 按 mtime 和长度失效。见 09.5.3。
- 超过 512 KB 的 WebP/GIF，以及 BMP、TIFF、AVIF、HEIC，仍显示为没有封面。见 09.5.3。

## 验收缺口

NOT RUN 放在这里。它不是已知缺陷，也不表示可以放行。

- 库外 `player_open_file`、WAV 经界面打开、托盘三态：09.5.5 写明未做。保持 NOT RUN。
- `da5fe14` 的记录写过：没有跑 `cargo fmt`、`cargo clippy`、`cargo test`；实机没有播放真实音频、没有扫描、没有测 DPI、托盘或系统媒体键。09.5.5 后来写了自动检查和部分实机，但指纹未记录，不能用来勾掉这些缺口。
- 浏览器 harness 不是 Rust 播放。

## 发布决定

这是 2026-11-15 这次文档整理做出的决定，不是 09.5.3 或 09.5.5 当时已经写完的验收结论。

库外打开、WAV 经界面打开、托盘三态这三项 NOT RUN，不阻塞把当前封面加载改动当作一个待审阅的验收单元。它们阻塞把该单元称为完整发布。

## 源码事实

| 状态 | 事实 |
|---|---|
| 源码可见 | Svelte 5、TypeScript、Vite、Tailwind v4、Tauri v2、Rust |
| 源码可见 | 产品版本 `0.1.0`。`docs/TASK-V0.9.*.md` 是任务编号，不是安装包版本 |
| 源码可见 | IPC 在 `src/lib/services/music.ts`。播放快照有 `revision` |
| 源码可见 | `src/app.css` 3072 行。通用组件仍是手写的 |
| 源码可见 | `package.json` 有 `@playwright/test`。没有 shadcn-svelte、Bits UI、Vitest、WebdriverIO、tauri-specta、TanStack Virtual |
| 源码可见 | Symphonia feature 为 mp3、flac、wav、pcm |
| 源码可见 | 数据库演进是 `db.rs` 的建表和 `ensure_column` |
| 源码可见 | 队列用 `retainQueue`。宽屏队列覆盖页面 |

`src/app.css` 的行数是 `da5fe14` 时记下的，后来的未提交改动没有在这里重数。

## 已记录的验证

这些是 `da5fe14` 提交前、同一工作区里的结果。指纹：未记录，无法重建。不属于 `decbefb`，也不属于更晚的未提交改动。

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
- `docs/TASK-V0.9.2-VISUAL-POLISH.md` 写宽屏正在播放会为队列让出宽度。当前 CSS 是队列覆盖，`min-width: 1200px` 时非沉浸页隐藏遮罩。
- `docs/INITIALIZATION.md` 和其余 `docs/TASK-*.md` 是历史交接，可能落后于后续提交。09.5.3 和 09.5.5 仍是当前未提交工作的任务记录。
- 规范包 `music-mine-ai-guidelines-v3-project-wide/` 基于 `2d8d8d2`，其中「测试仍因格式化失败」「尚未安装 Playwright」「README 仍是 Task 02」已过时。以本仓库文档为准。该目录未纳入版本管理。

## 后续优先级

| 优先级 | 事项 | 完成标志 |
|---|---|---|
| P1 | 补 Rust 检查记录 | `cargo fmt --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test` 有 PASS 或明确失败，并且能对上源码指纹 |
| P1 | 实机播放与扫描 | 隔离目录里播放一条 fixture 或扫描一个临时文件夹，并记录 IPC 结果 |
| P1 | 实机窗口外观 | DPI 125% 或 150%，以及托盘行为有记录 |
| P2 | 全局 CSS 责任 | 只在改到的区域把独有规则移出 `app.css`，并有浏览器对比 |
| P2 | Select 组件库试点 | 单独任务。先安装验证，再迁移。未开始 |

不要把上表写成已经完成。
