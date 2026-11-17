import { expect, test } from '@playwright/test';

test('ListenBrainz defaults off and validates credentials before enabling, then disables locally', async ({
  page,
}) => {
  await page.goto('/tests/harness/index.html');
  await page.locator('.sidebar').getByRole('button', { name: '设置', exact: true }).click();
  const card = page.getByRole('region', { name: '听歌记录 (ListenBrainz / Last.fm)' });
  const toggle = card.getByRole('switch', { name: '启用 Scrobble' });
  await expect(toggle).not.toBeChecked();
  await toggle.focus();
  await toggle.press('Space');
  await expect(card.getByRole('status')).toContainText('启用前');
  await expect(toggle).not.toBeChecked();
  await card.getByLabel('ListenBrainz 用户名', { exact: true }).fill('fixture');
  const token = card.getByLabel('User Token', { exact: true });
  await expect(token).toHaveAttribute('type', 'password');
  await token.fill('invalid');
  await card.getByRole('button', { name: '保存并测试连接' }).click();
  await expect(card.getByRole('status')).toContainText('Token 无效');
  await expect(toggle).not.toBeChecked();
  await token.fill('fixture-valid-token');
  await card.getByRole('button', { name: '保存并测试连接' }).click();
  await expect(card.getByRole('status')).toContainText('连接成功');
  await expect(token).toHaveValue('');
  for (const width of [1280, 900]) {
    await page.setViewportSize({ width, height: width === 900 ? 640 : 800 });
    await token.scrollIntoViewIfNeeded();
    const bounds = await token.boundingBox();
    expect(bounds!.x).toBeGreaterThanOrEqual(0);
    expect(bounds!.x + bounds!.width).toBeLessThanOrEqual(width);
  }
  await card.screenshot({ path: 'test-results/listenbrainz-settings-narrow.png' });
  await toggle.focus();
  await toggle.press('Space');
  await expect(toggle).toBeChecked();
  await expect(card.getByRole('status')).toContainText('已启用');
  await toggle.press('Space');
  await expect(toggle).not.toBeChecked();
  await expect(card.getByRole('status')).toContainText('已关闭');
  await page.locator('.sidebar').getByRole('button', { name: '音乐空间', exact: true }).click();
  await expect(
    page.getByRole('heading', { name: 'ListenBrainz 口味推荐（本地曲库）' }),
  ).toHaveCount(0);
});

test('enabled recommendations use local cards with playback and context menu at both sizes', async ({
  page,
}) => {
  await page.addInitScript(() => {
    (window as any).__mockListenBrainz = {
      enabled: true,
      username: 'fixture',
      token_configured: true,
      pending_count: 2,
    };
  });
  await page.goto('/tests/harness/index.html');
  const shelf = page.getByRole('region', { name: 'ListenBrainz 口味推荐（本地曲库）' });
  await expect(shelf).toBeVisible();
  for (const width of [1280, 900]) {
    await page.setViewportSize({ width, height: width === 900 ? 640 : 800 });
    await shelf.scrollIntoViewIfNeeded();
    const box = await shelf.boundingBox();
    expect(box!.x + box!.width).toBeLessThanOrEqual(width);
  }
  await shelf.screenshot({ path: 'test-results/listenbrainz-shelf-narrow.png' });
  const card = shelf.locator('.card').first();
  await card.click({ button: 'right' });
  await expect(page.getByRole('menu')).toBeVisible();
  await page.keyboard.press('Escape');
  await card.getByRole('button', { name: /^播放 / }).click();
  await expect(page.locator('.pb-title')).toContainText('测试歌曲 4');
});

for (const failed of [false, true])
  test(`recommendation ${failed ? 'failed' : 'empty'} state keeps the local home usable`, async ({
    page,
  }) => {
    await page.addInitScript((failed) => {
      (window as any).__mockListenBrainz = {
        enabled: true,
        username: 'fixture',
        token_configured: true,
        pending_count: 0,
      };
      (window as any).__mockListenBrainzFailure = failed;
      (window as any).__mockListenBrainzEmpty = !failed;
    }, failed);
    await page.goto('/tests/harness/index.html');
    await expect(page.getByRole('heading', { name: '最近添加', exact: true })).toBeVisible();
    await expect(
      page.getByRole('heading', { name: 'ListenBrainz 口味推荐（本地曲库）' }),
    ).toHaveCount(0);
  });
