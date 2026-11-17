# IPC 与播放快照

涉及 `src-tauri/src/` 的 command、`src/lib/services/music.ts`、`src/lib/stores/*.svelte.ts` 和调用它们的组件。

## 当前事实

- 前端业务 `invoke` 只在 `music.ts`。窗口主题/歌词事件也由该文件封装，WindowControls 直接使用官方窗口 API（非业务 invoke）。没有 tauri-specta，类型是手写的。
- TASK-30 核对基线：`main` / HEAD `afaed378a30cc09dc7da4a686df9089c59bf2b5b` 上的当前工作区，45 个注册命令，43 个业务封装，2 个例外。
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

- `desktop_preferences({ closeToTray?: boolean | null, queueOpen?: boolean | null, theme?: 'dark' | 'light' | 'system' | null }) -> Preferences`：全空/null 只读内存，不写盘。更新时合并当前偏好保存；`listenbrainz_token` 和 `ai_api_key` 在返回副本中固定为 null，专用配置只返回 token_configured / has_key。其余 Rust 字段为 output_device、close_to_tray、queue_open、theme（旧文件可为空字符串）、bounds、maximized、lyrics_overlay_bounds、lyrics_overlay_locked、listenbrainz_enabled、listenbrainz_username、ai_enabled、ai_base_url、ai_model；前端类型使用其中子集。文件保存失败返回固定中文错误，不含绝对路径或配置正文。

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

## 显式在线元数据补全（TASK-26～TASK-30）

