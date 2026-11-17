<script lang="ts">
  import { onMount } from 'svelte';
  import { X, Copy, Play, ListEnd, ListPlus } from 'lucide-svelte';
  import Artwork from './Artwork.svelte';
  import Select from './Select.svelte';
  import IconButton from './IconButton.svelte';
  import Modal from './Modal.svelte';
  import {
    trackCover,
    audioInfo,
    playTrack,
    queueAction,
    addToPlaylist,
    type LibraryTrack,
    type AudioInfo,
  } from '../services/music';
  import { perform } from '../stores/player.svelte';
  import { library, refreshSummary } from '../stores/library.svelte';
  import { errorText, duration, fileSize, channelLayout } from '../utils/format';
  let { track, onclose }: { track: LibraryTrack; onclose: () => void } = $props();
  let cover = $state<string | null>(null);
  let audio = $state<AudioInfo | null>(null);
  let audioError = $state(false);
  let busy = $state(false);
  let feedback = $state('');
  let playlist = $state('');
  onMount(() => {
    let disposed = false;
    void trackCover(track.id)
      .then((v) => {
        if (!disposed) cover = v;
      })
      .catch(() => {});
    void audioInfo(track.id)
      .then((v) => {
        if (!disposed) audio = v;
      })
      .catch(() => {
        if (!disposed) audioError = true;
      });
    return () => {
      disposed = true;
    };
  });
  async function run(action: () => Promise<void>) {
    if (busy) return;
    busy = true;
    feedback = '';
    try {
      await action();
    } catch (e) {
      feedback = errorText(e);
    } finally {
      busy = false;
    }
  }
  async function copy() {
    await navigator.clipboard.writeText(track.path);
    feedback = '已复制文件路径';
  }
</script>

