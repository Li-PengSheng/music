# IPC 与播放快照

涉及 `src-tauri/src/` 的 command、`src/lib/services/music.ts`、`src/lib/stores/*.svelte.ts` 和调用它们的组件。

## 当前事实

- 前端只在 `music.ts` 里调用 `invoke` 和窗口主题 API。没有 tauri-specta，类型是手写的。
- `PlayerSnapshot` 字段见 `docs/architecture.md`。`accept()` 拒绝更小的 `revision`。`control()` 同时只跑一个播放命令。
- `volume` 是 `0.0..=1.0` 的用户感知音量比例；`PlayerStatus`、`PlayerSnapshot`、前端和 `playback-v1.json` 保留原始比例，音频回调按三次方曲线（`x^3`）映射为 PCM 振幅增益。例如 20% 仍传输并保存 `0.2`，输出增益为 `0.008`；50% 的增益为 `0.125`。未加载会话时的 Rust 初始音量保持 `1.0`。
- `App.svelte` 用 `setTimeout(refresh, 200)` 轮询 `player_get_state`。
- 命令返回 `Result<T, String>`。错误文本由 Rust 产生，界面用现有通知显示。
- `verifyConnection()` 调用 `bridge_probe`，并核对 `nonce` 与 `backend === 'rust'`。
- 普通浏览器里 `isTauri()` 为假，不会显示已连接。`tests/harness/mock-ipc.js` 会设置 `isTauri` 并回答 `backend: 'rust'`，那是测试夹具。

## 修改一个 command 时

1. 核对 Rust 命令名、参数和 serde。注意 `Option`、camelCase 和枚举写法。
2. 不扩大 `capabilities/default.json`。桌面歌词窗口使用单独的 `capabilities/lyrics-overlay.json`。路径和数值在 Rust 校验。
3. 更新 `music.ts` 的类型和封装，并搜索全部调用点与 `tests/harness/mock-ipc.js`。
4. 成功、失败和取消都要有界面反馈。前端不编造成功快照。
5. 返回的播放快照交给 `accept()`。调用点不直接覆盖 `player.snapshot`。
6. 浏览器 Mock 只证明前端契约。真实 `invoke` 要在构建出的 exe 里跑一次；不能跑就写 `NOT RUN`。
7. 改已有协议时列出全部调用点和兼容方式。

## 进度

Rust 是播放位置的权威。界面显示 `position_ms`。拖动进度条只保留松开前的草稿，结果以命令返回的快照为准。

不要用定时器在两次轮询之间插值进度。不要把约 200ms 的轮询改成事件推送，除非先有测量说明它是瓶颈。

列表、专辑网格和歌曲详情调用 `library_thumb`。队列封面调用 `library_queue_thumb`，只匹配曲库中的完整路径，不按标题查找。Rust 在后台线程把内嵌 JPEG/PNG 限制尺寸后编码成 JPEG，缓存在应用缓存目录的 `covers`；不超过 512 KB 的 WebP/GIF 原样以对应 MIME 的 data URL 返回。`player_artwork` 只读当前播放文件，返回最长边不超过 960 的图片。进度轮询不请求封面。

这三个封面命令的返回值：`null` 表示确定没有可用封面（无图片、格式不支持、超出尺寸限制、图片损坏），会缓存到文件变化为止；错误字符串是不含路径的代码，表示这次拿不到、稍后可能成功，不缓存：`cover-file-missing`、`cover-unreadable`，`library_queue_thumb` 另有 `cover-not-in-library`。前端经 `src/lib/utils/cover-loader.ts` 限并发、按优先级（当前曲目、可见行、预取）排队，可取消，暂时性错误最多重试两次。`library_cover_stats` 返回封面缓存计数和耗时（只有数字），用于诊断。

## 运行控制台与音频文件参数

