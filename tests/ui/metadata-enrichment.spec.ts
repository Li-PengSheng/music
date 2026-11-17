import { expect, test, type Page } from '@playwright/test';
const calls = (page: Page, cmd: string) =>
  page.evaluate((cmd) => (window as any).__mockLog.filter((c: any) => c.cmd === cmd), cmd);
async function open(page: Page, fromDetails = true) {
  await page.goto('/tests/harness/index.html?tracks=4');
  if (fromDetails) {
    await page.locator('.sidebar').getByRole('button', { name: /^歌曲/ }).click();
    await page.locator('.track-row').nth(1).click({ button: 'right' });
    await page.getByRole('menuitem', { name: '歌曲详情', exact: true }).click();
    const dialog = page.getByRole('dialog');
    await expect(dialog.getByLabel('歌曲名', { exact: true })).toHaveCount(0);
    await dialog.getByRole('button', { name: '前往标签补全', exact: true }).click();
    await expect(dialog).toHaveCount(0);
  } else
    await page.locator('.sidebar').getByRole('button', { name: '标签补全', exact: true }).click();
  await expect(page.getByRole('heading', { name: '标签补全', exact: true })).toBeVisible();
  await expect(page.getByRole('list', { name: '补全队列' }).getByRole('listitem')).toHaveCount(4);
  return page.getByRole('region', { name: '当前曲目补全' });
}
test('details jumps to the selected track and previews before explicit application', async ({
  page,
}) => {
  const editor = await open(page);
  await expect(editor.getByRole('heading', { name: '测试歌曲 2', exact: true })).toBeVisible();
  await expect(
    page.locator('.sidebar').getByRole('button', { name: '标签补全', exact: true }),
  ).toHaveAttribute('aria-current', 'page');
  await expect(page.locator('.crumbs')).toContainText('音乐库');
  await expect(editor.getByRole('table', { name: '标签字段对比' })).toContainText('测试歌曲 2');
  expect((await calls(page, 'metadata_search_candidates')).map((c: any) => c.args.trackId)).toEqual(
    [2],
  );
  expect(await calls(page, 'metadata_apply_enrichment')).toHaveLength(0);
  await expect(
    editor.getByRole('checkbox', { name: '同时写入原始音频文件标签', exact: true }),
  ).not.toBeChecked();
  await editor.getByLabel('歌曲名', { exact: true }).fill('编辑关键词');
  await editor.getByLabel('艺术家', { exact: true }).fill('编辑艺人');
  await editor.getByRole('button', { name: '重新检索', exact: true }).click();
  await expect.poll(async () => (await calls(page, 'metadata_search_candidates')).length).toBe(2);
  await expect(editor.getByRole('button', { name: '应用当前补全', exact: true })).toBeEnabled();
  expect((await calls(page, 'metadata_search_candidates'))[1].args).toEqual({
    trackId: 2,
    titleQuery: '编辑关键词',
    artistQuery: '编辑艺人',
    forceRefresh: true,
  });
  await editor.getByRole('button', { name: '应用当前补全', exact: true }).click();
  await expect(editor.getByRole('status')).toContainText('已应用当前补全');
  expect((await calls(page, 'metadata_apply_enrichment'))[0].args.payload).toMatchObject({
    track_id: 2,
    dedicated_cover: true,
    write_audio_tags: false,
    overwrite_cover: false,
    overwrite_lyrics: false,
  });
  await page.locator('.sidebar').getByRole('button', { name: /^歌曲/ }).click();
  await expect(page.locator('.row-title').nth(1)).toHaveText('在线曲名');
});
test('sidebar stays local; filtered select-all searches and applies the selected matches', async ({
  page,
}) => {
  await open(page, false);
  expect(await calls(page, 'metadata_search_candidates')).toHaveLength(0);
  await page.getByRole('combobox', { name: '待补全筛选' }).click();
  await page.getByRole('option', { name: '信息不全（缺歌手/专辑）', exact: true }).click();
  await expect(page.getByRole('list', { name: '补全队列' }).getByRole('listitem')).toHaveCount(2);
  await page.getByRole('checkbox', { name: '全选当前筛选', exact: true }).check();
  await page.getByRole('button', { name: '批量检索所选', exact: true }).click();
  await expect(page.getByRole('status').filter({ hasText: '检索 2 / 2' })).toBeVisible();
  expect((await calls(page, 'metadata_search_candidates')).map((c: any) => c.args.trackId)).toEqual(
    [1, 3],
  );
  expect(await calls(page, 'metadata_apply_enrichment')).toHaveLength(0);
  await page.getByRole('button', { name: '批量应用已匹配', exact: true }).click();
  await page
    .getByRole('dialog')
    .getByRole('button', { name: '确认应用 2 首', exact: true })
    .click();
  await expect(page.getByRole('status').filter({ hasText: '应用 2 / 2' })).toBeVisible();
  expect(
    (await calls(page, 'metadata_apply_enrichment')).map((c: any) => c.args.payload.track_id),
  ).toEqual([1, 3]);
  for (const name of ['缺封面', '缺歌词']) {
    await page.getByRole('combobox', { name: '待补全筛选' }).click();
    await page.getByRole('option', { name, exact: true }).click();
    await expect(page.getByRole('list', { name: '补全队列' }).getByRole('listitem')).toHaveCount(2);
  }
});
test('candidate, lyrics and explicit opt-ins survive focus changes', async ({ page }) => {
  const editor = await open(page);
  await editor.getByRole('combobox', { name: '标签与封面候选' }).click();
  await page.getByRole('option', { name: /另一候选/ }).click();
  await editor.getByRole('combobox', { name: '同步歌词候选' }).click();
  await page.getByRole('option', { name: /另一歌词版本/ }).click();
  await expect(editor.getByLabel('LRC 预览')).toHaveText('[00:03.00]另一歌词');
  await editor.getByRole('checkbox', { name: '同时写入原始音频文件标签', exact: true }).check();
  await editor.getByRole('checkbox', { name: '允许覆盖已有封面', exact: true }).check();
  await page.getByRole('button', { name: '查看：测试歌曲 1', exact: true }).click();
  await page.getByRole('button', { name: '查看：测试歌曲 2', exact: true }).click();
  await expect(editor.getByLabel('目标歌曲名', { exact: true })).toHaveValue('另一候选');
  await expect(
    editor.getByRole('checkbox', { name: '同时写入原始音频文件标签', exact: true }),
  ).toBeChecked();
  await editor.getByRole('button', { name: '应用当前补全', exact: true }).click();
  await expect(editor.getByRole('status')).toContainText('已应用当前补全');
  expect((await calls(page, 'metadata_apply_enrichment'))[0].args.payload).toMatchObject({
    write_audio_tags: true,
    overwrite_cover: true,
    synced_lyrics: '[00:03.00]另一歌词',
  });
});
test('cancel and navigation stop scheduling the next request', async ({ page }) => {
  await open(page, false);
  await page.evaluate(() => {
    (window as any).__mockMetadataDelay = 350;
  });
  await page.getByRole('checkbox', { name: '全选当前筛选', exact: true }).check();
  await page.getByRole('button', { name: '批量检索所选', exact: true }).click();
  await expect.poll(async () => (await calls(page, 'metadata_search_candidates')).length).toBe(1);
  await page.getByRole('button', { name: '取消批量操作', exact: true }).click();
  await expect(page.getByRole('status').filter({ hasText: '已取消' })).toBeVisible();
  expect(await calls(page, 'metadata_search_candidates')).toHaveLength(1);
  await page.getByRole('button', { name: '批量检索所选', exact: true }).click();
  await page.locator('.sidebar').getByRole('button', { name: '音乐空间', exact: true }).click();
  await page.waitForTimeout(500);
  expect(await calls(page, 'metadata_search_candidates')).toHaveLength(2);
  await expect(page.getByRole('alert')).toHaveCount(0);
});
test('empty and failed searches, and failed writes preserve local metadata', async ({ page }) => {
  const editor = await open(page, false);
  await page.evaluate(() => {
    (window as any).__mockMetadataEmpty = true;
  });
  await editor.getByRole('button', { name: '重新检索', exact: true }).click();
  await expect(editor.getByText('没有找到标签候选')).toBeVisible();
  await expect(editor.getByRole('button', { name: '应用当前补全', exact: true })).toBeDisabled();
  await page.evaluate(() => {
    (window as any).__mockMetadataEmpty = false;
    (window as any).__mockFailures = { metadata_apply_enrichment: 'METADATA_FILE_EXISTS' };
  });
  await editor.getByRole('button', { name: '重新检索', exact: true }).click();
  await expect(editor.getByRole('button', { name: '应用当前补全', exact: true })).toBeEnabled();
  await editor.getByRole('button', { name: '应用当前补全', exact: true }).click();
  await expect(editor.getByRole('alert')).toContainText('METADATA_FILE_EXISTS');
  await expect(editor.getByRole('heading', { name: '测试歌曲 1', exact: true })).toBeVisible();
  await page.evaluate(() => {
    (window as any).__mockFailures = {};
  });
  await editor.getByRole('button', { name: '应用当前补全', exact: true }).click();
  await expect(editor.getByRole('status')).toContainText('已应用当前补全');
  await expect(page.locator('.player-bar')).toContainText('在线曲名');
  await page
    .locator('.player-bar')
    .getByRole('button', { name: '打开正在播放', exact: true })
    .click();
  await expect(page.locator('.immersive')).toContainText('在线歌词第一行');
});
for (const width of [1280, 900])
  test(`workbench ${width} fits both columns and keeps safe options reachable`, async ({
    page,
  }) => {
    if (width === 1280)
      await page.addInitScript(() => localStorage.setItem('music-mine-theme', 'dark'));
    await page.setViewportSize({ width, height: width === 900 ? 640 : 800 });
    const editor = await open(page);
    await expect(editor.getByRole('table', { name: '标签字段对比' })).toBeVisible();
    const overflow = await page
      .locator('.enrichment-workbench')
      .evaluate((el) => el.scrollWidth > el.clientWidth + 1);
    expect(overflow).toBe(false);
    expect(
      await page.locator('#main-content').evaluate((el) => el.scrollHeight > el.clientHeight + 1),
    ).toBe(false);
    await page.screenshot({ path: `tests/perf/out/task28-workbench-${width}-top.png` });
    await editor
      .getByRole('button', { name: '应用当前补全', exact: true })
      .scrollIntoViewIfNeeded();
    await expect(
      editor.getByRole('button', { name: '应用当前补全', exact: true }),
    ).toBeInViewport();
    await page.screenshot({ path: `tests/perf/out/task28-workbench-${width}.png` });
  });

