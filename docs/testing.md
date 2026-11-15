# 测试与验收

三种结果分开写。上一级通过不能写成下一级通过。没跑写 `NOT RUN`。

| 层次 | 命令 | 能证明 | 不能证明 |
|---|---|---|---|
| 自动测试 | `pnpm check`、`pnpm test`；Rust 侧 `cargo fmt --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test` | 类型、纯函数、从源码抽出的交互函数、Rust fixture | 布局、真实 IPC、Windows DPI |
| 浏览器验证 | `pnpm test:ui`，必要时 `pnpm test:ui:update` | harness 里的点击、键盘、主题、视口和截图 | Rust 出声、真实数据库、WebView2、DPI |
| Windows Tauri 实机 | 按 `docs/isolation.md` 构建并启动。普通 debug exe 使用系统数据目录 | WebView2、真实 command、窗口尺寸、隔离目录里的持久化 | 未操作的硬件、托盘、DPI 或曲库规模 |

`pnpm test` 是 `node --experimental-strip-types --test tests/*.test.mjs`。它不包含 `tests/ui` 和 `tests/perf`。

Playwright 已安装：`@playwright/test`，配置在 `playwright.config.ts`。浏览器是本机 Playwright Chromium，视口 1280×800，`deviceScaleFactor` 为 1，时区 `Asia/Shanghai`。截图在 `tests/ui/snapshots/`。WebdriverIO 和 Tauri WebDriver 没有安装。

harness 设置 `window.isTauri`，因此壳层会显示连接成功。这只说明夹具回答了 `bridge_probe`。播放位置停在夹具给出的时间，不会自己前进。

## 桌面启动

使用构建出的 exe。Craft Anything 的持久开发服务器和 `pnpm dev` 只能预览界面。

隔离用户数据的步骤只维护在 `docs/isolation.md`。只改 `APPDATA` 会打开正在使用的曲库和设置。

便携包：`powershell -File scripts/build-portable.ps1`。调试用的同一配置加 `-Debug`，或 `pnpm portable:debug`。产物是 `dist-portable/Music-Mine-Portable-x64.zip`。这份配置只在传入 `--config src-tauri/tauri.portable.conf.json` 时合并，普通 `pnpm tauri dev` 和 `pnpm tauri build` 仍使用系统数据目录。

## 按改动选择证据

- 纯 CSS：`pnpm check` 或 `pnpm build`，加上受影响页面的浏览器验证。
- TypeScript / Svelte：相关 `pnpm test`、`pnpm check`，界面行为用 `pnpm test:ui` 或同等浏览器操作。
- Rust：相关 `cargo test`、fmt、clippy。
- IPC：类型和 harness 一起改，并在 exe 里调用一次。
- 音频设备或数据库迁移：fixture 或临时库，加上对应实机或迁移结果。真实用户库不参与。

## 测试写法

优先断言输入输出、DOM 行为和计算样式。可以从源码抽出真实函数再执行。

不要新增只匹配空格、类名拼写或临时变量名的断言。这类断言已在 `5a7c9b3` 里删过一批。仍会读源码的检查包括：主题色允许列表、Modal 接线、层级 token、删除歌单的确认顺序、从 `App.svelte` 编译封面效果。它们仍可能因格式化失败。不要再增加这一类。

禁止把实现改回旧代码，只为了让过时字符串测试通过。

## 完成报告

```text
基线：branch / HEAD
目标：
修改文件：
自动测试：PASS / FAIL / NOT RUN
浏览器验证：PASS / FAIL / NOT RUN（视口、主题、操作）
Windows Tauri 实机：PASS / FAIL / NOT RUN（exe 路径、数据目录、操作）
未验证：
Git：未提交 / 已提交的 SHA
```

## 验收来源

新任务在验收开始前运行 `powershell -File scripts/worktree-fingerprint.ps1`。验收命令结束后、写入记录之前再运行一次。两次不同则这次验收无效。

指纹包含已暂存差异、未暂存的受跟踪差异，以及 `git ls-files --others --exclude-standard` 列出的未跟踪文件的相对路径和内容。已忽略的临时产物不计入。即将写入的验收记录可以用 `-Exclude` 排除，避免记录自己把指纹改掉。

09.5.3 和 09.5.5 没有保存指纹。写「未记录，无法重建」。不要用后来的工作区补算后填进这两份记录。
