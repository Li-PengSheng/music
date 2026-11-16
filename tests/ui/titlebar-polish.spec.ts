import { expect, test } from '@playwright/test';

test('window controls fit both headers and preserve interactive drag boundaries', async ({
  page,
}) => {
  await page.goto('/tests/harness/index.html');
  for (const size of [
    { width: 1280, height: 800 },
    { width: 900, height: 640 },
  ]) {
    await page.setViewportSize(size);
    for (const selector of ['.topbar', '.immersive-header']) {
      if (selector === '.immersive-header')
        await page
          .locator('.player-bar')
          .getByRole('button', { name: '打开正在播放', exact: true })
          .click();
      const header = page.locator(selector);
      await expect(header).toHaveAttribute('data-tauri-drag-region', '');
      const controls = header.getByRole('group', { name: '窗口控制' });
      await expect(controls).toBeVisible();
      const bounds = await controls.boundingBox();
      expect(bounds!.x + bounds!.width).toBeLessThanOrEqual(size.width);
      expect(bounds!.width).toBe(120);
      expect(
        await header
          .locator('button[data-tauri-drag-region], input[data-tauri-drag-region]')
          .count(),
      ).toBe(0);
      await controls.getByRole('button', { name: '最大化', exact: true }).click();
      await controls.getByRole('button', { name: '还原窗口', exact: true }).click();
      await controls.getByRole('button', { name: '最小化', exact: true }).click();
      await controls.getByRole('button', { name: '关闭窗口', exact: true }).click();
      if (selector === '.topbar') {
        const search = await header.locator('.search').boundingBox();
        expect(search!.x + search!.width).toBeLessThan(bounds!.x);
      } else {
        await expect(page.locator('.current-line')).toHaveCSS('font-weight', '700');
        await expect(page.locator('.current-line')).toHaveCSS(
          'transform',
          'matrix(1.03, 0, 0, 1.03, 0, 0)',
        );
        await page.getByRole('button', { name: '收起正在播放 (Esc)' }).click();
      }
    }
  }
  const commands = await page.evaluate(() =>
    (window as any).__mockLog.map((entry: any) => entry.cmd),
  );
  for (const name of ['minimize', 'toggle_maximize', 'close'])
    expect(commands).toContain('plugin:window|' + name);
});

test('artist avatars load real images and retain initials when loading fails', async ({ page }) => {
  await page.addInitScript(() => {
    (window as any).__mockThumb =
      'data:image/svg+xml,' +
      encodeURIComponent(
        '<svg xmlns="http://www.w3.org/2000/svg" width="96" height="96"><rect width="96" height="96" fill="green"/></svg>',
      );
  });
  await page.goto('/tests/harness/index.html');
  const avatar = page.locator('.artist-avatar').first();
  await avatar.scrollIntoViewIfNeeded();
  await expect(avatar.locator('.artwork')).toHaveAttribute('data-image', 'loaded');
  await expect(avatar).toHaveCSS('border-radius', '50%');
  await avatar.locator('img').evaluate((image: HTMLImageElement) => {
    image.src = 'data:image/png;base64,invalid';
  });
  await expect(avatar.locator('.artwork')).toHaveAttribute('data-image', 'error');
  await expect(avatar.locator('.artist-initial')).toBeVisible();
  await expect(avatar.locator('.artwork-placeholder')).toHaveCSS('visibility', 'hidden');
});

test('reduced motion keeps lyric focus unscaled and plain browser has no window buttons', async ({
  page,
}) => {
  await page.addInitScript(() => localStorage.setItem('music-mine-reduce-motion', '1'));
  await page.goto('/tests/harness/index.html');
  await page
    .locator('.player-bar')
    .getByRole('button', { name: '打开正在播放', exact: true })
    .click();
  await expect(page.locator('.current-line')).toHaveCSS('transform', 'none');
  await page.goto('/');
  await expect(page.locator('.topbar')).toBeVisible();
  await expect(page.getByRole('group', { name: '窗口控制' })).toHaveCount(0);
});
