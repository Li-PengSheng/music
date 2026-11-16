<script lang="ts">
  import {
    X,
    ListMusic,
    GripVertical,
    Ellipsis,
    Play,
    ListStart,
    ListEnd,
    ListX,
    Trash2,
  } from 'lucide-svelte';
  import QueueCover from '../components/QueueCover.svelte';
  import IconButton from '../components/IconButton.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import { tooltip } from '../components/tooltip';
  import { reorderQueue, playQueued, queueAction, type Track } from '../services/music';
  import { player, control } from '../stores/player.svelte';
  import { openMenu, confirmAction, notify, type MenuItem } from '../stores/ui.svelte';
  import { duration } from '../utils/format';
  import { retainQueue } from '../utils/queue';

  let { open, onclose }: { open: boolean; onclose: () => void } = $props();
  let closeButton = $state<HTMLButtonElement>();
  let list = $state<HTMLDivElement>();
  $effect(() => {
    if (open) closeButton?.focus({ preventScroll: true });
  });

  // Every player poll delivers a fresh queue array. Keep the rendered one until the tracks really change,
  // so polling, opening and closing never re-diff every row.
  let rendered: readonly Track[] = [];
  let queue = $derived.by(() => {
    rendered = retainQueue(rendered, player.snapshot?.queue ?? []);
    return rendered;
  });
  let currentIndex = $derived(queue.findIndex((t) => t.path === player.snapshot?.track_id));
  let upcoming = $derived(currentIndex < 0 ? queue.length : queue.length - currentIndex - 1);

  // Queue items are identified by file path: Rust deduplicates the queue by path.
  let drag = $state<{
    path: string;
    from: number;
    to: number;
    offset: number;
    startY: number;
    height: number;
  } | null>(null);
  let announce = $state('');

  function rows() {
    return Array.from(list?.querySelectorAll<HTMLElement>('[data-queue-row]') ?? []);
  }
  function startDrag(e: PointerEvent, t: Track, i: number) {
    if (e.button !== 0 || player.busy) return;
    const row = (e.currentTarget as HTMLElement).closest<HTMLElement>('[data-queue-row]');
    if (!row) return;
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    drag = {
      path: t.path,
      from: i,
      to: i,
      offset: 0,
      startY: e.clientY,
      height: row.offsetHeight,
    };
    e.preventDefault();
  }
  function moveDrag(e: PointerEvent) {
    if (!drag || !list) return;
    const box = list.getBoundingClientRect();
    if (e.clientY < box.top + 32) list.scrollTop -= 10;
    else if (e.clientY > box.bottom - 32) list.scrollTop += 10;
    drag.offset = e.clientY - drag.startY;
    let to = 0;
    rows().forEach((row, index) => {
      if (index === drag!.from) return;
      const r = row.getBoundingClientRect();
      if (e.clientY > r.top + r.height / 2) to++;
    });
    drag.to = to;
  }
  async function endDrag() {
    const current = drag;
    drag = null;
    if (current && current.to !== current.from) {
      await control(() => reorderQueue(current.path, current.to));
      announce = `已移动到第 ${current.to + 1} 位`;
    }
  }
  async function moveBy(t: Track, i: number, delta: number) {
    const to = Math.max(0, Math.min(queue.length - 1, i + delta));
    if (to === i) return;
    if (await control(() => reorderQueue(t.path, to))) {
      announce = `${t.title} 已移动到第 ${to + 1} 位`;
      requestAnimationFrame(() => rows()[to]?.querySelector<HTMLElement>('.queue-handle')?.focus());
    }
  }
  function handleKey(e: KeyboardEvent, t: Track, i: number) {
    if (e.key === 'ArrowUp' || e.key === 'ArrowDown') {
      e.preventDefault();
      void moveBy(t, i, e.key === 'ArrowUp' ? -1 : 1);
    } else if (e.key === 'Home' || e.key === 'End') {
      e.preventDefault();
      void moveBy(t, i, e.key === 'Home' ? -i : queue.length);
    }
  }
  function shift(i: number) {
    if (!drag) return 0;
    if (i === drag.from) return drag.offset;
    if (drag.from < i && i <= drag.to) return -drag.height;
    if (drag.to <= i && i < drag.from) return drag.height;
    return 0;
  }

  function itemMenu(t: Track, i: number): MenuItem[] {
    const next = currentIndex < 0 ? 0 : i > currentIndex ? currentIndex + 1 : currentIndex;
    return [
      {
        label: '播放',
        icon: Play,
        disabled: i === currentIndex,
        action: () => control(() => playQueued(t.path)),
      },
      {
        label: '移到下一首播放',
        icon: ListStart,
        disabled: i === currentIndex || i === currentIndex + 1,
        action: () => control(() => reorderQueue(t.path, next)),
      },
      {
        label: '移到队列末尾',
        icon: ListEnd,
        disabled: i === queue.length - 1,
        action: () => control(() => reorderQueue(t.path, queue.length - 1)),
      },
      { kind: 'separator' },
      {
        label: '从队列移除',
        icon: ListX,
        danger: true,
        action: () => control(() => queueAction('remove', null, t.path)),
      },
    ];
  }

  async function clear() {
    const removing = currentIndex < 0 ? queue.length : queue.length - 1;
    const ok = await confirmAction({
      title: '清空播放队列？',
      message:
        currentIndex < 0
          ? `将从播放队列移除全部 ${removing} 首歌曲。`
          : `将从播放队列移除其他 ${removing} 首歌曲，正在播放的歌曲会保留并继续播放。`,
      detail: '只影响播放队列，不会修改歌单或任何音乐文件。',
      confirmLabel: '清空队列',
      danger: true,
    });
    if (ok && (await control(() => queueAction('clear')))) notify('播放队列已清空');
  }