test('manual target fields apply without any online candidate and keep audio opt-in off', async ({
  page,
}) => {
  const editor = await open(page, false);
  await page.evaluate(() => {
    (window as any).__mockMetadataEmpty = true;
  });
  await editor.getByRole('button', { name: '重新检索', exact: true }).click();
  await expect(editor.getByText('没有找到标签候选')).toBeVisible();
  await editor.getByLabel('目标歌曲名', { exact: true }).fill('手填歌曲');
  await editor.getByLabel('目标艺术家', { exact: true }).fill('手填歌手');
  await editor.getByLabel('目标专辑', { exact: true }).fill('手填专辑');
  await editor.getByLabel('目标年份', { exact: true }).fill('2024');
  expect(await calls(page, 'metadata_apply_enrichment')).toHaveLength(0);
  await editor.getByRole('button', { name: '应用当前补全', exact: true }).click();
  await expect(editor.getByRole('status')).toContainText('已应用当前补全');
  expect((await calls(page, 'metadata_apply_enrichment'))[0].args.payload).toMatchObject({
    tags: { title: '手填歌曲', artist: '手填歌手', album: '手填专辑', year: 2024 },
    write_audio_tags: false,
    save_cover: false,
    save_lyrics: false,
  });
  await expect(editor.getByRole('heading', { name: '手填歌曲', exact: true })).toBeVisible();
});

