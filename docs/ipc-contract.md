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
2. 不扩大 capability。路径和数值在 Rust 校验。
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

## 音频输出设备

- `player_output_devices()` 返回按名称排序、去重的可用输出设备名称；枚举不打开输出流。`MUSIC_MINE_NULL_OUTPUT` 存在时只返回 `['null']`，不访问系统设备。
- `player_set_output_device({ device: string | null })` 返回权威 `PlayerSnapshot`，必须经 `control()` / `accept()` 接收。`null` 或空白名称表示跟随系统默认设备；非空名称精确匹配 cpal 输出设备。设备消失或打开失败时记录日志并尝试系统默认设备，`output_device_id` 始终表示实际打开的设备名称，不表示首选项。
- 首选设备保存在 `desktop-v1.json` 的可选 `output_device` 字段；旧配置缺失时默认为 `null`。设备暂时断开时仍保存所选名称并在列表标记「已断开」。启动先应用设备偏好，再恢复暂停会话。现有 `desktop_preferences` 参数不变，返回值新增此字段；主题/窗口设置不会覆盖设备选择。
- 切换在音频工作线程完成：停止旧回调并读取最终帧时钟，重开共享模式输出，再解码/Seek 恢复当前曲目、位置、音量和暂停状态。所有输出都不可用时保持错误并停止，返回快照中的 error，界面不显示成功；此时不保存新偏好。偏好写入失败则尝试恢复之前的选择并返回错误。
- 设置页沿用实际输出名称的紧凑读数，鼠标悬停或键盘聚焦时显示现有 Select 控件；下拉框提供系统默认项和设备列表，切换后刷新列表与偏好。旧播放命令、volume 和 playback-v1.json 契约不变。

## 曲库文件夹与歌单管理

- `library_remove_folder` 接收 `{ path: string }`，只接受 summary 中已登记的目录。与扫描共用 `Catalog.operation` 锁，在事务内移除目录记录及该路径下的曲目数据库记录，清理外键关联；按路径分隔符边界匹配，`%` / `_` 按字面量处理。绝不删除或修改磁盘文件，不改变当前 Rust 播放快照或队列。前端二次确认后调用 `removeFolder`，刷新 summary、封面缓存和 `coverEpoch`。
- `library_playlist_edit` 保留 rename/delete/remove_track，并新增 `action: 'reorder'`，参数 `{ playlistId: number, from: number, to: number }` 为零基完整歌单位置；其余 name/trackId 可省略。Rust 在同一操作锁内检查缺失参数、不存在的歌单、空歌单及越界位置，失败返回错误且不改变顺序；合法同位置为无操作。只更新歌单成员位置，不改变曲库歌曲和文件。旧调用不需要新增参数。
- UI 仅在没有搜索词的歌单中提供上下移菜单与 `Alt+ArrowUp` / `Alt+ArrowDown`。禁用边界与进行中的操作；成功后递增 `library.revision`、重载列表、恢复目标行焦点并播报位置。搜索匹配行不能作为完整歌单索引使用。

## 错误协议

现有 `Result<T, String>` 保持不变。新的复杂命令可以在单独任务里使用可序列化的 `code`、`message`、可选 `details`。不在无关的 UI 任务里批量替换错误类型。

日志可以保留调试上下文。界面和记录里不额外散播用户绝对路径、歌词正文或整库内容。

## 命令清单

检查比较四个名字集合：Rust `generate_handler!`、`src/lib/services/music.ts` 的 `invoke`、`tests/harness/mock-ipc.js` 的 handlers、下面两份清单。提取时忽略注释，允许多行、泛型和同一命令多次调用。参数、可空性和返回类型不在这次检查里。

已注册但前端没有封装的命令必须写进例外清单。例外清单必须等于「已注册命令减去前端封装」。命令被删除、后来补了前端封装，或新注册了一个前端未使用的命令时，过时或缺漏的名字都会让 `tests/ipc-surface.test.mjs` 失败。当前例外只有 `library_cover` 和 `library_favorite`。

```ipc-commands
bridge_probe
desktop_preferences
library_audio_info
library_cancel
library_cover
library_cover_stats
library_favorite
library_home
library_import
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
