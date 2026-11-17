import { expect, test, type Page } from '@playwright/test';
import { writeFile } from 'node:fs/promises';

const HARNESS = '/tests/harness/index.html';
const presets = [
  {
    value: 'harmony',
    label: '鸿蒙黑体（HarmonyOS Sans · 默认现代）',
    sans: 'HarmonyOS Sans SC',
    display: 'HarmonyOS Sans SC',
  },
  {
    value: 'puhuiti',
    label: '阿里普惠体 3.0（现代清晰）',
    sans: 'Alibaba PuHuiTi 3.0',
    display: 'Alibaba PuHuiTi 3.0',
  },
  {
    value: 'serif',
    label: '思源宋体 VF（中式书卷 · 歌词与标题衬线）',
    sans: 'HarmonyOS Sans SC',
    display: 'Noto Serif SC',
  },
  { value: 'kaiti', label: '传统楷书（华文楷体 · 温润手书）', sans: 'STKaiti', display: 'STKaiti' },
  {
    value: 'system',
    label: 'Windows 原生（Segoe UI Variable + 微软雅黑）',
    sans: 'Segoe UI Variable Text',
    display: 'Segoe UI Variable Display',
  },
];
async function selectFont(page: Page, label: string) {
  await page.getByRole('combobox', { name: '字体风格', exact: true }).click();
  await page.getByRole('option', { name: label, exact: true }).click();
}

test('fresh fonts use harmony before the bundle and survive unavailable storage', async ({
  page,
}) => {
  await page.goto(HARNESS);
  await expect(page.locator('html')).toHaveAttribute('data-font', 'harmony', { timeout: 2000 });
  await expect(page.locator('body')).toHaveCSS('font-family', /HarmonyOS Sans SC/);
  await page.addInitScript(() => {
    Storage.prototype.getItem = () => {
      throw new Error('denied');
    };
    Storage.prototype.setItem = () => {
      throw new Error('denied');
    };
  });
  await page.reload();
  await expect(page.locator('html')).toHaveAttribute('data-font', 'harmony');
  await page.locator('.sidebar').getByRole('button', { name: '设置', exact: true }).click();
  await selectFont(page, presets[3].label);
  await expect(page.locator('html')).toHaveAttribute('data-font', 'kaiti');
  await expect(page.getByRole('alert')).toHaveCount(0);
});

test('five fonts preview live, persist on reload and preserve theme, palette and playback', async ({
  page,
}, testInfo) => {
  await page.addInitScript(() => {
    localStorage.setItem('music-mine-theme', 'dark');
    localStorage.setItem('music-mine-palette', 'cinnabar');
  });
  await page.goto(HARNESS);
  await expect(page.locator('.pb-volume-value')).toHaveText('70%');
  await page.locator('.sidebar').getByRole('button', { name: '设置', exact: true }).click();
  await expect(page.getByRole('combobox', { name: '音频输出设备' })).toBeEnabled();
  const mutationCount = () =>
    page.evaluate(
      () =>
        (window as any).__mockLog.filter((entry: any) =>
          /^(player_(play|pause|stop|seek|set_volume|set_output_device|configure)|library_(play|queue|queue_tracks)|desktop_preferences|plugin:window\|set_theme)$/.test(
            entry.cmd,
          ),
        ).length,
    );
  const before = await mutationCount();
  const fontSession = await page.context().newCDPSession(page);
  await fontSession.send('DOM.enable');
  await fontSession.send('CSS.enable');
  const rendered: Record<string, unknown> = {};
  // Start with another family so choosing the unsaved default is a real value change too.
  for (const preset of [presets[1], presets[0], ...presets.slice(2)]) {
    await selectFont(page, preset.label);
    await expect(page.locator('html')).toHaveAttribute('data-font', preset.value);
    await expect(page.locator('body')).toHaveCSS(
      'font-family',
      new RegExp(preset.sans.replaceAll('.', '\\.')),
    );
    await expect(page.locator('.page-title')).toHaveCSS('font-family', new RegExp(preset.display));
    await expect(page.locator('.brand')).toHaveCSS('font-family', new RegExp(preset.display));
    await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
    await expect(page.locator('html')).toHaveAttribute('data-palette', 'cinnabar');
    await expect(page.locator('.pb-volume-value')).toHaveText('70%');
    await expect(page.locator('.player-bar .pb-time').first()).toHaveText('0:42');
    expect(await page.evaluate(() => localStorage.getItem('music-mine-font'))).toBe(preset.value);
    const { root } = await fontSession.send('DOM.getDocument');
    const { nodeId } = await fontSession.send('DOM.querySelector', {
      nodeId: root.nodeId,
      selector: '.brand-name',
    });
    rendered[preset.value] = await fontSession.send('CSS.getPlatformFontsForNode', { nodeId });
  }
  const fontReport = testInfo.outputPath('rendered-local-fonts.json');
  await writeFile(fontReport, JSON.stringify(rendered, null, 2));
  await testInfo.attach('rendered-local-fonts', {
    path: fontReport,
    contentType: 'application/json',
  });
  await fontSession.detach();
  expect(await mutationCount()).toBe(before);
  await page.reload();
  await expect(page.locator('html')).toHaveAttribute('data-font', 'system');
});

