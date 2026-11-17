import { expect, test, type Page } from '@playwright/test';

const rows = (page: Page) => page.getByRole('list', { name: '补全队列' }).getByRole('listitem');
const calls = (page: Page, cmd: string) =>
  page.evaluate((cmd) => (window as any).__mockLog.filter((c: any) => c.cmd === cmd), cmd);
async function open(page: Page, recent = true) {
  await page.addInitScript((recent) => {
    const now = Math.floor(Date.now() / 1000);
    (window as any).__mockTrackOverrides = {
      1: { added_at: now - 20 * 86400 },
      2: { added_at: now - (recent ? 2 : 15) * 86400 },
      3: { added_at: now - (recent ? 1 : 10) * 86400 },
      4: { added_at: now - 30 * 86400 },
    };
    const complete = {
      missing_artist: false,
      missing_album: false,
      missing_cover: false,
      missing_lyrics: false,
    };
    (window as any).__mockMetadataStatuses = {
      1: { ...complete, missing_cover: true },
      2: complete,
      3: { ...complete, missing_lyrics: recent },
      4: complete,
    };
  }, recent);
  await page.goto('/tests/harness/index.html?tracks=4');
  await page.locator('.sidebar').getByRole('button', { name: '标签补全', exact: true }).click();
  await expect(page.getByRole('heading', { name: '标签补全工作台' })).toBeVisible();
}
async function filter(page: Page, label: string) {
  await page.getByRole('combobox', { name: '待补全筛选' }).click();
  await page.getByRole('option', { name: label, exact: true }).click();
}
test('new arrivals are the default, sorted by newest first with badges and a missing-information refinement', async ({
  page,
}) => {
  await open(page);
  await expect(page.getByRole('combobox', { name: '待补全筛选' })).toContainText(
    '新入库（近 7 天）',
  );
  await expect(rows(page)).toHaveCount(2);
  await expect(rows(page).first()).toContainText('测试歌曲 3');
  await expect(rows(page).last()).toContainText('测试歌曲 2');
  await expect(rows(page).getByText('新歌', { exact: true })).toHaveCount(2);
  await filter(page, '新入库且缺信息');
  await expect(rows(page)).toHaveCount(1);
  await expect(rows(page).first()).toContainText('测试歌曲 3');
  await filter(page, '全部曲目');
  await expect(rows(page)).toHaveCount(4);
  await expect(rows(page).getByRole('button', { name: /^查看：/ })).toHaveText([
    /测试歌曲 3/,
    /测试歌曲 2/,
    /测试歌曲 1/,
    /测试歌曲 4/,
  ]);
  expect(await calls(page, 'metadata_search_candidates')).toHaveLength(0);
});
test('no recent tracks falls back to all four missing-information dimensions', async ({ page }) => {
  await open(page, false);
  await expect(page.getByRole('combobox', { name: '待补全筛选' })).toContainText(
    '信息不全（缺歌手/专辑/封面/歌词）',
  );
  await expect(rows(page)).toHaveCount(1);
  await expect(rows(page).first()).toContainText('测试歌曲 1');
  await expect(rows(page).getByText('新歌', { exact: true })).toHaveCount(0);
  expect(await calls(page, 'metadata_search_candidates')).toHaveLength(0);
});
test('ignoring removes a selected new track from todos and all tracks can restore it', async ({
  page,
}) => {
  await open(page);
  await page.getByRole('checkbox', { name: '选择：测试歌曲 3', exact: true }).check();
  await rows(page).filter({ hasText: '测试歌曲 3' }).hover();
  await page.getByRole('button', { name: '忽略：测试歌曲 3', exact: true }).click();
  await expect(rows(page)).toHaveCount(1);
  await expect(page.getByText('已选 0 首 · 可应用 0 首', { exact: true })).toBeVisible();
  expect((await calls(page, 'metadata_set_enrichment_ignored'))[0].args).toEqual({
    trackId: 3,
    ignored: true,
  });
  await filter(page, '缺歌词');
  await expect(rows(page)).toHaveCount(0);
  await filter(page, '全部曲目');
  const ignored = rows(page).filter({ hasText: '测试歌曲 3' });
  await expect(ignored.getByText('已忽略', { exact: true })).toBeVisible();
  await ignored.hover();
  await page.getByRole('button', { name: '恢复补全：测试歌曲 3', exact: true }).click();
  await filter(page, '新入库（近 7 天）');
  await expect(rows(page)).toHaveCount(2);
  expect((await calls(page, 'metadata_set_enrichment_ignored'))[1].args).toEqual({
    trackId: 3,
    ignored: false,
  });
  expect(await calls(page, 'metadata_search_candidates')).toHaveLength(0);
});

for (const width of [1280, 900])
  test(`new-track badges and ignore actions remain reachable at ${width}`, async ({ page }) => {
    await page.setViewportSize({ width, height: width === 900 ? 640 : 800 });
    if (width === 1280)
      await page.addInitScript(() => localStorage.setItem('music-mine-theme', 'dark'));
    await open(page);
    await expect(rows(page)).toHaveCount(2);
    const row = rows(page).first();
    await row.hover();
    const ignore = row.getByRole('button', { name: '忽略：测试歌曲 3', exact: true });
    await ignore.focus();
    await expect(ignore).toHaveCSS('opacity', '1');
    await expect(ignore).toBeInViewport();
    await expect(row.getByText('新歌', { exact: true })).toBeInViewport();
    expect(
      await page
        .locator('.enrichment-workbench')
        .evaluate((e) => e.scrollWidth > e.clientWidth + 1),
    ).toBe(false);
    expect(
      await page.locator('#main-content').evaluate((e) => e.scrollHeight > e.clientHeight + 1),
    ).toBe(false);
    await page.screenshot({ path: `tests/perf/out/task30-new-tracks-${width}.png` });
  });
