import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const read = (f) => readFileSync(f, 'utf8');
const tokens = read('src/lib/styles/tokens.css').replace(/\/\*[\s\S]*?\*\//g, '');
const [darkBlock, lightBlock] = tokens.split(/:root\[data-theme=['"]?light['"]?\]/);
const parse = (block) =>
  Object.fromEntries([...block.matchAll(/(--[\w-]+):([^;]+);/g)].map((m) => [m[1], m[2].trim()]));
const dark = parse(darkBlock);
const light = { ...dark, ...parse(lightBlock) };

function rgba(value) {
  const short = value.match(/^#([0-9a-f])([0-9a-f])([0-9a-f])$/i);
  if (short)
    value =
      '#' +
      short
        .slice(1)
        .map((c) => c + c)
        .join('');
  const hex = value.match(/^#([0-9a-f]{6})$/i);
  if (hex) return [0, 2, 4].map((i) => parseInt(hex[1].slice(i, i + 2), 16)).concat(1);
  const fn = value.match(/^rgba?\(([^)]+)\)$/);
  if (fn) {
    const p = fn[1].split(',').map(Number);
    return [p[0], p[1], p[2], p[3] ?? 1];
  }
  throw new Error(`not a literal colour: ${value}`);
}
/** Composite a (possibly translucent) colour over an opaque base. */
function over(top, base) {
  const [r, g, b, a] = rgba(top);
  const [R, G, B] = rgba(base);
  return [r * a + R * (1 - a), g * a + G * (1 - a), b * a + B * (1 - a), 1];
}
function luminance([r, g, b]) {
  const c = (v) => {
    v /= 255;
    return v <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4;
  };
  return 0.2126 * c(r) + 0.7152 * c(g) + 0.0722 * c(b);
}
function contrast(a, b) {
  const [x, y] = [luminance(a), luminance(b)].sort((p, q) => q - p);
  return (x + 0.05) / (y + 0.05);
}

for (const [name, t] of [
  ['dark', dark],
  ['light', light],
]) {
  test(`${name}: text meets 4.5:1 on every surface it sits on`, () => {
    const surfaces = {
      bg: t['--bg'],
      sidebar: t['--bg-sidebar'],
      player: t['--bg-player'],
      card: t['--surface'],
      overlay: t['--surface-overlay'],
      drawer: t['--bg-drawer'],
    };
    surfaces['hover on bg'] = over(t['--hover'], t['--bg']).slice(0, 3).map(Math.round);
    surfaces['selected on sidebar'] = over(t['--selected'], t['--bg-sidebar'])
      .slice(0, 3)
      .map(Math.round);
    for (const [where, surface] of Object.entries(surfaces)) {
      const base = Array.isArray(surface) ? [...surface, 1] : rgba(surface);
      for (const text of ['--text', '--text-2', '--text-3']) {
        const ratio = contrast(rgba(t[text]), base);
        assert.ok(ratio >= 4.5, `${text} on ${where}: ${ratio.toFixed(2)}`);
      }
    }
  });
  test(`${name}: accent and danger stay legible; primary button text contrasts with the accent`, () => {
    assert.ok(
      contrast(rgba(t['--accent']), rgba(t['--bg'])) >= 4.5,
      'accent text (current song) on bg',
    );
    assert.ok(
      contrast(rgba(t['--accent-ink']), rgba(t['--accent'])) >= 4.5,
      'primary button label',
    );
    assert.ok(
      contrast(rgba(t['--danger']), rgba(t['--surface-overlay'])) >= 4.5,
      'danger menu items',
    );
    assert.ok(
      contrast(rgba(t['--on-danger']), rgba(t['--danger-bg'])) >= 4.5,
      'danger button label',
    );
    assert.ok(
      contrast(rgba(t['--focus']), rgba(t['--bg'])) >= 3,
      'focus ring is a visible non-text indicator',
    );
  });
}

test('light theme uses the soft warm palette with no pure-white surfaces', () => {
  for (const token of [
    '--bg',
    '--bg-sidebar',
    '--bg-player',
    '--surface',
    '--surface-overlay',
    '--bg-drawer',
    '--toast-bg',
  ])
    assert.notEqual(light[token].toLowerCase(), '#ffffff', `${token} is not pure white`);
  assert.equal(light['--bg-player'], light['--bg-sidebar'], 'player bar blends with the sidebar');
  const [r, g, b] = rgba(light['--accent']);
  const max = Math.max(r, g, b),
    min = Math.min(r, g, b);
  assert.ok((max - min) / max < 0.3, 'brand green is low-saturation');
});

test('dark theme keeps its core palette', () => {
  for (const [token, value] of [
    ['--bg', '#121414'],
    ['--bg-sidebar', '#090b0c'],
    ['--surface', '#1c2220'],
    ['--accent', '#b9d69b'],
    ['--text', '#e8ece7'],
  ])
    assert.equal(dark[token], value);
  assert.equal(dark['--hover'], dark['--surface'], 'hover fill unchanged from V0.9.1');
  assert.equal(dark['--selected'], dark['--surface-hover'], 'selected fill unchanged from V0.9.1');
});
