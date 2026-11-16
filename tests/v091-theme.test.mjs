import { sourceStyles } from './helpers/source-styles.mjs';
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import vm from 'node:vm';
import {
  parseTheme,
  resolveTheme,
  THEME_KEY,
  PALETTE_KEY,
  PALETTES,
  parsePalette,
} from '../src/lib/utils/theme.ts';

const read = (f) => readFileSync(f, 'utf8');

/** Run public/theme-init.js against a fake document, as WebView2 does before the bundle loads. */
function boot(stored, systemLight, palette) {
  const root = { dataset: {}, style: {} };
  const store = stored === undefined ? {} : { [THEME_KEY]: stored };
  store[PALETTE_KEY] = palette;
  const context = {
    localStorage: { getItem: (k) => (k in store ? store[k] : null) },
    matchMedia: (q) => ({
      matches: q === '(prefers-color-scheme: light)' ? systemLight : !systemLight,
    }),
    document: { documentElement: root },
  };
  context.window = context;
  vm.runInNewContext(read('public/theme-init.js'), context);
  return root;
}

test('missing and unknown themes default to light while saved choices are preserved', () => {
  for (const v of [null, undefined, '', 'Dark', 'blue']) assert.equal(parseTheme(v), 'light');
  assert.equal(parseTheme('dark'), 'dark');
  assert.equal(parseTheme('light'), 'light');
  assert.equal(parseTheme('system'), 'system');
  assert.equal(boot(undefined, true).dataset.theme, 'light', 'no saved choice defaults to light');
});

test('palette boot matches the parser for every mode, including old and invalid preferences', () => {
  for (const value of [null, undefined, '', 'unknown', 'celadon', 'cinnabar', 'azure', 'amber'])
    for (const mode of ['dark', 'light', 'system'])
      for (const systemLight of [false, true]) {
        const root = boot(mode, systemLight, value);
        assert.equal(root.dataset.palette, parsePalette(value));
        assert.equal(root.dataset.theme, resolveTheme(mode, !systemLight));
      }
  const root = { dataset: {}, style: {} };
  vm.runInNewContext(read('public/theme-init.js'), {
    document: { documentElement: root },
    localStorage: {
      getItem() {
        throw new Error('denied');
      },
    },
  });
  assert.equal(root.dataset.palette, 'celadon');
  assert.equal(root.dataset.theme, 'light');
});

test('resolved theme follows the choice, and the system only for 跟随系统', () => {
  assert.equal(resolveTheme('dark', false), 'dark');
  assert.equal(resolveTheme('light', true), 'light');
  assert.equal(resolveTheme('system', true), 'dark');
  assert.equal(resolveTheme('system', false), 'light');
});

test('pre-render script agrees with resolveTheme for every saved choice (no flash)', () => {
  for (const choice of ['dark', 'light', 'system'])
    for (const systemLight of [true, false]) {
      const root = boot(choice, systemLight);
      const expected = resolveTheme(choice, !systemLight);
      assert.equal(
        root.dataset.theme,
        expected,
        `${choice} / system ${systemLight ? 'light' : 'dark'}`,
      );
      assert.equal(root.style.colorScheme, expected);
    }
  const html = read('index.html');
  const init = html.indexOf('<script src="/theme-init.js"></script>');
  assert.ok(
    init > 0 && init < html.indexOf('</head>') && init < html.indexOf('type="module"'),
    'classic script in <head>, before the bundle',
  );
  assert.doesNotMatch(html, /<script>[^<]/, 'no inline script: CSP is default-src self');
});