test('serif keeps small lists sans-serif and updates open desktop lyrics including translations', async ({
  page,
  context,
}) => {
  await context.addInitScript(() => {
    (window as any).__mockLyrics = {
      lines: [{ time_ms: 0, text: '本地原文\u2009本地译文' }],
      offset_ms: 0,
      synced: true,
      source: 'fixture',
    };
  });
  await page.goto(HARNESS);
  const overlay = await context.newPage();
  await overlay.goto(`${HARNESS}?window=lyrics`);
  await expect(overlay.locator('.lyrics-main')).toBeVisible();
  await page.locator('.sidebar').getByRole('button', { name: '设置', exact: true }).click();
  await selectFont(page, presets[2].label);
  await expect(overlay.locator('html')).toHaveAttribute('data-font', 'serif');
  await expect(overlay.locator('.lyrics-main')).toHaveCSS('font-family', /Noto Serif SC/);
  await expect(overlay.locator('.lyrics-sub')).toHaveCSS('font-family', /Noto Serif SC/);
  await page.locator('.sidebar').getByRole('button', { name: /^歌曲/ }).click();
  await expect(page.locator('.track-row').first()).toHaveCSS('font-family', /HarmonyOS Sans SC/);
  await page
    .locator('.player-bar')
    .getByRole('button', { name: '打开正在播放', exact: true })
    .click();
  await expect(page.locator('.artwork-panel h1')).toHaveCSS('font-family', /Noto Serif SC/);
  await expect(page.locator('.lyric-lines > button').first()).toHaveCSS(
    'font-family',
    /Noto Serif SC/,
  );
  await page.evaluate(() => localStorage.removeItem('music-mine-font'));
  await expect(overlay.locator('html')).toHaveAttribute('data-font', 'harmony');
  await overlay.close();
});

test('long font labels and menus remain readable at both window sizes in both themes', async ({
  page,
}, testInfo) => {
  await page.goto(HARNESS);
  await page.locator('.sidebar').getByRole('button', { name: '设置', exact: true }).click();
  for (const size of [
    { width: 1280, height: 800 },
    { width: 900, height: 640 },
  ]) {
    await page.setViewportSize(size);
    for (const theme of ['浅色', '深色']) {
      await page.getByRole('combobox', { name: '主题', exact: true }).click();
      await page.getByRole('option', { name: theme, exact: true }).click();
      for (const preset of presets) {
        await selectFont(page, preset.label);
        const picker = page.getByRole('combobox', { name: '字体风格', exact: true });
        const row = picker.locator('..');
        await expect(row.getByText('字体风格', { exact: true })).toBeVisible();
        const text = picker.locator('span').first();
        expect(await text.evaluate((el) => el.scrollWidth <= el.clientWidth + 1)).toBe(true);
        const bounds = await picker.boundingBox();
        expect(bounds!.x).toBeGreaterThanOrEqual(0);
        expect(bounds!.x + bounds!.width).toBeLessThanOrEqual(size.width);
        await page.screenshot({
          path: testInfo.outputPath(`fonts-${size.width}-${theme}-${preset.value}.png`),
          animations: 'disabled',
        });
      }
    }
  }
});
