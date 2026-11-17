import { expect, test } from '@playwright/test';

const HARNESS = '/tests/harness/index.html';

test('body retains native smoothing, optical sizing and readable display metrics', async ({
  page,
}) => {
  await page.goto(HARNESS);
  await expect(page.locator('body')).toHaveCSS('-webkit-font-smoothing', 'auto');
  await expect(page.locator('body')).toHaveCSS('font-optical-sizing', 'auto');
  await expect(page.locator('body')).toHaveCSS('font-synthesis', 'none');
  await expect(page.locator('body')).toHaveCSS('text-rendering', /optimizelegibility/i);
  const title = page.locator('.page-title');
  expect(
    await title.evaluate((el) => parseFloat(getComputedStyle(el).letterSpacing)),
  ).toBeGreaterThan(0);
  await page.locator('.sidebar').getByRole('button', { name: /^专辑/ }).click();
  await page.locator('.card-open').first().click();
  const hero = page.locator('.hero-title');
  await expect(hero).toBeVisible();
  expect(
    await hero.evaluate((el) => parseFloat(getComputedStyle(el).letterSpacing)),
  ).toBeGreaterThan(0);
});

test('home artists show library track totals and center names without tracks', async ({
  page,
}, testInfo) => {
  await page.addInitScript(() => {
    (window as any).__mockHomeArtists = [
      ['艺术家 0', 9001],
      ['无曲目艺术家', 3000],
    ];
  });
  await page.goto(HARNESS);
  const section = page.getByRole('region', { name: '常听艺术家' });
  const cards = section.locator('.artist-pill');
  await expect(cards).toHaveCount(2);
  await expect(cards.first().locator('small')).toHaveText('28 首歌曲');
  await expect(section).not.toContainText(/次播放|播放次数|9001|3000/);
  const missing = cards.nth(1);
  await expect(missing.locator('strong')).toHaveText('无曲目艺术家');
  await expect(missing.locator('small')).toHaveCount(0);
  await missing.scrollIntoViewIfNeeded();
  const text = await missing.locator('.artist-pill-text').boundingBox();
  const avatar = await missing.locator('.artist-avatar').boundingBox();
  expect(Math.abs(text!.y + text!.height / 2 - avatar!.y - avatar!.height / 2)).toBeLessThan(1);
  await page.screenshot({ path: testInfo.outputPath('artists-1280.png'), animations: 'disabled' });
  await page.setViewportSize({ width: 900, height: 640 });
  await missing.scrollIntoViewIfNeeded();
  await expect(cards.first().locator('small')).toHaveText('28 首歌曲');
  await page.screenshot({ path: testInfo.outputPath('artists-900.png'), animations: 'disabled' });
});

for (const font of ['harmony', 'serif']) {
  test(`Now Playing metadata and single-line specs stay contained with long ${font} titles`, async ({
    page,
  }, testInfo) => {
    await page.addInitScript((value) => {
      localStorage.setItem('music-mine-font', value);
      localStorage.setItem('music-mine-theme', value === 'serif' ? 'dark' : 'light');
      localStorage.setItem('music-mine-reduce-motion', '1');
      (window as any).__lyricsFixture = {
        current_track: {
          path: 'C:\\Fixture\\001.flac',
          title: '夜色中的回声 · ' + '长标题与中文衬线字形'.repeat(10),
          artist: '歌手与联合演奏者 '.repeat(20),
          album: '专辑与长篇作品名称 '.repeat(20),
          duration_ms: 180000,
        },
      };
    }, font);
    await page.goto(HARNESS);
    await page
      .locator('.player-bar')
      .getByRole('button', { name: '打开正在播放', exact: true })
      .click();
    for (const size of [
      { width: 1280, height: 800 },
      { width: 900, height: 640 },
    ]) {
      await page.setViewportSize(size);
      for (const layout of ['封面与歌词', '大封面']) {
        await page.getByRole('combobox', { name: '正在播放布局' }).click();
        await page.getByRole('option', { name: layout, exact: true }).click();
        const specs = page.getByLabel('专业音频参数');
        await expect(specs).toContainText('FLAC');
        await expect(specs).toContainText('44.1 kHz');
        await expect(specs).toContainText('16 bit');
        await expect(specs).toContainText('27.8 MB');
        await expect(specs).not.toContainText(/播放|次/);
        await expect(specs).toHaveCSS('white-space', 'nowrap');
        await expect(specs).toHaveCSS('text-overflow', 'ellipsis');
        await expect(specs).toHaveCSS('font-size', '11px');
        const panel = page.locator('.artwork-panel');
        const panelBox = await panel.boundingBox();
        const contentBox = await page.locator('.immersive-content').boundingBox();
        const boxes = [];
        for (const selector of ['.large-artwork', 'h1', 'p', 'small', '.audio-specs']) {
          const item = panel.locator(selector);
          await expect(item).toBeInViewport({ ratio: 1 });
          const box = await item.boundingBox();
          expect(box!.x).toBeGreaterThanOrEqual(panelBox!.x - 0.5);
          expect(box!.x + box!.width).toBeLessThanOrEqual(panelBox!.x + panelBox!.width + 0.5);
          expect(box!.y).toBeGreaterThanOrEqual(contentBox!.y - 0.5);
          expect(box!.y + box!.height).toBeLessThanOrEqual(
            contentBox!.y + contentBox!.height + 0.5,
          );
          boxes.push(box!);
        }
        for (let i = 1; i < boxes.length - 1; i++)
          expect(boxes[i].y + boxes[i].height).toBeLessThanOrEqual(boxes[i + 1].y + 0.5);
        if (size.width === 1280 || layout === '大封面')
          await expect(panel.locator('h1')).toHaveCSS('margin-top', '16px');
        expect(
          await panel.evaluate(
            (el) => el.scrollHeight <= el.clientHeight + 1 && el.scrollWidth <= el.clientWidth + 1,
          ),
        ).toBe(true);
        expect(await specs.evaluate((el) => el.scrollHeight <= el.clientHeight + 1)).toBe(true);
        if (layout === '封面与歌词')
          expect(
            await page
              .locator('.lyric-lines > button')
              .first()
              .evaluate((el) => parseFloat(getComputedStyle(el).letterSpacing)),
          ).toBeGreaterThan(0);
        await page.screenshot({
          path: testInfo.outputPath(`playing-${size.width}-${layout}.png`),
          animations: 'disabled',
        });
      }
    }
  });
}