- `app_console_logs({ clear?: boolean | null }) -> string[]`：省略、null 或 false 时返回本次进程的日志快照（最多 1000 行）；true 时在同一锁内清空内存并返回空数组，不删除磁盘日志。锁不可用时返回错误。每行保留 UTC 时间戳、操作、音频设备、SMTC 状态和扫描统计；返回前将本地盘符、UNC 和 POSIX 绝对目录前缀替换成 `<path>/`，保留文件名。Console 的“复制摘要”使用这份脱敏结果。
- `library_audio_info({ id: number }) -> AudioInfo` 只返回六个文件技术字段：`format: string`（大写）、`sample_rate / channels / bitrate: number | null`、`bit_depth: number | null`（Lofty properties）、`file_size: number | null`（磁盘字节数）。TASK-20 移除播放次数字段及专用历史查询，所有消费者（music.ts、TrackDetails、NowPlaying、mock）同时更新；没有播放次数兼容字段。无法识别的位深为 null；读取失败返回错误。
- `library_track_by_path({ path: string }) -> LibraryTrack | null`：复用现有完整路径查找，不按标题匹配，不在曲库时返回 null；数据库失败返回错误。供正在播放页切歌时先取得曲库 ID 再请求 audioInfo。库外文件不展示库内参数。旧 library_track(id) 调用保持兼容。
- 调用点：`music.ts` 封装全部 invoke；`ConsoleModal.svelte` 打开期间每两秒读取日志、coverStats、mediaStatus、librarySummary，关闭清理计时器并忽略旧响应。`TrackDetails.svelte` 与 `NowPlaying.svelte` 展示 AudioInfo，后者仅切换曲目时请求并丢弃过期结果。
- Console 内置 play / pause / next / prev / vol 0–100 复用 control()/accept()；theme、palette 复用本地偏好；scan 复用扫描命令并反馈启动或选择结束。TASK-22 明确授权增加默认关闭的 AI 对话，见下文；不提供 shell，原有能力权限不变。
- 收听历史仍按原有后台 200ms 检查时序记录，用于最近播放与常听艺术家排序；TASK-20 不更改写入规则或数据库 schema。没有面向 UI 的播放次数事件/订阅。`library_home` 的艺术家元组形状保留，界面只用姓名，并从 `library_summary.artists[].count` 显示库内歌曲数；数量为 0 时不展示副标题。

## 音频输出设备

- `player_output_devices()` 返回按名称排序、去重的可用输出设备名称；枚举不打开输出流。`MUSIC_MINE_NULL_OUTPUT` 存在时只返回 `['null']`，不访问系统设备。
- `player_set_output_device({ device: string | null })` 返回权威 `PlayerSnapshot`，必须经 `control()` / `accept()` 接收。`null` 或空白名称表示跟随系统默认设备；非空名称精确匹配 cpal 输出设备。设备消失或打开失败时记录日志并尝试系统默认设备，`output_device_id` 始终表示实际打开的设备名称，不表示首选项。
- 首选设备保存在 `desktop-v1.json` 的可选 `output_device` 字段；旧配置缺失时默认为 `null`。设备暂时断开时仍保存所选名称并在列表标记「已断开」。启动先应用设备偏好，再恢复暂停会话。现有 `desktop_preferences` 参数不变，返回值新增此字段；主题/窗口设置不会覆盖设备选择。
- 切换在音频工作线程完成：停止旧回调并读取最终帧时钟，重开共享模式输出，再解码/Seek 恢复当前曲目、位置、音量和暂停状态。所有输出都不可用时保持错误并停止，返回快照中的 error，界面不显示成功；此时不保存新偏好。偏好写入失败则尝试恢复之前的选择并返回错误。
- 设置页沿用实际输出名称的紧凑读数，鼠标悬停或键盘聚焦时显示现有 Select 控件；下拉框提供系统默认项和设备列表，切换后刷新列表与偏好。旧播放命令、volume 和 playback-v1.json 契约不变。

## 曲库文件夹与歌单管理

- TASK-23 已移除旧版数据库导入：`library_import` 不再注册或封装，也不属于 ipc-unused 兼容例外；设置页没有旧库导入入口。现有曲库 schema 升级仍保留，不进行任何用户数据删除或格式重置。

