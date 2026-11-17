import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import ts from 'typescript';

test('enrichment services carry camelCase search terms, explicit selections and propagate failures', async () => {
  const calls = [];
  const candidates = { tags: [], lyrics: [], errors: [] };
  const track = { id: 8, title: 'Updated' };
  const statuses = [
    {
      track_id: 8,
      enrichment_ignored: false,
      missing_artist: true,
      missing_album: false,
      missing_cover: null,
      missing_lyrics: true,
    },
  ];
  let fail = false;
  const code = ts.transpileModule(readFileSync('src/lib/services/music.ts', 'utf8'), {
    compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 },
  }).outputText;
  const dependencies = {
    '@tauri-apps/api/core': {
      isTauri: () => true,
      invoke: async (cmd, args) => {
        calls.push({ cmd, args });
        if (fail) throw 'METADATA_FILE_EXISTS';
        return cmd === 'metadata_set_enrichment_ignored'
          ? null
          : cmd === 'metadata_search_candidates'
            ? candidates
            : cmd === 'metadata_track_statuses'
              ? statuses
              : track;
      },
    },
    '@tauri-apps/api/window': {},
    '@tauri-apps/api/event': {},
    '../utils/cover-loader': { createCoverLoader: () => ({ stats: {} }) },
  };
  const api = {};
  new Function('require', 'exports', code)((name) => dependencies[name], api);
  assert.equal(await api.searchMetadataCandidates(8), candidates);
  assert.equal(await api.searchMetadataCandidates(8, 'Title', 'Artist'), candidates);
  assert.equal(await api.searchMetadataCandidates(8, 'Title', 'Artist', true), candidates);
  const payload = {
    track_id: 8,
    update_library_tags: true,
    write_audio_tags: false,
    save_cover: false,
    save_lyrics: false,
  };
  assert.equal(await api.applyMetadataEnrichment(payload), track);
  assert.equal(await api.metadataTrackStatuses([8]), statuses);
  assert.equal(await api.setEnrichmentIgnored(8, true), null);
  await api.setEnrichmentIgnored(8, false);
  assert.deepEqual(calls, [
    {
      cmd: 'metadata_search_candidates',
      args: { trackId: 8, titleQuery: null, artistQuery: null, forceRefresh: false },
    },
    {
      cmd: 'metadata_search_candidates',
      args: { trackId: 8, titleQuery: 'Title', artistQuery: 'Artist', forceRefresh: false },
    },
    {
      cmd: 'metadata_search_candidates',
      args: { trackId: 8, titleQuery: 'Title', artistQuery: 'Artist', forceRefresh: true },
    },
    { cmd: 'metadata_apply_enrichment', args: { payload } },
    { cmd: 'metadata_track_statuses', args: { trackIds: [8] } },
    { cmd: 'metadata_set_enrichment_ignored', args: { trackId: 8, ignored: true } },
    { cmd: 'metadata_set_enrichment_ignored', args: { trackId: 8, ignored: false } },
  ]);
  fail = true;
  await assert.rejects(
    api.applyMetadataEnrichment(payload),
    (error) => error === 'METADATA_FILE_EXISTS',
  );
});
