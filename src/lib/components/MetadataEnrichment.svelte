<script lang="ts">
  import Artwork from './Artwork.svelte';
  import Select from './Select.svelte';
  import { trackCover } from '../services/music';
  import { library } from '../stores/library.svelte';
  import { canApply, setCandidate, type EnrichmentEntry } from '../utils/enrichment-workbench';

  let {
    entry,
    disabled,
    onsearch,
    onapply,
  }: { entry: EnrichmentEntry; disabled: boolean; onsearch: () => void; onapply: () => void } =
    $props();
  let tag = $derived(entry.result?.tags[Number(entry.tagIndex)] ?? null);
  let lyric = $derived(entry.result?.lyrics[Number(entry.lyricIndex)] ?? null);
  let cover = $state<string | null>(null);
  let id = $derived(entry.track.id);
  $effect(() => {
    const trackId = id;
    library.coverEpoch;
    let cancelled = false;
    cover = null;
    void trackCover(trackId)
      .then((next) => {
        if (!cancelled) cover = next;
      })
      .catch(() => {});
    return () => {
      cancelled = true;
    };
  });
  let fields = $derived([
    ['歌曲名', entry.track.title, 'title'],
    ['艺术家', entry.track.artist, 'artist'],
    ['专辑', entry.track.album, 'album'],
    ['专辑艺术家', entry.track.album_artist, 'album_artist'],
    ['年份', entry.track.year, 'year'],
    ['音轨号', entry.track.track_number, 'track_number'],
    ['流派', entry.track.genre, 'genre'],
  ] as const);
  function edited() {
    entry.draftDirty = true;
    entry.error = '';
  }
</script>

