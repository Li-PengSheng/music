# 任务：可选 AI Console 自然语言与安全工具调用

## 基线

- 分支与 HEAD：main / b2c35e73d6814677639ac413cdb4370ccac26716。
- 未提交改动：开工时工作区干净。此前 current-state 的未提交说明已落后于当前 HEAD。
- 工作区指纹：最终验收前后、最终隔离实机后为 `93e1064f25f3776e839d2b372945133aec9a5427065296d3ca7e03d7905f9caf`。排除本单 active/done 和 docs/current-state.md。

## 状态

- 状态：完成
- 依赖与前置任务：现有 Console、Desktop 偏好、ureq 2.12.1/Rustls 与播放器/曲库服务。
- 下一步：归档并停止；真实外部服务验收保留为 NOT RUN。
- 提交：待提交；禁止自动 commit。
- 遗留事项的任务 ID：TASK-22 真实第三方模型服务/真实凭证验收。

## 范围

- 本次用户明确授权 AI 接入，覆盖 AGENTS.md 原“不接入 AI 服务”的产品范围；不修改 AGENTS.md。
- 默认关闭。凭证仅存 desktop-v1.json；ai_config 只返回 has_key，不向 WebView/localStorage 返回 Key。旧偏好默认 OpenAI / gpt-4o-mini；可配置兼容端点与模型，不承诺各服务的非标准扩展。
- 复用 ureq/Rustls，无新增依赖。Rust spawn_blocking 代理单轮 Chat Completions，HTTP(S) 严格解析、禁止 URL 内凭证/查询/片段、禁止重定向，30 秒超时，限制请求/响应体与消息/工具数量；不持有 Controller/偏好锁等待网络。日志只记录计数、允许的工具名与脱敏错误码。
- Console 保留本地命令，明确的错误短命令仍给本地用法；自然语言才进入 AI。最多 5 次模型请求，每轮工具数量有界，串行执行允许列表中的六种工具；执行前验证参数与曲目 ID，播放通过 control/accept 接收 Rust 快照。只返回必要的曲库元数据，不把本地路径、封面、歌词、日志或密钥发给模型。
- 对话只留当前 Console 会话内存，支持日志/对话切换、停止后续工具和清空对话。关闭弹窗停止后续轮次/工具；已在途请求或本地操作可能完成，不在关闭后继续操作。中断时保存完成/部分完成的工具结果，未执行调用补 not_executed，保持后续消息配对；显式清空以代数标记忽略旧响应，配置变化后不向新服务传旧上下文。
- 不提供 shell、任意网络、删歌/删除文件、修改配置密钥工具。现有数据库/音频引擎与能力权限不变；测试只用 fixture 和模拟/本机 HTTP，不调用真实外网 AI。

## 验收

- [x] 自动测试：PASS。最终 pnpm format、pwsh -File scripts/check.ps1 均退出 0。Svelte 0 errors / 0 warnings；前端 95/95（新增10项工具/循环/歌词事件测试）；Rust fmt/clippy通过，单元115 passed / 1 ignored（新增6项配置、持久化、URL、密钥返回、响应/消息边界测试），音频集成3/3、歌词集成6/6，Null输出。无真实外网请求。
- [x] 浏览器验证：PASS。最终 pnpm test:ui 58/58，退出0；新增5项AI配置保存/清除/默认关闭、短命令不联网/自然语言引导、工具闭环/最终回复、关闭后取消、歌单已创建但后续网络失败仍保留上下文的测试。1280×800 / 900×640；查看设置卡片和对话截图，无横向溢出。既有快照零像素差异，无任何快照更新。
- [x] Windows Tauri 实机：限定范围 PASS。最终哈希绑定隔离 exe + 新建三首 fixture 曲库 + 本机 HTTP 模拟服务。在真实 WebView2 中保存/开启/清除配置，验证 Key 文件持久化但查询/desktop_preferences/日志不暴露；三轮真实 Rust HTTP 代理和六种工具闭环完成检索/概况、创建2首歌单、替换队列开始Null播放、音量30%、外观/字体和桌面歌词。服务校验 URL、Authorization、User-Agent、model/tools/stream，工具数据无 path/folders。另一次延迟模型响应在关闭 Console 后没有执行后续音量工具。1280×850 /100%，实例已关闭；仅本机模拟服务，不等同外部服务成功。
- [x] 未打开用户正在使用的曲库和设置