- `metadata_search_candidates(track_id: i64, title_query: Option<String>, artist_query: Option<String>, force_refresh: Option<bool>) -> Result<EnrichmentCandidates, String>`；JS `searchMetadataCandidates(trackId, titleQuery = null, artistQuery = null, forceRefresh = false)`，参数 `{ trackId, titleQuery, artistQuery, forceRefresh }`。Rust省略/null等价false。只在工作台点击检索、批量检索或详情显式跳转到目标曲目时请求候选，先检查缓存；普通侧栏进入只读取本地完整度，不自动联网/补全用户文件。空查询优先使用本地标签，缺失时从文件名回退并去除前导曲号；缺失艺术家时支持 - / － / —：无空格优先歌名/歌手，空格分隔优先歌手/歌名，某来源无结果时在该来源尝试相反方向，最多两组。人工明确歌手时不拆标题；返回的查询词取实际命中的组合（优先标签来源，其次歌词），无匹配则保留首选组合。只发送曲名/艺术家，不发送本地路径或文件。
- 返回 `{ title_query, artist_query, tags: TagCandidate[], lyrics: LyricsCandidate[], errors: string[] }`。最多10项去重标签（iTunes/MB各最多5项）、3项有效同步歌词；MusicBrainz、iTunes、LRCLIB 独立并行，失败不丢弃其他来源结果。`TagCandidate` 为 `{ source: 'MusicBrainz'|'iTunes'|'LRCLIB'|'Manual', title, artist, album, album_artist, year: number|null, track_number: number|null, release_mbid: string|null, cover_preview_url: string|null, cover_preview: string|null, cover_url: string|null, genre: string|null }`；旧客户端省略 source 时默认 MusicBrainz，新增可空字段省略等价 null。preview_url 是 CAA `/front-250` 或 Apple CDN600px地址，preview 是已校验缩略图 data URL（不存在/失败为 null），浏览器不直接加载远程图片。`LyricsCandidate` 为 `{ id, title, artist, album, preview, synced_lyrics }`，preview 保留前5行时间轴。
- Rust 在 spawn_blocking 使用现有 ureq 2.12.1/Rustls，User-Agent `MusicMine/0.1.0`，HTTP 超时10秒，JSON≤2MB；MusicBrainz 请求至少间隔1.1秒，LRCLIB 至少300ms（含反向尝试）。全局单个补全网络操作，竞争返回 `METADATA_BUSY`；CAA 最多5跳且总预算10秒，逐跳只允许无凭证/查询/片段的 HTTPS CAA/Internet Archive 主机，不能通过 payload 自选 URL。封面完整图 `/front` 下载≤5MB，仅接受真实可解码 JPEG/PNG，限制8192px和80MB解码分配。404/限流/网络/解析只返回固定 `METADATA_*` 错误码，不返回正文、底层错误和路径；不自动重试。封面预览失败保留文本候选。
- `metadata_apply_enrichment(payload: ApplyEnrichmentRequest) -> Result<LibraryTrack, String>`；JS `applyMetadataEnrichment(payload)` 传 `{ payload }`，内部字段使用 snake_case。payload：`{ track_id, tags: TagCandidate|null, update_library_tags: boolean, synced_lyrics: string|null, save_lyrics: boolean, save_cover: boolean, release_mbid: string|null, write_audio_tags: boolean, overwrite_cover: boolean, overwrite_lyrics: boolean, dedicated_cover?: boolean, cover_url?: string|null }`。布尔字段未传时全部 false；仅单曲应用或用户确认的批量应用调用，未知顶层字段拒绝。tags 的预览字段不用于下载，cover_url 是显式选择的 Apple CDN地址且在 Rust 严格验证；未传继续由 release_mbid 构造 CAA请求。工作台 dedicated_cover=true，保存同名 `歌曲名.cover.jpg`；旧调用未传时保存共享 cover.jpg。封面读取优先独立图，再共享图，最后内嵌图，两个 sidecar 的指纹参与缓存失效。
- 下载在 Catalog.operation 锁外，写入在锁内重新验证 ID、字段、格式、文件类型和写权限。库标签在 SQLite 事务更新 tracks；专辑/艺术家为动态分组，无新增表。保留 ID、path、duration、disc、added_at、歌单和历史；genre 仅当所选目标提供非 null 值时更新，否则保留。默认仅更新库及勾选的同目录 sidecar，不修改音频。`.lrc`≤256KB、需有效时间轴；封面转为真实 JPEG。现有文件默认 `METADATA_FILE_EXISTS`，仅对应 overwrite=true 时替换；链接/目录/只读目标拒绝。Windows 使用同目录 create_new 临时文件、sync和无覆盖 MoveFileExW；替换已有文件需明确授权并保留回滚副本。文件发布或 SQLite 失败尝试恢复所有文件；回滚失败返回 `METADATA_ROLLBACK_FAILED` 并保留备份。不是跨文件系统的断电事务，进程崩溃恢复未实现。
- `write_audio_tags` 默认 false，仅当更新文本或保存封面时写入音频副本；使用 lofty 支持的既有 MP3/FLAC/WAV，先检测可写和 tag type，保留其他标签；回读验证后原子替换，失败不直接覆盖原音频。保存封面且音频写入被勾选时同嵌封面；文本由 update_library_tags 控制，不勾选时不改音频文本。
- 成功使 Rust 当前/队列元数据更新，不重开音频、不 Seek、不创建收听会话；PlayerSnapshot 新增 `metadata_revision: u64`，只在当前曲目的元数据刷新时递增，正常 revision 仍由 Rust 决定。前端经 getPlayerState/accept 刷新；library.revision、coverEpoch 和前端缓存失效，详情更新，当前封面/音频参数/主窗口歌词和桌面歌词按 metadata_revision 重载。封面 Store 同时把 cover.jpg 指纹纳入缓存 key；不改播放计时。
- 浏览器 harness 只模拟上述搜索/确认 DTO 和页面刷新，既不联网也不写文件，不能证明真实服务、Rust IPC 或硬件播放。

### 工作台本地完整度与批处理（TASK-27）

