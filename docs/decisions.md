# 技术决策

只记录会影响后续多个任务的选择。

## ADR-001 — Rust + Tauri v2 + Svelte 5

- 状态：已采用。代码可见。
- 决定：Rust 负责音频和曲库，Svelte 5 负责界面，Tauri 负责窗口和 IPC。
- 后果：接受 WebView2 的成本。播放事实仍在 Rust。

## ADR-002 — 先修测试，再谈组件迁移

- 状态：测试部分已做。组件迁移未开始。
- 决定：先用行为测试和截图基线稳住界面，再单独考虑通用组件。
- 证据：`5a7c9b3` 替换了格式化即失败的字符串断言。`9f141df` 加入 Playwright 和基准截图。

## ADR-003 — 用行为而不是源码拼写做测试

- 状态：已采用。
- 决定：纯函数做单测，交互做浏览器操作，Windows 专有能力用构建出的 exe。
- 后果：仍有少量检查会读取源码。不再新增只匹配空格或类名的断言。

## ADR-004 — 不自动提交

- 状态：已采用。替代旧规则「after any modification, finish git commit」。
- 决定：任务结束时报告 diff 和三种验收结果。只有当前任务明确要求时才 `git commit` 或 `git push`。
- 原因：失败和误改会被自动提交带进历史。用户数据隔离也需要先看 diff。

## ADR-005 — 通用组件后续用 shadcn-svelte 与 Bits UI

- 状态：方向已记录。包未安装，迁移未开始。
- 决定：将来的通用交互优先 shadcn-svelte；焦点、键盘和浮层原语用 Bits UI。外观仍用本项目 tokens。播放器业务组件保持自定义。
- 约束：没有单独的迁移任务时，继续用手写组件。初始化命令不得覆盖 `src/app.css`。
- 试点顺序：只先做 `src/lib/Select.svelte`。浏览器和 Windows 窗口都通过后再决定下一项。

## ADR-006 — 一份根约定，按模块展开

- 状态：已采用。文件在本仓库，尚未推送。
- 决定：根 `AGENTS.md` 放安全边界和完成标准。Rust、IPC、前端、测试分文件。`CLAUDE.md` 只引用 `AGENTS.md`。

## ADR-007 — 保留现有 IPC 错误类型

- 状态：已记录，代码未改。
- 当前事实：命令返回 `Result<T, String>`，TypeScript 类型手写在 `music.ts`。
- 决定：新的复杂接口可以另用结构化错误。现有命令不批量改签名。

## ADR-008 — Windows 数据目录隔离

- 状态：已采用。
- 事实：Tauri `app_data_dir()` 使用系统已知文件夹。进程 `APPDATA` 不会重定向它。
- 决定：实机实验使用临时 `app.appDirectoriesOverride`，验收后删除并重新构建。
- 原因：未隔离启动会写入用户的 `desktop-v1.json`、播放会话和曲库。

## 新决策模板

```markdown
## ADR-NNN — 标题
- 状态：提议 / 已采用 / 已弃用
- 决定：
- 替代：
- 后果：
- 验证：
```
