<script lang="ts">
  import { getContext, onMount, tick, untrack } from 'svelte';
  import { Ellipsis, Play, Clock3 } from 'lucide-svelte';
  import Cover from './Cover.svelte';
  import { tooltip } from './tooltip';
  import { queryLibrary, playTrack, type LibraryQuery, type LibraryTrack } from '../services/music';
  import { player, perform } from '../stores/player.svelte';
  import { library } from '../stores/library.svelte';
  import { openMenu, notifyError } from '../stores/ui.svelte';
  import { memory } from '../stores/nav.svelte';
  import { songMenu, openDetails } from '../actions';
  import { visibleRange, pagesFor, scrollToRow } from '../utils/virtual-range';
  import { duration } from '../utils/format';

  let {
    query,
    memoryKey,
    playlistId = null,
    numbering = 'index',
    sortable = true,
    onsort,
    ontotal,
  }: {
    query: LibraryQuery;
    memoryKey: string;
    playlistId?: number | null;
    numbering?: 'index' | 'track';
    sortable?: boolean;
    onsort?: (sort: string) => void;
    ontotal?: (total: number, loaded: boolean) => void;
  } = $props();

  const ROW = 56,
    PAGE = 100;
  const scroller = getContext<() => HTMLElement | undefined>('scroller');

  let total = $state(0);
  let loaded = $state(false);
  let failed = $state(false);
  let pages = $state<Record<number, LibraryTrack[]>>({});
  let range = $state({ start: 0, end: 0 });
  let selected = $state(-1);
  let host = $state<HTMLDivElement>();
  let generation = 0;
  const inflight = new Map<number, number>();

  let queryKey = $derived(JSON.stringify(query));

  async function fetchPage(page: number, gen: number) {
    if (inflight.get(page) === gen) return;
    inflight.set(page, gen);
    try {
      const result = await queryLibrary({ ...query, offset: page * PAGE, limit: PAGE });
      if (gen !== generation) return;
      pages[page] = result.tracks;
      total = result.total;
      if (!loaded) {
        loaded = true;
        ontotal?.(total, true);
        void tick().then(restore);
      } else ontotal?.(total, true);
    } catch (e) {
      if (gen === generation) {
        failed = true;
        notifyError(e);
      }
    } finally {
      if (inflight.get(page) === gen) inflight.delete(page);
    }
  }

  function measure() {
    const el = scroller(),
      list = host;
    if (!el || !list) return;
    const listTop =
      list.getBoundingClientRect().top - el.getBoundingClientRect().top + el.scrollTop;
    const next = visibleRange(el.scrollTop, el.clientHeight, listTop, ROW, total);
    if (next.start !== range.start || next.end !== range.end) range = next;
    const gen = generation;
    for (const p of pagesFor(next.start, Math.max(next.end, 1), PAGE))
      if (!pages[p]) void fetchPage(p, gen);
  }

  // A new query starts over; a library revision refreshes loaded pages in place without flashing.
  $effect(() => {
    queryKey;
    untrack(() => {
      generation++;
      pages = {};
      loaded = false;
      failed = false;
      total = 0;
      selected = -1;
      range = { start: 0, end: 0 };
      ontotal?.(0, false);
      void fetchPage(0, generation);
    });
  });
  let lastRevision = library.revision;
  $effect(() => {
    const rev = library.revision;
    if (rev === lastRevision) return;
    lastRevision = rev;
    untrack(() => {
      generation++;
      const gen = generation;
      const keep = Object.keys(pages).map(Number);
      if (!keep.length) keep.push(0);
      for (const p of keep) void fetchPage(p, gen);
    });
  });
  $effect(() => {
    total;
    untrack(measure);
  });

  onMount(() => {
    const el = scroller();
    if (!el) return;
    let frame = 0;
    const onScroll = () => {
      if (!frame)
        frame = requestAnimationFrame(() => {
          frame = 0;
          measure();
        });
    };
    el.addEventListener('scroll', onScroll, { passive: true });
    const ro = new ResizeObserver(onScroll);
    ro.observe(el);
    return () => {
      el.removeEventListener('scroll', onScroll);
      ro.disconnect();
      cancelAnimationFrame(frame);
      memory.set(memoryKey, { ...memory.get(memoryKey), scroll: el.scrollTop });
    };
  });
  function restore() {
    const el = scroller();
    const saved = memory.get(memoryKey)?.scroll;
    if (el && saved) el.scrollTop = saved;
    measure();
  }

  let rows = $derived.by(() => {
    const out: { index: number; track: LibraryTrack | undefined }[] = [];
    for (let i = range.start; i < range.end; i++)
      out.push({ index: i, track: pages[Math.floor(i / PAGE)]?.[i % PAGE] });
    return out;
  });

  let currentPath = $derived(player.snapshot?.track_id ?? null);
  let playing = $derived(player.snapshot?.playback_state === 'playing');

  async function start(t: LibraryTrack) {
    await perform(() => playTrack(t.id));
  }
  function menu(e: MouseEvent | HTMLElement, t: LibraryTrack, i: number) {
    selected = i;
    openMenu(e, songMenu(t, { playlistId, onDetails: () => openDetails(t) }), t.title + ' 操作');
  }

  async function focusRow(i: number) {
    const el = scroller(),
      list = host;
    if (!el || !list || i < 0 || i >= total) return;
    selected = i;
    const listTop =
      list.getBoundingClientRect().top - el.getBoundingClientRect().top + el.scrollTop;
    const target = scrollToRow(i, el.scrollTop, el.clientHeight, listTop, ROW, ROW);
    if (target != null) {
      el.scrollTop = target;
      measure();
    }
    await tick();
    host?.querySelector<HTMLElement>(`[data-index="${i}"]`)?.focus({ preventScroll: true });
  }
  function key(e: KeyboardEvent, i: number, t: LibraryTrack | undefined) {
    if (e.isComposing || e.target !== e.currentTarget) return;
    const page = Math.max(1, Math.floor((scroller()?.clientHeight ?? ROW * 8) / ROW) - 1);
    let next = -1;
    if (e.key === 'ArrowDown') next = Math.min(total - 1, i + 1);
    else if (e.key === 'ArrowUp') next = Math.max(0, i - 1);
    else if (e.key === 'PageDown') next = Math.min(total - 1, i + page);
    else if (e.key === 'PageUp') next = Math.max(0, i - page);
    else if (e.key === 'Home' && e.ctrlKey) next = 0;
    else if (e.key === 'End' && e.ctrlKey) next = total - 1;
    else if (e.key === 'Enter' && t) {
      e.preventDefault();
      void start(t);
      return;
    }
    if (next >= 0) {
      e.preventDefault();
      void focusRow(next);
    }
  }

  let sort = $derived(query.sort ?? 'added');
  const columns = [
    { key: 'title', label: '标题' },
    { key: 'artist', label: '艺术家' },
    { key: 'album', label: '专辑' },
  ];