- `metadata_track_statuses(track_ids: Vec<i64>) -> Result<MetadataStatus[], String>`；JS `metadataTrackStatuses(trackIds)`，参数 `{ trackIds }`。每次最多200个正整数 ID，缺失 ID 忽略；在 spawn_blocking 读取当前曲库和本地文件，绝不联网或写文件/缓存。返回 `{ track_id, enrichment_ignored: boolean, missing_artist: boolean, missing_album: boolean, missing_cover: boolean|null, missing_lyrics: boolean|null }`。无忽略记录时enrichment_ignored=false。空/未知艺术家算缺失；空/未知或文件夹回退专辑算缺失；封面复用当前展示格式与大小策略、查独立/共享/内嵌；歌词指同名 `.lrc` 文件存在性。读取失败返回 null（未知），筛选缺失时不将其当 true。
- 工作台分页读取完整曲库并分批查询状态，筛选 all/info/cover/lyrics。共享 enrichment Store 在当前WebView会话内保留筛选、曲目、勾选ID、焦点、逐曲候选/状态/查询词/落盘选项及编辑草稿，不使用localStorage。普通焦点切换/返回页面直接复用已有结果；详情跳转为 `{ view: 'enrichment', trackId }`，选中目标且仅该目标首次触发普通检索。离开页面取消后续批量调度，当前请求可完成并写回共享Store。library.revision变化时增量同步本地曲目/完整度，删除消失ID、增加新ID，保留路径未变条目的候选/草稿/勾选；不隐式批量检索。右侧“重新检索”明确forceRefresh=true，批量仅检索无有效会话结果或查询词已变化的勾选曲目；来源错误不清空已有结果和草稿。
- 批量检索按所选固定顺序逐首 await，曲目间至少1.1秒，不隐式应用；批量应用仅取勾选且已匹配、有实际输出的曲目，确认后冻结逐曲 payload 串行写入。默认不写音频/不覆盖 sidecar，已有本地封面/歌词默认不勾保存；用户可逐曲调整。进度显示已完成/总数，单曲失败保留本地值和固定错误，不中断其他曲目。取消停止等待和后续曲目；当前 IPC/已完成写入不能撤销，仍刷新公共曲库/播放快照，不能将已提交写入报告为失败。

### 多源兜底与手动目标（TASK-28）

- iTunes 仅请求 `https://itunes.apple.com/search?term=...&media=music&entity=song&limit=5&country=cn`，CN空/失败尝试US；每请求8秒、来源间隔3.1秒（约20次/分钟）。不下载试听音频。只解析曲名/歌手/专辑/专辑歌手/年份/曲号/genre及artworkUrl100，保留MB独有候选，按标准化 title+artist+album 去重，同一版本优先iTunes。相同图片URL的预览在本次搜索内复用。
- MB和iTunes均零标签时，由LRCLIB首条有标题/艺术家且有有效同步歌词的记录生成标记LRCLIB的兜底候选。不虚构年份/曲号/封面。只在两个标签来源都因网络/HTTP失败且最终零标签候选时返回其固定错误码；已有有效标签或LRCLIB兜底时不展示无关单源失败。JSON/过大响应错误与网络错误分别使用固定 *_RESPONSE / *_NETWORK，不回传底层正文或路径。
- 所有原始方向查询无结果后，才移除标题末尾平衡的 (...) / （...） / [...] 再尝试。支持重复/嵌套后缀，保留中间括号、未闭合括号及只剩括号的标题，不清洗原文件或艺术家。命中的查询词返回前端。
- Apple封面只接受无凭证/查询/片段、默认HTTPS端口、`.mzstatic.com` 主机、`/image/thumb/`路径及`/600x600bb.jpg`尾部。解析API图片地址时仅将100x100文件名替为600x600bb.jpg，并可升级HTTP到HTTPS；客户端不能任意选择HTTP站点。逐跳复核同一允许列表，8秒总下载预算、最多5跳、≤5MB、JPEG/PNG解码限制沿用；CSP不变，返回data URL。
- 工作台目标草稿独立于本地曲目和原候选。标题/艺术家/专辑/年份可直接编辑；无在线候选时仍可手填后单曲确认应用。切换焦点保留草稿，切换候选显式替换草稿；来源在下拉标记。空标题/非法年份禁用文本应用；未明确提供genre时保留原值，提供时在只读对比行展示。改目标、搜索词或选择候选均不自动落盘。所有原音频/覆盖默认值保持不变。

### SQLite 候选缓存（TASK-29）

- 初始化增量创建 `enrichment_cache(track_id INTEGER PRIMARY KEY, query_key TEXT NOT NULL, payload_json TEXT NOT NULL, fetched_at INTEGER NOT NULL)`。query_key为查询回退/拆分后首选(title,artist)各自大小写/空白归一化的JSON二元组；payload_json包含完整返回DTO（含有界封面预览/LRC），fetched_at为Unix秒。
- force_refresh非true且同track_id/同query_key、0≤年龄<7天、payload≤16MB并可反序列化时直接返回，不进入网络客户端/限流器。关键词改变、强制刷新、过期/未来时间、损坏或过大payload均重新检索。无错误的有界结果用UPSERT替换候选字段（保留ignored）；正常空结果也缓存，来源错误不覆盖旧缓存。缓存读/写错误仅返回METADATA_CACHE_READ / METADATA_CACHE_WRITE。
- HTTP期间不持有Catalog/SQLite锁；写缓存取得Catalog.operation并重新校验曲目仍存在、路径与查询键未变，在途移除不会重建孤立记录。成功应用在原写事务中删除未忽略曲目的缓存；已忽略时仅清空query_key/payload_json并将fetched_at置0，保留忽略标记。移除文件夹在删除曲目的同一事务清理孤立缓存（含忽略行）。缓存仅保存候选，不自动应用到曲库标签或用户文件。

