# 任务：字体渲染设置、播放次数展示清理与首页/正在播放布局精修

## 基线

- 分支与 HEAD：`main` / `ef60136108699cd30a7cb1d00057b3fa1ee0bada`。
- 未提交改动：保留两轮 TASK-18、TASK-19 全部实现/文档/快照与用户既有 38 项图标/repomix 删除。开工状态与局部文件副本、截图哈希在已忽略的 `.checkpoints/task20/`。
- 工作区指纹：`10772dec321f8ea34658cf04d195ab8e5b9f066568a38839280fcd1d0b570cb8`，最终验收前后相同。排除本单的 active/done 两个路径及 docs/current-state.md，归档不改变验收指纹。

## 状态

- 状态：完成（自动测试、浏览器与限定范围隔离 exe 验收通过）。
- 依赖与前置任务：已有字体 token、专业音频参数与精简侧栏入口。
- 下一步：按用户要求归档本单并更新 current-state 后停止；不自动提交。
- 提交：不自动 commit。
- 遗留事项的任务 ID：TASK-20 的 Windows Tauri/ClearType 与 DPI 实机验证。

## 范围

- 要改：body 移除 antialiased、启用 optical sizing，保留 synthesis/legibility；标题和歌词字距/行高 token；封面下间距与单行规格胶囊；窄窗封面/文字分组；首页艺术家改用曲库曲目数量，零曲目无副标题并居中。
- 清理：删除 AudioInfo.play_count、NowPlaying/TrackDetails 文案与读数、已无消费者的 library-listen-recorded 前后端事件及其专用计数读取。保留 listening_history 与原有 record_listen 写入时序、最近播放和常听艺术家排序；不迁移数据库。
- 不要改：字体预设、播放快照/进度、音频引擎或回调、用户数据库/音乐/歌词/设置、依赖与主 Tauri 配置。
- 涉及业务 IPC 的缩减：AudioInfo 保留六个技术字段，更新 music.ts/mock/Rust/测试/IPC 文档；不扩大权限。真实新契约的 exe 验收若未跑则记 NOT RUN。

## 验收

- [x] 自动测试：PASS。pnpm format 与 pwsh -File scripts/check.ps1 均退出 0；Svelte 0 error / 0 warning，前端 85/85；Rust fmt/clippy 通过，单元 98 passed / 1 ignored，音频集成 3/3，歌词集成 6/6。检查使用空音频输出，既有 Cargo PDB 警告不变。两项 Rust AudioInfo 用例已先失败再通过，验证序列化只有六个文件技术字段。
- [x] 浏览器验证：PASS。pnpm test:ui 最终 49/49，退出 0；新增 4 项锐化设置/艺术家数量/布局测试，替换或移除播放次数旧断言，保留技术参数和切歌过期响应测试。1280×800 / 900×640、鸿蒙浅色/宋体深色、长文本、封面与歌词/纯封面、艺术家零/非零数量均通过；文字上下顺序与控件边界无溢出。规格单行 11px 胶囊可通过 title 查看完整文本。9 张基线确认截图后更新，普通复跑零像素差异。
- [x] Windows Tauri 实机：PASS（限定范围）。哈希绑定的隔离 exe 在独立临时三首仓库 fixture 库中验证：WebView2 computed smoothing=auto / optical sizing=auto / synthesis=none；内部 9001 次收听的艺术家显示 3 首库内歌曲；真实 library_audio_info 在详情呈现六项技术参数，正在播放仅显示技术规格且无次数，11px 单行胶囊完整可见。1280×850 / 100%，空音频输出。ClearType 主观锐度、硬件出声、125%/150% DPI 为 NOT RUN；不把 CSS 属性通过说成这些已验证。
- [x] 未打开用户正在使用的曲库和设置。

## 来源

- 任务 ID：TASK-20-SHARPEN-AND-LAYOUT-CLEANUP。
- 时间：2026-11-17。
- HEAD：同基线。
- 工作区指纹：同基线；命令 `pwsh -File scripts/worktree-fingerprint.ps1 -Exclude docs/tasks/TASK-20-SHARPEN-AND-LAYOUT-CLEANUP.md,docs/tasks/done/TASK-20-SHARPEN-AND-LAYOUT-CLEANUP.md,docs/current-state.md`。快照更新完成后、最终普通 UI/check.ps1 前采集，完成后及归档后相同。
- 命令、工具版本、构建配置：pnpm 12.21.0、Node v24.14.0、Playwright 1.65.1、PowerShell 7.6.10、Cargo 1.99.0；pnpm format、pnpm test:ui、pwsh -File scripts/check.ps1 最终全部退出 0。首轮完整 UI 为 44 passed / 5 个截图用例失败（基线变化）；检查两种视口截图后执行 pnpm test:ui --update-snapshots，49/49，再普通复跑 49/49 验零差异。Playwright 沿用沙箱外执行以正常清理 Vite 子进程。
- 隔离来源：`pwsh -File scripts/build-isolated.ps1 -Config <test-root>/isolation.json -TestRoot <test-root>`；测试根 `%TEMP%/music-mine-task20-a82a50bcc5be4d2094da7fdebd3313ae`，数据 `<test-root>/data`，WebView2 数据 `<test-root>/webview`；CLI 合并配置，未改主配置。启动前 Test-IsolationSource 核对哈希与覆盖目录；MUSIC_MINE_NULL_OUTPUT=1，CDP 9339，结束核对 PID/启动时间/exe 路径后关闭。
- exe：`src-tauri/target/debug/music-mine-desktop.exe`；SHA-256 `31E9EF93EC566A7535C685999F981694ACDB9840142860A0F2F5348CB0DB3B6D`。配置 SHA-256 `F8743CFB49BA385F06C567F0EC9DB8550A052C4FF36ADB066A54AF8FADD916E6`；绑定记录 `tests/perf/out/isolation-sources/31E9EF93EC566A7535C685999F981694ACDB9840142860A0F2F5348CB0DB3B6D.json`。
- 本地验收产物（已忽略）：tests/perf/out/task20-run.json 定位测试根；<test-root>/desktop-result.json、artists-real.png、details-real.png、playing-real.png；tests/perf/out/task20-validate.ps1 与 task20-desktop.mjs 执行本轮实机检查。初次并行 Cargo 测试重写 debug exe，来源校验以 MISSING_RECORD 拒绝启动；检查全部结束后重新独立隔离构建，最终哈希验收通过，未启动不匹配的产物。
- 三层结果：自动测试 PASS；浏览器 PASS；Windows Tauri 实机 PASS（上述限定范围）。只读审阅通过，未发现 Important/Critical 问题；收听历史 SQL、时序、schema 与 HEAD 一致，没有删除内部最近播放/排行数据。

## 结束报告

- 改动文件：app.css、tokens.css、NowPlaying/TrackDetails/HomeView、music.ts、mock；Rust metadata/commands 和 lib 专用事件清理，catalog/db 与 catalog/mod 恢复为无专用读数的原有写入实现，相关测试更新；新增 sharpen-layout.spec.ts 4 项测试、清理旧菜单规格测试的次数断言与监听测试、9 张快照；IPC 文档/current-state/本单。无依赖/主配置/音频引擎或回调变更。
- 三条验收结果：自动测试 PASS；浏览器 PASS；隔离 Windows Tauri 实机 PASS（限定范围）。git diff --check 通过；用户既有 38 项删除与其他任务工作已保留。
- 未验证项：Windows Tauri、真实字体栅格化与 DPI。
- Git：未提交；完成后按用户要求移至 docs/tasks/done/，更新 current-state 后停止。