</script>

<aside id="playback-queue" class="queue" aria-label="播放队列" inert={!open}>
  <header class="queue-header">
    <div class="queue-heading">
      <h2>播放队列</h2>
      <p>
        {queue.length ? `${queue.length} 首 · 待播 ${upcoming} 首` : '空'}
      </p>
    </div>
    <IconButton
      label="清空队列"
      disabled={queue.length <= (currentIndex < 0 ? 0 : 1) || player.busy}
      onclick={clear}><Trash2 size={17} /></IconButton
    >
    <IconButton label="关闭队列 (Esc)" bind:element={closeButton} onclick={onclose}
      ><X size={18} /></IconButton
    >
  </header>
  <div
    class="queue-list"
    bind:this={list}
    role="list"
    aria-label="队列歌曲"
    class:dragging={!!drag}
  >
    {#each queue as t, i (t.path)}
      <div
        class="queue-item"
        role="listitem"
        data-queue-row
        class:current={i === currentIndex}
        class:played={currentIndex >= 0 && i < currentIndex}
        class:lifted={drag?.from === i}
        style:transform={drag ? `translateY(${shift(i)}px)` : undefined}
        oncontextmenu={(e) => openMenu(e, itemMenu(t, i), t.title + ' 队列操作')}
      >
        <button
          type="button"
          class="queue-handle"
          aria-label={`调整顺序：${t.title}，按上下方向键移动`}
          use:tooltip={'拖动调整顺序'}
          onpointerdown={(e) => startDrag(e, t, i)}
          onpointermove={moveDrag}
          onpointerup={endDrag}
          onpointercancel={() => (drag = null)}
          onkeydown={(e) => handleKey(e, t, i)}><GripVertical size={16} /></button
        >
        <button
          type="button"
          class="queue-main"
          aria-current={i === currentIndex ? 'true' : undefined}
          aria-label={(i === currentIndex ? '正在播放：' : '播放 ') + t.title}
          onclick={() => {
            if (i !== currentIndex) void control(() => playQueued(t.path));
          }}
        >
          <span class="queue-cover"
            ><QueueCover track={t} current={i === currentIndex} />{#if i === currentIndex}<span
                class="eq"
                class:paused={player.snapshot?.playback_state !== 'playing'}
                aria-hidden="true"><i></i><i></i><i></i></span
              >{/if}</span
          >
          <span class="queue-text"
            ><strong>{t.title}</strong><small>{t.artist || '未知艺术家'}</small></span
          >
          <span class="queue-duration">{duration(t.duration_ms)}</span>
        </button>
        <IconButton
          label={'更多操作：' + t.title}
          size="sm"
          class="queue-more"
          onclick={(e) =>
            openMenu(e.currentTarget as HTMLElement, itemMenu(t, i), t.title + ' 队列操作')}
          ><Ellipsis size={16} /></IconButton
        >
      </div>
    {:else}
      <EmptyState
        icon={ListMusic}
        title="播放队列是空的"
        message="在音乐库中双击歌曲，或使用右键菜单“加入播放队列”。"
      />
    {/each}
  </div>
  <p class="sr-only" aria-live="polite">{announce}</p>
</aside>
