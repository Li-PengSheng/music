# 任务：Console 本地命令与 AI 输入区区分

## 基线

- 分支与 HEAD：main / b2c35e73d6814677639ac413cdb4370ccac26716。
- 未提交改动：保留TASK-22/23全部未提交改动、归档记录与既有npm.cmd删除；增量修改Console及相关UI测试/说明。
- 工作区指纹：`6d653ca07fc2f456548f4223e11f1b77298c36101feacf128a2e594b203d6b7d`，最终验收前后相同；排除本单active/done与docs/current-state.md。

## 状态

- 状态：完成
- 依赖与前置任务：已有Console视图切换、aiConfig和自然语言调用闭环。
- 下一步：归档并停止；Windows实机与真实服务未运行。
- 提交：待提交，不自动commit。
- 遗留事项的任务 ID：已有实机/外网缺口保持，本轮不扩展。

## 范围

- 日志页：Terminal、本地短命令/仅执行内置控制指令、指定短命令占位符、等宽输入、次按钮“执行”。
- AI页：Sparkles、AI对话指令、当前模型/未启用徽章、指定自然语言占位符、正文输入字体、accent-line柔和边框、主按钮“发送/发送中…”。长模型名可截断并通过title完整查看，标题允许换行。
- 日志页自然语言自动切AI，AI页提交统一走对话；日志短命令保留原实现。通过现有诊断轮询读取只读AI配置，关闭后不接收旧响应，无新增计时器或invoke位置。
- 只改Console UI、相关测试和行为说明；不改Rust、IPC签名、音频、数据库、用户数据、权限或既有快照。

## 验收

- [x] 自动测试：PASS。pnpm format、pwsh -File scripts/check.ps1均退出0；Svelte 0 errors/0 warnings，前端单元95/95，Rust fmt/clippy通过、单元114 passed/1 ignored、音频集成3/3、歌词集成6/6，Null输出。未改Rust/音频测试，本轮没有重现上一任务的暂停时钟失败；不据此声称修复该问题。既有Cargo PDB名称冲突警告保留。
- [x] 浏览器验证：PASS。pnpm test:ui 62/62退出0，既有快照零像素差异，无更新。新增3项输入区回归，覆盖两种标题/说明/占位符、正文与等宽字体、强调色边框、模型/未启用徽章、次/主按钮、发送中状态、日志自然语言自动切AI并完成工具执行，以及AI页不绕过对话直接执行短命令。1280×800/900×640、浅色/深色、超长模型名均通过，查看窄窗截图无横向溢出。原有11项AI/Console交互一起定向复跑，共14/14通过。
- [ ] Windows Tauri 实机：NOT RUN。本轮纯前端改动，仅使用浏览器夹具，不启动未核验的现有exe，不打扰之前记录的已有debug进程。真实WebView2/DPI、外部AI均未验证。
- [x] 未打开用户正在使用的曲库和设置

## 来源

- 任务 ID：TASK-24-CONSOLE-INPUT-DISTINCTION
- 时间：2026-11-17
- HEAD：b2c35e73d6814677639ac413cdb4370ccac26716
- 工作区指纹：同基线；命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-24-CONSOLE-INPUT-DISTINCTION.md,docs/tasks/done/TASK-24-CONSOLE-INPUT-DISTINCTION.md,docs/current-state.md`。三项门禁前后和归档前后相同。
- 命令、工具版本、构建配置：pnpm12.21.0、Node v24.14.0、Playwright1.65.1、Cargo1.99.0、PowerShell7.6.10。新增UI回归先因没有“本地短命令”标签失败，再实现后通过。旧跨模式测试改用稳定输入ID，专门的新测试验证可访问标签随视图切换。最终三项指定命令退出0，无快照更新。
- 产物哈希：无新exe构建。忽略产物 tests/perf/out/task24-format-final.log、task24-ui-final.log、task24-check-final.log；窄窗截图 test-results/console-input-ai-narrow.png 已查看。
- 三层结果和未验证项：自动PASS，浏览器PASS，Windows Tauri NOT RUN；仅使用内存配置/模拟模型，未访问真实用户数据或外部AI。

## 结束报告

- 改动文件：ConsoleModal.svelte；tests/ui/console-input.spec.ts、ai-console.spec.ts、menu-console-specs.spec.ts；IPC行为说明、本单和current-state。已有TASK-22/23改动与npm.cmd删除保留。
- 三条验收结果：见上，三项门禁全部0、既有快照不变。
- 未验证项：Windows WebView2/DPI、真实AI服务；不打扰已有debug进程。
- Git：不提交，完成后归档并更新current-state。
