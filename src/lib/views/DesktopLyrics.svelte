<script lang="ts">
  import { onMount } from 'svelte';
  import { Lock, Pause, Play, SkipBack, SkipForward, X } from 'lucide-svelte';
  import IconButton from '../components/IconButton.svelte';
  import {
    desktopAvailable,
    getLyrics,
    getPlayerState,
    pause,
    play,
    queueAction,
    type Lyrics,
  } from '../services/music';
  import { accept, control, player } from '../stores/player.svelte';
  import { notifyError, ui } from '../stores/ui.svelte';
  import { loadLocalPreferences, watchFontPreference } from '../stores/preferences';
  import { activeLine, groupLines } from '../utils/lyrics-view';
  import { hideDesktopLyrics, lockDesktopLyrics } from '../stores/lyrics-overlay';

  let snapshot = $derived(player.snapshot);
  let lyrics = $state<Lyrics | null>(null);
  let playing = $derived(snapshot?.playback_state === 'playing');
  let track = $derived(snapshot?.current_track ?? null);
  let rows = $derived(groupLines(lyrics?.lines ?? [], lyrics?.synced ?? false));
  let active = $derived(
    lyrics?.synced
      ? activeLine(rows, (snapshot?.position_ms ?? 0) - (snapshot?.lyrics_offset_ms ?? 0))
      : -1,
  );
  let line = $derived(active >= 0 ? rows[active] : null);
  let hasQueue = $derived(!!snapshot?.queue.length);

  onMount(() => {
    loadLocalPreferences();
    const stopFont = watchFontPreference();
    if (!desktopAvailable) {
      player.connection = '浏览器预览';
      return stopFont;
    }
    let disposed = false;
    let timer: ReturnType<typeof setTimeout>;
    let lyricTrack = '';
    let lyricTicket = 0;
    let lyricPending = false;
    let lyricLoaded = false;
    let retryAt = 0;
    let reported = false;
    async function refresh() {
      try {
        const next = await getPlayerState();
        if (disposed) return;
        accept(next);
        player.connected = true;
        reported = false;
        const key = next.track_id ?? '';
        if (key !== lyricTrack) {
          lyricTrack = key;
          ++lyricTicket;
          lyricPending = false;
          lyricLoaded = false;
          retryAt = 0;
          lyrics = null;
        }
        if (key && !lyricLoaded && !lyricPending && Date.now() >= retryAt) {
          const ticket = lyricTicket;
          lyricPending = true;
          // Lyrics IO must not hold up playback polling. Invalidate late results
          // on track changes and retry transient failures without switching songs.
          void getLyrics()
            .then((loaded) => {
              if (!disposed && ticket === lyricTicket) {
                lyrics = loaded;
                lyricLoaded = true;
              }
            })
            .catch(() => {
              if (!disposed && ticket === lyricTicket) retryAt = Date.now() + 1000;
            })
            .finally(() => {
              if (!disposed && ticket === lyricTicket) lyricPending = false;
            });
        }
      } catch (error) {
        if (!disposed) {
          player.connected = false;
          if (!reported) {
            reported = true;
            notifyError(error);
          }
        }
      } finally {
        if (!disposed) timer = setTimeout(refresh, 200);
      }
    }
    void refresh();
    return () => {
      disposed = true;
      clearTimeout(timer);
      stopFont();
    };
  });
</script>

<div
  class="desktop-lyrics"
  class:reduce-motion={ui.reduceMotion}
  role="region"
  aria-label="桌面歌词"
