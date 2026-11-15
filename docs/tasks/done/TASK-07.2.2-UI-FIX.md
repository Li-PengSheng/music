# Music Mine — 任务 07.2.2 UI 精修

日期：2026-11-13。范围止于 07.2.2，不进入 07.3。

## 范围与检查点

遵循 docs/RFC-v2.0.md。Craft 当前 app_in_root=true，app 就是本项目根目录，沿用 src/ 与 src-tauri/，没有创建不参与构建的第二套 app/。
开始修改前在现有项目专用 Git 仓库 .checkpoints/task061/.git 创建检查点 **5928fb0**（工作树为本项目根目录）。顶层 Git 沿用现状，未覆盖其他项目。
没有修改 Rust、数据库结构、用户音乐文件或收藏数据，没有接入新业务或大型 UI 依赖。

## 问题根因与修复

### 队列与页面滚动

原队列是固定宽度浮层，主内容没有为其让出空间，头部大间距、内容没有独立滚动区域，歌曲与操作混排。页面本身还叠加固定播放栏补偿 padding，导致空白、裁切与双层滚动。

现在 shell 使用两行两列 Grid：侧栏与工作区位于上行，播放栏单独占满下行；工作区由顶栏与内容区组成。主页面和队列列表各自拥有必要的纵向滚动，根页面不滚动。宽窗口内容区分配约 360px 队列列；小于 1200px 时采用带背景关闭按钮的覆盖抽屉。抽屉不覆盖底部播放栏，头部固定显示标题、数量及关闭按钮。

队列项使用稳定 path key，显示 44px 封面、歌曲名、艺术家、时长和当前歌曲高亮。上移、下移、删除为独立 34px 图标按钮，提供 title 与 aria-label；移除原来不必要的拖动状态。打开后聚焦关闭按钮，关闭/Escape 后恢复队列触发按钮焦点。展开/收起仍只保存桌面偏好，不触发播放命令。

**重复项约束：** controller.rs 的 Enqueue 按 path 去重，PlayNext 也先移除同路径项，Select/Reorder/Remove 都以 path 为身份。保留这些既有业务规则。相同标题、不同路径的歌曲可分别操作；当前接口不能表示同一路径的多个独立队列实例。本轮不新增重复队列业务，也不声称兼容手工篡改会话中的重复 path。若以后允许重复，需另外设计后端队列项身份与当前项身份。

### 菜单与选择器

原来的播放模式、排序、布局、退出行为和歌单选择均使用系统 select，下拉样式依赖 Windows/WebView。新增轻量 Select.svelte，使用浏览器 Popover 顶层与自定义 listbox 按钮，不受父容器 overflow 裁切；按触发按钮和可用视口定位，支持向上展开。

统一深绿色表面、边框、圆角、阴影、悬停与选中状态。支持方向键、Home/End、字符定位、Enter/Space 原生按钮选择、Escape、Tab、焦点恢复、外部点击关闭、滚动/窗口变化关闭以及输入法组合保护。选择器与歌曲菜单通过 document 事件互斥；原歌曲菜单增加方向键导航与操作后焦点恢复。

### 音乐库封面与导航搜索

原歌曲行没有挂载已有 Cover，专辑网格已经使用真实封面。现在歌曲标题同列加入 44px 封面，保留艺术家、专辑、时长、收藏、操作列及稳定歌曲 ID key；继续沿用 IntersectionObserver、trackCover 缓存和 Artwork 占位，播放进度更新不会重建行封面。保留每页 50 首歌曲和每页 36 个分组，未一次解码全曲库。专辑沿用真实封面网格，艺术家沿用圆形代表封面/统一占位。

队列封面按可见区域解析：通过现有 library_query 分页查找标题候选，再按完整 path 匹配 ID，使用现有 library_cover；队列路径 Promise 缓存上限 100，扫描时清理两层缓存。曲库外文件无法匹配时统一占位，没有另写封面提取器。大量同名候选可能需要多个只读分页请求，未增加后端接口。

只移除左侧“我喜欢的”入口，收藏按钮、数据库和原有入口逻辑保留。整理导航与底部设置间距。搜索框的 focus-within 边框与柔和光环统一，输入框去除系统轮廓并由容器提供可访问焦点指示；图标、清除按钮、Ctrl F 提示保持对齐，窄窗口收缩，Ctrl+F 和 IME 原逻辑保留。

