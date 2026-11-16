import { expect, test } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  await page.goto('/tests/harness/select.html');
});

test('Select delegates navigation, IME, Escape and Tab without changing the value', async ({
  page,
}) => {
  const trigger = page.getByRole('combobox', { name: '播放模式', exact: true });
  const highlighted = page.locator('.select-panel [data-highlighted]');
  await trigger.focus();
  await page.keyboard.press('ArrowDown');
  await expect(highlighted).toHaveText('选择模式');
  await page.keyboard.press('ArrowUp');
  await expect(highlighted).toHaveText('单曲循环');
  await page.keyboard.press('Home');
  await expect(highlighted).toHaveText('选择模式');
  await page.keyboard.press('End');
  await expect(highlighted).toHaveText('单曲循环');
  await page.keyboard.press('ArrowDown');
  await expect(highlighted).toHaveText('选择模式');
  await trigger.dispatchEvent('keydown', { key: 'ArrowDown', isComposing: true });
  await expect(highlighted).toHaveText('选择模式');
  await trigger.dispatchEvent('keydown', { key: '随' });
  await expect(highlighted).toHaveText('随机播放');
  await page.keyboard.press('Escape');
  await expect(page.getByRole('listbox')).toHaveCount(0);
  await expect(trigger).toBeFocused();
  await trigger.press('ArrowDown');
  await page.keyboard.press('Tab');
  await expect(page.getByRole('listbox')).toHaveCount(0);
  await expect(page.getByRole('button', { name: '外部更新' })).toBeFocused();
  await expect(page.getByLabel('绑定值')).toHaveText('(empty)');
  await expect(page.getByLabel('回调')).toHaveText('[]');
  await trigger.press('ArrowDown');
  await page.keyboard.press('Home');
  await page.keyboard.press('ArrowDown');
  await page.keyboard.press('Enter');
  await expect(page.getByLabel('绑定值')).toHaveText('sequence');
  await expect(page.getByLabel('回调')).toHaveText('["sequence"]');
});

test('Select preserves binding, callbacks, empty values, disabled state and both themes', async ({
  page,
}) => {
  const trigger = page.getByRole('combobox', { name: '播放模式', exact: true });
  await expect(page.getByRole('combobox', { name: '禁用选择' })).toBeDisabled();
  await page.getByRole('button', { name: '外部更新' }).click();
  await expect(trigger).toHaveText('随机播放');
  await expect(page.getByLabel('回调')).toHaveText('[]');
  for (const theme of ['dark', 'light']) {
    await page.locator('html').evaluate((el, theme) => (el.dataset.theme = theme), theme);
    await trigger.click();
    const panel = page.getByRole('listbox', { name: '播放模式' });
    await expect(panel).toBeVisible();
    const colors = await panel.evaluate((el) => {
      const style = getComputedStyle(el);
      const probe = document.createElement('div');
      probe.style.background = 'var(--surface-overlay)';
      el.append(probe);
      const expected = getComputedStyle(probe).backgroundColor;
      probe.remove();
      return { actual: style.backgroundColor, expected };
    });
    expect(colors.actual).toBe(colors.expected);
    await page.getByRole('option', { name: '顺序播放' }).click();
    await expect(page.getByLabel('绑定值')).toHaveText('sequence');
    await expect(trigger).toBeFocused();
    await trigger.click();
    await page.getByRole('option', { name: '选择模式' }).click();
    await expect(page.getByLabel('绑定值')).toHaveText('(empty)');
  }
  await expect(page.getByLabel('回调')).toHaveText('["sequence","","sequence",""]');
  await trigger.click();
  await page.getByRole('button', { name: '外部更新' }).click();
  await expect(page.getByRole('listbox')).toHaveCount(0);
  await page.getByRole('combobox', { name: '空列表' }).click();
  await expect(page.getByRole('option')).toHaveCount(0);
  await page.keyboard.press('Escape');
});

test('Select stays usable inside a native dialog and dismisses before its parent', async ({
  page,
}) => {
  await page.setViewportSize({ width: 900, height: 640 });
  await page.getByRole('button', { name: '打开详情' }).click();
  const dialog = page.getByRole('dialog');
  const trigger = dialog.getByRole('combobox', { name: '弹窗模式' });
  await trigger.click();
  const panel = dialog.getByRole('listbox');
  await expect(panel).toBeVisible();
  await page.getByRole('option', { name: '随机播放' }).click();
  await expect(trigger).toHaveText('随机播放');
  await expect(trigger).toBeFocused();
  await trigger.press('ArrowDown');
  await page.keyboard.press('Escape');
  await expect(panel).toHaveCount(0);
  await expect(dialog).toBeVisible();
  await trigger.click();
  await page.mouse.click(4, 4);
  await expect(panel).toHaveCount(0);
  await expect(dialog).toBeVisible();
  await page.keyboard.press('Escape');
  await expect(dialog).toHaveCount(0);
});

test('Select flips at viewport edges and scrolls long lists with the keyboard', async ({
  page,
}) => {
  await page.setViewportSize({ width: 900, height: 640 });
  const trigger = page.getByRole('combobox', { name: '长列表' });
  await trigger.click();
  const panel = page.getByRole('listbox', { name: '长列表' });
  const box = await panel.boundingBox();
  expect(box).toBeTruthy();
  expect(box!.x).toBeGreaterThanOrEqual(8);
  expect(box!.x + box!.width).toBeLessThanOrEqual(892);
  expect(box!.y).toBeGreaterThanOrEqual(8);
  expect(box!.y + box!.height).toBeLessThanOrEqual(632);
  expect(box!.height).toBeLessThanOrEqual(320);
  await page.keyboard.press('End');
  await expect(page.locator('[data-highlighted]')).toHaveText('项目 59');
  await expect(page.getByRole('option', { name: '项目 59', exact: true })).toBeInViewport();
  await page.keyboard.press('Enter');
  await expect(trigger).toHaveText('项目 59');
  await expect(trigger).toBeFocused();
});
