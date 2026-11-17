<script lang="ts">
  import Select from '../components/Select.svelte';
  import {
    getLyrics,
    setLyricsOffset,
    seek,
    trackByPath,
    audioInfo,
    type AudioInfo,
    type Lyrics,
  } from '../services/music';
  import { fileSize, channelLayout } from '../utils/format';
  import { tick, onMount } from 'svelte';
  import { groupLines, activeLine, scrollTarget } from '../utils/lyrics-view';
  import Artwork from '../components/Artwork.svelte';
  import IconButton from '../components/IconButton.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import { ChevronDown, Music2, LocateFixed } from 'lucide-svelte';
  import DesktopLyricsButton from '../components/DesktopLyricsButton.svelte';
  import WindowControls from '../components/WindowControls.svelte';
  import { player, control } from '../stores/player.svelte';
  import { ui, notifyError } from '../stores/ui.svelte';
  import { ambientColor } from '../utils/color';

  let { artwork, onclose }: { artwork: string | null; onclose: () => void } = $props();
  let snapshot = $derived(player.snapshot);

  let lyrics = $state<Lyrics | null>(null);
  let loading = $state(false);
  let manual = $state(false);
  let layout = $state('split');
  let container = $state<HTMLDivElement>();
  let trackId = $derived(snapshot?.track_id);
  let metadataRevision = $derived(snapshot?.metadata_revision ?? 0);
  let audio = $state<AudioInfo | null>(null);
  $effect(() => {
    const path = trackId;
    metadataRevision;
    let cancelled = false;
    audio = null;
    if (path)
      void trackByPath(path)
        .then((track) => (!cancelled && track ? audioInfo(track.id) : null))
        .then((info) => {
          if (!cancelled) audio = info;
        })
        .catch(() => {});
    return () => {
      cancelled = true;
    };
  });
  let specs = $derived(
    audio
      ? [
          audio.format,
          audio.channels ? channelLayout(audio.channels) : null,
          audio.sample_rate ? `${audio.sample_rate / 1000} kHz` : null,
          audio.bit_depth ? `${audio.bit_depth} bit` : null,
          audio.bitrate ? `${audio.bitrate} kbps` : null,
          audio.file_size !== null ? fileSize(audio.file_size) : null,
        ].filter((value): value is string => value !== null)
      : [],
  );
  $effect(() => {
    const path = ui.immersive ? trackId : null;
    metadataRevision;
    let cancelled = false;
    lyrics = null;
    manual = false;
    loading = !!path;
    if (path) {
      getLyrics()
        .then((l) => {
          if (!cancelled && ui.immersive) lyrics = l;
        })
        .catch((e) => {
          // Svelte pauses effects during the closing transition before destruction.
          if (!cancelled && ui.immersive) notifyError(e);
        })
        .finally(() => {
          if (!cancelled && ui.immersive) loading = false;
        });
    }
    return () => {
      cancelled = true;
    };
  });

  // Ambient colour: computed once per song from the already-loaded cover, never per progress tick.
  const ambientCache = new Map<string, string | null>();
  let ambient = $state<string | null>(null);
  $effect(() => {
    const id = trackId,
      src = artwork;
    if (!id || !src) {
      ambient = null;
      return;
    }
    if (ambientCache.has(id)) {
      ambient = ambientCache.get(id)!;
      return;
    }
    let cancelled = false;
    const img = new Image();
    img.src = src;
    img
      .decode()
      .then(() => {
        if (cancelled) return;
        const canvas = document.createElement('canvas');
        canvas.width = canvas.height = 24;
        const ctx = canvas.getContext('2d', { willReadFrequently: true });
        if (!ctx) return;
        ctx.drawImage(img, 0, 0, 24, 24);
        const rgb = ambientColor(ctx.getImageData(0, 0, 24, 24).data);
        const value = rgb ? rgb.join(' ') : null;
        ambientCache.set(id, value);
        if (ambientCache.size > 60) ambientCache.delete(ambientCache.keys().next().value!);
        if (!cancelled) ambient = value;
      })
      .catch(() => {});
    return () => {
      cancelled = true;
    };
  });

  let fontSize = $state(22);
  onMount(() => {
    try {
      const saved = Number(localStorage.getItem('lyrics-font-size'));
      if (saved >= 16 && saved <= 32) fontSize = saved;
    } catch {
      /* optional preference */
    }
  });
  function changeSize(value: number) {
    fontSize = value;
    try {
      localStorage.setItem('lyrics-font-size', String(value));
    } catch {
      /* keep session preference */
    }
  }
  let rows = $derived(groupLines(lyrics?.lines ?? [], lyrics?.synced ?? false));
  let active = $derived(
    lyrics?.synced
      ? activeLine(rows, (snapshot?.position_ms ?? 0) - (snapshot?.lyrics_offset_ms ?? 0))
      : -1,
  );
  function watchSize(node: HTMLDivElement) {
    const observer = new ResizeObserver(() => {
      if (manual) return;
      const line = node.querySelector<HTMLElement>('[data-line="' + active + '"]');
      if (line)
        node.scrollTo({
          top: scrollTarget(
            line.offsetTop,
            line.offsetHeight,
            node.clientHeight,
            node.scrollHeight,
          ),
          behavior: 'instant',
        });
    });
    observer.observe(node);
    return {
      destroy() {
        observer.disconnect();
      },
    };
  }
  let scrollVersion = 0;
  $effect(() => {
    const index = active;
    const browsing = manual;
    const host = container;
    rows;
    fontSize;
    layout;
    const version = ++scrollVersion;
    if (!browsing && host) {
      void tick().then(() => {
        if (version !== scrollVersion) return;
        const line = host.querySelector<HTMLElement>('[data-line="' + index + '"]');
        host.scrollTo({
          top: line
            ? scrollTarget(line.offsetTop, line.offsetHeight, host.clientHeight, host.scrollHeight)
            : 0,
          behavior: ui.reduceMotion ? 'instant' : 'smooth',
        });
      });
    }
    return () => {
      scrollVersion++;
    };
  });
  async function jump(time: number) {
    await control(() =>
      seek(
        Math.min(snapshot?.duration_ms ?? 0, Math.max(0, time + (snapshot?.lyrics_offset_ms ?? 0))),
      ),
    );
    manual = false;
  }
