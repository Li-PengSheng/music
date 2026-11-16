<script lang="ts">
  import {
    House,
    Clock3,
    Music2,
    Disc3,
    MicVocal,
    ListMusic,
    Plus,
    Settings2,
    HardDrive,
  } from 'lucide-svelte';
  import { nav, go } from '../stores/nav.svelte';
  import { library } from '../stores/library.svelte';
  import { openMenu } from '../stores/ui.svelte';
  import { playlistMenu, localMenu, newPlaylist } from '../actions';
  import { tooltip } from '../components/tooltip';
  import { LOCAL_COLLECTION, shadowsBuiltIn } from '../utils/collections';

  let active = $derived(nav.route.view);
  let libraryActive = $derived(
    active === 'songs' ||
      active === 'albums' ||
      active === 'artists' ||
      active === 'album' ||
      active === 'artist',
  );
  const top = [
    { view: 'home', label: '音乐空间', icon: House },
    { view: 'recent', label: '最近添加', icon: Clock3 },
  ] as const;
  let sections = $derived([
    { view: 'songs', label: '歌曲', icon: Music2, count: library.summary.tracks, match: ['songs'] },
    {
      view: 'albums',
      label: '专辑',
      icon: Disc3,
      count: library.summary.albums.length,
      match: ['albums', 'album'],
    },
    {
      view: 'artists',
      label: '艺术家',
      icon: MicVocal,
      count: library.summary.artists.length,
      match: ['artists', 'artist'],
    },
  ] as const);
</script>

