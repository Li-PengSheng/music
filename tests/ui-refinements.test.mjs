import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { extractFunction } from './helpers/source-ast.mjs';

test('selector actual key handler navigates, ignores IME and restores focus on Escape/Tab', () => {
  const source = readFileSync('src/lib/components/Select.svelte', 'utf8');
  const start = source.search(/function key\s*\(/);
  const end = source.indexOf('onMount(', start);
  const handler = source
    .slice(start, end)
    .replace(/function key\s*\([^)]*\)/, 'function key(e)')
    .replace(/\s+as\s+[A-Za-z0-9_]+/g, '');
  const doc = { activeElement: null };
  let restored = 0,
    closed = 0;
  const items = ['顺序播放', '随机播放', '单曲循环', '列表循环'].map((label) => ({
    label,
    focus() {
      doc.activeElement = this;
    },
  }));
  const options = items.map((o) => ({ label: o.label }));
  const key = new Function('panel', 'document', 'options', 'close', handler + ';return key;')(
    { querySelectorAll: () => items },
    doc,
    options,
    (restore) => {
      closed++;
      if (restore) restored++;
    },
  );
  const event = (value, extra = {}) => ({
    key: value,
    isComposing: false,
    preventDefault() {
      this.prevented = true;
    },
    stopPropagation() {},
    ...extra,
  });
  items[0].focus();
  key(event('ArrowUp'));
  assert.equal(doc.activeElement, items[3]);
  key(event('Home'));
  assert.equal(doc.activeElement, items[0]);
  key(event('End'));
  assert.equal(doc.activeElement, items[3]);
  key(event('ArrowDown'));
  assert.equal(doc.activeElement, items[0]);
  key(event('ArrowDown', { isComposing: true }));
  assert.equal(doc.activeElement, items[0]);
  key(event('Escape'));
  key(event('Tab'));
  assert.equal(closed, 2);
  assert.equal(restored, 2);
});

test('context menu key handler: arrows wrap, typeahead, IME ignored, Escape/Tab close and restore focus', () => {
  const source = readFileSync('src/lib/components/ContextMenu.svelte', 'utf8');
  const handler = extractFunction(source, 'key', 'run');
  const doc = { activeElement: null };
  const closes = [];
  const buttons = ['播放', '下一首播放', '加入播放队列', '收藏'].map((text) => ({
    textContent: text,
    focus() {
      doc.activeElement = this;
    },
  }));
  const key = new Function('items', 'document', 'closeMenu', handler + ';return key;')(
    () => buttons,
    doc,
    (restore) => closes.push(restore),
  );
  const event = (value, extra = {}) => ({
    key: value,
    isComposing: false,
    ctrlKey: false,
    altKey: false,
    preventDefault() {
      this.prevented = true;
    },
    stopPropagation() {},
    ...extra,
  });
  buttons[0].focus();
  key(event('ArrowUp'));
  assert.equal(doc.activeElement, buttons[3]);
  key(event('ArrowDown'));
  assert.equal(doc.activeElement, buttons[0]);
  key(event('End'));
  assert.equal(doc.activeElement, buttons[3]);
  key(event('Home'));
  assert.equal(doc.activeElement, buttons[0]);
  key(event('加'));
  assert.equal(doc.activeElement, buttons[2]);
  key(event('ArrowDown', { isComposing: true }));
  assert.equal(doc.activeElement, buttons[2]);
  const esc = event('Escape');
  key(esc);
  assert.equal(esc.prevented, true);
  key(event('Tab'));
  assert.deepEqual(closes, [true, true]);
});

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