</script>

<section
  style:--lyric-size={fontSize + 'px'}
  style:--ambient={ambient ?? undefined}
  class="immersive"
  class:reduce-motion={ui.reduceMotion}
  class:has-ambient={!!ambient && !ui.reduceMotion}
  aria-label="正在播放"
>
  <header class="immersive-header" data-tauri-drag-region>
    <IconButton label="收起正在播放 (Esc)" onclick={onclose}><ChevronDown size={20} /></IconButton>
    <span class="immersive-caption">正在播放</span>
    <DesktopLyricsButton />
    <Select
      label="正在播放布局"
      bind:value={layout}
      options={[
        { value: 'split', label: '封面与歌词' },
        { value: 'cover', label: '大封面' },
        { value: 'lyrics', label: '纯歌词' },
      ]}
    />
    <WindowControls />
  </header>
  <div
    class="immersive-content"
    class:cover-only={layout === 'cover'}
    class:lyrics-only={layout === 'lyrics'}
  >
    {#if layout !== 'lyrics'}<div class="artwork-panel">
        <div class="large-artwork"><Artwork src={artwork} alt="当前专辑封面" /></div>
        <div class="artwork-meta">
          <h1 title={snapshot?.current_track?.title}>
            {snapshot?.current_track?.title ?? '未在播放'}
          </h1>
          <p title={snapshot?.current_track?.artist}>
            {snapshot?.current_track?.artist ?? '从音乐库选择一首歌曲'}
          </p>
          {#if snapshot?.current_track?.album}<small title={snapshot.current_track.album}
              >{snapshot.current_track.album}</small
            >{/if}
          {#if specs.length}<div
              class="audio-specs num"
              aria-label="专业音频参数"
              title={specs.join(' · ')}
            >
              {specs.join(' · ')}
            </div>{/if}
        </div>
      </div>{/if}
    {#if layout !== 'cover'}<div class="lyrics-panel">
        <div class="lyric-tools">
          <span class="lyric-source">{lyrics?.source ?? '本地歌词'}</span>
          <label class="lyric-field"
            >字号 <input
              class="slider slider-sm"
              style:--fill={((fontSize - 16) / 16) * 100 + '%'}
              aria-label="歌词字号"
              type="range"
              min="16"
              max="32"
              step="1"
              value={fontSize}
              oninput={(e) => changeSize(Number(e.currentTarget.value))}
            /><output>{fontSize}</output></label
          >
          <label class="lyric-field"
            >偏移 <input
              class="input input-sm"
              aria-label="歌词时间偏移（毫秒，正数延后）"
              type="number"
              min="-60000"
              max="60000"
              step="100"
              value={snapshot?.lyrics_offset_ms ?? 0}
              onchange={(e) => control(() => setLyricsOffset(Number(e.currentTarget.value)))}
            /> ms</label
          >
          {#if manual}<button
              type="button"
              class="btn btn-ghost btn-sm"
              onclick={() => (manual = false)}><LocateFixed size={14} />回到当前行</button
            >{/if}
        </div>
        <div
          class="lyric-lines"
          class:empty-lyrics={!lyrics?.lines.length}
          bind:this={container}
          use:watchSize
          onwheel={() => (manual = true)}
          ontouchmove={() => (manual = true)}
          onpointerdown={() => (manual = true)}
          onkeydown={(e) => {
            if (['ArrowDown', 'ArrowUp', 'PageDown', 'PageUp', 'Home', 'End'].includes(e.key))
              manual = true;
          }}
          tabindex="0"
          role="listbox"
          aria-label="歌词，滚动后可回到当前行"
        >
          {#if loading}<p class="lyric-empty">
              正在读取歌词…
            </p>{:else if lyrics?.lines.length}{#each rows as line, i}<button
                role="option"
                aria-selected={active === i}
                data-line={i}
                class:current-line={active === i}
                class:past-line={active > i}
                disabled={!lyrics.synced || snapshot?.playback_state === 'idle'}
                onclick={() => jump(line.time_ms)}
                >{#each line.parts as part, j}<span class:lyric-secondary={j > 0}
                    >{part || '♪'}</span
                  >{/each}</button
              >{/each}{:else}<div class="lyric-empty">
              <EmptyState
                icon={Music2}
                title="暂无歌词"
                message="把同名 .lrc 文件放在歌曲旁边，或使用歌曲内嵌歌词。"
              />
            </div>{/if}
        </div>
      </div>{/if}
  </div>
</section>

<style>
  .audio-specs {
    min-width: 0;
    max-width: 100%;
    margin-top: 8px;
    padding: 3px 10px;
    border-radius: 999px;
    background: rgb(var(--ink) / 0.05);
    color: var(--text-3);
    font-size: 11px;
    font-variant-numeric: tabular-nums;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  :global {
    /* ============================================================ now playing */
    .immersive-layer {
      position: fixed;
      inset: 0 0 var(--player-h) 0;
      z-index: var(--z-immersive);
    }

    .immersive {
      position: absolute;
      inset: 0;
      display: flex;
      flex-direction: column;
      overflow: hidden;
      background: var(--bg);
    }

    .immersive::before {
      content: '';
      position: absolute;
      inset: 0;
      pointer-events: none;
      background: radial-gradient(ellipse 70% 80% at 22% 40%, var(--glow), transparent 70%);
    }

    .immersive.has-ambient::before {
      background:
        radial-gradient(
          ellipse 75% 85% at 22% 40%,
          rgb(var(--ambient) / var(--ambient-a)),
          transparent 72%
        ),
        radial-gradient(
          ellipse 60% 70% at 85% 80%,
          rgb(var(--ambient) / var(--ambient-b)),
          transparent 70%
        );
    }

    .immersive > * {
      position: relative;
    }

    .immersive-header {
      display: flex;
      align-items: center;
      gap: var(--space-3);
      height: 56px;
      padding: 0 var(--page-x) 0 calc(var(--page-x) - 8px);
      flex-shrink: 0;
    }

    .immersive-caption {
      flex: 1;
      color: var(--text-2);
      font-size: var(--fs-small);
      font-weight: var(--fw-semibold);
    }

    .immersive-content {
      flex: 1;
      min-height: 0;
      display: grid;
      grid-template-columns: minmax(0, 1fr) minmax(0, 1.2fr);
      gap: clamp(24px, 5vw, 80px);
      width: 100%;
      max-width: 1240px;
      margin: 0 auto;
      padding: var(--space-4) clamp(20px, 4vw, 64px) var(--space-6);
      align-items: center;
    }

    .cover-only,
    .lyrics-only {
      grid-template-columns: minmax(0, 1fr);
      max-width: 760px;
    }

    .artwork-panel {
      min-width: 0;
      max-height: 100%;
      display: flex;
      flex-direction: column;
      align-items: center;
      text-align: center;
      overflow: hidden;
    }

    .large-artwork {
      position: relative;
      width: min(100%, 42vh, 440px);
      aspect-ratio: 1;
      border-radius: var(--radius-xl);
      overflow: hidden;
      box-shadow: var(--shadow-art);
      flex-shrink: 0;
    }

    .artwork-meta {
      min-width: 0;
      width: 100%;
      max-width: 100%;
      flex-shrink: 0;
      display: flex;
      flex-direction: column;
      align-items: center;
    }

    .artwork-panel h1 {
      margin-top: var(--space-4);
      max-width: 100%;
      font-family: var(--font-display);
      font-size: clamp(20px, 2vw, 28px);
      font-weight: 700;
      line-height: var(--leading-title);
      letter-spacing: var(--tracking-title);
      overflow-wrap: anywhere;
      display: -webkit-box;
      -webkit-line-clamp: 2;
      line-clamp: 2;
      -webkit-box-orient: vertical;
      overflow: hidden;
    }

    .artwork-panel p {
      margin-top: 6px;
      color: var(--text-2);
      font-size: 15px;
      max-width: 100%;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .artwork-panel small {
      margin-top: 2px;
      max-width: 100%;
      color: var(--text-3);
      font-size: var(--fs-meta);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .lyrics-panel {
      height: 100%;
      min-height: 0;
      min-width: 0;
      display: flex;
      flex-direction: column;
    }

    .lyric-tools {
      display: flex;
      align-items: center;
      flex-wrap: wrap;
      gap: 8px 16px;
      padding-bottom: var(--space-3);
      color: var(--text-3);
      font-size: var(--fs-small);
      flex-shrink: 0;
    }

    .lyric-source {
      max-width: 40%;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .lyric-field {
      display: inline-flex;
      align-items: center;
      gap: 8px;
    }

    .lyric-field output {
      min-width: 20px;
      font-variant-numeric: tabular-nums;
    }

    .lyric-lines {
      position: relative;
      flex: 1;
      min-height: 0;
      overflow-x: hidden;
      overflow-y: auto;
      overscroll-behavior: contain;
      padding: 38% 12px 60%;
      scrollbar-gutter: stable;
      overflow-anchor: none;
      mask-image: linear-gradient(to bottom, transparent, #000 12%, #000 82%, transparent);
      outline: none;
    }

    .lyric-lines:focus-visible {
      box-shadow: inset 0 0 0 2px var(--focus);
      border-radius: var(--radius);
    }

    .lyric-lines > button {
      display: block;
      width: 100%;
      max-width: 36rem;
      margin-inline: auto;
      padding: 10px 8px;
      border-radius: var(--radius);
      text-align: start;
      font-family: var(--font-display);
      font-size: clamp(16px, calc(var(--lyric-size) * 0.7 + 0.5vw), 32px);
      font-weight: 600;
      line-height: var(--leading-lyrics);
      letter-spacing: var(--tracking-lyrics);
      color: rgb(var(--ink) / var(--lyric-dim));
      white-space: pre-wrap;
      overflow-wrap: anywhere;
      transition:
        color var(--dur) var(--ease),
        transform var(--dur) var(--ease);
      transform-origin: left center;
    }

    .lyric-lines > button:not(:disabled):hover {
      color: rgb(var(--ink) / 0.7);
      background: rgb(var(--ink) / 0.04);
    }

    .lyric-lines > button.past-line {
      color: rgb(var(--ink) / var(--lyric-past));
    }

    .lyric-lines > button.current-line {
      color: rgb(var(--ink) / 1);
      font-weight: 700;
    }

    .immersive:not(.reduce-motion) .lyric-lines > button.current-line {
      transform: scale(1.03);
    }

    .immersive.reduce-motion .lyric-lines > button {
      transition: none;
    }

    .lyric-lines > button:disabled {
      cursor: default;
    }

    .lyric-lines > button > span {
      display: block;
    }

    .lyric-lines > button .lyric-secondary {
      font-size: 0.74em;
      font-weight: 400;
      margin-top: 0.2em;
    }

    .lyric-lines.empty-lyrics {
      display: grid;
      place-items: center;
      padding: 24px;
      mask-image: none;
    }

    .lyric-empty {
      color: var(--text-3);
    }
  }
</style>
