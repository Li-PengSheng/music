# IPC 与播放快照

涉及 `src-tauri/src/` 的 command、`src/lib/services/music.ts`、`src/lib/stores/*.svelte.ts` 和调用它们的组件。

## 当前事实

- 前端只在 `music.ts` 里调用 `invoke` 和窗口主题 API。没有 tauri-specta，类型是手写的。
- `PlayerSnapshot` 字段见 `docs/architecture.md`。`accept()` 拒绝更小的 `revision`。`control()` 同时只跑一个播放命令。
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

封面走现有缓存。进度轮询不传输大图。

## 错误协议

现有 `Result<T, String>` 保持不变。新的复杂命令可以在单独任务里使用可序列化的 `code`、`message`、可选 `details`。不在无关的 UI 任务里批量替换错误类型。

日志可以保留调试上下文。界面和记录里不额外散播用户绝对路径、歌词正文或整库内容。