<Modal class="dialog-wide" aria-labelledby="song-detail-title" {onclose}>
  <header class="dialog-header">
    <h2 class="dialog-heading-sm">歌曲详情</h2>
    <IconButton label="关闭歌曲详情" onclick={onclose}><X size={18} /></IconButton>
  </header>
  <div class="dialog-body details-body">
    <div class="details-hero">
      <div class="details-cover"><Artwork src={cover} /></div>
      <div class="details-head">
        <h3 id="song-detail-title">{track.title}</h3>
        <p>{track.artist || '未知艺术家'}</p>
        <p class="muted">{track.album || '未知专辑'}</p>
      </div>
    </div>
    <div class="details-actions">
      <button
        type="button"
        class="btn btn-primary"
        disabled={busy}
        onclick={() =>
          run(async () => {
            await perform(() => playTrack(track.id));
          })}><Play size={16} fill="currentColor" />播放</button
      >
      <button
        type="button"
        class="btn btn-secondary"
        disabled={busy}
        onclick={() =>
          run(async () => {
            if (await perform(() => queueAction('add', track.id))) feedback = '已加入播放队列';
          })}><ListEnd size={16} />加入队列</button
      >
    </div>
    <section class="details-section">
      <h4>歌曲信息</h4>
      <dl class="info-grid">
        <dt>专辑</dt>
        <dd>{track.album || '未知专辑'}</dd>
        <dt>艺术家</dt>
        <dd>{track.artist || '未知艺术家'}</dd>
        {#if track.album_artist && track.album_artist !== track.artist}<dt>专辑艺术家</dt>
          <dd>{track.album_artist}</dd>{/if}
        <dt>时长</dt>
        <dd class="num">{duration(track.duration_ms)}</dd>
        {#if track.track_number}<dt>曲目</dt>
          <dd class="num">
            {track.disc_number ? track.disc_number + '-' : ''}{track.track_number}
          </dd>{/if}
        {#if track.year}<dt>年份</dt>
          <dd class="num">{track.year}</dd>{/if}
        {#if track.genre}<dt>流派</dt>
          <dd>{track.genre}</dd>{/if}
      </dl>
    </section>
    <section class="details-section">
      <h4>音频文件</h4>
      <dl class="info-grid">
        {#if audio}
          <dt>格式</dt>
          <dd>{audio.format}</dd>
          {#if audio.sample_rate}<dt>采样率</dt>
            <dd class="num">
              {(audio.sample_rate / 1000).toLocaleString(undefined, { maximumFractionDigits: 1 })} kHz
            </dd>{/if}
          {#if audio.channels}<dt>声道</dt>
            <dd>{channelLayout(audio.channels)}</dd>{/if}
          {#if audio.bit_depth}<dt>位深</dt>
            <dd class="num">{audio.bit_depth} bit</dd>{/if}
          {#if audio.bitrate}<dt>码率</dt>
            <dd class="num">{audio.bitrate} kbps</dd>{/if}
          {#if audio.file_size !== null}<dt>文件大小</dt>
            <dd class="num">{fileSize(audio.file_size)}</dd>{/if}
        {:else}<dt>音频参数</dt>
          <dd>{audioError ? '暂时无法读取' : '正在读取…'}</dd>{/if}
      </dl>
      <label class="field-label" for="detail-path">文件位置</label>
      <div class="path-copy">
        <input
          id="detail-path"
          class="input mono"
          readonly
          value={track.path}
          title={track.path}
        /><IconButton label="复制文件路径" onclick={() => run(copy)}><Copy size={16} /></IconButton>
      </div>
    </section>
    <section class="details-section">
      <h4>添加到歌单</h4>
      <div class="playlist-add">
        <Select
          label="选择歌单"
          bind:value={playlist}
          options={[
            {
              value: '',
              label: library.summary.playlists.length ? '选择歌单' : '请先在侧边栏创建歌单',
            },
            ...library.summary.playlists.map((p) => ({ value: String(p.id), label: p.name })),
          ]}
        />
        <button
          type="button"
          class="btn btn-secondary"
          disabled={!playlist || busy}
          onclick={() =>
            run(async () => {
              await addToPlaylist(Number(playlist), track.id);
              await refreshSummary();
              feedback = '已加入歌单';
            })}><ListPlus size={16} />加入歌单</button
        >
      </div>
    </section>
    <p class="detail-feedback" role="status">{feedback}</p>
  </div>
</Modal>

<style>
  :global {
    /* ============================================================ song details dialog */
    .details-body {
      color: var(--text);
    }

    .details-hero {
      display: flex;
      align-items: center;
      gap: var(--space-4);
    }

    .details-cover {
      position: relative;
      flex-shrink: 0;
      width: 96px;
      height: 96px;
      border-radius: var(--radius);
      overflow: hidden;
    }

    .details-head {
      min-width: 0;
    }

    .details-head h3 {
      font-size: var(--fs-section);
      font-weight: var(--fw-semibold);
      line-height: 1.35;
      overflow-wrap: anywhere;
    }

    .details-head p {
      margin-top: 2px;
      color: var(--text-2);
      font-size: var(--fs-meta);
      overflow-wrap: anywhere;
    }

    .details-head p.muted {
      color: var(--text-3);
    }

    .details-actions {
      display: flex;
      gap: 8px;
      flex-wrap: wrap;
      margin: var(--space-4) 0 var(--space-2);
    }

    .details-section {
      padding: var(--space-4) 0 0;
      margin-top: var(--space-4);
      border-top: 1px solid var(--line);
    }

    .details-section h4 {
      margin-bottom: var(--space-3);
      color: var(--text-2);
      font-size: var(--fs-small);
      font-weight: var(--fw-semibold);
    }

    .info-grid {
      display: grid;
      grid-template-columns: 96px minmax(0, 1fr);
      gap: 8px 16px;
      font-size: var(--fs-meta);
      line-height: 1.5;
    }

    .info-grid dt {
      color: var(--text-3);
    }

    .info-grid dd {
      color: var(--text);
      overflow-wrap: anywhere;
    }

    .details-section .field-label {
      margin-top: var(--space-4);
    }

    .path-copy {
      display: flex;
      align-items: center;
      gap: 6px;
    }

    .path-copy .input {
      flex: 1;
      min-width: 0;
      text-overflow: ellipsis;
    }

    .playlist-add {
      display: flex;
      align-items: center;
      gap: 8px;
      flex-wrap: wrap;
    }

    .playlist-add .select-trigger {
      flex: 1;
      min-width: 180px;
    }

    .detail-feedback {
      min-height: 20px;
      margin-top: var(--space-3);
      color: var(--accent);
      font-size: var(--fs-meta);
    }
  }
</style>