<section class="metadata-enrichment" aria-label="当前曲目补全">
  <header>
    <p class="eyebrow">当前曲目</p>
    <h2>{entry.track.title}</h2>
    <p class="muted">{entry.track.artist} · {entry.track.album}</p>
  </header>
  <div class="enrichment-search">
    <label>歌曲名<input class="input" bind:value={entry.titleQuery} {disabled} /></label>
    <label
      >艺术家<input
        class="input"
        bind:value={entry.artistQuery}
        {disabled}
        placeholder="缺失时自动拆分标题"
      /></label
    >
    <button type="button" class="btn btn-secondary" {disabled} onclick={onsearch}
      >{entry.status === 'searching' ? '检索中…' : '重新检索'}</button
    >
  </div>
  {#if entry.error}<p role="alert" class="enrichment-error">{entry.error}</p>{/if}
  {#if entry.result}
    {#each entry.result.errors as warning}<p class="enrichment-hint">
        部分来源不可用：{warning}
      </p>{/each}
    {#if entry.result.tags.length}
      <Select
        label="标签与封面候选"
        bind:value={entry.tagIndex}
        {disabled}
        options={entry.result.tags.map((t, i) => ({
          value: String(i),
          label: `[${t.source ?? 'MusicBrainz'}] ${t.title} · ${t.artist} · ${t.album}`,
        }))}
        onchange={(value) => setCandidate(entry, value)}
      />
    {:else}<p class="enrichment-hint">没有找到标签候选</p>{/if}
  {:else if entry.status !== 'searching'}<p class="enrichment-hint">
      调整搜索词后检索，或勾选左侧曲目进行批量检索。仅发送曲名与艺术家到 MusicBrainz / iTunes /
      LRCLIB。
    </p>{/if}
  <table aria-label="标签字段对比">
    <thead><tr><th>字段</th><th>本地信息</th><th>匹配结果</th></tr></thead><tbody>
      {#each fields as [label, current, key]}{@const next =
          key === 'genre'
            ? (entry.draft.genre ?? entry.track.genre)
            : key === 'album_artist'
              ? entry.draft.album_artist || entry.draft.artist
              : entry.draft[key]}<tr
          ><th scope="row">{label}</th><td>{current || '—'}</td><td
            class:changed={current !== next}
          >
            {#if key === 'title' || key === 'artist' || key === 'album'}
              <input
                class="input target-input"
                aria-label={`目标${label}`}
                bind:value={entry.draft[key]}
                {disabled}
                oninput={edited}
              />
            {:else if key === 'year'}
              <input
                class="input target-input"
                aria-label="目标年份"
                type="number"
                min="0"
                max="9999"
                step="1"
                value={entry.draft.year ?? ''}
                {disabled}
                oninput={(event) => {
                  entry.draft.year =
                    event.currentTarget.value === '' ? null : Number(event.currentTarget.value);
                  edited();
                }}
              />
            {:else}{next || '—'}{/if}
          </td></tr
        >{/each}
    </tbody>
  </table>
  <p class="enrichment-hint">
    匹配结果中的歌曲名、艺术家、专辑和年份可直接编辑；没有候选也可手动填写后应用。
  </p>
  <div class="enrichment-previews">
    <figure>
      <div class="preview-cover"><Artwork src={cover} alt="本地封面" /></div>
      <figcaption>本地封面</figcaption>
    </figure>
    <figure>
      <div class="preview-cover"><Artwork src={tag?.cover_preview ?? null} alt="候选封面" /></div>
      <figcaption>候选封面</figcaption>
    </figure>
    <div class="lyrics-preview">
      {#if entry.result?.lyrics.length}
        <Select
          label="同步歌词候选"
          bind:value={entry.lyricIndex}
          {disabled}
          options={entry.result.lyrics.map((l, i) => ({
            value: String(i),
            label: `${l.title} · ${l.artist} · ${l.album}`,
          }))}
        />
        {#if lyric}<pre aria-label="LRC 预览">{lyric.preview}</pre>{/if}
      {:else}<p class="enrichment-hint">
          {entry.result ? '没有找到同步歌词候选' : '同步歌词预览'}
        </p>{/if}
    </div>
  </div>
  <fieldset {disabled}>
    <legend>落盘选项</legend>
    <div class="enrichment-options">
      <label><input type="checkbox" bind:checked={entry.updateTags} {disabled} />更新曲库标签</label
      >
      <label
        ><input
          type="checkbox"
          bind:checked={entry.saveLyrics}
          disabled={!lyric || disabled}
        />保存同步歌词 (.lrc)</label
      >
      <label
        ><input
          type="checkbox"
          bind:checked={entry.saveCover}
          disabled={!(tag?.release_mbid || tag?.cover_url) || disabled}
        />保存专属封面</label
      >
      <label
        ><input type="checkbox" bind:checked={entry.writeAudio} />同时写入原始音频文件标签</label
      >
      <label
        ><input
          type="checkbox"
          bind:checked={entry.overwriteLyrics}
          disabled={!entry.saveLyrics || disabled}
        />允许覆盖已有歌词</label
      >
      <label
        ><input
          type="checkbox"
          bind:checked={entry.overwriteCover}
          disabled={!entry.saveCover || disabled}
        />允许覆盖已有封面</label
      >
    </div>
    <p class="enrichment-hint">
      默认仅更新曲库与同目录 .lrc /
      歌曲名.cover.jpg，不修改原始音频文件。已有文件默认保留；有本地封面或歌词时默认跳过该项。
    </p>
  </fieldset>
  <footer>
    <button
      type="button"
      class="btn btn-primary"
      disabled={disabled || !canApply(entry)}
      onclick={onapply}>应用当前补全</button
    >{#if entry.status === 'applied'}<p role="status">已应用当前补全</p>{/if}
  </footer>
</section>

<style>
  .metadata-enrichment {
    display: grid;
    gap: var(--space-3);
    min-width: 0;
  }
  h2 {
    font-size: var(--fs-section);
    overflow-wrap: anywhere;
  }
  .eyebrow,
  legend {
    color: var(--text-3);
    font-size: var(--fs-small);
  }
  .muted,
  .enrichment-hint,
  figcaption {
    color: var(--text-3);
    font-size: var(--fs-meta);
    overflow-wrap: anywhere;
  }
  .enrichment-search {
    display: flex;
    align-items: end;
    flex-wrap: wrap;
    gap: var(--space-2);
  }
  .enrichment-search label {
    display: grid;
    gap: var(--space-1);
    flex: 1;
    min-width: 120px;
    font-size: var(--fs-meta);
  }
  .enrichment-search input {
    width: 100%;
  }
  table {
    width: 100%;
    table-layout: fixed;
    border-collapse: collapse;
    font-size: var(--fs-meta);
  }
  th,
  td {
    padding: 6px;
    text-align: left;
    overflow-wrap: anywhere;
    border-bottom: 1px solid var(--line);
    vertical-align: top;
  }
  th {
    color: var(--text-3);
    font-weight: var(--fw-medium);
  }
  th:first-child {
    width: 22%;
  }
  td.changed {
    background: var(--accent-soft);
    color: var(--text);
  }
  .target-input {
    width: 100%;
    min-width: 0;
    height: 30px;
    padding: 4px 6px;
    font-size: inherit;
  }
  .enrichment-previews {
    display: grid;
    grid-template-columns: 80px 80px minmax(0, 1fr);
    gap: var(--space-2);
    align-items: start;
  }
  .preview-cover {
    width: 80px;
    height: 80px;
    overflow: hidden;
    border-radius: var(--radius);
  }
  figcaption {
    margin-top: var(--space-1);
    text-align: center;
  }
  .lyrics-preview {
    min-width: 0;
  }
  pre {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
    font-size: var(--fs-small);
    max-height: 130px;
    overflow-y: auto;
    margin-top: var(--space-2);
  }
  fieldset {
    border: 0;
    border-top: 1px solid var(--line);
    min-width: 0;
    padding-top: var(--space-2);
  }
  .enrichment-options {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: var(--space-2);
    margin: var(--space-2) 0;
    font-size: var(--fs-small);
  }
  .enrichment-options label {
    display: flex;
    align-items: center;
    gap: var(--space-1);
  }
  input[type='checkbox'] {
    accent-color: var(--accent);
    flex-shrink: 0;
  }
  .enrichment-error {
    color: var(--danger);
    overflow-wrap: anywhere;
  }
  footer {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-3);
    align-items: center;
    font-size: var(--fs-meta);
  }
  @media (max-width: 1000px) {
    .enrichment-previews {
      grid-template-columns: 64px 64px minmax(0, 1fr);
    }
    .preview-cover {
      width: 64px;
      height: 64px;
    }
    .enrichment-options {
      grid-template-columns: 1fr;
    }
  }
</style>
