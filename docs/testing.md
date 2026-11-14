# 测试与验收

三种结果分开写。上一级通过不能写成下一级通过。没跑写 `NOT RUN`。

| 层次 | 命令 | 能证明 | 不能证明 |
|---|---|---|---|
| 自动测试 | `pnpm check`、`pnpm test`；Rust 侧 `cargo fmt --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test` | 类型、纯函数、从源码抽出的交互函数、Rust fixture | 布局、真实 IPC、Windows DPI |
| 浏览器验证 | `pnpm test:ui`，必要时 `pnpm test:ui:update` | harness 里的点击、键盘、主题、视口和截图 | Rust 出声、真实数据库、WebView2、DPI |
| Windows Tauri 实机 | `pnpm tauri build --debug --no-bundle`，再启动 `src-tauri/target/debug/music-mine-desktop.exe` | WebView2、真实 command、窗口尺寸、隔离目录里的持久化 | 未操作的硬件、托盘、DPI 或曲库规模 |

`pnpm test` 是 `node --experimental-strip-types --test tests/*.test.mjs`。它不包含 `tests/ui` 和 `tests/perf`。

Playwright 已安装：`@playwright/test`，配置在 `playwright.config.ts`。浏览器是本机 Playwright Chromium，视口 1280×800，`deviceScaleFactor` 为 1，时区 `Asia/Shanghai`。截图在 `tests/ui/snapshots/`。WebdriverIO 和 Tauri WebDriver 没有安装。

harness 设置 `window.isTauri`，因此壳层会显示连接成功。这只说明夹具回答了 `bridge_probe`。播放位置停在夹具给出的时间，不会自己前进。

## 桌面启动

使用构建出的 exe。Craft Anything 的持久开发服务器和 `pnpm dev` 只能预览界面。

隔离用户数据的步骤在 `docs/rust-guidelines.md`。只改 `APPDATA` 会打开正在使用的曲库和设置。

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
