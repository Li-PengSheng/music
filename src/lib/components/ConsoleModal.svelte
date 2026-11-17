<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { X, RefreshCw, Copy, Trash2, Terminal, Sparkles } from 'lucide-svelte';
  import Modal from './Modal.svelte';
  import IconButton from './IconButton.svelte';
  import {
    appConsoleLogs,
    coverStats,
    mediaStatus,
    librarySummary,
    play,
    pause,
    queueAction,
    setVolume,
    scanFolder,
    scanProgress,
    desktopAvailable,
    type PlayerSnapshot,
    type AiMessage,
    aiConfig,
    type AiConfig,
  } from '../services/music';
  import { player, control } from '../stores/player.svelte';
  import { library, refreshSummary } from '../stores/library.svelte';
  import { setTheme, setPalette } from '../stores/preferences';
  import { PALETTES, type Palette, type ThemeChoice } from '../utils/theme';
  import { errorText } from '../utils/format';
  import { runAiConversation, type AiEvent } from '../utils/ai-conversation';
  import { go } from '../stores/nav.svelte';

  let { onclose }: { onclose: () => void } = $props();
  let logs = $state<string[]>([]);
  let stats = $state<Record<string, number> | null>(null);
  let media = $state('读取中…');
  let tracks = $state<number | null>(null);
  let command = $state('');
  let feedback = $state('');
  let refreshError = $state('');
  let busy = $state(false);
  let refreshing = $state(false);
  let logHost = $state<HTMLDivElement>();
  let view = $state<'logs' | 'ai'>('logs');
  let aiPreferences = $state<AiConfig | null>(null);
  let aiConfigError = $state(false);
  const modelLabel = $derived(
    !desktopAvailable
      ? '桌面不可用'
      : aiConfigError
        ? '配置不可用'
        : !aiPreferences
          ? '读取中…'
          : aiPreferences.enabled
            ? aiPreferences.model
            : '未启用',
  );
  let events = $state<AiEvent[]>([]);
  let history: AiMessage[] = [];
  let historyRevision: number | undefined;
  let chatGeneration = 0;
  let chatHost = $state<HTMLDivElement>();
  let aiRequest = $state<AbortController | null>(null);
  let disposed = false;
  let snapshot = $derived(player.snapshot);
  const engineNames = { idle: '空闲', paused: '已暂停', playing: '播放中' };
  let engine = $derived(
    !player.connected
      ? player.connection
      : snapshot?.error
        ? `错误：${snapshot.error}`
        : engineNames[snapshot?.playback_state ?? 'idle'],
  );
  let cache = $derived(
    stats
      ? `内存 ${stats.memoryHits ?? 0} · 磁盘 ${stats.diskHits ?? 0} / 请求 ${stats.requests ?? 0}`
      : '暂不可用',
  );

  async function refresh(clear = false) {
    if (refreshing || disposed) return;
    if (!desktopAvailable) {
      refreshError = '运行日志与设备诊断需要在桌面应用中使用';
      return;
    }
    refreshing = true;
    const follow = !logHost || logHost.scrollHeight - logHost.scrollTop - logHost.clientHeight < 32;
    const results = await Promise.allSettled([
      appConsoleLogs(clear),
      coverStats(),
      mediaStatus(),
      librarySummary(),
      aiConfig(),
    ]);
    if (disposed) return;
    const [logResult, coverResult, mediaResult, libraryResult, aiResult] = results;
    const errors: string[] = [];
    if (logResult.status === 'fulfilled') logs = logResult.value;
    else errors.push(`日志：${errorText(logResult.reason)}`);
    if (coverResult.status === 'fulfilled') stats = coverResult.value;
    else {
      stats = null;
      errors.push(`封面统计：${errorText(coverResult.reason)}`);
    }
    if (mediaResult.status === 'fulfilled') media = mediaResult.value;
    else {
      media = '暂不可用';
      errors.push(`SMTC：${errorText(mediaResult.reason)}`);
    }
    if (libraryResult.status === 'fulfilled') tracks = libraryResult.value.tracks;
    else {
      tracks = null;
      errors.push(`曲库：${errorText(libraryResult.reason)}`);
    }
    aiConfigError = aiResult.status === 'rejected';
    if (aiResult.status === 'fulfilled') {
      if (!aiPreferences || aiResult.value.revision >= aiPreferences.revision)
        aiPreferences = aiResult.value;
    } else errors.push(`AI 配置：${errorText(aiResult.reason)}`);
    refreshError = errors.join('；');
    refreshing = false;
    if (follow) {
      await tick();
      if (!disposed && logHost) logHost.scrollTop = logHost.scrollHeight;
    }
    if (clear)
      feedback = logResult.status === 'fulfilled' ? '已清空本次会话的内存日志' : '清空日志失败';
  }

  onMount(() => {
    let timer: ReturnType<typeof setTimeout>;
    async function poll() {
      await refresh();
      if (!disposed) timer = setTimeout(poll, 2000);
    }
    void poll();
    return () => {
      disposed = true;
      aiRequest?.abort();
      clearTimeout(timer);
    };
  });

  async function copySummary() {
    try {
      await navigator.clipboard.writeText(
        [
          'Music Mine · 运行诊断',
          `引擎：${engine}`,
          `输出设备：${snapshot?.output_device_id ?? '未打开'}`,
          `SMTC：${media}`,
          `封面缓存：${cache}`,
          `曲库：${tracks ?? '未知'} 首`,
          refreshError,
          '',
          ...logs,
        ]
          .filter(Boolean)
          .join('\n'),
      );
      if (!disposed) feedback = '已复制诊断摘要与脱敏日志';
    } catch (error) {
      if (!disposed) feedback = `复制失败：${errorText(error)}`;
    }
  }

  async function runPlayer(action: () => Promise<PlayerSnapshot>) {
    if (!desktopAvailable) throw new Error('播放命令需要在桌面应用中使用');
    if (!(await control(action))) throw new Error('命令未完成，请查看错误提示或稍后重试');
    if (player.snapshot?.error) throw new Error(player.snapshot.error);
  }
  function openSettings() {
    onclose();
    go({ view: 'settings' });
  }
  function clearChat() {
    chatGeneration++;
    aiRequest?.abort();
    events = [];
    history = [];
    historyRevision = undefined;
    feedback = '已清空本次对话';
  }
  async function natural(text: string) {
    const request = new AbortController();
    aiRequest = request;
    const generation = ++chatGeneration;
    view = 'ai';
    let revision: number | undefined;
    try {
      const result = await runAiConversation(history, text, {
        signal: request.signal,
        historyRevision,
        onConfig: (value) => {
          revision = value;
        },
        onCheckpoint: (messages, value) => {
          if (!disposed && generation === chatGeneration) {
            history = messages;
            historyRevision = value;
          }
        },
        onEvent: (event) => {
          if (
            disposed ||
            generation !== chatGeneration ||
            (request.signal.aborted && event.role !== 'tool')
          )
            return;
          const follow =
            !chatHost || chatHost.scrollHeight - chatHost.scrollTop - chatHost.clientHeight < 32;
          events = [...events, event].slice(-200);
          if (follow)
            void tick().then(() => {
              if (chatHost && !disposed) chatHost.scrollTop = chatHost.scrollHeight;
            });
        },
      });
      if (!disposed && generation === chatGeneration && !request.signal.aborted) {
        history = result;
        historyRevision = revision;
        feedback = 'AI 对话已完成';
      }
    } finally {
      if (aiRequest === request) aiRequest = null;
    }
  }
  async function execute() {
    if (busy || disposed) return;
    const text = command.trim();
    if (!text) return;
    busy = true;
    feedback = '';
    const [name, ...args] = text.toLowerCase().split(/\s+/);
    try {
      if (view === 'ai') {
        await natural(text);
      } else if (['play', 'pause', 'next', 'prev'].includes(name) && !args.length) {
        await runPlayer(
          name === 'play'
            ? play
            : name === 'pause'
              ? pause
              : () => queueAction(name === 'next' ? 'next' : 'previous'),
        );
        feedback = `已执行 ${name} · ${engineNames[player.snapshot?.playback_state ?? 'idle']}`;
      } else if (name === 'vol') {
        if (args.length !== 1 || !/^\d+(\.\d+)?$/.test(args[0]) || Number(args[0]) > 100)
          throw new Error('音量用法：vol 30（0–100）');
        await runPlayer(() => setVolume(Number(args[0]) / 100));
        feedback = `音量 ${Math.round((player.snapshot?.volume ?? 0) * 100)}%`;
      } else if (
        name === 'theme' &&
        args.length === 1 &&
        ['dark', 'light', 'system'].includes(args[0])
      ) {
        setTheme(args[0] as ThemeChoice);
        feedback = `主题已切换：${args[0]}`;
      } else if (
        name === 'palette' &&
        args.length === 1 &&
        PALETTES.some((p) => p.value === args[0])
      ) {
        setPalette(args[0] as Palette);
        feedback = `色系已切换：${args[0]}`;
      } else if (name === 'scan' && !args.length) {
        if (!desktopAvailable) throw new Error('扫描需要在桌面应用中使用');
        if (library.progress?.running) throw new Error('扫描已经在进行');
        await scanFolder();
        library.progress = await scanProgress();
        if (!library.progress.running) await refreshSummary();
        feedback = library.progress.running
          ? '扫描已启动，可在主窗口查看进度'
          : '文件夹选择已结束，扫描状态已刷新';
      } else if (
        ['play', 'pause', 'next', 'prev', 'vol', 'theme', 'palette', 'scan'].includes(name)
      ) {
        throw new Error(
          '可用指令：play / pause / next / prev / vol 30 / theme dark|light|system / palette celadon|cinnabar|azure|amber / scan',
        );
      } else {
        await natural(text);
      }
      command = '';
      void refresh();
    } catch (error) {
      if (!disposed) feedback = errorText(error);
    } finally {
      busy = false;
    }
  }