## 来源

- 任务 ID：TASK-22-AI-AGENT-CONSOLE
- 时间：2026-11-17
- HEAD：b2c35e73d6814677639ac413cdb4370ccac26716
- 工作区指纹：同基线；命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-22-AI-AGENT-CONSOLE.md,docs/tasks/done/TASK-22-AI-AGENT-CONSOLE.md,docs/current-state.md`。首轮开始为 `28a216706a69ffefd3728f993ddf038200ea903cca89f4852255adec7db09477`；验收过程中发现与任务无关的已跟踪 npm.cmd 被删除，未恢复/修改该差异。额外排除 npm.cmd 后仍为首轮指纹，确认仅此差异；以当前完整状态重新执行三项门禁，前后均为最终指纹，不用首轮不一致结果作为最终门禁来源。
- 命令、工具版本、构建配置：pnpm12.21.0、Node v24.14.0、Playwright1.65.1、Cargo1.99.0、PowerShell7.6.10。格式/UI/check最终全部0。RED->GREEN：初始URL和默认配置缺失，工具与循环缺失；只读审阅发现歌词自身状态事件误判取消、已完成写入在后续网络失败时丢失历史，两项均先失败复现后修复。复审无遗留Critical/Important。既有Cargo PDB名称冲突警告保留。
- 构建配置：门禁后使用 `pwsh -File scripts/build-isolated.ps1 -Config <测试根>/isolation.json -TestRoot <测试根>`，底层 `pnpm exec tauri build --debug --no-bundle --config <测试根>/isolation.json`。最终测试根 `%TEMP%/music-mine-task22-final-1d88579dfdde4c1ba7f3ab9dd9293357`；appDirectoriesOverride仅CLI合并到该根/data，未改主配置。检查前调用Test-IsolationSource，关闭前核对PID/启动时间/exe身份。Cargo测试会重写debug exe，最终实机使用门禁后重建产物。
- 产物哈希：`src-tauri/target/debug/music-mine-desktop.exe` SHA-256 `74BBC18819CE8F9E106D8848FE71EC63F3610F741150FC204E7EE1EDE95A305D`；配置 SHA-256 `B8D99F7797E83115CE663F7E67D5D01817A6892B206F5BFC95783D0C53556B6C`。来源记录 `tests/perf/out/isolation-sources/74BBC18819CE8F9E106D8848FE71EC63F3610F741150FC204E7EE1EDE95A305D.json`（忽略产物，absolute-override）。
- 证据产物：tests/perf/out/task22-{format,ui,check,build}-final.log；最终测试根 desktop-result.json、ai-settings-real.png、ai-chat-real.png、process.json、stderr.log。本机模拟模型接收4次请求（3次正常闭环、1次取消），无真实模型Key或账户。首轮实机另有隔离产物，最终记录以上述根和哈希为准。
- 三层结果和未验证项：自动 PASS，浏览器 PASS，Windows隔离本机模拟限定范围 PASS。外部OpenAI/Gemini/DeepSeek/OpenRouter/实际Ollama服务、真实密钥/外网TLS、硬件出声及125%/150%DPI仍NOT RUN。兼容标准格式不等同已验证所有厂商。
- API 依据：https://developers.openai.com/api/reference/resources/chat/subresources/completions/methods/create 与 https://developers.openai.com/api/docs/guides/function-calling 。本机 ureq request_url().as_url() 可严格解析 URL，不新增 url crate。

## 结束报告

- 改动文件：Rust lib.rs、platform/{mod,desktop,ai_agent}.rs；前端 music.ts、ai-tools.ts、ai-conversation.ts、lyrics-overlay.ts、AiSettings.svelte、SettingsView.svelte、ConsoleModal.svelte；harness和3个AI单元测试文件/AI UI测试；IPC、本单、current-state。Cargo依赖/锁、数据库schema、音频引擎、capability、主配置与快照未改。npm.cmd额外删除单独保留，不属于AI实现。
- 三条验收结果：见上；三项指定门禁全部0，最终前后指纹相同。
- 未验证项：真实厂商服务/凭证、硬件出声、DPI。
- Git：不提交。
- 完成后归档到 docs/tasks/done/ 并停止。