- `library_remove_folder` 接收 `{ path: string }`，只接受 summary 中已登记的目录。与扫描共用 `Catalog.operation` 锁，在事务内移除目录记录及该路径下的曲目数据库记录，清理外键关联；按路径分隔符边界匹配，`%` / `_` 按字面量处理。绝不删除或修改磁盘文件，不改变当前 Rust 播放快照或队列。前端二次确认后调用 `removeFolder`，刷新 summary、封面缓存和 `coverEpoch`。
- `library_playlist_edit` 保留 rename/delete/remove_track，并新增 `action: 'reorder'`，参数 `{ playlistId: number, from: number, to: number }` 为零基完整歌单位置；其余 name/trackId 可省略。Rust 在同一操作锁内检查缺失参数、不存在的歌单、空歌单及越界位置，失败返回错误且不改变顺序；合法同位置为无操作。只更新歌单成员位置，不改变曲库歌曲和文件。旧调用不需要新增参数。
- UI 仅在没有搜索词的歌单中提供上下移菜单与 `Alt+ArrowUp` / `Alt+ArrowDown`。禁用边界与进行中的操作；成功后递增 `library.revision`、重载列表、恢复目标行焦点并播报位置。搜索匹配行不能作为完整歌单索引使用。

## 桌面歌词窗口

- `desktop_lyrics_window({ open: boolean | null, locked: boolean | null })` 返回窗口当前是否可见。
- `open: true` 创建或显示标签为 `lyrics-overlay` 的无边框透明置顶窗口，地址是应用入口 `?window=lyrics`；已存在时只 `show()`。`false` 只 `hide()`，不销毁。`null` 不改变可见性。
- `locked: true` 写入 `desktop-v1.json` 的 `lyrics_overlay_locked`，并调用 `set_ignore_cursor_events(true)`，鼠标穿透该窗口。`false` 恢复点击。`null` 不改变锁定。主窗口播放栏和托盘菜单「解锁桌面歌词穿透」都可以解除锁定。该菜单项仅在锁定时出现。
- 拖动和缩放后的外框位置与内框尺寸写入 `lyrics_overlay_bounds`。下次打开时，只有完整落在某一显示器工作区内、且不小于 `360×92` 的边界才会恢复，否则居中到默认 `640×108`。旧配置缺少这两个字段时，锁定为 `false`，边界为空。
- 命令结束后 Rust 发出 `lyrics-overlay` 事件，载荷为 `{ open, locked }`，主窗口用它同步按钮。主进程退出时关闭该窗口。
- 悬浮窗只轮询播放状态和歌词，不挂载曲库、队列或封面加载。

## ListenBrainz（默认关闭）

