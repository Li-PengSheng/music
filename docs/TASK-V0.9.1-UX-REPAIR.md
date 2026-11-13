# V0.9.1 UX 修复与主题 — 任务报告

## 基线与检查点

仓库当前 HEAD 为 `4601d5b`（任务描述中的 `e792016` / `16ee4a8` 不存在于本仓库，按“使用最新仓库信息”以 `4601d5b` 为基线）。

| 检查点 | 提交 | 回滚方式 |
| --- | --- | --- |
| 起点 | `4601d5b`，标签 `v0.9.1-start` | `git reset --hard v0.9.1-start` |
| A（P0 弹窗、列表、队列） | `c2557f1` | |
| B（本地歌单、移除收藏 UI） | `5c46fed` | |
| C（标题栏与深浅主题） | `96b3ae0` | |

未改动：Rust 音频引擎、播放状态、歌曲数据库结构与数据、用户音频与 LRC 文件。没有新增 AI、语音、在线功能或打包配置。

## 根因与修复

### A. 弹窗、列表与队列

- **弹窗不居中**：Tailwind v4 preflight 的 `*{margin:0}` 覆盖了 `<dialog>` 的 UA `margin:auto`，弹窗贴到左上角。现在 `.dialog` 是 `position:fixed; inset:0; margin:auto`，并限制最大高度为视口高度。
- **关闭行为不一致**：新建歌单、确认框和歌曲详情各写一套 `showModal` 和焦点处理。现在都使用共享的 `Modal.svelte`，逻辑在 `utils/modal.ts`：
  - 只有按下和抬起都落在背景上才算点击背景；用于关闭弹出菜单的那次点击不会连带关闭弹窗。
  - Esc 在 keydown 阶段处理，避免 Chromium 跳过或强制触发 `cancel`；输入法组字时不处理。
  - 提交中不可关闭；关闭后焦点回到打开前的元素。
  - 取消不会创建歌单；提交失败时弹窗保持打开并显示错误。
  - 弹窗位于浏览器 top layer，因此不会被顶栏或播放栏遮挡。
- **歌曲详情**：居中，宽 600px（在 1280×800 实测 588×737），内部滚动，`overscroll-behavior:contain`。
- **表头被行覆盖**：sticky 偏移是相对滚动容器的 padding 盒计算的，`top:0` 在顶部内边距下方才吸住，行会从缝隙里滚过。现在表头是 `top:calc(var(--page-top) * -1)`，`z-index:var(--z-sticky)`，没有用 padding 补丁。
- **层级混乱**：新增统一的 z-index token（`--z-sticky` < `--z-player` < `--z-immersive` < `--z-queue-scrim` < `--z-queue` < `--z-toast`）。弹窗、菜单和提示框都在 top layer。
- **队列在“正在播放”页打不开**：队列原本在 `.content` 网格里，沉浸层盖在它上面。现在队列是 App 级的固定层：
  - 宽屏时与沉浸层并排，沉浸层让出 `--queue-w`。
  - 窗口宽度不足 1200px 时变成抽屉，配遮罩；点击遮罩只关闭队列。
  - 打开或关闭队列不调用播放 IPC。
- **Esc 一次关闭多层**：新增 `utils/layers.ts` 的 `escapeLayer`，每次按 Esc 只关闭最上面的一层，顺序为队列、正在播放、提示条。

### B. 本地歌单与收藏

- **“本地”需手工维护**：新增内置路由 `{view:'local'}`，直接使用不带过滤条件的音乐库查询（`localQuery`）：
  - 不复制任何行，导入或重扫后自动包含所有歌曲。
  - 支持播放全部、随机播放、搜索和排序。
  - 右键菜单里没有删除或重命名。
  - 按路由类型识别，不按名称匹配。