>
  <div class="lyrics-drag" data-tauri-drag-region></div>
  <div class="lyrics-copy" data-tauri-drag-region>
    {#if line}
      <p class="lyrics-main" data-current="true" title={line.parts[0] || '♪'}>
        {line.parts[0] || '♪'}
      </p>
      {#if line.parts.length > 1}
        <p class="lyrics-sub lyric-secondary" title={line.parts.slice(1).join(' · ')}>
          {line.parts.slice(1).join(' · ')}
        </p>
      {/if}
    {:else if track}
      <p class="lyrics-main" title={track.title}>{track.title}</p>
      <p class="lyrics-sub lyric-secondary">暂无同步歌词</p>
    {:else}
      <p class="lyrics-main">未在播放</p>
    {/if}
  </div>
  <div class="lyrics-controls" role="toolbar" aria-label="桌面歌词控制">
    <p
      class="lyrics-kicker"
      title={track ? [track.title, track.artist].filter(Boolean).join(' · ') : ''}
    >
      {track ? [track.title, track.artist].filter(Boolean).join(' · ') : '桌面歌词'}
    </p>
    <div class="lyrics-buttons">
      <IconButton
        label="上一首"
        size="sm"
        disabled={!hasQueue || player.busy}
        onclick={() => control(() => queueAction('previous'))}><SkipBack size={15} /></IconButton
      >
      <IconButton
        label={playing ? '暂停' : '播放'}
        size="sm"
        disabled={!track || player.busy}
        onclick={() => control(playing ? pause : play)}
        >{#if playing}<Pause size={15} />{:else}<Play size={15} />{/if}</IconButton
      >
      <IconButton
        label="下一首"
        size="sm"
        disabled={!hasQueue || player.busy}
        onclick={() => control(() => queueAction('next'))}><SkipForward size={15} /></IconButton
      >
      <IconButton label="锁定穿透" size="sm" onclick={() => lockDesktopLyrics()}
        ><Lock size={15} /></IconButton
      >
      <IconButton label="关闭桌面歌词" size="sm" onclick={() => hideDesktopLyrics()}
        ><X size={15} /></IconButton
      >
    </div>
  </div>
  {#if ui.toast}
    <p class="lyrics-toast" role={ui.toast.tone === 'error' ? 'alert' : 'status'}>
      {ui.toast.message}
    </p>
  {/if}
</div>

<style>
  :global(html:has(.desktop-lyrics)),
  :global(html:has(.desktop-lyrics) body),
  :global(html:has(.desktop-lyrics) #app) {
    background: transparent;
  }

  :global(.desktop-lyrics) {
    --text: var(--lyrics-ink);
    --text-2: var(--lyrics-ink);
    --text-3: var(--lyrics-ink-dim);
    --surface-hover: var(--lyrics-hover);
    --accent: var(--lyrics-ink);
    position: relative;
    height: 100vh;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    padding: 10px 20px;
    overflow: hidden;
    border-radius: 14px;
    border: 1px solid var(--lyrics-panel-line);
    background: var(--lyrics-panel);
    box-shadow: var(--lyrics-shadow);
    color: var(--lyrics-ink);
    backdrop-filter: blur(18px);
    user-select: none;
  }

  :global(.lyrics-drag) {
    position: absolute;
    inset: 0;
  }

  :global(.lyrics-copy) {
    position: relative;
    z-index: 1;
    width: 100%;
    min-width: 0;
    text-align: center;
    pointer-events: none;
    transition: transform var(--dur-fast) var(--ease);
    text-shadow: var(--lyrics-text-shadow);
  }

  :global(.lyrics-kicker),
  :global(.desktop-lyrics .lyric-secondary) {
    margin: 0;
    color: var(--lyrics-ink-dim);
    font-size: var(--fs-small);
    line-height: 1.4;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.lyrics-main) {
    margin: 0;
    color: var(--lyrics-ink);
    font-family: var(--font-display);
    font-size: 21px;
    font-weight: 700;
    line-height: 1.3;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 100%;
  }

  :global(.lyrics-main:only-child) {
    font-size: 23px;
  }

  :global(.desktop-lyrics .lyric-secondary) {
    font-family: var(--font-display);
    font-size: 14px;
    font-weight: 500;
    line-height: 1.35;
    margin-top: 4px;
    max-width: 100%;
    color: var(--lyrics-ink-dim);
  }

  :global(.lyrics-controls) {
    position: absolute;
    z-index: 2;
    top: 6px;
    right: 12px;
    left: 12px;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    opacity: 0;
    pointer-events: none;
    transition: opacity var(--dur-fast) var(--ease);
  }

  :global(.lyrics-kicker) {
    min-width: 0;
    text-align: left;
  }

  :global(.lyrics-buttons) {
    display: flex;
    align-items: center;
    flex-shrink: 0;
    gap: 2px;
  }

  :global(.lyrics-buttons .icon-btn) {
    width: 26px;
    height: 26px;
  }

  :global(.desktop-lyrics:hover .lyrics-copy),
  :global(.desktop-lyrics:focus-within .lyrics-copy) {
    transform: translateY(6px);
  }

  :global(.desktop-lyrics.reduce-motion .lyrics-copy),
  :global(.desktop-lyrics.reduce-motion .lyrics-controls) {
    transition: none;
  }

  :global(.desktop-lyrics:hover .lyrics-controls),
  :global(.desktop-lyrics:focus-within .lyrics-controls) {
    opacity: 1;
    pointer-events: auto;
  }

  :global(.lyrics-toast) {
    position: absolute;
    z-index: 3;
    right: 12px;
    bottom: 8px;
    left: 12px;
    margin: 0;
    color: var(--lyrics-ink);
    font-size: var(--fs-small);
    text-align: center;
    text-shadow: 0 1px 1px rgba(0, 0, 0, 0.5);
  }
</style>