- `listenbrainz_config(enabled?: boolean | null, username?: string | null, token?: string | null) -> Result<ListenBrainzConfig, string>`；返回 `{ enabled: boolean, username: string | null, token_configured: boolean, pending_count: number }`，不返回 Token。参数全空为本地查询；仅传 enabled 是本地开关，开启要求已有验证凭证；关闭立即停止新采集与新请求，已在途请求可能完成，队列保留。客户端使用 camelCase 参数（这三项同名）。
- 修改 username/token 的保存请求在后台调用 `GET /1/validate-token`，使用 Authorization 头；核对服务返回的 user_name，不一致或验证失败不保存。null 保持原值；空 username 清除名字并允许验证后自动填入；空 token 清除凭证，必须同时禁用。并发设置使用版本防止旧验证响应重新启用。设置文件保存明文 Token，仅用于用户显式启用的服务；不写 localStorage、不经 desktop_preferences 或配置查询暴露。日志在 stderr/磁盘/Console 写入前以 `***` 脱敏。
- `listenbrainz_recommendations() -> Result<LibraryTrack[], string>`：禁用返回空数组，不联网。启用后请求 `GET /1/cf/recommendation/user/{username}/recording?count=50`，再以 MBID 批量读取官方录音元数据；以标准化空白、忽略大小写的「艺术家 + 曲名」精确匹配本地 tracks，保持远端顺序并按本地 ID 去重。当前曲库无 MBID 列，不做只凭标题或模糊匹配。推荐未生成/无可用元数据时回退用户公开最近 50 次收听。远端候选缓存 10 分钟，失败冷却 30 秒，每次从当前 SQLite 重新匹配，避免已移除曲目残留。首页无结果或失败时不显示推荐架，现有本地功能保持可用。
- Desktop 的 `desktop-v1.json` 增加 `listenbrainz_enabled`（旧文件默认 false）、`listenbrainz_username`、`listenbrainz_token`。desktop_preferences 返回中的 token 固定 null，其余配置保留；专用配置接口报告 token_configured。
- SQLite 增量建立 `scrobble_outbox(id, artist, title, album, duration_ms, listened_at, created_at, username)`，username 绑定账户，切换账户后不把旧队列上传给新账户。成功确认才删除，失败保留；支持重启恢复，以开始播放时的 Unix 秒数作为 listened_at。服务端确认前进程退出可能导致同时间戳重传。
- 后台采集每 200ms 读取 Rust 快照；内部不序列化的会话/Seek 序号用于识别同曲重播与跳跃。只累计同会话、无 Seek、连续 playing 的实际进度，且不超过单调时钟间隔。暂停/跳跃边界的采样区间保守不计，可能少计约 200ms；不把 Seek 距离算作收听。总长至少 30 秒、累计达到 `min(duration_ms / 2, 240000)` 时每次会话入队一次。UI 不参与计时。
- 独立 observer 负责采集/持久化，network 线程统一 HTTP（ureq 2.12.1 + Rustls，无 OpenSSL）；Controller 锁释放后才执行数据库/日志/网络。开始播放发 playing_now（无 listened_at），正式记录单条 single、多条 import、每批最多 50 条。网络超时 10 秒，不跟随重定向，响应体上限 2MB；请求至少间隔 1.1 秒，遵守 X-RateLimit-Reset-In/Retry-After，队列失败指数退避到 15 分钟。错误只暴露固定码，不回传 HTTP 正文、头或带凭证 URL。
- Last.fm 不是另一个已接入的 API；设置页说明用户可自行在 ListenBrainz 网站关联。

## AI Console（默认关闭，TASK-22）