test('LRCLIB backfilled tags show their source and allow Simplified Chinese target edits', async ({
  page,
}) => {
  const editor = await open(page, false);
  await page.evaluate(() => {
    (window as any).__mockMetadataBackfill = true;
  });
  await editor.getByRole('button', { name: '重新检索', exact: true }).click();
  await expect(editor.getByRole('combobox', { name: '标签与封面候选' })).toContainText('[LRCLIB]');
  await expect(editor.getByLabel('目标艺术家', { exact: true })).toHaveValue('王唯旖');
  await expect(editor.getByLabel('目标专辑', { exact: true })).toHaveValue('无情画 - Single');
  await editor.getByLabel('目标歌曲名', { exact: true }).fill('后来的我们');
  await editor.getByRole('button', { name: '应用当前补全', exact: true }).click();
  await expect(editor.getByRole('status')).toContainText('已应用当前补全');
  expect((await calls(page, 'metadata_apply_enrichment'))[0].args.payload.tags).toMatchObject({
    source: 'LRCLIB',
    title: '后来的我们',
    artist: '王唯旖',
    album: '无情画 - Single',
  });
  await expect(editor.getByRole('alert')).toHaveCount(0);
});

test('iTunes source and its constrained artwork URL reach the confirmed payload', async ({
  page,
}) => {
  const editor = await open(page, false);
  await page.evaluate(() => {
    (window as any).__mockMetadataSource = 'iTunes';
  });
  await editor.getByRole('button', { name: '重新检索', exact: true }).click();
  await expect(editor.getByRole('combobox', { name: '标签与封面候选' })).toContainText('[iTunes]');
  await expect(editor.getByRole('checkbox', { name: '保存专属封面', exact: true })).toBeChecked();
  await editor.getByRole('button', { name: '应用当前补全', exact: true }).click();
  await expect(editor.getByRole('status')).toContainText('已应用当前补全');
  expect((await calls(page, 'metadata_apply_enrichment'))[0].args.payload).toMatchObject({
    save_cover: true,
    cover_url: 'https://is1-ssl.mzstatic.com/image/thumb/Music/fixture/600x600bb.jpg',
  });
});

