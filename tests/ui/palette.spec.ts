import { expect, test } from '@playwright/test';

const HARNESS = '/tests/harness/index.html';

test('palette preview is immediate, persistent, and leaves playback alone', async ({ page }) => {
  await page.goto(HARNESS);
  await expect(page.locator('html')).toHaveAttribute('data-palette', 'celadon');
  await expect(page.locator('.pb-volume-value')).toHaveText('70%');
  await expect(page.locator('.player-bar .pb-time').first()).toHaveText('0:42');

  await page.locator('.sidebar').getByRole('button', { name: '设置' }).click();
  await page.getByRole('combobox', { name: '配色意境' }).click();
  await page.getByRole('option', { name: '宣纸 · 朱砂（书卷）' }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
  await expect(page.locator('html')).toHaveAttribute('data-palette', 'cinnabar');
  await expect(page.locator('body')).toHaveCSS('background-color', 'rgb(22, 19, 18)');
  await expect(page.locator('.pb-volume-value')).toHaveText('70%');
  await expect(page.locator('.player-bar .pb-time').first()).toHaveText('0:42');

  await page.getByRole('combobox', { name: '主题' }).click();
  await page.getByRole('option', { name: '浅色' }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
  await expect(page.locator('html')).toHaveAttribute('data-palette', 'cinnabar');
  await expect(page.locator('body')).toHaveCSS('background-color', 'rgb(246, 242, 234)');
  await expect(page.locator('.pb-volume-value')).toHaveText('70%');

  await page.reload();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
  await expect(page.locator('html')).toHaveAttribute('data-palette', 'cinnabar');
  await expect(page.locator('.pb-volume-value')).toHaveText('70%');
  await expect(page.locator('.player-bar .pb-time').first()).toHaveText('0:42');
});