- **同名用户歌单**：已有的名为“本地”的用户歌单原样保留：数据、ID、重命名和删除都不变，侧栏显示“自建”标签。新建或改名为“本地”会被拒绝，并提示这是系统歌单。
- **收藏**：从界面上完全移除，包括爱心按钮、“仅显示收藏”筛选、右键菜单项、详情页按钮和空状态文案，以及前端的 `favoriteTrack`、`setFavorite` 和 `favorite` 查询字段。Rust 的 `library_favorite` 命令和数据库里的收藏数据有意保留，没有改动数据库层。

### C. 标题栏与主题

- **标题栏颜色**：使用 `getCurrentWindow().setTheme()`，经 `services/music.ts` 调用，并在 capability 中新增 `core:window:allow-set-theme`。选择“跟随系统”时传 `null`，交还给 Windows。
  - Rust 把选择保存到 `desktop-v1.json` 的 `theme` 字段，`setup` 在页面加载前调用 `set_theme`，并设置对应的窗口背景色。
  - `tauri.conf.json` 默认 `"theme":"Dark"`，`backgroundColor` 为 `#121414`。
  - 没有使用自绘标题栏，Snap、DPI、双击最大化、边缘缩放等行为都保持原生。
- **设置 → 外观**：可选深色、浅色或跟随系统，并沿用原有的“减少动态效果”开关。原“窗口与外观”卡片拆为“外观”和“窗口”两张。
  - 老用户和未知值一律视为深色。
  - 切换立即生效，保存在 localStorage 和桌面偏好两处。
  - “跟随系统”同时监听 `prefers-color-scheme` 和窗口的 `theme-changed` 事件。
- **无白闪**：`public/theme-init.js` 是同源的普通脚本，因为 CSP 不允许内联脚本。它在 `<head>` 里、bundle 之前设置 `data-theme` 和 `color-scheme`；`<html>` 默认 `data-theme="dark"`。
- **同一套 token 和组件**：浅色主题只在 `:root[data-theme=light]` 中覆盖已有的 token，使用任务给定的色值；文字对比度约为 14:1、5.9:1 和 4.8:1。
  - `app.css` 中硬编码的深色值都已换成 token：滚动条、滑块、输入框悬停、提示框、提示条、封面占位、进度条、播放键悬停、氛围光、歌词颜色和阴影。
  - 只保留遮罩用的 `#000` 和封面上的小阴影。
- **切换主题时封面不闪、播放状态不变**：氛围色按曲目缓存，与主题无关；切换主题不调用任何播放 IPC。

## 修改的文件

- 新增：
  - `src/lib/components/Modal.svelte`
  - `src/lib/utils/modal.ts`
  - `src/lib/utils/layers.ts`
  - `src/lib/utils/collections.ts`
  - `src/lib/utils/theme.ts`
  - `public/theme-init.js`
  - `tests/harness/*`：仅用于浏览器测试的模拟 IPC，不进入构建产物
  - `tests/v091-layers.test.mjs`
  - `tests/v091-collections.test.mjs`
  - `tests/v091-theme.test.mjs`
  - 本报告
- 前端修改：
  - `App.svelte`、`app.css`、`index.html`、`tsconfig.json`
  - `components/Dialog.svelte`、`components/TrackList.svelte`
  - `TrackDetails.svelte`、`Queue.svelte`
  - `layout/Sidebar.svelte`、`layout/TopBar.svelte`
  - `views/SongsView.svelte`、`views/HomeView.svelte`、`views/SettingsView.svelte`
  - `actions.ts`、`services/music.ts`
  - `stores/nav.svelte.ts`、`stores/library.svelte.ts`、`stores/preferences.ts`、`stores/ui.svelte.ts`
  - `styles/tokens.css`、`utils/playlist-name.ts`
- Rust 与配置修改：
  - `src-tauri/src/desktop.rs`：只新增 `theme` 偏好
  - `src-tauri/src/catalog_tests.rs`：只新增测试
  - `src-tauri/tauri.conf.json`
  - `src-tauri/capabilities/default.json`
  - `src-tauri/gen/schemas/capabilities.json`：构建时重新生成

