<script lang="ts">
  import { onMount } from 'svelte';
  import { Wand2, RefreshCw, Search, SquareCheck, X } from 'lucide-svelte';
  import Select from '../components/Select.svelte';
  import MetadataEnrichment from '../components/MetadataEnrichment.svelte';
  import { library } from '../stores/library.svelte';
  import {
    enrichment as model,
    enrichmentWorkbench as work,
    synchronizeEnrichment,
  } from '../stores/enrichment.svelte';
  import { ui, confirmAction } from '../stores/ui.svelte';
  import { go } from '../stores/nav.svelte';
  import {
    filteredEntries,
    canApply,
    effectiveFilter,
    isNewTrack,
    type MissingFilter,
  } from '../utils/enrichment-workbench';

  let { trackId }: { trackId?: number } = $props();
  let active = $derived(model.entries.find((e) => e.track.id === model.activeId));
  let filter = $derived(effectiveFilter(model, model.filter));
  let rows = $derived(filteredEntries(model, filter, ui.query));
  let selectableRows = $derived(rows.filter((e) => !e.missing.enrichment_ignored));
  let selected = $derived(model.entries.filter((e) => e.checked));
  let applicable = $derived(selected.filter((e) => e.status === 'matched' && canApply(e)));
  let allSelected = $derived(selectableRows.length > 0 && selectableRows.every((e) => e.checked));
  const labels = {
    idle: '未检索',
    searching: '检索中',
    matched: '已匹配',
    empty: '无结果',
    applied: '已应用',
    error: '失败',
  };
  let disposed = false;
  onMount(() => {
    model.now = Math.floor(Date.now() / 1000);
    void synchronizeEnrichment(!model.loaded);
    return () => {
      disposed = true;
      if (model.busy) work.cancel();
    };
  });
  $effect(() => {
    const revision = library.revision;
    if (!model.busy && !model.loading && !model.syncing && model.syncedRevision !== revision)
      void synchronizeEnrichment();
  });
  let handledTarget: number | undefined;
  function focusTrack(id: number) {
    model.activeId = id;
    handledTarget = id;
    go({ view: 'enrichment', trackId: id });
  }
  $effect(() => {
    const target = trackId;
    if (
      !model.loaded ||
      !target ||
      target === handledTarget ||
      model.busy ||
      model.loading ||
      model.syncing
    )
      return;
    handledTarget = target;
    const entry = model.entries.find((e) => e.track.id === target);
    if (entry) {
      if (
        !filteredEntries(model, effectiveFilter(model, model.filter), '').some(
          (e) => e.track.id === target,
        )
      )
        model.filter = 'all';
      model.activeId = target;
      if (entry.status === 'idle') void work.searchCurrent(entry);
    }
  });
  async function batchApply() {
    const count = applicable.length;
    if (!count || model.busy) return;
    const audio = applicable.filter((e) => e.writeAudio).length;
    if (
      await confirmAction({
        title: '批量应用补全',
        message: `将按每首曲目预览中的选择应用 ${count} 首已匹配歌曲。`,
        detail: `写原始音频：${audio} 首。已有文件仅在逐曲明确勾选覆盖时替换；取消批处理只停止后续曲目。`,
        confirmLabel: `确认应用 ${count} 首`,
      })
    ) {
      if (!disposed) await work.batchApply();
    }
  }
</script>

