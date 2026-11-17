# 任务：全量架构、代码质量与文档一致性深度 Review

## 基线

- 分支与 HEAD：`main` / `d5f69abf2da382d58822b99e0e1293a152ad9669`。
- 未提交改动：开工时工作区干净。
- 工作区指纹：`eb1ed01dcd47151b5fa5cd342d12357d7c1fc5c95a57736d9ee5ca83a76d17a3`。最终验收前执行 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-25-FULL-REVIEW.md,docs/tasks/done/TASK-25-FULL-REVIEW.md,docs/current-state.md,docs/architecture.md`；验收前、自动门禁后、UI结束后、归档后均相同。排除项仅为报告自引用，不排除源码/测试/其他文档。

## 状态

- 状态：完成（审查报告与本轮修复；R11–R13 保留，非全项架构/实机放行）
- 依赖与前置任务：当前源码中的 Console、ListenBrainz、AI、字体预设和旧版导入移除。
- 下一步：R11 Controller I/O 解耦设计、R12 完整后台退出及隔离 exe/真实外部服务验收；本报告不自动创建新任务或提交。
- 提交：待提交；用户要求不自动 commit。
- 遗留事项的任务 ID：既有实机验收缺口沿用 current-state；本轮新增项记录在本单。

## 范围

- 要改：审查发现的可证实缺陷、回归测试及当前架构/IPC/README/现状文档。
- 不要改：用户数据、主 Tauri 配置、音频解码算法/播放命令 API 与 revision、历史任务基线和无关格式。审查中发现 PCM 阻塞锁和错误回调日志，纳入本任务修复；完整门禁复现暂停时钟竞态后，增加工作线程回调屏障，保留严格断言并增加受控回归。
- 是否涉及音频、数据库、IPC 或用户数据目录：审查 Rust、数据库调用边界及 IPC；不打开真实用户数据，不迁移数据库。
- 计划：先追踪锁、HTTP、日志与返回 DTO，再检查组件销毁/异步过期、40 组主题字体组合，最后按源码刷新文档并运行三项指定门禁。

## 验收

- [x] 自动测试：PASS，format 与 check.ps1 退出0；前端95、Rust单元118/1 ignored、音频3、歌词6。
- [x] 浏览器验证：PASS，68/68、退出0，既有快照零像素差异，无基准更新。
- [ ] Windows Tauri 实机：NOT RUN
- [x] 未打开用户正在使用的曲库和设置

实机必须使用构建出的 exe。会写数据时按 `docs/isolation.md`。不要修改 `src-tauri/tauri.conf.json`。

## 来源

- 任务 ID：TASK-25-FULL-REVIEW
- 时间：2026-11-17
- HEAD：`d5f69abf2da382d58822b99e0e1293a152ad9669`
- 工作区指纹：同上。
- 命令、工具版本、构建配置：Node 24.14.0、pnpm 12.21.0、Rust/cargo 1.99.0；Windows x86_64-pc-windows-msvc；check.ps1 使用 MUSIC_MINE_NULL_OUTPUT=1。Playwright 为 Chromium / deviceScaleFactor=1；无基准更新。
- 产物哈希：未构建 exe。
- 三层结果和未验证项：自动 PASS、浏览器 PASS、Windows Tauri 实机 NOT RUN；真实外网 Scrobble / AI、Windows WebView2 / DPI / 硬件属于实机外部依赖项。

## 结束报告

- 改动文件：27 个（含本报告）：AGENTS/README 与 docs 的 architecture/current-state/ipc-contract/rust-guidelines/frontend-guidelines；Cargo.toml/lock、Rust lib/audio.player/catalog.library/commands.catalog/infrastructure.logging/platform.desktop/scrobble/ai_agent；ConsoleModal/AiSettings/ListenBrainzSettings/HomeView/SettingsView/NowPlaying 与 library store；mock-ipc 和 full-review.spec.ts。完整修改以 git diff/status 为准。
- 三条验收结果：自动 PASS（format/check均0），浏览器 PASS（UI 68/68、0、快照零差异），Windows Tauri 实机 NOT RUN。
- 未验证项：R11–R13 与本报告实机/外部依赖清单。主配置/权限/快照基准无改动，未访问真实数据。
- Git：不自动提交。
- 完成后把本记录移到 `docs/tasks/done/`。

## 审查方法与结论边界

先读 AGENTS、current-state、architecture、ipc-contract、rust-guidelines、frontend-guidelines、任务模板，再读 testing/isolation。以源码追踪 command→锁→I/O→返回 DTO；对照前端 service、mock、命令清单；检查组件/effect/轮询/事件清理、依赖 feature 图、SQL 参数与 UI 文本渲染。没有打开旧播放器或真实用户数据，没有调用真实外网服务，也没有读取历史任务来替代当前证据。

本报告是**有保留的审查结论**：下列可局部修复项已修改并加入相关验证；Controller 锁内 I/O 与完整进程退出仍有未闭环项，不能声称四维度全部零风险或 Windows 发布放行。审查报告归档不代表这些债务已修复。

## 分级发现与处理

| ID / 级别 | 发现、机制与证据 | 本轮处理 / 验证 |
|---|---|---|
| R01 / P1 | audio/player.rs 的 fill_output 使用阻塞 ring/capture Mutex；诊断采样每次 callback 分配/扩容，cpal error callback 同步调用 LogBuffer（stderr/锁及可选文件输出） | 改用 try_lock，ring 竞争填静音、计 underrun 且不消耗队列；capture 只写预分配容量；错误回调只置 stream_dead，由既有 worker 记录。callback_never_waits_for_producer_or_diagnostic_locks 在持锁时要求回调完成，callback_capture_stops_at_preallocated_capacity 验证不增长；既有 poison、音量与采样测试保留。硬件断音/拔插 NOT RUN |
| R02 / P2 | desktop.configure_output 由 lib.rs 持有 Controller 后调用，跨越 desktop-v1.json 写盘；同时还持有偏好锁 | 增加 output_operation 串行设备设置，执行动作后释放 Controller，再合并/保存当前偏好；失败重取 Controller 回退。既有保存/失败回退测试适配，共享偏好不会被旧副本覆盖。Controller 自身 persist 仍见 R11 |
| R03 / P2 | desktop_preferences 查询也写盘，导致诊断/设置读取触发文件 I/O；配置保存原始系统错误不必要暴露 | 参数全空只读；只在修改请求时保存；文件错误固定码/中文信息，不回显主题输入。Preferences.public 固定清除两种密钥，新增 public_preferences_never_serialize_credentials |
| R04 / P2 | 公共播放/Catalog 错误可能透出绝对目录；扫描完成 LogBuffer 写盘仍持 progress 锁 | 复用 logging.redact_paths 到 execute/work 错误边界；扫描只在锁内生成统计文字，锁外写日志。保留本地功能有意使用的 track/path/扫描目录契约，不假称所有 DTO 都无路径 |
| R05 / P2 | ListenBrainz 两个常驻线程没有 join handle；AI 退出后可继续返回响应 | Scrobbler 保存 observer/network handles，ExitRequested 现有 exiting 标志停止循环，RunEvent::Exit 收拢线程。AI 拒绝退出后新请求与响应。网络超时/退避和账户队列语义保留；正常退出的 WebView2/网络竞争仍 NOT RUN，主控制线程与在途 AI 见 R12 |
| R06 / P2 | Home 的 load 可被旧结果/错误覆盖；Settings 初始/保存/设备异步结果在离开后仍写状态/弹通知；watchScans 在销毁后仍应用进度 | Home 每轮 AbortSignal 丢弃旧结果，Settings disposed 检查，scan poll 接收后检查销毁。新增实际挂起 IPC 的 Home/Settings 浏览器回归 |
| R07 / P2 | NowPlaying 歌词切歌票据没有销毁清理，旧失败通知未过滤；收起动画暂停 effect，销毁清理尚未来得及执行 | effect 取消标志 + 回调核对 ui.immersive，音频参数/ResizeObserver 保留清理，图片 decode/滚动 tick 丢弃过期任务。浏览器测试真实复现“关闭后晚到歌词失败”，修复后通过 |
| R08 / P2 | AI/ListenBrainz 设置组件的读取已有保护，但保存/开关/清除与异步 DOM checkbox 回写无销毁保护 | 统一抑制销毁后返回结果/错误/checkbox 写入，销毁清空输入凭证；Console 补复制/错误/会话 generation 检查。已在途 invoke/写入不谎称被撤销，工具闭环与部分失败协议保持 |
| R09 / P2 | Symphonia 未禁用默认集合，隐式包含未提供入口的 Ogg/MKV/Vorbis；未使用 Library 容器依赖 allow(dead_code) | default-features=false，显式 mp3/flac/wav/pcm/adpcm，保留此前默认带来的 ADPCM WAV 能力；锁文件移除三项无用包。删除未使用 Library。ureq 原本已仅 json/tls、rusqlite 仅 bundled，未升级依赖、未重写解码器 |
| R10 / P2 | ListenBrainz 推荐逐候选×全库反复创建归一化字符串；架构/README/约定与近几轮功能漂移 | 本地曲目归一化一次，候选键索引保留首个匹配、远端顺序/去重/删除后重匹配语义。更新数据流、代码地图、41/39/2 IPC 清单说明、服务 DTO/隐私、外部依赖与构建指南；mock 补全公开偏好字段/空密钥。AGENTS 两条旧规则修改前已说明旧/新/原因 |
| R14 / P1 | 最终首轮 check 退出101，音频集成出现 tone.flac 暂停后150→155ms：回调已越过 paused 检查，worker 先确认 Pause，回调稍后才 fetch_add 最后一块帧数 | CallbackGuard 原子进入/退出，worker pause_callbacks 先暂停再等在途回调完成，用于 Pause/Stop/时钟重置/输出切换释放；回调不等待、不加锁。受控同一回调临界区测试在移除屏障时明确失败（pause acknowledged before in-flight callback finished），恢复屏障后通过；原集成严格断言不变并通过。首版通过竞争 ring 观测回调的测试会干扰 try_lock，已改成受控 guard，避免把测试调度当修复证据 |

## 核查通过及保留项

- **架构/网络**：scrobble 与 AI HTTP/响应体读取期间未持 Controller、Catalog 操作/进度或偏好锁；SQLite outbox 读取后发请求，成功后短事务确认。账户切换/禁用、配置 revision、限流/退避、有界请求/响应、禁止重定向仍在。
- **安全**：listenbrainz_config / ai_config 只返回凭证是否配置，desktop_preferences 两个密钥字段固定 null。固定网络错误不返回服务正文/URL/头，AI 仅标准字段，拒绝工具参数反射 Key；日志网络路径只记录固定码/计数并在所有日志出口前替换凭证。现有 Rust 模块测试覆盖这些边界。
- **前端**：Console 串行2秒轮询清理、AI AbortSignal/配置版本/5轮上限、允许工具参数检查和部分写入结果保留；Home 推荐 effect 已有 alive 清理；NowPlaying 参数查询/ResizeObserver、字体 storage、原生菜单与窗口事件监听均有对应清理。AI 工具没有定时器、监听、任意 IPC 或 shell 分发。
- **样式**：未新增产品颜色或改视觉基准。既有 sourceStyles 全 CSS/Svelte 检查与 Token AA 测试保留；新增 4 页面 × 2 视口 × 40 组合检查（320状态），覆盖主体/次级/Console 元文字/主按钮 Token 对比度≥4.5、容器横向溢出和边界。不是逐个像素/所有文本状态的全站 WCAG 认证；字体缺失时回退仍属于真实环境限制。
- **死代码/契约**：源代码/service/mock 中无旧版导入命令/入口；数据库 schema 升级不是旧导入残留，不删除。两个 IPC-unused 例外 library_cover/library_favorite 保留兼容。名称集合测试不代表 mock 的全部网络/校验语义等价，文档明确列出边界。

| 剩余 ID / 级别 | 事实与后续要求 |
|---|---|
| R11 / P1 | Controller::execute/attach_session/flush_session 仍在串行锁内读取标签/会话并保存 playback-v1.json，State 可处理 EOF。Catalog initialized/operation 锁刻意保护 DDL/事务。严格“Controller/Catalog 所有锁内零磁盘重 I/O”未满足。应单独设计会话/标签 I/O 解耦，保留 revision、EOF、错误与落盘顺序；本轮不冒险改播放状态机 |
| R12 / P2 | player-controller 仍依 exiting 停止但未 join；在途 AI spawn_blocking 不能强制取消。ListenBrainz join 改善了其线程收拢，不能推导所有后台任务都完全清理；Windows 实际退出、超时、媒体/托盘资源竞态需隔离 exe |
| R13 / P2 | 凭证仍明文落在 desktop-v1.json；LogBuffer 文件出口若启用会保留原路径，当前 Controller::new 使用 Player::new(None)，播放器未启用该文件出口；用户配置远端 HTTP AI 时链路不加密。公开 DTO/Console 脱敏不等同静态加密、磁盘路径全脱敏或 HTTPS 强制。后续系统凭据库/错误结构化需独立迁移方案 |

## 门禁与实机证据

- 最终 `pnpm format`：退出0；日志 `tests/perf/out/task25-format-final.log`，退出码文件 `task25-format-exit.txt`。
- 最终 `pwsh -File scripts/check.ps1`：退出0；format:check、Svelte 0 errors/0 warnings、前端95/95、Rust fmt/clippy -D warnings、单元118 passed/1 ignored、音频3/3、歌词6/6。Null输出，不证明硬件出声；既有 Cargo PDB 文件名冲突警告仍保留。日志 `tests/perf/out/task25-check-final.log`、退出码文件 `task25-check-exit.txt`。
- 最终 `pnpm test:ui`：退出0，68/68（原有62+新增6）通过，1.3分钟。Playwright maxDiffPixels=0 / threshold=0，所有既有截图通过且 `git diff --exit-code -- tests/ui/snapshots` 为0，无更新基准。新增320组合状态检查；已查看4张900×640页面截图。日志 `tests/perf/out/task25-ui-final.log`、退出码文件 `task25-ui-exit.txt`。
- 前置证据：前端95/95、首轮 Rust115/1 ignored/音频3/歌词6通过；之后补充2项回调测试定向4/4通过。前置完整UI68/68通过。不能将前置结果替代最终指纹门禁。
- 保留失败日志 `task25-check-pause-failure.log`（暂停修复前指纹 d2c2e8af1f18eafc8da8cbecc07b4cde285ea1a45d8f25878a9234b7d5ef43aa，退出101，117单元通过/1 ignored后音频2/3）；暂停受控红/绿日志 `task25-pause-controlled-red.log` / `task25-pause-controlled-green.log`，音频修复后定向 `task25-audio-fixed.log`。不放宽断言，不把通过的复跑当根因证明。
- 调试记录：新UI用例先因错误导航名称超时；纠正后复现收起动画期间旧歌词错误，增加打开状态检查后定向通过。沙箱内 Playwright 用例结束后 Windows 子进程树回收挂起，终止了本轮自己启动的测试 session；经工具批准在沙箱外跑完整门禁。未按进程名结束用户播放器。
- Windows Tauri 实机：NOT RUN；无当前构建 exe/来源记录，不启动未知来源产物。真实外网 ListenBrainz/AI、硬件音频/设备拔插、125%/150% DPI、WebView2 字体、托盘/SMTC/任务栏、原生选择/扫描、库外打开/WAV经UI打开均保持 NOT RUN。
