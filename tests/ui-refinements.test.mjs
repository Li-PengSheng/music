import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

test('playlist deletion is confirmed, addresses the playlist by ID and states files are kept', () => {
  const actions = readFileSync('src/lib/actions.ts', 'utf8');
  const body = actions.slice(actions.indexOf('export async function deletePlaylistDialog'));
  assert.match(body, /confirmAction\(/);
  assert.match(body, /只删除歌单，不删除本地音乐文件/);
  assert.match(body, /await deletePlaylist\(playlist\.id\)/);
  assert.ok(
    body.indexOf('confirmAction(') < body.indexOf('await deletePlaylist('),
    'confirmation happens before deletion',
  );
  assert.match(
    body,
    /catch\s*\(\s*e\s*\)\s*\{\s*notifyError\s*\(\s*e\s*\)\s*;?\s*\}/,
    'failures surface the real error',
  );
  const service = readFileSync('src/lib/services/music.ts', 'utf8');
  assert.match(
    service,
    /deletePlaylist\s*=\s*\(\s*playlistId\s*:\s*number\s*\)\s*=>\s*invoke\s*<\s*void\s*>\s*\(\s*'library_playlist_edit'\s*,\s*\{\s*action\s*:\s*'delete'\s*,\s*playlistId\b/,
  );
});
