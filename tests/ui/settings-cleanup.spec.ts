import { expect, test } from '@playwright/test';

test('settings ends with shortcuts and has no legacy database import entry at either window size', async ({
  page,
}) => {
  await page.goto('/tests/harness/index.html');
  await page.locator('.sidebar').getByRole('button', { name: '设置', exact: true }).click();
  const main = page.locator('#main-content');
  await expect(main.getByRole('heading', { name: '键盘快捷键', exact: true })).toBeVisible();
  await expect(main.getByRole('heading', { name: '从旧版迁移', exact: true })).toHaveCount(0);
  await expect(main.getByRole('button', { name: '导入旧版数据库', exact: true })).toHaveCount(0);
  await expect(main.getByRole('heading', { level: 3 }).last()).toHaveText('键盘快捷键');
  for (const width of [1280, 900]) {
    await page.setViewportSize({ width, height: width === 900 ? 640 : 800 });
    const shortcuts = main.getByRole('heading', { name: '键盘快捷键', exact: true });
    await shortcuts.scrollIntoViewIfNeeded();
    const bounds = await shortcuts.boundingBox();
    expect(bounds!.x).toBeGreaterThanOrEqual(0);
    expect(bounds!.x + bounds!.width).toBeLessThanOrEqual(width);
  }
});
