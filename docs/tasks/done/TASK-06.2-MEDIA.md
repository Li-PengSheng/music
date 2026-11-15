# 06.2 Windows 媒体控制与快捷键

开始前检查点：现有项目内 `.checkpoints/task061/.git`，提交 `21fa4cb`。Craft 配置 `app_in_root: true`，本轮继续修改根目录应用，未另建 app/ 或访问旧工程。

## 实现

- `src/lib/shortcuts.ts` 集中判定七个窗口快捷键；App 使用一个由 Svelte 管理生命周期的 window 监听。输入框、textarea、select、contenteditable 不触发播放/切歌；Space 在按钮、链接等保留默认行为。忽略长按、IME 合成、额外修饰键与已处理事件。Ctrl+F 聚焦并选中搜索，退出歌词覆盖层后再聚焦。弹窗开启时暂停其他快捷键；Escape 依次关闭当前详情、新建歌单、提示、队列或歌词页面。队列关闭继续调用持久化服务。
- 播放、暂停、上下曲、选文件仍由 `src/lib/services/music.ts` 调用现有 IPC；没有 UI 计时推进播放。
- `src-tauri/src/media.rs` 使用 windows-rs 0.58（本机已有缓存），通过主窗口 HWND 的 `ISystemMediaTransportControlsInterop::GetForWindow` 建立 SMTC，不创建 Windows MediaPlayer。系统按钮调用现有 Controller Actions；现有 Rust 后台 worker 每次获取实际状态后同时刷新托盘与 SMTC。状态变更同步标题、艺术家、专辑及 playing/paused/stopped，初始化与同步失败可在设置页显示。退出时解除按钮订阅、禁用媒体会话，阻止 worker 重新启用。
- 设置集中展示当前实际打开的音频输出设备、音乐文件夹、关闭到托盘、自动窗口恢复、快捷键和 SMTC 状态。已有 desktop-v1.json、曲库文件夹与播放会话持久化沿用；无新增设备切换。自定义全局快捷键是可选项，本轮不引入插件或注册任何全局按键。
- 专辑封面暂未接入 SMTC，设置明确提示；不影响应用内已有封面。

依据：[Microsoft 桌面 WinRT 支持说明](https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/winrt-api-desktop-app-support) 要求桌面应用使用互操作接口；[GetForWindow](https://learn.microsoft.com/en-us/windows/win32/api/systemmediatransportcontrolsinterop/nf-systemmediatransportcontrolsinterop-isystemmediatransportcontrolsinterop-getforwindow) 绑定属于本进程的顶层窗口。

## 验证与限制

- 原 06.1 的 61 项 Rust 测试（含真实输出设备测试、Controller 队列/EOF、会话恢复、窗口偏好保存）通过。新增 3 项快捷键测试加原 4 项前端测试，共 68 项通过。
- Svelte 检查 0 errors / 0 warnings；前端 build、cargo check、cargo clippy --all-targets -- -D warnings、cargo test、cargo fmt 通过；rebuild_check 的 build 与 own-tests 通过。
- pnpm 默认版本切换受网络/签名验证失败阻碍，沿用 06.1 本机参数 `--pm-on-fail=ignore --config.verify-deps-before-run=false` 使用已有工具执行测试和构建；Rust 使用 --offline。不声称干净安装已通过。
- 已构建 `src-tauri/target/debug/music-mine-desktop.exe`（custom-protocol，嵌入 dist，无开发服务器）。Cargo 仍提示既有 lib/bin PDB 文件名碰撞。
- 使用 Computer Use 尝试启动该 EXE，被自动审批拒绝：`Computer Use was not approved to use music-mine-desktop`。未绕过拒绝；窗口、真实 IPC 与 SMTC 实机成功均不能标记通过，SMTC 设置文案保留“等待实机媒体键验收”。

待人工验收：启动构建的 EXE，使用测试音频检查 Space 暂停/恢复、Ctrl+左右切歌、Ctrl+O 文件选择与取消、Ctrl+L 歌词切换；在搜索/歌词偏移输入框输入空格和 Ctrl+左右，确认不触发播放；Ctrl+F 搜索焦点，详情/歌单/队列/歌词 Escape 逐层关闭；检查 Windows 媒体面板信息与实体媒体键；关闭到托盘后切歌/暂停，对照托盘、系统媒体面板和重新显示的窗口；退出并重启确认关闭选项、窗口位置与队列保留。多应用媒体会话竞争、音频设备断开和长期后台运行也需实际 Windows 环境验收。
