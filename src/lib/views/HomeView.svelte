<script lang="ts">
  import { Play, Pause, FolderPlus, ChevronRight, Music2 } from 'lucide-svelte';
  import Cover from '../components/Cover.svelte';
  import Artwork from '../components/Artwork.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import { tooltip } from '../components/tooltip';
  import {
    desktopAvailable,
    homeData,
    queryLibrary,
    playTrack,
    play,
    pause,
    type LibraryTrack,
  } from '../services/music';
  import { player, perform, control } from '../stores/player.svelte';
  import { library } from '../stores/library.svelte';
  import { ui, openMenu, notifyError } from '../stores/ui.svelte';
  import { go } from '../stores/nav.svelte';
  import {
    songMenu,
    playlistMenu,
    localMenu,
    artistMenu,
    openDetails,
    importFolder,
    playCollection,
  } from '../actions';
  import { LOCAL_COLLECTION, localQuery } from '../utils/collections';
  import { duration, initial, nameHue } from '../utils/format';

  let { artwork }: { artwork: string | null } = $props();

  let recent = $state<LibraryTrack[]>([]);
  let added = $state<LibraryTrack[]>([]);
  let artists = $state<[string, number][]>([]);
  let loading = $state(true);

  async function load() {
    if (!desktopAvailable) {
      loading = false;
      return;
    }
    try {
      const [h, a] = await Promise.all([homeData(), queryLibrary({ limit: 12, sort: 'added' })]);
      recent = h.recent;
      artists = h.artists;
      added = a.tracks;
    } catch (e) {
      notifyError(e);
    } finally {
      loading = false;
    }
  }
  // Reload when the library changes or a different song starts (recent plays move).
  let trackId = $derived(player.snapshot?.track_id);
  $effect(() => {
    library.revision;
    trackId;
    void load();
  });

  let s = $derived(player.snapshot);
  let playing = $derived(s?.playback_state === 'playing');
  let progress = $derived(
    s?.duration_ms ? Math.min(100, (s.position_ms / s.duration_ms) * 100) : 0,
  );

  const hour = new Date().getHours();
  const greeting =
    hour < 5
      ? '夜深了'
      : hour < 11
        ? '早上好'
        : hour < 13
          ? '中午好'
          : hour < 18
            ? '下午好'
            : '晚上好';

  function trackMenu(e: MouseEvent, t: LibraryTrack) {
    openMenu(e, songMenu(t, { onDetails: () => openDetails(t) }), t.title + ' 操作');
  }
  function artistGroup(name: string) {
    return (
      library.summary.artists.find((a) => a.name === name) ?? {
        key: name,
        name,
        artist: '',
        count: 0,
        cover_id: 0,
      }
    );
  }
</script>

