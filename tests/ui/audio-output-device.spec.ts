import { expect, test } from '@playwright/test';

async function settings(page: import('@playwright/test').Page, query = '') {
  await page.goto('/tests/harness/index.html' + query);
  await page.locator('.sidebar').getByRole('button', { name: '设置', exact: true }).click();
  return page.getByRole('combobox', { name: '音频输出设备', exact: true });
}

test('device selector switches via IPC, keeps playback and offers the system default', async ({
  page,
}) => {
  const select = await settings(page);
  await expect(select).toBeEnabled();
  await select.click();
  await expect(page.getByRole('option', { name: '跟随系统默认设备', exact: true })).toBeVisible();
  await page.getByRole('option', { name: 'Headphones', exact: true }).click();
  await expect(select).toContainText('Headphones');
  await expect(page.getByRole('status').filter({ hasText: '音频输出已切换' })).toBeVisible();
  await expect
    .poll(() => page.evaluate(() => (window as any).__mockSnapshot().output_device_id))
    .toBe('Headphones');
  expect(
    await page.evaluate(() => {
      const s = (window as any).__mockSnapshot();
      return [s.position_ms, s.playback_state, s.volume];
    }),
  ).toEqual([42000, 'paused', 0.7]);
  await select.click();
  await page.getByRole('option', { name: '跟随系统默认设备', exact: true }).click();
  await expect
    .poll(() =>
      page.evaluate(() =>
        (window as any).__mockLog
          .filter((c: any) => c.cmd === 'player_set_output_device')
          .map((c: any) => c.args),
      ),
    )
    .toEqual([{ device: 'Headphones' }, { device: null }]);
  await expect
    .poll(() =>
      page.evaluate(
        () =>
          (window as any).__mockLog.filter((c: any) => c.cmd === 'player_output_devices').length,
      ),
    )
    .toBeGreaterThanOrEqual(3);
});

test('disconnected preference remains visible; enumeration and switch errors recover', async ({
  page,
}) => {
  const select = await settings(page, '?outputDevice=Unplugged');
  await expect(select).toContainText('Unplugged（已断开）');
  await page.evaluate(() => {
    (window as any).__mockFailures = { player_set_output_device: 'fixture output refused' };
  });
  await select.click();
  await page.getByRole('option', { name: 'Headphones', exact: true }).click();
  await expect(page.getByRole('alert').filter({ hasText: 'fixture output refused' })).toBeVisible();
  await expect(select).toContainText('Unplugged（已断开）');
  await expect(select).toBeEnabled();
  await page.evaluate(() => {
    (window as any).__mockFailures = {};
    (window as any).__mockOutputDevices = [];
  });
  await select.click();
  await page.getByRole('option', { name: '跟随系统默认设备', exact: true }).click();
  await select.click();
  await expect(page.getByRole('option')).toHaveCount(1);
  await page.keyboard.press('Escape');
  await expect(select).toBeFocused();
});

test('device enumeration errors leave the default option usable and can recover', async ({
  page,
}) => {
  await page.addInitScript(() => {
    (window as any).__mockFailures = { player_output_devices: 'fixture enumeration failed' };
  });
  const select = await settings(page);
  await expect(
    page.getByRole('alert').filter({ hasText: 'fixture enumeration failed' }),
  ).toBeVisible();
  await expect(select).toBeEnabled();
  await select.click();
  await expect(page.getByRole('option')).toHaveCount(1);
  await page.keyboard.press('Escape');
  await page.evaluate(() => {
    (window as any).__mockFailures = {};
  });
  // Return to the page to re-enumerate freshly attached devices.
  await page.locator('.sidebar').getByRole('button', { name: 'Music Mine 音乐空间' }).click();
  await page.locator('.sidebar').getByRole('button', { name: '设置', exact: true }).click();
  await select.click();
  await expect(page.getByRole('option', { name: 'Headphones', exact: true })).toBeVisible();
});
