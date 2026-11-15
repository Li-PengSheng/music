# 音频迁移任务 03

日期：2026-11-13。源码迁移、构建和音频设备测试完成；桌面 UI → IPC 和人工听音验收尚未完成，不能据此宣称基础工程已全面验证。

## 检查点与边界

主仓库 .git 在本环境只读，常规 git add/commit 被拒绝。修改产品源码前，在项目内建立独立 Git 检查点：

- Git 目录：.checkpoints/audio-before.git
- 提交：3645818（checkpoint before audio migration）
- 查看：git --git-dir=.checkpoints/audio-before.git --work-tree=. show HEAD

Craft 工程配置 app_in_root=true，因此直接修改当前项目根目录的 src/ 和 src-tauri/，没有创建重复 app/。
旧版 E:/Code/Project/music-mine/app 仅作为只读来源；没有修改旧工程、音乐文件或打开任何数据库。

## 实际复用

| 旧版来源 | 新版位置 | 复用内容 |
| --- | --- | --- |
| src/player.rs | src-tauri/src/player.rs | Symphonia Decoder、StreamResampler、Shared 环形缓冲、fill_output 回调、cpal 默认设备及格式回退、Output 流生命周期、设备故障检测、帧计数进度、分析及设备自测、7 项原有测试 |
| src/logging.rs | src-tauri/src/logging.rs | 有界日志缓冲；Player::new(None) 不写旧版数据目录 |
| Cargo.toml | src-tauri/Cargo.toml | Symphonia 0.5（mp3/flac/wav/pcm）、cpal 0.15、lofty 0.21、rfd 0.15；锁定解析结果于新版 Cargo.lock |
| tests/fixtures/tone.mp3、tone.flac、tone.wav | src-tauri/tests/fixtures/ | 复制编码样本用于非静音解码和真实设备控制测试 |
| tests/engine.rs、tests/real_files.rs | src-tauri/tests/audio.rs | 参考旧版断言，增加迁移后的同步控制、格式和错误测试；没有迁移依赖数据库的扫描测试 |
| src/metadata.rs | src-tauri/src/controller.rs | 参考 lofty 主标签和其他标签回退、文件名兜底的读取方式；只构造当前歌曲 DTO，不迁移 Track 数据库实体 |

没有依赖旧版 crate，因此没有带入 egui/eframe、数据库、AI、EQ、独占模式或 DSP。
旧版使用自己的线性重采样实现，迁移保留该实现，没有另写解码或重采样算法，也没有额外引入 rubato。

只读来源 SHA256：

- player.rs: 4D3233E1372F5C6482FB06883B36C96D047B71E25C5138DE300E7197E8CC47BB
- logging.rs: 9D79BF2EF56BF9D067B38F6CCB60F19E534E42904FE9933923A00D3EA3EAE50B

## 迁移适配

- 七个 player_* Commands 均已注册。打开文件使用原生选择器并暂停预载，点击播放才恢复输出。
- Controller 由 Mutex 串行保护，Tauri 命令通过 spawn_blocking 执行文件读取和等待，音频解码仍在旧核心专用线程，cpal Stream 由该线程持有。
- 增加 FIFO Synchronize 消息，在引擎完成此前命令并发布实际状态后返回；不把通道发送成功当作播放成功。
- PlayerSnapshot 包含当前歌曲、状态、毫秒位置/时长、音量、设备和错误；revision 防止较旧响应覆盖新快照。
- 修复旧核心的短文件预载/暂停尾缓冲状态：保留 draining 状态，暂停期间不按墙钟结束；新增测试先暴露该故障，再验证修复。
- Seek 复用 begin_track 和原 Decoder.seek，重新预载并保留暂停状态，返回 seek 错误；跳到末尾停止。失败切歌/跳转会清空旧音频，避免显示失败后仍播放旧缓冲。
- Clippy 修正仅涉及不必要分配、Copy、切片及补零写法。
- src/lib/services/music.ts 集中所有 invoke；原 Svelte 界面增加文件选择、暂停/播放/停止、进度滑块和真实音量。
- 根组件跨页面保存快照；每 200ms 向 Rust 读取状态，前端不累加播放时间。拖动时仅预览目标位置，完成后使用 Rust 返回值。
- 演示专辑仍明确标注、不可播放；未迁移完整曲库、队列或持久化。

## 实际验证

| 检查 | 结果 |
| --- | --- |
| cargo test --offline -- --test-threads=1 | 12 项通过：9 单元、3 集成；未跳过格式/控制集成测试 |
| MP3 / FLAC / WAV | 三种格式非静音 PCM（peak > 0.35、RMS > 0.2），真实设备帧计数前进 |
| 暂停恢复、seek、音量、EOF | 三种格式均测试；额外验证回调输出样本乘以 0.25、短文件长时间暂停不丢缓冲 |
| 损坏文件、连续切歌 | 明确失败且进度为零；最终状态只保留最后文件 |
| cargo check --offline | 通过 |
| cargo fmt --check | 通过 |
| cargo clippy --offline --all-targets -- -D warnings | 通过 |
| Svelte 检查 | 0 errors、0 warnings |
| 前端生产构建 | 通过 |
| cargo build --offline --features custom-protocol | 通过；生成内嵌 dist 的 target/debug/music-mine-desktop.exe |
| pnpm exec tauri dev | 使用下述环境参数、关闭 beforeDevCommand/devUrl 并指定 custom-protocol，编译运行成功；未启动 Vite 常驻服务 |
| rebuild_check | build passed；工具 own-tests 明确没有运行 JS 测试，不能代替上述 Rust 测试 |

设备日志：Speakers (Realtek(R) Audio)，48000 Hz、2 声道、F32。测试验证设备回调收到音频，不等于已确认扬声器可听。

本机可用 pnpm 为 11.23.0，而 packageManager 固定 10.28.2。自动切换受网络/签名查询失败阻塞；
pnpm install --offline 未通过（另一次使用可用版本安装因非交互式重建 node_modules 被中止），没有宣称完成全新安装验证。
利用已有依赖运行检查时使用可用 Node 与 pnpm 入口，参数 --pm-on-fail=ignore --config.verify-deps-before-run=false，
保留项目锁文件和声明版本。等效直接调用本地 svelte-check、Vite 也通过。

Cargo 1.99 对原有相同归一化名称的 bin/lib PDB 输出有 collision 警告；不影响本轮成功构建或严格 Clippy。

## 桌面验收限制及下一步

默认 WebView2 数据目录首次启动报 HRESULT 0x800700AA（资源正在使用）。
测试进程设置 WEBVIEW2_USER_DATA_FOLDER 为项目内 .tools/webview-smoke 或 .tools/webview-dev-check 后启动，
进程报告 Music Mine 窗口句柄并打开音频输出，但 Computer Use 的窗口枚举未显示它。
尝试通过 Computer Use 启动构建后的程序，工具明确返回：

> Computer Use was not approved to use music-mine-desktop

未尝试绕过该应用访问限制；测试进程已停止，也未留下开发服务器。
因此尚未实际点击桌面播放按钮或验证原生选择器 → Rust 的完整往返，不能把进程日志等同于可见窗口/IPC 验收。

仍需人工在正常桌面环境中：

1. 运行构建的可执行文件，点击添加音乐选择真实 MP3/FLAC/WAV，核对标题、歌手和时长。
2. 点击播放、暂停/继续、停止、拖动进度和音量，确认声音及状态；测试播放到结尾和选择损坏文件。
3. 切换页面、切换窗口和最小化期间确认音乐不中断。
4. 确认 Windows 实际选中的扬声器出声；完成这些步骤后才能确认用户定义的成功标准。