test('returning to the workbench preserves filter, checked IDs, focus, results and target draft without re-search', async ({
  page,
}) => {
  const editor = await open(page);
  await expect(editor.getByRole('combobox', { name: '标签与封面候选' })).toBeVisible();
  await editor.getByLabel('目标歌曲名', { exact: true }).fill('保留草稿');
  await page.getByRole('checkbox', { name: '选择：测试歌曲 2', exact: true }).check();
  await page.getByRole('combobox', { name: '待补全筛选' }).click();
  await page.getByRole('option', { name: '缺封面', exact: true }).click();
  const searches = (await calls(page, 'metadata_search_candidates')).length;
  const statuses = (await calls(page, 'metadata_track_statuses')).length;
  await page.locator('.sidebar').getByRole('button', { name: '音乐空间', exact: true }).click();
  await page.locator('.sidebar').getByRole('button', { name: '标签补全', exact: true }).click();
  await expect(editor.getByLabel('目标歌曲名', { exact: true })).toHaveValue('保留草稿');
  await expect(page.getByRole('checkbox', { name: '选择：测试歌曲 2', exact: true })).toBeChecked();
  await expect(page.getByRole('button', { name: '查看：测试歌曲 2', exact: true })).toHaveAttribute(
    'aria-pressed',
    'true',
  );
  await expect(page.getByRole('combobox', { name: '待补全筛选' })).toContainText('缺封面');
  expect((await calls(page, 'metadata_search_candidates')).length).toBe(searches);
  expect((await calls(page, 'metadata_track_statuses')).length).toBe(statuses);
  await editor.getByRole('button', { name: '重新检索', exact: true }).click();
  await expect
    .poll(async () => (await calls(page, 'metadata_search_candidates')).length)
    .toBe(searches + 1);
  expect((await calls(page, 'metadata_search_candidates')).at(-1).args.forceRefresh).toBe(true);
});

test('library revision removes vanished session entries after folder removal', async ({ page }) => {
  const editor = await open(page, false);
  await editor.getByLabel('目标歌曲名', { exact: true }).fill('幸存草稿');
  await page.getByRole('checkbox', { name: '选择：测试歌曲 1', exact: true }).check();
  await page.locator('.sidebar').getByRole('button', { name: '设置', exact: true }).click();
  await page.locator('.folder-list li').hover();
  await page.getByRole('button', { name: '移除音乐文件夹：C:\\Fixture' }).click();
  await page.getByRole('dialog').getByRole('button', { name: '移除', exact: true }).click();
  await page.locator('.sidebar').getByRole('button', { name: '标签补全', exact: true }).click();
  await expect(page.getByRole('list', { name: '补全队列' }).getByRole('listitem')).toHaveCount(0);
  await expect(page.getByText('已选 0 首 · 可应用 0 首', { exact: true })).toBeVisible();
  expect(await calls(page, 'metadata_search_candidates')).toHaveLength(0);
});
