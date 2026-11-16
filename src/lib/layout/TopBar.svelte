<script lang="ts">
  import { ChevronLeft, ChevronRight, Search, X } from 'lucide-svelte';
  import { nav, go, back, forward } from '../stores/nav.svelte';
  import { library } from '../stores/library.svelte';
  import { player } from '../stores/player.svelte';
  import { ui } from '../stores/ui.svelte';
  import IconButton from '../components/IconButton.svelte';
  import { tooltip } from '../components/tooltip';
  import { LOCAL_COLLECTION } from '../utils/collections';

  let { searchInput = $bindable() }: { searchInput?: HTMLInputElement } = $props();

  let crumbs = $derived.by(() => {
    const r = nav.route;
    switch (r.view) {
      case 'home':
        return { parent: null, title: '音乐空间' };
      case 'recent':
        return { parent: null, title: '最近添加' };
      case 'songs':
        return { parent: '音乐库', title: '歌曲' };
      case 'albums':
        return { parent: '音乐库', title: '专辑' };
      case 'artists':
        return { parent: '音乐库', title: '艺术家' };
      case 'album':
        return {
          parent: '专辑',
          title: library.summary.albums.find((a) => a.key === r.key)?.name ?? '专辑',
        };
      case 'artist':
        return { parent: '艺术家', title: r.name || '未知艺术家' };
      case 'playlist':
        return {
          parent: '我的歌单',
          title: library.summary.playlists.find((p) => p.id === r.id)?.name ?? '歌单',
        };
      case 'local':
        return { parent: '我的歌单', title: LOCAL_COLLECTION.name };
      case 'settings':
        return { parent: null, title: '设置' };
    }
  });
  const parentRoute = () => {
    const v = nav.route.view;
    if (v === 'album') go({ view: 'albums' });
    else if (v === 'artist') go({ view: 'artists' });
    else go({ view: 'songs' });
  };

  function input() {
    const v = nav.route.view;
    if (ui.query.trim() && (v === 'home' || v === 'settings')) go({ view: 'songs' });
  }
  function key(e: KeyboardEvent) {
    if (e.isComposing) return;
    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      if (ui.query) ui.query = '';
      else searchInput?.blur();
    }
  }
</script>

<header class="topbar">
  <div class="history-buttons">
    <IconButton label="后退" disabled={!nav.canBack} onclick={back}
      ><ChevronLeft size={18} /></IconButton
    >
    <IconButton label="前进" disabled={!nav.canForward} onclick={forward}
      ><ChevronRight size={18} /></IconButton
    >
  </div>
  <div class="crumbs">
    {#if crumbs.parent}<button type="button" class="crumb-parent" onclick={parentRoute}
        >{crumbs.parent}</button
      ><span class="crumb-sep" aria-hidden="true">/</span>{/if}
    <h1 class="crumb-title" use:tooltip={{ text: crumbs.title, overflow: true }}>{crumbs.title}</h1>
  </div>
  <div class="search" role="search">
    <Search size={16} aria-hidden="true" />
    <input
      bind:this={searchInput}
      bind:value={ui.query}
      type="search"
      placeholder="搜索歌曲、艺术家或专辑"
      aria-label="搜索歌曲、艺术家或专辑"
      autocomplete="off"
      spellcheck="false"
      oninput={input}
      onkeydown={key}
    />
    {#if ui.query}<button
        type="button"
        class="search-clear"
        aria-label="清除搜索"
        onclick={() => {
          ui.query = '';
          searchInput?.focus();
        }}><X size={14} /></button
      >
    {:else}<kbd aria-hidden="true">Ctrl F</kbd>{/if}
  </div>
  <span class="local-status" class:offline={!player.connected} use:tooltip={player.connection}>
    <span class="status-dot" aria-hidden="true"></span><span class="local-label">本地模式</span>
  </span>
</header>