### 新入库聚焦与忽略（TASK-30）

- `metadata_set_enrichment_ignored(track_id: i64, ignored: bool) -> Result<(), String>`；JS `setEnrichmentIgnored(trackId, ignored)`，参数 `{ trackId, ignored }`。spawn_blocking、Catalog.operation锁内校验正ID及曲目存在，再UPSERT ignored。非正ID返回METADATA_IDS_INVALID、缺失曲目METADATA_TRACK_MISSING、数据库失败METADATA_DATABASE、工作线程错误METADATA_WORKER；不联网、不改音频/歌词/封面。
- enrichment_cache通过ensure_column增量增加`ignored INTEGER NOT NULL DEFAULT 0`。无候选时以空查询键/空payload/零时间保存标记；候选刷新、应用与过期均不取消忽略，恢复只能由用户明确设置false。
- library_query的query新增可选snake_case `enrichment_ignored: boolean|null`：省略/null不筛选（其他曲库视图保持原行为），false排除忽略，true仅忽略；无缓存记录等同false。计数与分页使用相同条件，sort='added'及默认均按added_at DESC,id DESC，重扫/补全不改变原入库时间。
- 工作台默认新入库（近7天），有效时间范围now-7天≤added_at≤now且added_at>0；无未忽略新歌时自动展示“信息不全（缺歌手/专辑/封面/歌词）”。回退不覆盖会话中的默认偏好，新歌扫描后可自动回到近7天；用户主动选的其他筛选保留。另有“新入库且缺信息”、缺歌手/专辑、缺封面、缺歌词、全部曲目。未知完整度null不算缺失，新歌徽标按同一时间规则。
- ignored曲目从全部待办筛选隐藏，只在全部曲目显示已忽略并可一键恢复；设置成功才更新会话并清除勾选，失败保留原状态。全选/批搜/批应用跳过已忽略曲目；普通曲库/播放/歌单不受影响。详情显式跳转的目标若不符合当前筛选，切到全部以显示目标。

## 错误协议

现有 `Result<T, String>` 保持不变。新的复杂命令可以在单独任务里使用可序列化的 `code`、`message`、可选 `details`。不在无关的 UI 任务里批量替换错误类型。

日志可以保留调试上下文。界面和记录里不额外散播用户绝对路径、歌词正文或整库内容。

TASK-25 在公共播放 execute 与 Catalog work 错误边界复用绝对目录脱敏；桌面配置保存返回固定错误。曲库 track/path、扫描当前目录与用户显式选择路径属于本地功能契约，仍在 WebView 内使用，不发送给模型。此变更不保证任意用户文本/音频快照 error 内永远没有路径，也不改为结构化错误协议。AI/ListenBrainz 网络错误仍固定码，密钥不在返回 DTO；HTTP 服务可用意味着用户选择远端 HTTP 时凭证会以未加密链路传送，推荐 HTTPS 或本机服务。

## Harness 覆盖边界

`tests/ipc-surface.test.mjs` 比对注册/封装/mock/本文命令名，不能证明所有参数校验等价。新服务 DTO、camelCase 参数、null/清除凭证与禁用路径由服务单测、AI/ListenBrainz 浏览器用例和 Rust 模块单测分层覆盖。mock 只模拟服务开关与结果，不做真实 Token 验证、网络限流/TLS、outbox 持久化或 Rust 响应体安全验证；这些不能被称为已在 WebView2 验收。TASK-25 补全 desktop_preferences mock 的公开字段和两种 null 密钥字段。

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
metadata_search_candidates
metadata_apply_enrichment
metadata_track_statuses
metadata_set_enrichment_ignored
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
