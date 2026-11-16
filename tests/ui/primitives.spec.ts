import { expect, test } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  await page.goto('/tests/harness/primitives.html');
});

test('menu navigates disabled items, wraps, searches, ignores IME and restores Escape/Tab focus', async ({
  page,
}) => {
  const trigger = page.getByRole('button', { name: '操作', exact: true });
  await trigger.click();
  const play = page.getByRole('menuitem', { name: '播放', exact: true });
  await page.keyboard.press('Home');
  await expect(play).toBeFocused();
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('menuitem', { name: '下一首播放' })).toBeFocused();
  await page.keyboard.press('End');
  await expect(page.getByRole('menuitem', { name: '失败操作' })).toBeFocused();
  await page.keyboard.press('ArrowDown');
  await expect(play).toBeFocused();
  await play.dispatchEvent('keydown', { key: '加' });
  const queue = page.getByRole('menuitem', { name: '加入播放队列' });
  await expect(queue).toBeFocused();
  await queue.dispatchEvent('keydown', { key: 'ArrowDown', isComposing: true });
  await expect(queue).toBeFocused();
  await page.keyboard.press('Escape');
  await expect(page.getByRole('menu')).toHaveCount(0);
  await expect(trigger).toBeFocused();
  await trigger.click();
  await expect(play).toBeFocused();
  await page.keyboard.press('Tab');
  await expect(page.getByRole('menu')).toHaveCount(0);
  await expect(trigger).toBeFocused();
});

test('menu handles point/rectangle anchors, collisions, replacement, outside clicks and action errors', async ({
  page,
}) => {
  await page.setViewportSize({ width: 900, height: 640 });
  const trigger = page.getByRole('button', { name: '操作', exact: true });
  await trigger.click();
  await page.getByRole('menuitem', { name: '加入播放队列' }).click();
  await expect(page.getByLabel('选择结果')).toHaveText('queue');
  await expect(trigger).toBeFocused();
  const edge = page.getByRole('button', { name: '边缘操作' });
  await edge.click({ button: 'right' });
  const menu = page.getByRole('menu');
  const box = await menu.boundingBox();
  expect(box).toBeTruthy();
  expect(box!.x).toBeGreaterThanOrEqual(8);
  expect(box!.y).toBeGreaterThanOrEqual(8);
  expect(box!.x + box!.width).toBeLessThanOrEqual(892);
  expect(box!.y + box!.height).toBeLessThanOrEqual(632);
  await page.getByRole('button', { name: '外部按钮' }).click();
  await expect(menu).toHaveCount(0);
  await trigger.click();
  await page.getByRole('menuitem', { name: '失败操作' }).click();
  await expect(page.getByLabel('错误')).toContainText('test action failed');
});

test('native Dialog preserves initial selection, focus trapping, drag-out and busy/IME dismissal guards', async ({
  page,
}) => {
  const trigger = page.getByRole('button', { name: '打开弹窗' });
  await trigger.click();
  const dialog = page.getByRole('dialog');
  const input = page.getByRole('textbox', { name: '名称' });
  await expect(input).toBeFocused();
  expect(
    await input.evaluate((el: HTMLInputElement) => [el.selectionStart, el.selectionEnd]),
  ).toEqual([0, 4]);
  await input.press('Shift+Tab');
  await expect(dialog.getByRole('button', { name: '关闭', exact: true })).toBeFocused();
  await input.focus();
  await input.dispatchEvent('keydown', { key: 'Escape', isComposing: true });
  await expect(dialog).toBeVisible();
  const box = await input.boundingBox();
  await page.mouse.move(box!.x + 8, box!.y + 8);
  await page.mouse.down();
  await page.mouse.move(4, 4);
  await page.mouse.up();
  await expect(dialog).toBeVisible();
  await dialog.getByRole('button', { name: '切换忙碌' }).click();
  await page.keyboard.press('Escape');
  await page.mouse.click(4, 4);
  await expect(dialog).toBeVisible();
  await dialog.getByRole('button', { name: '切换忙碌' }).click();
  await page.mouse.click(4, 4);
  await expect(dialog).toHaveCount(0);
  await expect(trigger).toBeFocused();
});

test('native Slider and Floating UI tooltip retain keyboard and viewport behavior', async ({
  page,
}) => {
  const slider = page.getByRole('slider', { name: '测试音量' });
  await slider.focus();
  await page.keyboard.press('ArrowRight');
  await expect(page.getByLabel('音量结果')).toHaveText('51');
  await page.keyboard.press('End');
  await expect(page.getByLabel('音量结果')).toHaveText('100');
  await page.getByRole('button', { name: '边缘操作' }).hover();
  const tip = page.getByRole('tooltip');
  await expect(tip).toHaveText('边缘提示');
  await expect(tip).toBeInViewport();
  await page.mouse.move(4, 4);
  await expect(tip).toBeHidden();
});