CSS 新增少量 --mm-* 表面、边框、焦点、圆角、阴影、侧栏/播放栏尺寸与点击区域变量，统一细滚动条、行高、图标按钮、文本省略和当前歌曲高亮，保留品牌绿色。

## 实际修改文件

- src/App.svelte：内容区 Grid、队列组件绑定、Escape 与焦点、移除侧栏收藏入口、播放模式 Select。
- src/app.css：设计变量、布局滚动责任、响应式队列、搜索与菜单、封面行、按钮和滚动条。
- src/lib/Queue.svelte（新增）：独立队列头部与列表、真实操作绑定、封面与高亮。
- src/lib/QueueCover.svelte（新增）：可见时加载队列封面，销毁后丢弃异步结果。
- src/lib/Select.svelte（新增）：通用可访问 Popover 选择器。
- src/lib/Library.svelte：歌曲行封面、排序与设置选择器、长文本提示。
- src/lib/NowPlaying.svelte：布局选择器。
- src/lib/TrackDetails.svelte：歌单选择器。
- src/lib/services/music.ts：队列封面身份解析与缓存，所有 IPC 保持在 service 内。
- src/lib/song-menu.ts：选择器互斥与键盘导航、操作后焦点恢复。
- tests/ui-refinements.test.mjs（新增）：执行实际选择器键盘处理函数及歌曲菜单 action，验证导航、IME、Escape/Tab 和互斥。
- docs/TASK-07.2.2-UI-FIX.md：本报告。

## 自动验证实际结果

- pnpm test：12/12 通过（包括既有 1500 次快照更新封面稳定性、乱序响应、快捷键和菜单回归）。
- pnpm exec svelte-check：0 errors，0 warnings。
- pnpm build：通过。
- cargo test --offline：54 个库测试 + 3 个音频集成测试 + 6 个歌词集成测试，共 63 项全部通过。
- cargo check --offline：通过。
- cargo fmt --check：通过。
- cargo clippy --offline -- -D warnings：通过。
- cargo build --offline --features custom-protocol --bin music-mine-desktop：通过，最终可执行文件为 src-tauri/target/debug/music-mine-desktop.exe，嵌入 dist，不依赖开发服务器。
- rebuild_check：build 与 own-tests 通过。该检查没有完成真实 Windows 窗口验收。

pnpm 沿用本环境参数 --pm-on-fail=ignore --config.verify-deps-before-run=false。本轮没有新增依赖，也没有重新运行 pnpm install，不宣称基础工程完整验收。Rust 构建仍有原有 bin/lib PDB 输出重名警告，本轮未扩大范围修改它。

Chrome DevTools 浏览器检查尝试失败：环境找不到 Google Chrome 可执行文件。键盘函数/模拟 DOM 自动测试不是 WebView 视觉或真实 IPC 测试。

## 人工待验收（尚未验证）

本会话此前真实桌面自动化操作已被自动审批拒绝：Computer Use was not approved to use music-mine-desktop。未绕过此拒绝，也未用持续开发服务器替代真实桌面启动测试。本轮未打开真实用户曲库做迁移实验。

请退出旧实例后用上述最终 EXE 验收：

- 1280×800、1366×768、1920×1080，与 Windows 100%、125%、150% 缩放的组合；主内容宽度、无横向溢出、侧栏/顶栏/播放栏固定，队列列或覆盖抽屉适时切换。
- 真实音乐库 471 首歌曲、封面与统一缺图占位；连续滚动及翻页，长歌名/艺术家/专辑提示，专辑与艺术家卡片；同曲持续播放封面不闪，连续切歌封面正确。
- 长队列独立滚动，头部固定；每个上移、下移、删除、选中均作用于正确文件，包括同名不同路径歌曲；队列展开/收起、关闭按钮、背景关闭和 Escape，音频不中断。
- 所有选择器与歌曲右键/操作菜单：深绿色外观、打开方向、视口边缘、互斥、外部点击、方向键、Enter、Escape、Tab、焦点恢复，弹窗内歌单选择不被裁切。
- 搜索默认/悬停/焦点/清除状态、Ctrl+F、中文输入法组合输入及窄窗口提示对齐。
- 播放、暂停、Seek、音量、歌词、页面切换、快捷键、托盘、SMTC、关闭窗口行为、重启后队列偏好与封面；真实 IPC 往返与音频连续性。

以上未获人工确认，不以自动测试代替验收。07.3 未开始。