- `ai_config(enabled?: boolean | null, base_url?: string | null, api_key?: string | null, model?: string | null) -> Result<AiConfig, string>`，JS 参数 `{ enabled, baseUrl, apiKey, model }`。返回 `{ enabled: boolean, base_url: string, model: string, has_key: boolean, revision: number }`，永不回传 Key。参数全空为只读查询；null 保持原值；空 apiKey 清除密钥并禁用。设置和开关仅本地保存，不验证远端账户。旧 desktop-v1.json 默认 ai_enabled=false、ai_base_url=https://api.openai.com/v1、ai_model=gpt-4o-mini、ai_api_key=null。desktop_preferences 的 ai_api_key 固定 null；持久化文件保留密钥，其他偏好更新不覆盖它。revision 是进程内 AI 配置版本，每次保存递增，模型响应过期即拒绝执行。
- `ai_chat_turn(messages: AiMessage[], tools: JSON[]) -> Result<AiTurnResponse, string>`，JS 参数同名。AiMessage 为 `{ role: 'system'|'user'|'assistant'|'tool', content: string|null, tool_calls?: AiToolCall[], tool_call_id?: string }`；AiToolCall 为 `{ id: string, type: 'function', function: { name: string, arguments: string } }`。返回 `{ message: AiMessage, finish_reason: string }`，只选取标准助手字段，不传透提供商扩展；finish_reason 使用允许列表。禁用/缺 Key 返回 AI_DISABLED / AI_KEY_MISSING，不发 HTTP。
- Rust 复用 ureq 2.12.1/Rustls，spawn_blocking 发 POST `{base_url}/chat/completions`，Authorization: Bearer Key、MusicMine/0.1.0、stream=false；tools 非空时 tool_choice=auto。仅接受可解析的 HTTP(S)，拒绝 URL 内凭证/查询/片段/控制字符；本机 HTTP 服务可用。30 秒超时、不跟随重定向、不自动重试；请求体消息/工具部分最多 256KB，响应最多 1MB，消息最多100条、每条内容64KB，工具定义仅六种允许名称、每响应最多8次调用。API错误只返回固定码/HTTP状态，不返回 URL、响应正文或头。日志只记录请求/工具结果计数、工具允许名和固定码，写入 stderr/文件/Console 前脱敏 Key。HTTP 和响应体读取期间不持有 Controller 或偏好锁；同时最多一个模型请求。
- 工具闭环由前端 ai-conversation.ts 驱动，每个用户提问最多5次模型请求。第5次仍要求工具则停止，不执行该轮工具。串行调用 ai-tools.ts 的允许列表；每次模型请求/工具执行前核对启用和配置版本。TASK-24将输入区与视图绑定：日志页的短命令在本地处理，格式错误仍给本地用法；自然语言自动切到AI页，AI页提交统一发送对话。未启用/缺Key提示前往设置。两页分别显示本地/AI标题、图标、字体、占位符和执行/发送按钮；AI模型徽章复用每2秒只读配置轮询，未启用明确标记，关闭后忽略旧响应。IPC参数/返回值不变。
- 六种工具：get_library_overview（歌曲数、前30专辑/艺术家、前100歌单）；search_library（keyword、排序，最多50条，返回 ID/曲名/艺术家/专辑/时长）；create_playlist_with_tracks（先验证最多50个ID，再创建并添加，刷新侧栏；部分失败返回新歌单ID和已添加数，不声称回滚）；play_tracks（验证ID，control/accept应用模式和替换队列播放）；control_playback（播放/暂停/上一首/下一首/0–100音量）；set_appearance（受限主题/色系/字体、明确桌面歌词开关）。所有参数按允许字段、类型、范围校验；无任意 invoke、shell、扫描/删除文件或配置凭证工具。不发送歌曲路径、文件夹、封面、歌词或运行日志给模型；名称按100字符裁剪。用户输入、标签和工具结果均视作不可信数据。
- Console 日志与 AI 对话分视图，显示用户问题、工具名/JSON结果和最终文本（按文本转义显示）。对话仅存本次弹窗内存，可清空/停止；关闭/停止不继续执行后续轮次或工具，已在途请求和本地操作可能完成。失败或停止时保留已完成工具记录，未执行调用补充 not_executed 结果，保持下一轮消息配对有效，避免重复创建。清空后的旧响应不能恢复历史；配置改变后开始新模型上下文。旧历史按完整用户轮次裁剪，不截断工具交换。
- 只支持标准 Chat Completions 文本/function calling 格式；各服务的非标准鉴权/推理扩展不承诺兼容。密钥仍须由用户填写，无需鉴权的本机服务可用占位值。真实外网服务验收与模拟服务测试分别记录。

## 错误协议

现有 `Result<T, String>` 保持不变。新的复杂命令可以在单独任务里使用可序列化的 `code`、`message`、可选 `details`。不在无关的 UI 任务里批量替换错误类型。

日志可以保留调试上下文。界面和记录里不额外散播用户绝对路径、歌词正文或整库内容。

## 命令清单

检查比较四个名字集合：Rust `generate_handler!`、`src/lib/services/music.ts` 的 `invoke`、`tests/harness/mock-ipc.js` 的 handlers、下面两份清单。提取时忽略注释，允许多行、泛型和同一命令多次调用。参数、可空性和返回类型不在这次检查里。

已注册但前端没有封装的命令必须写进例外清单。例外清单必须等于「已注册命令减去前端封装」。命令被删除、后来补了前端封装，或新注册了一个前端未使用的命令时，过时或缺漏的名字都会让 `tests/ipc-surface.test.mjs` 失败。当前例外只有 `library_cover` 和 `library_favorite`。

```ipc-commands
ai_config
ai_chat_turn
app_console_logs
bridge_probe
desktop_lyrics_window
desktop_preferences
library_audio_info
library_cancel
library_cover
library_cover_stats
library_favorite
library_home
library_play
library_playlist
library_playlist_edit
library_progress
library_query
library_remove_folder
library_queue
library_queue_thumb
library_queue_tracks
library_scan
library_summary
library_thumb
library_track
library_track_by_path
listenbrainz_config
listenbrainz_recommendations
media_status
player_artwork
player_configure
player_get_state
player_lyrics
player_open_file
player_output_devices
player_pause
player_play
player_seek
player_set_volume
player_set_output_device
player_stop
```

```ipc-unused
library_cover
library_favorite
```