test('choice persists locally and in desktop preferences, and system changes are followed', () => {
  const prefs = read('src/lib/stores/preferences.ts');
  assert.match(prefs, /localStorage\.setItem\(\s*THEME_KEY\s*,\s*choice\s*\)/);
  assert.match(prefs, /localStorage\.setItem\(\s*PALETTE_KEY\s*,\s*ui\.palette\s*\)/);
  const setPalette = prefs.slice(
    prefs.indexOf('export function setPalette'),
    prefs.indexOf('export function watchSystemTheme'),
  );
  assert.doesNotMatch(setPalette, /desktopPreferences|setWindowTheme|player/);
  assert.match(
    prefs,
    /desktopPreferences\(\s*null\s*,\s*null\s*,\s*choice\s*\)/,
    'Rust applies it to the window on the next start',
  );
  assert.match(prefs, /addEventListener\(\s*'change'\s*,\s*onMedia\s*\)/);
  assert.match(
    prefs,
    /onWindowThemeChanged\(\s*\(\s*t\s*\)\s*=>\s*\{\s*if\s*\(\s*ui\.theme\s*===\s*'system'\s*\)\s*paint\(\s*t\s*\)\s*;?\s*\}\s*\)/,
  );
  assert.doesNotMatch(
    prefs,
    /reduceMotion\s*=.*theme|theme.*reduceMotion\s*=/,
    'theme never touches reduce-motion',
  );
  const services = read('src/lib/services/music.ts');
  assert.match(
    services,
    /getCurrentWindow\(\)\.setTheme\(\s*choice\s*===\s*'system'\s*\?\s*null\s*:\s*choice\s*\)/,
  );
  assert.match(read('src-tauri/capabilities/default.json'), /"core:window:allow-set-theme"/);
  assert.match(read('src-tauri/tauri.conf.json'), /"theme":"Light","backgroundColor":"#f3f5f1"/);
  const desktop = read('src-tauri/src/platform/desktop.rs');
  assert.match(desktop, /w\.set_theme\(window_theme\(&preferences\.theme\)\)\?/);
  const settings = read('src/lib/views/SettingsView.svelte');
  assert.match(settings, /<h3 id="set-appearance">外观<\/h3>/);
  for (const label of ['深色', '浅色', '跟随系统'])
    assert.match(settings, new RegExp(`label\\s*:\\s*'${label}'`));
  assert.match(settings, /label="配色意境"/);
  assert.match(settings, /options=\{PALETTES\}/);
  assert.deepEqual(
    PALETTES.map((item) => item.label),
    ['松烟 · 秘色（青瓷）', '宣纸 · 朱砂（书卷）', '天青 · 汝窑（霁蓝）', '琥珀 · 沉香（暖金）'],
  );
});

test('both themes share one set of tokens and components', () => {
  const tokens = read('src/lib/styles/tokens.css');
  const dark = tokens.match(/:root\s*\{([^}]+)\}/)[1];
  const light = tokens.match(/:root\[data-theme=['"]?light['"]?\]\s*\{([^}]+)\}/)[1];
  const names = (block) => new Set([...block.matchAll(/(--[\w-]+):/g)].map((m) => m[1]));
  const darkNames = names(dark);
  for (const name of names(light))
    assert.ok(darkNames.has(name), `${name} overrides an existing token`);
  for (const [name, value] of [
    ['--bg', '#f3f5f1'],
    ['--bg-sidebar', '#e9ede7'],
    ['--surface', '#fafbf8'],
    ['--surface-hover', '#e2e8e0'],
    ['--text', '#29342c'],
    ['--accent', '#52705a'],
  ])
    assert.match(light, new RegExp(`${name}:\\s*${value};`, 'i'));
  const css = sourceStyles({ excludeTokens: true });
  const colours = [...css.matchAll(/#[0-9a-f]{3,8}\b|rgba?\((?!var\()[^)]*\)/gi)].map((m) =>
    m[0].replace(/\s+/g, ''),
  );
  const allowed = /^(?:#000|rgba\(0,0,0,0?\.[45]\))$/;
  assert.deepEqual(
    colours.filter((c) => !allowed.test(c)),
    [],
    'only masks and shadows over artwork stay literal',
  );
});