<header class="home-header">
  <h2 class="page-title">{greeting}</h2>
  {#if library.loaded && library.summary.tracks}<p class="page-meta">
      {library.summary.tracks} 首歌曲 · {library.summary.albums.length} 张专辑 · {library.summary
        .artists.length} 位艺术家
    </p>{/if}
</header>

{#if !desktopAvailable}
  <EmptyState
    icon={Music2}
    title="请在 Music Mine 桌面应用中使用"
    message="浏览器预览无法读取本地音乐库。"
  />
{:else if library.loaded && !library.summary.tracks}
  <EmptyState
    icon={FolderPlus}
    title="欢迎使用 Music Mine"
    message="导入一个音乐文件夹，开始建立你的本地音乐库。音乐文件和标签不会被修改。"
  >
    <button type="button" class="btn btn-primary" onclick={importFolder}
      ><FolderPlus size={16} />导入音乐文件夹</button
    >
  </EmptyState>
{:else}
  {#if s?.current_track}
    <section class="home-section" aria-labelledby="home-continue">
      <h3 id="home-continue" class="section-title">继续播放</h3>
      <div class="continue">
        <button
          type="button"
          class="continue-art"
          aria-label="打开正在播放"
          onclick={() => (ui.immersive = true)}
          ><Artwork src={artwork} alt={s.current_track.album} /></button
        >
        <div class="continue-info">
          <strong use:tooltip={{ text: s.current_track.title, overflow: true }}
            >{s.current_track.title}</strong
          >
          <span
            >{s.current_track.artist || '未知艺术家'}{s.current_track.album
              ? ' · ' + s.current_track.album
              : ''}</span
          >
          <div class="continue-progress">
            <div class="meter" role="presentation"><i style:width={progress + '%'}></i></div>
            <small>{duration(s.position_ms)} / {duration(s.duration_ms)}</small>
          </div>
        </div>
        <button
          type="button"
          class="pb-play continue-play"
          aria-label={playing ? '暂停' : '继续播放'}
          disabled={player.busy}
          onclick={() => control(playing ? pause : play)}
        >
          {#if playing}<Pause size={18} fill="currentColor" />{:else}<Play
              size={18}
              fill="currentColor"
              class="pb-play-icon"
            />{/if}
        </button>
      </div>
    </section>
  {/if}

  {#snippet shelf(id: string, heading: string, tracks: LibraryTrack[], more?: () => void)}
    <section class="home-section" aria-labelledby={id}>
      <div class="section-head">
        <h3 {id} class="section-title">{heading}</h3>
        {#if more}<button type="button" class="link-btn" onclick={more}
            >查看全部<ChevronRight size={15} /></button
          >{/if}
      </div>
      <div class="shelf">
        {#each tracks as t (t.id)}
          <article class="card" oncontextmenu={(e) => trackMenu(e, t)}>
            <div class="card-art">
              <Cover id={t.id} alt={t.album || t.title} size={320} />
              <button
                type="button"
                class="card-open"
                aria-label={'播放 ' + t.title}
                onclick={() => perform(() => playTrack(t.id))}
              ></button>
              <span class="card-play" aria-hidden="true"
                ><Play size={18} fill="currentColor" /></span
              >
            </div>
            <span class="card-title" use:tooltip={{ text: t.title, overflow: true }}>{t.title}</span
            >
            <p class="card-sub">{t.artist || '未知艺术家'}</p>
          </article>
        {/each}
      </div>
    </section>
  {/snippet}

  {#if loading}
    <div class="shelf" aria-busy="true">
      {#each Array(6) as _, i (i)}<div class="card">
          <div class="card-art skeleton"></div>
          <span class="skeleton skeleton-text"></span>
        </div>{/each}
    </div>
  {:else}
    {#if recent.length}{@render shelf('home-recent', '最近播放', recent)}{/if}
    {#if added.length}{@render shelf('home-added', '最近添加', added, () =>
        go({ view: 'recent' }),
      )}{/if}

    {#if artists.length}
      <section class="home-section" aria-labelledby="home-artists">
        <div class="section-head">
          <h3 id="home-artists" class="section-title">常听艺术家</h3>
          <button type="button" class="link-btn" onclick={() => go({ view: 'artists' })}
            >全部艺术家<ChevronRight size={15} /></button
          >
        </div>
        <div class="artist-row">
          {#each artists as [name, plays] (name)}
            <button
              type="button"
              class="artist-pill"
              onclick={() => go({ view: 'artist', name })}
              oncontextmenu={(e) => openMenu(e, artistMenu(artistGroup(name)), name + ' 操作')}
            >
              <span class="artist-avatar" aria-hidden="true">
                <span class="artist-initial small" style:--hue={nameHue(name)}>{initial(name)}</span
                >
                {#if artistGroup(name).cover_id > 0}
                  <Cover id={artistGroup(name).cover_id} alt={name} size={96} kind="artist" />
                {/if}
              </span>
              <span class="artist-pill-text"
                ><strong>{name || '未知艺术家'}</strong><small>{plays} 次播放</small></span
              >
            </button>
          {/each}
        </div>
      </section>
    {/if}

    <section class="home-section" aria-labelledby="home-playlists">
      <div class="section-head"><h3 id="home-playlists" class="section-title">我的歌单</h3></div>
      <div class="playlist-row">
        <div
          class="playlist-tile"
          role="group"
          aria-label={LOCAL_COLLECTION.name}
          oncontextmenu={(e) =>
            openMenu(e, localMenu(), '系统歌单「' + LOCAL_COLLECTION.name + '」操作')}
        >
          <button type="button" class="playlist-tile-main" onclick={() => go({ view: 'local' })}>
            <span class="playlist-tile-art"><Artwork kind="playlist" alt="" /></span>
            <span class="playlist-tile-text"
              ><strong>{LOCAL_COLLECTION.name}</strong><small
                >系统歌单 · {library.summary.tracks} 首歌曲</small
              ></span
            >
          </button>
          <button
            type="button"
            class="icon-btn icon-btn-sm playlist-tile-play"
            aria-label={'播放歌单 ' + LOCAL_COLLECTION.name}
            disabled={!library.summary.tracks}
            onclick={() => playCollection(localQuery('added', ''))}
            ><Play size={15} fill="currentColor" /></button
          >
        </div>
        {#each library.summary.playlists as p (p.id)}
          <div
            class="playlist-tile"
            role="group"
            aria-label={p.name}
            oncontextmenu={(e) => openMenu(e, playlistMenu(p), '歌单「' + p.name + '」操作')}
          >
            <button
              type="button"
              class="playlist-tile-main"
              onclick={() => go({ view: 'playlist', id: p.id })}
            >
              <span class="playlist-tile-art"><Artwork kind="playlist" alt="" /></span>
              <span class="playlist-tile-text"
                ><strong use:tooltip={{ text: p.name, overflow: true }}>{p.name}</strong><small
                  >{p.count} 首歌曲</small
                ></span
              >
            </button>
            <button
              type="button"
              class="icon-btn icon-btn-sm playlist-tile-play"
              aria-label={'播放歌单 ' + p.name}
              disabled={!p.count}
              onclick={() => playCollection({ playlist: p.id })}
              ><Play size={15} fill="currentColor" /></button
            >
          </div>
        {/each}
      </div>
      {#if !library.summary.playlists.length}<p class="hint">
          在侧边栏“我的歌单”旁点击 + 创建自己的歌单。
        </p>{/if}
    </section>
  {/if}
{/if}

<style>
  :global {
    /* ============================================================ home */
    .home-header {
      padding: var(--space-4) 0 var(--space-2);
    }

    .home-section {
      margin-top: var(--space-8);
    }

    .home-section:first-of-type {
      margin-top: var(--space-5);
    }

    .section-head {
      display: flex;
      align-items: baseline;
      justify-content: space-between;
      gap: 12px;
      margin-bottom: var(--space-4);
    }

    .section-title {
      font-family: var(--font-display);
      font-size: var(--fs-section);
      font-weight: var(--fw-semibold);
    }

    .home-section > .section-title {
      margin-bottom: var(--space-4);
    }

    .continue {
      display: flex;
      align-items: center;
      gap: var(--space-4);
      max-width: 720px;
      padding: var(--space-3);
      border-radius: var(--radius-lg);
      background: var(--surface);
    }

    .continue-art {
      position: relative;
      flex-shrink: 0;
      width: 72px;
      height: 72px;
      border-radius: var(--radius);
      overflow: hidden;
    }

    .continue-info {
      flex: 1;
      min-width: 0;
      display: flex;
      flex-direction: column;
      gap: 2px;
    }

    .continue-info strong {
      font-size: 15px;
      font-weight: var(--fw-semibold);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .continue-info > span {
      color: var(--text-2);
      font-size: var(--fs-meta);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .continue-progress {
      display: flex;
      align-items: center;
      gap: 10px;
      margin-top: 8px;
    }

    .continue-progress small {
      color: var(--text-3);
      font-size: var(--fs-small);
      font-variant-numeric: tabular-nums;
      white-space: nowrap;
    }

    .meter {
      flex: 1;
      height: 4px;
      border-radius: 2px;
      background: rgb(var(--ink) / 0.14);
      overflow: hidden;
    }

    .meter i {
      display: block;
      height: 100%;
      background: var(--accent);
      border-radius: inherit;
    }

    .artist-row,
    .playlist-row {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
      gap: var(--space-2);
    }

    .artist-pill {
      display: flex;
      align-items: center;
      gap: 12px;
      min-width: 0;
      padding: 8px 12px 8px 8px;
      border-radius: 999px;
      background: var(--surface);
      text-align: left;
      transition: background-color var(--dur-fast);
    }

    .artist-pill:hover {
      background: var(--surface-hover);
    }

    .artist-avatar {
      position: relative;
      width: 44px;
      height: 44px;
      flex-shrink: 0;
      border-radius: 50%;
      overflow: hidden;
    }
    .artist-avatar .cover-fill {
      position: absolute;
      inset: 0;
    }
    .artist-avatar .artwork {
      background: transparent;
    }
    .artist-avatar .artwork-placeholder {
      visibility: hidden;
    }

    .artist-pill-text {
      min-width: 0;
      display: flex;
      flex-direction: column;
    }

    .artist-pill-text strong,
    .playlist-tile-text strong {
      font-size: var(--fs-body);
      font-weight: var(--fw-semibold);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .artist-pill-text small,
    .playlist-tile-text small {
      color: var(--text-3);
      font-size: var(--fs-small);
    }

    .playlist-tile {
      position: relative;
      display: flex;
      align-items: center;
      min-width: 0;
      border-radius: var(--radius);
      background: var(--surface);
      transition: background-color var(--dur-fast);
    }

    .playlist-tile:hover {
      background: var(--surface-hover);
    }

    .playlist-tile-main {
      flex: 1;
      min-width: 0;
      display: flex;
      align-items: center;
      gap: 12px;
      padding: 8px;
      text-align: left;
      border-radius: inherit;
    }

    .playlist-tile-art {
      position: relative;
      flex-shrink: 0;
      width: 44px;
      height: 44px;
      border-radius: var(--radius-sm);
      overflow: hidden;
    }

    .playlist-tile-text {
      min-width: 0;
      display: flex;
      flex-direction: column;
    }

    .playlist-tile-play {
      margin-right: 8px;
      background: var(--accent);
      color: var(--accent-ink) !important;
      opacity: 0;
    }

    .playlist-tile:hover .playlist-tile-play,
    .playlist-tile:focus-within .playlist-tile-play {
      opacity: 1;
    }

    .playlist-tile-play:hover:not(:disabled) {
      background: var(--accent-hover) !important;
    }
  }
</style>
