# Music Mine — AI 开发约定

适用于本仓库。这是工程约束，不是产品需求。开工前看 `git status` 和 HEAD。事实优先级：当前代码 > `docs/current-state.md` > `docs/architecture.md` > `docs/tasks/done/` > `docs/RFC-v2.0.md`。RFC 是目标。`docs/tasks/done/` 和 `docs/history/` 是历史交接，不描述最新行为。

技术栈：Rust、Tauri v2、Svelte 5 Runes、TypeScript、Vite、Tailwind CSS v4。Windows 10/11 本地播放器。产品版本以 `package.json` 和 `src-tauri/tauri.conf.json` 的 `0.1.0` 为准。

## 安全边界

- 只改这个仓库。保留旧播放器项目。只有单独授权的迁移任务才能从旧项目提取音频代码。
- 禁止打开、迁移实验或改写用户正在使用的曲库数据库、音乐、歌词和设置。数据在 Tauri `app_data_dir()`：`library-tauri-v1.sqlite`、`playback-v1.json`、`desktop-v1.json`。
- Windows 上进程环境变量 `APPDATA` 不会改变这个目录。需要隔离时按 `docs/isolation.md` 用 CLI 合并配置，不修改 `src-tauri/tauri.conf.json`。没有与 exe 哈希绑定的构建来源记录，就不把那个 exe 当作隔离构建。
- 播放状态以 Rust 快照为准。前端通过 `src/lib/stores/player.svelte.ts` 的 `accept()` 接收带 `revision` 的快照。禁止用 UI 定时器推进进度，或把 Mock 成功说成 Rust 播放成功。
- 前端 `invoke` 只放在 `src/lib/services/music.ts`。
- 产品范围不接入 AI 服务，不重写解码器。这里的范围边界沿用 RFC 的 V1.0，不是 `package.json` 里的 `0.1.0`。格式支持以 `src-tauri/Cargo.toml` 里启用的 Symphonia feature 和已有 fixture 为准。
- 音频回调里不做磁盘、数据库、阻塞锁、无界分配或高频日志。UI 任务不改音频引擎和数据库。细则见 `docs/rust-guidelines.md`。
- 未经当前任务明确授权，不 `git commit`、不 `git push`、不 `git reset --hard`、不 `git clean -fd`。先给出 diff 和检查结果。

## 每个任务

1. 记录分支、HEAD 和未提交改动，保留用户工作。
2. 只读与任务相关的代码和文档。跨音频、数据库、IPC 或大范围 UI 时，先给短计划。
3. 做最小、可回退的修改。不顺手格式化或重构无关文件。
4. 跑与改动相关的检查。三条证据分开写：自动测试、浏览器验证、Windows Tauri 实机。没跑的写 `NOT RUN`。
5. 桌面启动验收使用按 `docs/isolation.md` 构建出的 exe。普通 `pnpm tauri build --debug --no-bundle` 的产物使用系统数据目录，不能用来做会写入曲库或设置的验收。Craft Anything 里的持久开发服务器、`pnpm dev` 和 Playwright harness 都不能代替构建出的 exe。
6. 有架构、契约或决策变化时，只更新对应文档。

按任务打开：Rust / 数据库 `docs/rust-guidelines.md`；IPC `docs/ipc-contract.md`；界面 `docs/frontend-guidelines.md`；验收 `docs/testing.md`；现状 `docs/current-state.md`；取舍 `docs/decisions.md`。

## 组件库

shadcn-svelte 和 Bits UI 没有安装。没有单独的迁移任务时，继续用仓库里的手写组件。禁止在功能任务里运行会覆盖 `src/app.css` 的初始化命令。

## 改这份约定

同一类失误重复出现时，补一条可执行规则或一个测试。改本文件前说明旧规则、新规则和原因。
