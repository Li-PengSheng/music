import { expect, test, type Page } from '@playwright/test';

const HARNESS = '/tests/harness/index.html';
const side = (page: Page, name: string | RegExp) =>
  page.locator('.sidebar').getByRole('button', { name, exact: typeof name === 'string' });

// Hold an actual IPC promise, rather than relying on timing or a second mock component.
async function holdNext(page: Page, command: string) {
  await page.evaluate((command) => {
    const w = window as any;
    const invoke = w.__TAURI_INTERNALS__.invoke;
    w.__reviewHeld = false;
    w.__TAURI_INTERNALS__.invoke = async (cmd: string, args: unknown) => {
      if (cmd !== command) return invoke(cmd, args);
      w.__TAURI_INTERNALS__.invoke = invoke;
      w.__reviewHeld = true;
      return new Promise((resolve, reject) => {
        w.__reviewResolve = resolve;
        w.__reviewReject = reject;
      });
    };
  }, command);
}

test('Home discards an old failed request after a newer track reload succeeds', async ({
  page,
}) => {
  await page.goto(HARNESS);
  await side(page, '设置').click();
  await holdNext(page, 'library_home');
  await side(page, '音乐空间').click();
  await page.waitForFunction(() => (window as any).__reviewHeld);
  await page.evaluate(() => (window as any).__TAURI_INTERNALS__.invoke('library_play', { id: 2 }));
  await expect(page.locator('.continue-info strong')).toHaveText('测试歌曲 2');
  await expect(page.getByRole('heading', { name: '最近播放', exact: true })).toBeVisible();
  await page.evaluate(() => (window as any).__reviewReject('obsolete home failure'));
  await page.waitForTimeout(100);
  await expect(page.getByRole('alert')).toHaveCount(0);
});

test('Settings and Now Playing ignore failures delivered after unmount', async ({ page }) => {
  await page.goto(HARNESS);
  await holdNext(page, 'media_status');
  await side(page, '设置').click();
  await page.waitForFunction(() => (window as any).__reviewHeld);
  await side(page, '音乐空间').click();
  await page.evaluate(() => (window as any).__reviewReject('obsolete settings failure'));
  await holdNext(page, 'player_lyrics');
  await page
    .locator('.player-bar')
    .getByRole('button', { name: '打开正在播放', exact: true })
    .click();
  await page.waitForFunction(() => (window as any).__reviewHeld);
  await page.getByRole('button', { name: '收起正在播放 (Esc)', exact: true }).click();
  await page.evaluate(() => (window as any).__reviewReject('obsolete lyrics failure'));
  await page.waitForTimeout(100);
  await expect(page.getByRole('alert')).toHaveCount(0);
});

for (const surface of ['settings', 'home', 'playing', 'console'] as const) {
  test(`${surface}: 40 palette/theme/font combinations fit both window sizes with readable tokens`, async ({
    page,
  }, testInfo) => {
    test.setTimeout(60_000);
    await page.goto(HARNESS);
    await expect(page.locator('.pb-volume-value')).toHaveText('70%');
    if (surface === 'settings') await side(page, '设置').click();
    if (surface === 'playing')
      await page
        .locator('.player-bar')
        .getByRole('button', { name: '打开正在播放', exact: true })
        .click();
    if (surface === 'console') await side(page, '控制台').click();
    for (const width of [1280, 900]) {
      await page.setViewportSize({ width, height: width === 900 ? 640 : 800 });
      for (const palette of ['celadon', 'cinnabar', 'azure', 'amber'])
        for (const theme of ['light', 'dark'])
          for (const font of ['harmony', 'puhuiti', 'serif', 'kaiti', 'system']) {
            const result = await page.evaluate(
              ({ palette, theme, font, surface }) => {
                const root = document.documentElement;
                Object.assign(root.dataset, { palette, theme, font });
                const probe = document.createElement('span');
                root.append(probe);
                const rgb = (token: string) => {
                  probe.style.color = `var(${token})`;
                  return getComputedStyle(probe)
                    .color.match(/[\d.]+/g)!
                    .slice(0, 3)
                    .map(Number);
                };
                const light = (color: number[]) => {
                  const c = color.map((v) => {
                    v /= 255;
                    return v <= 0.04045 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4;
                  });
                  return c[0] * 0.2126 + c[1] * 0.7152 + c[2] * 0.0722;
                };
                const ratio = (fg: string, bg: string) => {
                  const [a, b] = [light(rgb(fg)), light(rgb(bg))].sort((a, b) => b - a);
                  return (a + 0.05) / (b + 0.05);
                };
                const ratios = [
                  ratio('--text', '--bg'),
                  ratio('--text-2', '--surface'),
                  ratio('--text-3', '--surface-overlay'),
                  ratio('--accent-ink', '--accent'),
                ];
                probe.remove();
                const host = document.querySelector<HTMLElement>(
                  surface === 'console'
                    ? '[role="dialog"]'
                    : surface === 'playing'
                      ? '.immersive'
                      : '#main-content',
                )!;
                const box = host.getBoundingClientRect();
                const input = document.querySelector<HTMLElement>('#console-command');
                return {
                  ratios,
                  overflow: host.scrollWidth > host.clientWidth + 1,
                  left: box.left,
                  right: box.right,
                  inputRight: input?.getBoundingClientRect().right,
                  font: getComputedStyle(document.body).fontFamily,
                };
              },
              { palette, theme, font, surface },
            );
            const label = `${surface}/${width}/${palette}/${theme}/${font}`;
            expect(
              result.ratios.every((r) => r >= 4.5),
              `${label} contrast ${result.ratios}`,
            ).toBe(true);
            expect(result.overflow, label).toBe(false);
            expect(result.left, label).toBeGreaterThanOrEqual(0);
            expect(result.right, label).toBeLessThanOrEqual(width + 1);
            if (result.inputRight !== undefined)
              expect(result.inputRight, label).toBeLessThanOrEqual(width);
            expect(result.font.length, label).toBeGreaterThan(0);
          }
      await page.screenshot({
        path: testInfo.outputPath(`${surface}-${width}.png`),
        animations: 'disabled',
      });
    }
    await expect(page.getByRole('alert')).toHaveCount(0);
  });
}
