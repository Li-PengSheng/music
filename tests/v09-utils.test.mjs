import { test } from 'node:test';
import assert from 'node:assert/strict';
import { allowsNativeMenu, isBrowserAccelerator } from '../src/lib/utils/native-menu.ts';
import { placeAtPoint, placeAtAnchor } from '../src/lib/utils/menu-position.ts';
import { playlistNameError, suggestPlaylistName } from '../src/lib/utils/playlist-name.ts';
import { createHistory } from '../src/lib/utils/history.ts';
import { visibleRange, pagesFor, scrollToRow } from '../src/lib/utils/virtual-range.ts';
import { ambientColor } from '../src/lib/utils/color.ts';
import { duration, initial, nameHue } from '../src/lib/utils/format.ts';

const el = (editable) => ({
  closest: (selector) => (editable && selector.includes('textarea') ? {} : null),
});

test('native WebView menu only for text fields, or Shift+right click in development', () => {
  assert.equal(allowsNativeMenu(el(false), false, false), false);
  assert.equal(allowsNativeMenu(el(true), false, false), true);
  assert.equal(allowsNativeMenu(el(false), true, true), true, 'dev keeps Inspect via Shift');
  assert.equal(allowsNativeMenu(el(false), false, true), false, 'release ignores Shift');
  assert.equal(allowsNativeMenu(null, false, false), false);
  const k = (key, mods = {}) => ({
    key,
    ctrlKey: false,
    shiftKey: false,
    altKey: false,
    metaKey: false,
    ...mods,
  });
  assert.equal(isBrowserAccelerator(k('F5')), true);
  assert.equal(isBrowserAccelerator(k('r', { ctrlKey: true })), true);
  assert.equal(isBrowserAccelerator(k('P', { ctrlKey: true, shiftKey: true })), true);
  assert.equal(
    isBrowserAccelerator(k('f', { ctrlKey: true })),
    false,
    'Ctrl+F stays the app search shortcut',
  );
  assert.equal(isBrowserAccelerator(k('l', { ctrlKey: true })), false);
  assert.equal(isBrowserAccelerator(k(' ')), false);
});

test('menus stay inside the window and flip near edges', () => {
  assert.deepEqual(placeAtPoint(100, 100, 200, 180, 1280, 800), {
    left: 100,
    top: 100,
    maxHeight: 784,
  });
  const corner = placeAtPoint(1270, 790, 200, 180, 1280, 800);
  assert.equal(corner.left, 1070);
  assert.equal(corner.top, 610);
  const tall = placeAtPoint(10, 10, 200, 2000, 1280, 600);
  assert.equal(tall.top, 8);
  assert.equal(tall.maxHeight, 584);
  const below = placeAtAnchor(
    { left: 500, right: 532, top: 100, bottom: 132 },
    200,
    150,
    1280,
    800,
  );
  assert.deepEqual([below.left, below.top], [332, 136]);
  const above = placeAtAnchor(
    { left: 500, right: 532, top: 700, bottom: 732 },
    200,
    150,
    1280,
    800,
  );
  assert.equal(above.top, 546);
  const edge = placeAtAnchor({ left: 0, right: 20, top: 100, bottom: 120 }, 200, 150, 1280, 800);
  assert.equal(edge.left, 8);
});

test('playlist names: blank, too long and case-insensitive duplicates rejected', () => {
  const lists = [
    { id: 1, name: '本地' },
    { id: 2, name: 'Night Drive' },
  ];
  assert.equal(playlistNameError('   ', lists), '请输入歌单名称');
  assert.match(playlistNameError(' 本地 ', lists), /已存在/);
  assert.match(playlistNameError('night drive', lists), /已存在/);
  assert.equal(playlistNameError('Night Drive', lists, 2), null, 'renaming to itself is fine');
  assert.equal(playlistNameError('新しい 🎵', lists), null);
  assert.match(playlistNameError('x'.repeat(101), lists), /最多/);
  assert.equal(suggestPlaylistName([]), '新建歌单');
  assert.equal(suggestPlaylistName([{ name: '新建歌单' }, { name: '新建歌单 2' }]), '新建歌单 3');
});

test('history: back/forward, no duplicates, forgetting a deleted playlist', () => {
  const h = createHistory({ view: 'home' });
  h.push({ view: 'songs' });
  h.push({ view: 'songs' });
  h.push({ view: 'playlist', id: 7 });
  h.push({ view: 'albums' });
  assert.equal(h.canBack, true);
  h.back();
  assert.deepEqual(h.current, { view: 'playlist', id: 7 });
  h.back();
  assert.deepEqual(h.current, { view: 'songs' });
  h.forward();
  assert.deepEqual(h.current, { view: 'playlist', id: 7 });
  h.forget((r) => r.view === 'playlist' && r.id === 7, { view: 'songs' });
  assert.deepEqual(h.current, { view: 'songs' });
  h.forward();
  assert.deepEqual(h.current, { view: 'albums' });
  h.back();
  h.back();
  assert.deepEqual(h.current, { view: 'home' });
  assert.equal(h.canBack, false);
});

test('virtual list range, page loading and keyboard scrolling', () => {
  assert.deepEqual(visibleRange(0, 560, 100, 56, 1000, 4), { start: 0, end: 13 });
  assert.deepEqual(visibleRange(5600, 560, 100, 56, 1000, 4), { start: 94, end: 113 });
  assert.deepEqual(visibleRange(999999, 560, 0, 56, 30, 4), { start: 30, end: 30 });
  assert.deepEqual(visibleRange(0, 560, 0, 56, 0), { start: 0, end: 0 });
  assert.deepEqual(pagesFor(95, 215, 100), [0, 1, 2]);
  assert.deepEqual(pagesFor(5, 5, 100), []);
  assert.equal(scrollToRow(0, 500, 560, 100, 56), 100);
  assert.equal(scrollToRow(20, 0, 560, 100, 56), 100 + 21 * 56 - 560);
  assert.equal(scrollToRow(3, 0, 560, 100, 56), null);
});

test('ambient colour is muted and readable, ignoring transparent and extreme pixels', () => {
  const red = [];
  for (let i = 0; i < 64; i++) red.push(230, 30, 40, 255);
  const [r, g, b] = ambientColor(red);
  assert.ok(r > g && r > b, 'keeps the hue');
  const max = Math.max(r, g, b) / 255,
    min = Math.min(r, g, b) / 255;
  assert.ok((max + min) / 2 <= 0.43, 'lightness capped');
  assert.equal(ambientColor([0, 0, 0, 255, 255, 255, 255, 255, 10, 200, 10, 0]), null);
});

test('formatting helpers', () => {
  assert.equal(duration(0), '0:00');
  assert.equal(duration(61500), '1:01');
  assert.equal(duration(3723000), '1:02:03');
  assert.equal(initial('  ailee'), 'A');
  assert.equal(initial('周杰伦'), '周');
  assert.equal(initial(''), '?');
  assert.equal(nameHue('IU'), nameHue('IU'));
  assert.ok(nameHue('IU') >= 0 && nameHue('IU') < 360);
});