<aside class="sidebar" aria-label="侧边栏">
  <button
    type="button"
    class="brand"
    onclick={() => go({ view: 'home' })}
    aria-label="Music Mine 音乐空间"
  >
    <span class="brand-mark" aria-hidden="true"><Music2 size={18} strokeWidth={2.4} /></span>
    <span class="brand-name">Music Mine</span>
  </button>

  <nav class="nav" aria-label="主导航">
    <ul class="nav-group">
      {#each top as item (item.view)}
        <li>
          <button
            type="button"
            class="nav-item"
            class:active={active === item.view}
            aria-current={active === item.view ? 'page' : undefined}
            onclick={() => go({ view: item.view })}
          >
            <item.icon size={18} /><span class="nav-label">{item.label}</span>
          </button>
        </li>
      {/each}
    </ul>

    <div class="nav-heading" class:active-group={libraryActive}>音乐库</div>
    <ul class="nav-group">
      {#each sections as item (item.view)}
        {@const selected = (item.match as readonly string[]).includes(active)}
        <li>
          <button
            type="button"
            class="nav-item"
            class:active={selected}
            aria-current={selected ? 'page' : undefined}
            onclick={() => go({ view: item.view })}
          >
            <item.icon size={18} /><span class="nav-label">{item.label}</span
            >{#if library.loaded}<span class="nav-count">{item.count}</span>{/if}
          </button>
        </li>
      {/each}
    </ul>

    <div class="nav-heading nav-heading-row">
      <span>我的歌单</span>
      <button
        type="button"
        class="icon-btn icon-btn-sm"
        aria-label="新建歌单"
        use:tooltip={'新建歌单'}
        onclick={() => newPlaylist()}><Plus size={16} /></button
      >
    </div>
    <ul class="nav-group nav-playlists" aria-label="我的歌单">
      <li>
        <button
          type="button"
          class="nav-item"
          class:active={active === 'local'}
          aria-current={active === 'local' ? 'page' : undefined}
          onclick={() => go({ view: 'local' })}
          oncontextmenu={(e) =>
            openMenu(e, localMenu(), '系统歌单「' + LOCAL_COLLECTION.name + '」操作')}
          use:tooltip={'系统歌单：自动包含音乐库中的全部歌曲'}
        >
          <HardDrive size={18} /><span class="nav-label">{LOCAL_COLLECTION.name}</span
          >{#if library.loaded}<span class="nav-count">{library.summary.tracks}</span>{/if}
        </button>
      </li>
      {#each library.summary.playlists as list (list.id)}
        {@const selected = nav.route.view === 'playlist' && nav.route.id === list.id}
        <li>
          <button
            type="button"
            class="nav-item"
            class:active={selected}
            aria-current={selected ? 'page' : undefined}
            onclick={() => go({ view: 'playlist', id: list.id })}
            oncontextmenu={(e) => openMenu(e, playlistMenu(list), '歌单「' + list.name + '」操作')}
          >
            <ListMusic size={18} /><span
              class="nav-label"
              use:tooltip={{ text: list.name, overflow: true }}>{list.name}</span
            >
            {#if shadowsBuiltIn(list.name)}<span
                class="nav-tag"
                use:tooltip={'你创建的同名普通歌单，内容与系统歌单无关'}>自建</span
              >{/if}
            <span class="nav-count">{list.count}</span>
          </button>
        </li>
      {/each}
      {#if library.loaded && !library.summary.playlists.length}
        <li>
          <button type="button" class="nav-item nav-item-muted" onclick={() => newPlaylist()}
            ><Plus size={18} /><span class="nav-label">新建歌单</span></button
          >
        </li>
      {/if}
    </ul>
  </nav>

  <div class="sidebar-footer">
    <button
      type="button"
      class="nav-item"
      class:active={active === 'settings'}
      aria-current={active === 'settings' ? 'page' : undefined}
      onclick={() => go({ view: 'settings' })}
    >
      <Settings2 size={18} /><span class="nav-label">设置</span>
    </button>
  </div>
</aside>

<style>
  :global {
    /* ============================================================ sidebar */
    .sidebar {
      grid-column: 1;
      grid-row: 1;
      min-height: 0;
      display: flex;
      flex-direction: column;
      background: var(--bg-sidebar);
      padding: var(--space-3) var(--space-2) var(--space-2);
      overflow: hidden;
    }

    .brand {
      display: flex;
      align-items: center;
      gap: 10px;
      height: 40px;
      padding: 0 10px;
      margin-bottom: var(--space-3);
      border-radius: var(--radius);
      font-family: var(--font-display);
      font-size: 17px;
      font-weight: 700;
      letter-spacing: -0.2px;
      flex-shrink: 0;
    }

    .brand-mark {
      display: grid;
      place-items: center;
      width: 28px;
      height: 28px;
      border-radius: var(--radius);
      background: var(--accent);
      color: var(--accent-ink);
    }

    .nav {
      flex: 1;
      min-height: 0;
      display: flex;
      flex-direction: column;
    }

    .nav-group {
      display: flex;
      flex-direction: column;
      gap: 2px;
      flex-shrink: 0;
    }

    .nav-heading {
      display: flex;
      align-items: center;
      justify-content: space-between;
      min-height: 30px;
      margin-top: var(--space-4);
      padding: 0 6px 0 12px;
      color: var(--text-3);
      font-size: var(--fs-small);
      font-weight: var(--fw-semibold);
      flex-shrink: 0;
    }

    .nav-heading.active-group {
      color: var(--text-2);
    }

    .nav-item {
      position: relative;
      display: flex;
      align-items: center;
      gap: 12px;
      width: 100%;
      height: 36px;
      padding: 0 10px 0 12px;
      border-radius: var(--radius);
      color: var(--text-2);
      font-size: var(--fs-body);
      text-align: left;
      transition:
        background-color var(--dur-fast) var(--ease),
        color var(--dur-fast) var(--ease);
    }

    .nav-item > svg {
      flex-shrink: 0;
    }

    .nav-item:hover {
      background: var(--hover);
      color: var(--text);
    }

    .nav-item.active {
      background: var(--selected);
      color: var(--text);
      font-weight: var(--fw-semibold);
    }

    .nav-item.active::before {
      content: '';
      position: absolute;
      left: 0;
      top: 10px;
      bottom: 10px;
      width: 3px;
      border-radius: 0 3px 3px 0;
      background: var(--accent);
    }

    .nav-item.active > svg {
      color: var(--accent);
    }

    .nav-item-muted {
      color: var(--text-3);
    }

    .nav-label {
      flex: 1;
      min-width: 0;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .nav-tag {
      flex-shrink: 0;
      padding: 0 6px;
      border-radius: 999px;
      background: var(--surface-hover);
      color: var(--text-3);
      font-size: 11px;
      line-height: 18px;
    }

    .nav-count {
      flex-shrink: 0;
      color: var(--text-3);
      font-size: var(--fs-small);
      font-weight: 400;
      font-variant-numeric: tabular-nums;
    }

    /* Only the playlist list scrolls; brand, library groups and settings stay put. */
    .nav-playlists {
      flex: 1 1 auto;
      min-height: 72px;
      overflow-y: auto;
      overflow-x: hidden;
      margin-right: -4px;
      padding-right: 4px;
      padding-bottom: var(--space-2);
      flex-shrink: 1;
      mask-image: linear-gradient(to bottom, #000 calc(100% - 16px), transparent);
    }

    .sidebar-footer {
      flex-shrink: 0;
      padding-top: var(--space-2);
      border-top: 1px solid var(--line);
    }
  }
</style>
