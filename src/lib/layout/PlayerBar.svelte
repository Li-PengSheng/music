<script lang="ts">
  import {
    Play,
    Pause,
    SkipBack,
    SkipForward,
    Shuffle,
    Repeat,
    Repeat1,
    Volume2,
    Volume1,
    VolumeX,
    ListMusic,
    Maximize2,
  } from 'lucide-svelte';
  import Artwork from '../components/Artwork.svelte';
  import Slider from '../components/Slider.svelte';
  import IconButton from '../components/IconButton.svelte';
  import { tooltip } from '../components/tooltip';
  import { player, control, accept } from '../stores/player.svelte';
  import { ui } from '../stores/ui.svelte';
  import {
    play,
    pause,
    seek,
    setVolume,
    setMode,
    queueAction,
    type PlaybackMode,
  } from '../services/music';
  import { duration } from '../utils/format';
  import { DEFAULT_PERCENT, toPercent, toGain, muteTarget, latest } from '../utils/volume';
  import { notifyError } from '../stores/ui.svelte';

  const UNMUTE_KEY = 'music-mine-unmute-volume';

  let {
    artwork,
    ontogglequeue,
    queueTrigger = $bindable(),
  }: {
    artwork: string | null;
    ontogglequeue: () => void;
    queueTrigger?: HTMLButtonElement;
  } = $props();

  let s = $derived(player.snapshot);
  let track = $derived(s?.current_track ?? null);
  let playing = $derived(s?.playback_state === 'playing');
  let mode = $derived<PlaybackMode>(s?.playback_mode ?? 'sequential');
  let hasQueue = $derived(!!s?.queue.length);

  let dragging = $state<number | null>(null);
  let trackId = $derived(s?.track_id);
  $effect(() => {
    trackId;
    dragging = null;
  });
  let position = $derived(dragging ?? s?.position_ms ?? 0);
  let total = $derived(s?.duration_ms ?? 0);
  let canSeek = $derived(!!total && s?.playback_state !== 'idle');

  // The draft only covers the gap until Rust's snapshot confirms the last value sent.
  let volumeDraft = $state<number | null>(null);
  let volume = $derived(volumeDraft ?? (s ? toPercent(s.volume) : DEFAULT_PERCENT));
  let lastAudible = $state(loadUnmuteLevel());
  $effect(() => {
    if (s && s.volume > 0) lastAudible = toPercent(s.volume);
  });
  const volumeSync = latest((percent: number) => setVolume(toGain(percent)), {
    result: accept,
    error: notifyError,
    settled: () => {
      volumeDraft = null;
    },
  });

  function changeVolume(value: number) {
    volumeDraft = value;
    volumeSync.push(value);
  }
  function toggleMute() {
    const target = muteTarget(volume, lastAudible);
    if (target === 0) saveUnmuteLevel(volume);
    changeVolume(target);
  }
  function loadUnmuteLevel() {
    try {
      return Number(localStorage.getItem(UNMUTE_KEY)) || DEFAULT_PERCENT;
    } catch {
      return DEFAULT_PERCENT;
    }
  }
  function saveUnmuteLevel(value: number) {
    try {
      localStorage.setItem(UNMUTE_KEY, String(value));
    } catch {
      /* in-memory only */
    }
  }
  async function commitSeek(value: number) {
    await control(() => seek(value));
    dragging = null;
  }

  const repeatNext: Record<PlaybackMode, PlaybackMode> = {
    sequential: 'repeat_all',
    shuffle: 'repeat_all',
    repeat_all: 'repeat_one',
    repeat_one: 'sequential',
  };
  let repeatLabel = $derived(
    mode === 'repeat_all' ? '列表循环' : mode === 'repeat_one' ? '单曲循环' : '循环播放：关',
  );
</script>