## 测试结果（`96b3ae0`）

| 命令 | 结果 |
| --- | --- |
| `pnpm test` | 36/36 通过 |
| `pnpm exec svelte-check` | 0 错误，0 警告 |
| `pnpm build` | 成功 |
| `cargo test --offline -- --test-threads=1` | 60 + 3 + 6 通过 |
| `cargo fmt --check` | 通过 |
| `cargo clippy --all-targets -- -D warnings` | 通过 |
| `cargo build --features custom-protocol --bin music-mine-desktop` | 成功，生成 `src-tauri/target/debug/music-mine-desktop.exe` |

新增的自动化测试覆盖：

- 弹窗：点击背景与点击内部、Esc、输入法组字、提交中不可关闭、焦点恢复。
- 列表：表头位置，第 1、50、200、460、471 行可完整到达。
- 层级：队列是 App 级层，Esc 只关闭最上层。
- 本地歌单：随音乐库变化（Rust 测试：扫描、移除文件夹）；其他歌单和同名用户歌单不受影响；保留名称校验。
- 收藏：`src` 中没有任何收藏相关调用或文案。
- 主题：持久化；预渲染脚本与 `resolveTheme` 在所有组合下一致；跟随系统监听；浅色只覆盖已有 token；`app.css` 没有硬编码颜色。

浏览器测试环境（`tests/harness`，模拟 471 首歌和 12 首队列，1280×800 与 1100×700）的实测：

- 弹窗居中，背景、Esc、取消和失败提交的行为都正确。
- 详情页可内部滚动；表头不被覆盖，末行完整显示。
- 歌词页与队列可同时打开，窄窗口下是抽屉。
- 本地歌单显示 471 首，同名用户歌单带“自建”标签且显示 0 首。
- 浅色和深色立即切换，刷新后保持，`set_theme` 参数正确。
- 跟随系统会响应系统配色变化和窗口 `theme-changed` 事件，切换主题时播放状态不变。

## Windows 实机验收清单

开发期间桌面正被占用，没有操控真实窗口。以下项目在浏览器测试环境中已验证，但仍需在实机上用构建出的 exe 确认（标记为待人工验收）：

- [ ] 原生标题栏：深色和浅色各切换一次，颜色立即正确。
- [ ] 跟随系统：在 Windows“设置 → 个性化 → 颜色”中切换，标题栏和页面同时跟随。
- [ ] 启动：选择浅色后重启，没有白色或黑色闪屏，标题栏直接是浅色。老用户首次启动仍是深色。
- [ ] 新建歌单和删除确认弹窗居中：在 1280×800 和最小窗口 900×640 下，不被顶栏或播放栏遮挡。点击背景、Esc、Enter 和取消都正确，焦点会恢复。
- [ ] 歌曲详情：居中，可内部滚动，关闭后焦点回到原按钮。
- [ ] 471 首歌曲列表：滚到底部时表头不被覆盖，最后一行完整显示，列对齐。
- [ ] 正在播放（歌词）页打开队列：播放不中断；按 Esc 只关闭队列；窄窗口下是抽屉。
- [ ] 本地歌单显示全部歌曲，可以播放全部、随机播放、搜索和排序，不能删除。
- [ ] 输入框右键复制和粘贴正常（本轮没有改动该逻辑）。
- [ ] 没有多余的滚动条；导航和播放栏固定。
- [ ] 关闭到托盘、最大化和还原、Snap、DPI 切换、窗口位置恢复正常（本轮仍是原生标题栏，没有改动这些逻辑）。

## 未完成与说明

- 上面的实机项都标记为待人工验收，没有直接勾选。
- 收藏的数据和 Rust 命令有意保留，以后要恢复或彻底清理需要单独任务。
- `tests/harness` 只用于测试，不进入构建产物。
- 本轮到此停止：没有 AI、打包或新功能。