<section class="enrichment-page" aria-label="标签补全工作台">
  <header class="workbench-heading">
    <div>
      <Wand2 size={20} />
      <h2>标签补全工作台</h2>
    </div>
    <button
      type="button"
      class="btn btn-ghost btn-sm"
      disabled={model.loading || !!model.busy}
      onclick={() => synchronizeEnrichment(true)}><RefreshCw size={14} />刷新曲目</button
    >
  </header>
  <p class="workbench-note">先检索与比较，再确认应用。批量默认不修改原始音频文件。</p>
  {#if model.error}<p role="alert">{model.error}</p>{/if}
  <div class="enrichment-workbench">
    <aside class="enrichment-queue" aria-label="批量补全">
      <Select
        label="待补全筛选"
        value={filter}
        onchange={(value) => (model.filter = value as MissingFilter)}
        disabled={model.loading || !!model.busy}
        options={[
          { value: 'recent', label: '新入库（近 7 天）' },
          { value: 'recent-info', label: '新入库且缺信息' },
          { value: 'incomplete', label: '信息不全（缺歌手/专辑/封面/歌词）' },
          { value: 'info', label: '信息不全（缺歌手/专辑）' },
          { value: 'cover', label: '缺封面' },
          { value: 'lyrics', label: '缺歌词' },
          { value: 'all', label: '全部曲目' },
        ]}
      />
      <label class="select-all"
        ><input
          type="checkbox"
          checked={allSelected}
          aria-label="全选当前筛选"
          disabled={model.loading || !!model.busy || !selectableRows.length}
          onchange={(event) => work.selectAll(rows, event.currentTarget.checked)}
        />全选当前筛选 <span>{rows.length} 首</span></label
      >
      <div class="batch-buttons">
        <button
          type="button"
          class="btn btn-secondary btn-sm"
          disabled={model.loading || !!model.busy || !selected.length}
          onclick={() => work.batchSearch()}><Search size={14} />批量检索所选</button
        ><button
          type="button"
          class="btn btn-primary btn-sm"
          disabled={!!model.busy || !applicable.length}
          onclick={batchApply}><SquareCheck size={14} />批量应用已匹配</button
        >
      </div>
      <p class="selection-note">已选 {selected.length} 首 · 可应用 {applicable.length} 首</p>
      {#if model.progress.total}<p role="status" class="batch-progress">
          {model.progress.label}
          {model.progress.done} / {model.progress.total}{model.cancelled ? ' · 已取消' : ''}
        </p>{/if}
      {#if model.busy === 'search' || model.busy === 'apply'}<button
          type="button"
          class="btn btn-ghost btn-sm"
          disabled={model.cancelled}
          onclick={() => work.cancel()}><X size={14} />取消批量操作</button
        >
        <p class="selection-note">取消后停止后续曲目，当前请求可能完成。</p>{/if}
      {#if model.loading}<p role="status">正在读取本地完整度…</p>{/if}
      <ul class="enrichment-rows" aria-label="补全队列">
        {#each rows as entry (entry.track.id)}<li
            class="enrichment-row"
            class:active={entry.track.id === model.activeId}
          >
            <input
              type="checkbox"
              aria-label={`选择：${entry.track.title}`}
              bind:checked={entry.checked}
              disabled={!!model.busy || entry.missing.enrichment_ignored}
            />
            <button
              type="button"
              aria-label={`查看：${entry.track.title}`}
              aria-pressed={entry.track.id === model.activeId}
              onclick={() => focusTrack(entry.track.id)}
              ><strong>{entry.track.title}</strong>
              {#if isNewTrack(entry.track, model.now)}<span class="new-badge">新歌</span>{/if}
              {#if entry.missing.enrichment_ignored}<span class="ignored-badge">已忽略</span>{/if}
              <span>{entry.track.artist || '未知艺术家'}</span><small class:error={!!entry.error}
                >{entry.error ? '失败 · ' : ''}{labels[entry.status]}</small
              ></button
            >
            <button
              type="button"
              class="ignore-action"
              aria-label={`${entry.missing.enrichment_ignored ? '恢复补全' : '忽略'}：${entry.track.title}`}
              title={entry.missing.enrichment_ignored
                ? '恢复为待补全曲目'
                : '标记为无需补全，在待办筛选中隐藏'}
              disabled={!!model.busy || model.loading}
              onclick={() => work.setIgnored(entry, !entry.missing.enrichment_ignored)}
              >{entry.missing.enrichment_ignored ? '恢复' : '忽略'}</button
            >
          </li>{/each}
      </ul>
      {#if model.loaded && !rows.length}<p class="selection-note">当前筛选没有曲目</p>{/if}
    </aside>
    <div class="enrichment-editor">
      {#if active}<MetadataEnrichment
          entry={active}
          disabled={!!model.busy}
          onsearch={() => work.searchCurrent(active!, true)}
          onapply={() => work.applyCurrent(active!)}
        />
      {:else}<div class="workbench-empty">
          <Wand2 size={32} />
          <p>从左侧选择一首歌曲，查看本地信息与匹配结果。</p>
        </div>{/if}
    </div>
  </div>
</section>

<style>
  .enrichment-page {
    min-width: 0;
    min-height: 0;
    height: 100%;
    display: flex;
    flex-direction: column;
  }
  .workbench-heading,
  .workbench-heading > div {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }
  .workbench-heading {
    justify-content: space-between;
  }
  h2 {
    font-size: var(--fs-section);
  }
  .workbench-note {
    color: var(--text-3);
    font-size: var(--fs-meta);
    margin: var(--space-2) 0 var(--space-4);
  }
  .enrichment-workbench {
    display: grid;
    grid-template-columns: minmax(230px, 0.7fr) minmax(0, 1.3fr);
    gap: var(--space-4);
    flex: 1;
    min-height: 0;
  }
  .enrichment-queue {
    display: flex;
    flex-direction: column;
    min-height: 0;
    min-width: 0;
    gap: var(--space-2);
  }
  .select-all {
    display: flex;
    gap: var(--space-2);
    align-items: center;
    font-size: var(--fs-meta);
    flex-wrap: wrap;
  }
  .select-all span {
    margin-left: auto;
    color: var(--text-3);
  }
  input[type='checkbox'] {
    accent-color: var(--accent);
  }
  .batch-buttons {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
  }
  .selection-note,
  .batch-progress {
    font-size: var(--fs-small);
    color: var(--text-3);
  }
  .enrichment-rows {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    padding: 0;
    margin: 0;
    list-style: none;
    border-top: 1px solid var(--line);
  }
  .enrichment-row {
    display: flex;
    gap: var(--space-2);
    align-items: center;
    padding: var(--space-2);
    border-bottom: 1px solid var(--line);
    border-radius: var(--radius-sm);
  }
  .enrichment-row.active {
    background: var(--accent-soft);
  }
  .enrichment-row > input {
    flex-shrink: 0;
  }
  .enrichment-row > button:not(.ignore-action) {
    display: grid;
    flex: 1;
    min-width: 0;
    text-align: left;
    gap: 2px;
  }
  .enrichment-row strong {
    font-size: var(--fs-meta);
    font-weight: var(--fw-medium);
    overflow-wrap: anywhere;
  }
  .enrichment-row span,
  .enrichment-row small {
    color: var(--text-3);
    font-size: var(--fs-small);
  }
  .enrichment-row .error {
    color: var(--danger);
  }
  .enrichment-row .new-badge {
    justify-self: start;
    padding: 0 var(--space-1);
    border-radius: var(--radius-sm);
    background: var(--accent-soft);
    color: var(--accent);
  }
  .enrichment-row .ignored-badge {
    color: var(--text-3);
  }
  .ignore-action {
    flex-shrink: 0;
    color: var(--text-3);
    font-size: var(--fs-small);
    padding: var(--space-1);
    opacity: 0;
  }
  .enrichment-row:hover .ignore-action,
  .enrichment-row:focus-within .ignore-action {
    opacity: 1;
  }
  .enrichment-editor {
    min-width: 0;
    min-height: 0;
    overflow-y: auto;
    border: 1px solid var(--line);
    border-radius: var(--radius);
    padding: var(--space-4);
    background: var(--surface);
  }
  .workbench-empty {
    display: grid;
    justify-items: center;
    align-content: center;
    gap: var(--space-3);
    height: 100%;
    color: var(--text-3);
  }
  @media (max-width: 1000px) {
    .enrichment-workbench {
      grid-template-columns: 220px minmax(0, 1fr);
      gap: var(--space-2);
    }
    .enrichment-editor {
      padding: var(--space-3);
    }
  }
</style>