</script>

<Modal
  class="dialog-wide"
  aria-labelledby="console-title"
  {onclose}
  initialFocus="#console-command"
>
  <header class="dialog-header">
    <Terminal size={19} aria-hidden="true" />
    <h2 id="console-title">运行控制台 (Console)</h2>
    <IconButton label="关闭运行控制台" onclick={onclose}><X size={18} /></IconButton>
  </header>
  <div class="dialog-body console-body">
    <dl class="console-summary">
      <div>
        <dt>播放引擎</dt>
        <dd>{engine}</dd>
      </div>
      <div>
        <dt>当前输出设备</dt>
        <dd>{snapshot?.output_device_id ?? '未打开'}</dd>
      </div>
      <div>
        <dt>封面缓存命中</dt>
        <dd class="num">{cache}</dd>
      </div>
      <div>
        <dt>曲库规模</dt>
        <dd class="num">{tracks === null ? '暂不可用' : `${tracks} 首`}</dd>
      </div>
      <div class="console-media">
        <dt>系统媒体控制</dt>
        <dd>{media}</dd>
      </div>
    </dl>
    <div class="console-toolbar">
      <div class="console-views" role="group" aria-label="控制台视图">
        <button
          class="btn btn-ghost btn-sm"
          aria-pressed={view === 'logs'}
          onclick={() => (view = 'logs')}>运行日志</button
        >
        <button
          class="btn btn-ghost btn-sm"
          aria-pressed={view === 'ai'}
          onclick={() => (view = 'ai')}>AI 对话</button
        >
      </div>
      <span>{view === 'logs' ? '每 2 秒刷新' : '最多 5 轮 · 本次会话'}</span>
      <button
        class="btn btn-ghost btn-sm"
        disabled={refreshing || !desktopAvailable}
        onclick={() => refresh()}><RefreshCw size={14} />刷新</button
      >
      <button class="btn btn-ghost btn-sm" onclick={copySummary}><Copy size={14} />复制摘要</button>
      <button
        class="btn btn-ghost btn-sm"
        disabled={refreshing || !desktopAvailable}
        onclick={() => refresh(true)}><Trash2 size={14} />清空日志</button
      >
    </div>
    {#if view === 'logs'}<div
        class="console-logs mono"
        role="region"
        aria-label="运行日志"
        bind:this={logHost}
      >
        {#each logs as line}<div>{line}</div>{:else}<p>暂无运行日志</p>{/each}
      </div>{:else}
      <div
        class="console-logs console-chat"
        role="log"
        aria-label="AI 对话"
        aria-live="polite"
        bind:this={chatHost}
      >
        {#each events as event}
          <article class="chat-entry" class:chat-error={event.ok === false}>
            <strong
              >{event.role === 'user'
                ? '你'
                : event.role === 'assistant'
                  ? 'AI'
                  : `工具 · ${event.name}`}</strong
            >
            <div class:mono={event.role === 'tool'}>{event.content}</div>
          </article>
        {:else}<p>
            可以直接输入：“找几首爵士乐并开始播放”或“切换到深色宋体”。对话和必要的曲库元数据会发送到设置中的服务地址。
          </p>{/each}
      </div>
      <div class="console-chat-actions">
        <button class="btn btn-ghost btn-sm" onclick={openSettings}>前往 AI 设置</button>
        <button
          class="btn btn-ghost btn-sm"
          onclick={clearChat}
          disabled={!events.length && !aiRequest}>清空对话</button
        >
        <button
          class="btn btn-ghost btn-sm"
          disabled={!aiRequest}
          onclick={() => {
            aiRequest?.abort();
            feedback = 'AI 对话已停止，已完成的操作保留';
          }}>停止对话</button
        >
      </div>
    {/if}
    {#if refreshError}<p class="console-error">{refreshError}</p>{/if}
    <form
      class="console-command"
      onsubmit={(event) => {
        event.preventDefault();
        void execute();
      }}
    >
      <div class="console-input-heading">
        <label for="console-command">
          {#if view === 'logs'}<Terminal size={14} aria-hidden="true" />本地短命令
          {:else}<Sparkles size={14} aria-hidden="true" />AI 对话指令{/if}
        </label>
        {#if view === 'logs'}<span class="console-input-note">仅执行内置控制指令</span>
        {:else}<span class="console-model" title={modelLabel} aria-live="polite">{modelLabel}</span
          >{/if}
      </div>
      <div class="console-input-row">
        <input
          id="console-command"
          class="input"
          class:mono={view === 'logs'}
          class:console-ai-input={view === 'ai'}
          bind:value={command}
          placeholder={view === 'logs'
            ? 'play · pause · next · prev · vol 30 · theme dark · scan'
            : '直接输入需求，例如：“找几首适合深夜的歌建个歌单并播放”'}
          autocomplete="off"
          spellcheck="false"
          maxlength="4000"
          aria-describedby="console-help"
          onkeydown={(event) => {
            if (event.key === 'Enter' && event.isComposing) event.preventDefault();
          }}
        />
        <button
          class={view === 'logs' ? 'btn btn-secondary' : 'btn btn-primary'}
          type="submit"
          disabled={busy || !command.trim()}
          >{view === 'logs' ? '执行' : busy ? '发送中…' : '发送'}</button
        >
      </div>
      <p id="console-help">
        {view === 'logs'
          ? '回车执行；scan 打开文件夹选择器，自然语言自动切换到 AI 对话。'
          : '回车发送；需在设置中启用 AI 并配置密钥。'}
      </p>
    </form>
    <p class="console-feedback" role="status">{feedback}</p>
  </div>
</Modal>

<style>
  .console-views {
    display: flex;
    gap: 4px;
  }
  .console-views [aria-pressed='true'] {
    background: var(--selected);
    color: var(--text);
  }
  .console-chat-actions {
    display: flex;
    gap: 4px;
    flex-wrap: wrap;
  }
  .chat-entry + .chat-entry {
    margin-top: 12px;
  }
  .chat-entry strong {
    display: block;
    color: var(--text-2);
    font-size: var(--fs-small);
  }
  .chat-entry div {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
  .chat-error div {
    color: var(--danger);
  }
  .console-body {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .console-summary {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 12px 20px;
    padding: 12px;
    background: var(--surface);
    border: 1px solid var(--line);
    border-radius: var(--radius);
  }
  .console-summary dt {
    font-size: var(--fs-small);
    color: var(--text-3);
  }
  .console-summary dd {
    font-size: var(--fs-meta);
    color: var(--text);
    overflow-wrap: anywhere;
  }
  .console-media {
    grid-column: 1 / -1;
  }
  .console-toolbar {
    display: flex;
    align-items: center;
    gap: 4px;
    flex-wrap: wrap;
  }
  .console-toolbar > span {
    flex: 1;
    color: var(--text-3);
    font-size: var(--fs-small);
  }
  .console-logs {
    flex-shrink: 0;
    height: clamp(100px, 23vh, 220px);
    overflow: auto;
    overscroll-behavior: contain;
    padding: 12px;
    background: var(--surface);
    border: 1px solid var(--line);
    border-radius: var(--radius);
    font-size: var(--fs-small);
    line-height: 1.7;
    overflow-wrap: anywhere;
    white-space: pre-wrap;
  }
  .console-command label {
    display: flex;
    gap: 6px;
    align-items: center;
    font-size: var(--fs-meta);
    color: var(--text);
  }
  .console-input-heading {
    display: flex;
    gap: 12px;
    align-items: center;
    flex-wrap: wrap;
    margin-bottom: 6px;
  }
  .console-model {
    max-width: min(100%, 280px);
    min-width: 0;
    padding: 1px 8px;
    border: 1px solid var(--accent-line);
    border-radius: 999px;
    background: var(--accent-soft);
    color: var(--accent);
    font-size: var(--fs-small);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .console-input-note,
  .console-command p {
    font-size: var(--fs-small);
    color: var(--text-3);
  }
  .console-input-row {
    display: flex;
    gap: 8px;
  }
  .console-command input {
    flex: 1;
    min-width: 0;
  }
  .console-command input.console-ai-input {
    border-color: var(--accent-line);
    font-family: var(--font-sans);
  }
  .console-command p {
    margin-top: 4px;
  }
  .console-feedback {
    min-height: 20px;
    font-size: var(--fs-meta);
    color: var(--accent);
    overflow-wrap: anywhere;
  }
  .console-error {
    font-size: var(--fs-small);
    color: var(--danger);
    overflow-wrap: anywhere;
  }
</style>