<footer class="player-bar" aria-label="播放控制">
  <div class="pb-track">
    <button
      type="button"
      class="pb-cover"
      aria-label="打开正在播放"
      disabled={!track}
      onclick={() => (ui.immersive = true)}
      ><Artwork src={artwork} alt={track?.album || '专辑封面'} /></button
    >
    <div class="pb-meta">
      {#if track}
        <button
          type="button"
          class="pb-title"
          onclick={() => (ui.immersive = true)}
          use:tooltip={{ text: track.title, overflow: true }}>{track.title}</button
        >
        <span class="pb-artist" use:tooltip={{ text: track.artist, overflow: true }}
          >{track.artist || '未知艺术家'}</span
        >
      {:else}
        <span class="pb-title pb-idle">未在播放</span>
        <span class="pb-artist">从音乐库选择一首歌曲</span>
      {/if}
    </div>
  </div>

  <div class="pb-center">
    <div class="pb-controls">
      <IconButton
        label={mode === 'shuffle' ? '随机播放：开' : '随机播放：关'}
        active={mode === 'shuffle'}
        aria-pressed={mode === 'shuffle'}
        disabled={!s || player.busy}
        onclick={() => control(() => setMode(mode === 'shuffle' ? 'sequential' : 'shuffle'))}
        ><Shuffle size={17} /></IconButton
      >
      <IconButton
        label="上一首"
        size="lg"
        disabled={!hasQueue || player.busy}
        onclick={() => control(() => queueAction('previous'))}
        ><SkipBack size={19} fill="currentColor" /></IconButton
      >
      <button
        type="button"
        class="pb-play"
        aria-label={playing ? '暂停' : '播放'}
        use:tooltip={playing ? '暂停 (Space)' : '播放 (Space)'}
        disabled={!track || player.busy}
        onclick={() => control(playing ? pause : play)}
        >{#if playing}<Pause size={18} fill="currentColor" />{:else}<Play
            size={18}
            fill="currentColor"
            class="pb-play-icon"
          />{/if}</button
      >
      <IconButton
        label="下一首"
        size="lg"
        disabled={!hasQueue || player.busy}
        onclick={() => control(() => queueAction('next'))}
        ><SkipForward size={19} fill="currentColor" /></IconButton
      >
      <IconButton
        label={repeatLabel}
        active={mode === 'repeat_all' || mode === 'repeat_one'}
        aria-pressed={mode === 'repeat_all' || mode === 'repeat_one'}
        disabled={!s || player.busy}
        onclick={() => control(() => setMode(repeatNext[mode]))}
        >{#if mode === 'repeat_one'}<Repeat1 size={17} />{:else}<Repeat
            size={17}
          />{/if}</IconButton
      >
    </div>
    <div class="pb-progress">
      <span class="pb-time">{duration(position)}</span>
      <Slider
        class="pb-seek"
        label="播放进度"
        value={position}
        max={total || 1}
        valuetext={duration(position) + ' / ' + duration(total)}
        disabled={!canSeek || player.busy}
        oninput={(v) => (dragging = v)}
        onchange={commitSeek}
      />
      <span class="pb-time pb-time-end">{duration(total)}</span>
    </div>
  </div>

  <div class="pb-options">
    <IconButton
      label="正在播放与歌词 (Ctrl+L)"
      active={ui.immersive}
      disabled={!track}
      onclick={() => (ui.immersive = !ui.immersive)}><Maximize2 size={17} /></IconButton
    >
    <IconButton
      label="播放队列"
      active={ui.queueOpen}
      aria-expanded={ui.queueOpen}
      aria-controls="playback-queue"
      bind:element={queueTrigger}
      onclick={ontogglequeue}><ListMusic size={18} /></IconButton
    >
    <div class="pb-volume">
      <IconButton
        label={volume > 0 ? `静音（当前音量 ${volume}%）` : `取消静音（恢复到 ${lastAudible}%）`}
        disabled={!s}
        onclick={toggleMute}
      >
        {#if volume === 0}<VolumeX size={18} />{:else if volume < 50}<Volume1
            size={18}
          />{:else}<Volume2 size={18} />{/if}
      </IconButton>
      <Slider
        class="pb-volume-slider"
        label="音量"
        value={volume}
        valuetext={volume === 0 ? '0%，已静音' : volume + '%'}
        disabled={!s}
        oninput={changeVolume}
        onchange={changeVolume}
      />
      <span class="pb-volume-value" aria-hidden="true">{volume}%</span>
    </div>
  </div>
</footer>

<style>
  :global {
    /* ============================================================ player bar */
    .player-bar {
      grid-column: 1/-1;
      grid-row: 2;
      z-index: var(--z-player);
      display: grid;
      grid-template-columns: minmax(180px, 1fr) minmax(320px, 2fr) minmax(180px, 1fr);
      align-items: center;
      gap: var(--space-4);
      padding: 0 var(--space-4);
      background: var(--bg-player);
      border-top: 1px solid var(--line);
    }

    .pb-track {
      display: flex;
      align-items: center;
      gap: var(--space-3);
      min-width: 0;
    }

    .pb-cover {
      position: relative;
      flex-shrink: 0;
      width: 56px;
      height: 56px;
      border-radius: var(--radius-sm);
      overflow: hidden;
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.4);
    }

    .pb-cover:disabled {
      cursor: default;
    }

    .pb-meta {
      min-width: 0;
      display: flex;
      flex-direction: column;
      align-items: flex-start;
      gap: 2px;
    }

    .pb-title {
      max-width: 100%;
      color: var(--text);
      font-size: var(--fs-body);
      font-weight: var(--fw-semibold);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      text-align: left;
      border-radius: var(--radius-xs);
    }

    button.pb-title:hover {
      text-decoration: underline;
    }

    .pb-idle {
      color: var(--text-2);
    }

    .pb-artist {
      max-width: 100%;
      color: var(--text-3);
      font-size: var(--fs-small);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .pb-center {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 4px;
      min-width: 0;
      max-width: 640px;
      width: 100%;
      justify-self: center;
    }

    .pb-controls {
      display: flex;
      align-items: center;
      gap: var(--space-3);
    }

    .pb-play {
      display: grid;
      place-items: center;
      flex-shrink: 0;
      width: 38px;
      height: 38px;
      border-radius: 50%;
      background: var(--text);
      color: var(--bg);
      transition:
        transform var(--dur-fast) var(--ease),
        background-color var(--dur-fast);
    }

    .pb-play:hover:not(:disabled) {
      transform: scale(1.06);
      background: var(--play-hover);
    }

    .pb-play:active:not(:disabled) {
      transform: scale(0.96);
    }

    .pb-play:disabled {
      opacity: 0.4;
    }

    .pb-play-icon {
      margin-left: 2px;
    }

    .pb-progress {
      display: grid;
      grid-template-columns: 44px minmax(0, 1fr) 44px;
      align-items: center;
      gap: var(--space-2);
      width: 100%;
    }

    .pb-time {
      color: var(--text-3);
      font-size: 11px;
      text-align: right;
    }

    .pb-time-end {
      text-align: left;
    }

    .pb-options {
      display: flex;
      align-items: center;
      justify-content: flex-end;
      gap: var(--space-1);
      min-width: 0;
    }

    .pb-volume {
      display: flex;
      align-items: center;
      gap: 4px;
      min-width: 0;
    }

    .pb-volume .slider {
      width: clamp(64px, 8vw, 112px);
    }

    .pb-volume-value {
      min-width: 4ch;
      color: var(--text-2);
      font-size: var(--fs-small);
      font-variant-numeric: tabular-nums;
      text-align: right;
      user-select: none;
    }
  }
</style>