</script>

<div
  class="track-table"
  role="grid"
  aria-label="歌曲列表"
  aria-rowcount={total + 1}
  aria-busy={!loaded}
>
  <div class="track-head" role="row" aria-rowindex={1}>
    <span role="columnheader" class="col-index">#</span>
    {#each columns as c (c.key)}
      <span
        role="columnheader"
        class={'col-' + c.key}
        aria-sort={sortable && sort === c.key ? 'ascending' : undefined}
      >
        {#if sortable && onsort}<button
            type="button"
            class:sorted={sort === c.key}
            onclick={() => onsort(c.key)}>{c.label}</button
          >{:else}{c.label}{/if}
      </span>
    {/each}
    <span
      role="columnheader"
      class="col-duration"
      aria-label="时长"
      aria-sort={sortable && sort === 'duration' ? 'ascending' : undefined}
    >
      {#if sortable && onsort}<button
          type="button"
          class:sorted={sort === 'duration'}
          aria-label="按时长排序"
          onclick={() => onsort('duration')}><Clock3 size={15} /></button
        >{:else}<Clock3 size={15} />{/if}
    </span>
    <span role="columnheader" class="col-actions"><span class="sr-only">操作</span></span>
  </div>

  <div class="track-body" bind:this={host} style:height={total * ROW + 'px'}>
    <div class="track-window" style:transform={`translateY(${range.start * ROW}px)`}>
      {#each rows as { index, track: t } (t ? t.id : 'loading-' + index)}
        {@const current = !!t && t.path === currentPath}
        <div
          class="track-row"
          role="row"
          aria-rowindex={index + 2}
          aria-selected={selected === index}
          data-index={index}
          tabindex={selected === index || (selected < 0 && index === range.start) ? 0 : -1}
          class:selected={selected === index}
          class:current
          class:loading={!t}
          onclick={() => (selected = index)}
          ondblclick={() => {
            if (t) void start(t);
          }}
          onkeydown={(e) => key(e, index, t)}
          oncontextmenu={(e) => {
            if (t) menu(e, t, index);
            else e.preventDefault();
          }}
        >
          {#if t}
            <span role="gridcell" class="col-index">
              {#if current}<span
                  class="eq"
                  class:paused={!playing}
                  aria-label={playing ? '正在播放' : '已暂停'}><i></i><i></i><i></i></span
                >
              {:else}<span class="row-number"
                  >{numbering === 'track' && t.track_number ? t.track_number : index + 1}</span
                >{/if}
              <button
                type="button"
                class="row-play"
                tabindex="-1"
                aria-label={'播放 ' + t.title}
                onclick={(e) => {
                  e.stopPropagation();
                  void start(t);
                }}><Play size={15} fill="currentColor" /></button
              >
            </span>
            <span role="gridcell" class="col-title">
              <span class="row-cover"><Cover id={t.id} alt={t.album || t.title} size={96} /></span>
              <span class="row-title" use:tooltip={{ text: t.title, overflow: true }}
                >{t.title}</span
              >
            </span>
            <span role="gridcell" class="col-artist"
              ><span class="row-text" use:tooltip={{ text: t.artist, overflow: true }}
                >{t.artist || '未知艺术家'}</span
              ></span
            >
            <span role="gridcell" class="col-album"
              ><span class="row-text" use:tooltip={{ text: t.album, overflow: true }}
                >{t.album || '—'}</span
              ></span
            >
            <span role="gridcell" class="col-duration">{duration(t.duration_ms)}</span>
            <span role="gridcell" class="col-actions">
              <button
                type="button"
                class="row-action"
                tabindex="-1"
                aria-haspopup="menu"
                aria-label={'更多操作：' + t.title}
                use:tooltip={'更多操作'}
                onclick={(e) => {
                  e.stopPropagation();
                  menu(e.currentTarget as HTMLElement, t, index);
                }}><Ellipsis size={16} /></button
              >
            </span>
          {:else}
            <span role="gridcell" class="col-index"></span>
            <span role="gridcell" class="col-title"
              ><span class="row-cover skeleton"></span><span class="skeleton skeleton-text"
              ></span></span
            >
            <span role="gridcell" class="col-artist"
              ><span class="skeleton skeleton-text short"></span></span
            >
            <span role="gridcell" class="col-album"
              ><span class="skeleton skeleton-text short"></span></span
            >
            <span role="gridcell" class="col-duration"></span><span
              role="gridcell"
              class="col-actions"
            ></span>
          {/if}
        </div>
      {/each}
    </div>
  </div>
  {#if failed}<p class="list-error" role="alert">
      读取歌曲失败。<button
        type="button"
        class="btn btn-ghost btn-sm"
        onclick={() => {
          failed = false;
          generation++;
          pages = {};
          void fetchPage(0, generation);
        }}>重试</button
      >
    </p>{/if}
</div>
