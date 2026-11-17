<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { X, RefreshCw, Copy, Trash2, Terminal } from 'lucide-svelte';
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
  } from '../services/music';
  import { player, control } from '../stores/player.svelte';
  import { library, refreshSummary } from '../stores/library.svelte';
  import { setTheme, setPalette } from '../stores/preferences';
  import { PALETTES, type Palette, type ThemeChoice } from '../utils/theme';
  import { errorText } from '../utils/format';

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
    ]);
    if (disposed) return;
    const [logResult, coverResult, mediaResult, libraryResult] = results;
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
      feedback = '已复制诊断摘要与脱敏日志';
    } catch (error) {
      feedback = `复制失败：${errorText(error)}`;
    }
  }

  async function runPlayer(action: () => Promise<PlayerSnapshot>) {
    if (!desktopAvailable) throw new Error('播放命令需要在桌面应用中使用');
    if (!(await control(action))) throw new Error('命令未完成，请查看错误提示或稍后重试');
    if (player.snapshot?.error) throw new Error(player.snapshot.error);
  }
  async function execute() {
    if (busy) return;
    const text = command.trim();
    if (!text) return;
    busy = true;
    feedback = '';
    const [name, ...args] = text.toLowerCase().split(/\s+/);
    try {
      if (['play', 'pause', 'next', 'prev'].includes(name) && !args.length) {
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
      } else {
        throw new Error(
          '可用指令：play / pause / next / prev / vol 30 / theme dark|light|system / palette celadon|cinnabar|azure|amber / scan',
        );
      }
      command = '';
      void refresh();
    } catch (error) {
      feedback = errorText(error);
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
      <span>运行日志 · 每 2 秒刷新</span>
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
    <div class="console-logs mono" role="region" aria-label="运行日志" bind:this={logHost}>
      {#each logs as line}<div>{line}</div>{:else}<p>暂无运行日志</p>{/each}
    </div>
    {#if refreshError}<p class="console-error">{refreshError}</p>{/if}
    <form
      class="console-command"
      onsubmit={(event) => {
        event.preventDefault();
        void execute();
      }}
    >
      <label for="console-command">快捷命令<span>仅执行本地内置指令</span></label>
      <div>
        <input
          id="console-command"
          class="input mono"
          bind:value={command}
          placeholder="play · pause · next · prev · vol 30 · theme dark · palette cinnabar · scan"
          autocomplete="off"
          spellcheck="false"
          aria-describedby="console-help"
          onkeydown={(event) => {
            if (event.key === 'Enter' && event.isComposing) event.preventDefault();
          }}
        />
        <button class="btn btn-secondary" type="submit" disabled={busy || !command.trim()}
          >执行</button
        >
      </div>
      <p id="console-help">回车执行；scan 打开文件夹选择器。</p>
    </form>
    <p class="console-feedback" role="status">{feedback}</p>
  </div>
</Modal>

<style>
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
    gap: 12px;
    align-items: baseline;
    font-size: var(--fs-meta);
    color: var(--text);
    margin-bottom: 6px;
  }
  .console-command label span,
  .console-command p {
    font-size: var(--fs-small);
    color: var(--text-3);
  }
  .console-command > div {
    display: flex;
    gap: 8px;
  }
  .console-command input {
    flex: 1;
    min-width: 0;
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
