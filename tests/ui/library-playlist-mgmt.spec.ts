import { expect, test, type Page } from '@playwright/test';

type LogEntry = { cmd: string; args: Record<string, unknown> };
async function calls(page: Page, command: string) {
  return page.evaluate(
    (cmd) =>
      (window as unknown as { __mockLog: LogEntry[] }).__mockLog.filter(
        (entry) => entry.cmd === cmd,
      ),
    command,
  );
}
async function fail(page: Page, command: string, message: string | null) {
  await page.evaluate(
    ({ command, message }) => {
      const w = window as unknown as { __mockFailures: Record<string, string | null> };
      w.__mockFailures ??= {};
      w.__mockFailures[command] = message;
    },
    { command, message },
  );
}
async function openPlaylist(page: Page) {
  await page.goto('/tests/harness/index.html');
  await page.getByRole('list', { name: '我的歌单' }).getByRole('button', { name: /夜晚/ }).click();
  await expect(page.locator('.row-title')).toHaveText(['测试歌曲 3', '测试歌曲 5']);
}

test('folder removal requires confirmation, reports failure, refreshes records and preserves player state', async ({
  page,
}) => {
  await page.goto('/tests/harness/index.html');
  await page.locator('.sidebar').getByRole('button', { name: '设置', exact: true }).click();
  const folder = page.locator('.folder-list li');
  const remove = page.getByRole('button', { name: '移除音乐文件夹：C:\\Fixture' });
  await folder.hover();
  await remove.click();
  const dialog = page.getByRole('dialog');
  await expect(dialog).toContainText('仅从音乐库移除记录，不会删除磁盘上的任何音乐文件');
  await dialog.getByRole('button', { name: '取消', exact: true }).click();
  await expect(folder).toHaveCount(1);
  expect(await calls(page, 'library_remove_folder')).toHaveLength(0);
  await expect(remove).toBeFocused();
  await fail(page, 'library_remove_folder', 'fixture removal failure');
  await folder.hover();
  await remove.click();
  await dialog.getByRole('button', { name: '移除', exact: true }).click();
  await expect(
    page.getByRole('alert').filter({ hasText: 'fixture removal failure' }),
  ).toBeVisible();
  await expect(folder).toHaveCount(1);
  await fail(page, 'library_remove_folder', null);
  await folder.hover();
  await remove.click();
  await dialog.getByRole('button', { name: '移除', exact: true }).click();
  await expect(folder).toHaveCount(0);
  await expect(page.getByText('尚未添加音乐文件夹。')).toBeVisible();
  await expect(page.locator('.pb-time').first()).toHaveText('0:42');
  await page.getByRole('list', { name: '我的歌单' }).getByRole('button', { name: /夜晚/ }).click();
  await expect(page.locator('.track-row')).toHaveCount(0);
});

test('playlist menu and Alt arrows reorder whole positions, preserve focus and reject boundaries/search', async ({
  page,
}) => {
  await openPlaylist(page);
  const rows = page.locator('.track-row');
  await rows.first().click({ button: 'right' });
  await expect(page.getByRole('menuitem', { name: '上移一位' })).toBeDisabled();
  await page.getByRole('menuitem', { name: '下移一位' }).click();
  await expect(page.locator('.row-title')).toHaveText(['测试歌曲 5', '测试歌曲 3']);
  await expect(rows.nth(1)).toBeFocused();
  await expect(page.getByRole('status').filter({ hasText: '移至第 2 位' })).toBeVisible();
  await page.keyboard.press('Alt+ArrowDown');
  expect(await calls(page, 'library_playlist_edit')).toHaveLength(1);
  await page.keyboard.press('Alt+ArrowUp');
  await expect(page.locator('.row-title')).toHaveText(['测试歌曲 3', '测试歌曲 5']);
  await expect(rows.first()).toBeFocused();
  await page.keyboard.press('Alt+ArrowUp');
  expect(await calls(page, 'library_playlist_edit')).toHaveLength(2);
  await rows.last().click({ button: 'right' });
  await expect(page.getByRole('menuitem', { name: '下移一位' })).toBeDisabled();
  await page.keyboard.press('Escape');
  await page.getByRole('searchbox').fill('测试歌曲 5');
  await expect(rows).toHaveCount(1);
  await rows.first().click({ button: 'right' });
  await expect(page.getByRole('menuitem', { name: /上移一位|下移一位/ })).toHaveCount(0);
  await page.keyboard.press('Escape');
  await rows.first().focus();
  await page.keyboard.press('Alt+ArrowUp');
  expect(await calls(page, 'library_playlist_edit')).toHaveLength(2);
  await page.getByRole('button', { name: '清除搜索' }).click();
  await expect(rows).toHaveCount(2);
  await fail(page, 'library_playlist_edit', 'fixture reorder failure');
  await rows.first().focus();
  await page.keyboard.press('Alt+ArrowDown');
  await expect(
    page.getByRole('alert').filter({ hasText: 'fixture reorder failure' }),
  ).toBeVisible();
  await expect(page.locator('.row-title')).toHaveText(['测试歌曲 3', '测试歌曲 5']);
});

test('playlist sorting preserves absolute indices and focus across virtualized page boundaries', async ({
  page,
}) => {
  await page.goto('/tests/harness/index.html?playlistTracks=105');
  await page.getByRole('list', { name: '我的歌单' }).getByRole('button', { name: /夜晚/ }).click();
  await expect(page.locator('.row-title').first()).toHaveText('测试歌曲 1');
  await page.locator('.main').evaluate((el) => {
    el.scrollTop = 5600;
  });
  const source = page.locator('.track-row[data-index="99"]');
  const target = page.locator('.track-row[data-index="100"]');
  await expect(source.locator('.row-title')).toHaveText('测试歌曲 100');
  await source.focus();
  await page.keyboard.press('Alt+ArrowDown');
  await expect(target.locator('.row-title')).toHaveText('测试歌曲 100');
  await expect(source.locator('.row-title')).toHaveText('测试歌曲 101');
  await expect(target).toBeFocused();
  const edits = await calls(page, 'library_playlist_edit');
  expect(edits.at(-1)?.args).toEqual({ action: 'reorder', playlistId: 8, from: 99, to: 100 });
});
