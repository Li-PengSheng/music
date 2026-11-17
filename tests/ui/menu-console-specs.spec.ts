import { expect, test } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  await page.goto('/tests/harness/index.html');
  await expect(page.locator('.pb-volume-value')).toHaveText('70%');
});

test('sidebar opens Console directly and commands apply authoritative snapshots', async ({
  page,
}) => {
  const trigger = page.locator('.sidebar').getByRole('button', { name: '控制台', exact: true });
  await expect(trigger).toBeVisible({ timeout: 2000 });
  await expect(trigger).not.toHaveClass(/active/);
  await trigger.click();
  const dialog = page.getByRole('dialog', { name: '运行控制台 (Console)' });
  await expect(dialog).toBeVisible();
  await expect(page.getByRole('menu')).toHaveCount(0);
  await expect(trigger).toHaveClass(/active/);
  await expect(trigger).toHaveAttribute('aria-expanded', 'true');
  await expect(dialog).toContainText('Fixture output');
  await expect(dialog).toContainText('471 首');
  await expect(dialog).toContainText('内存 8 · 磁盘 2 / 请求 12');
  const input = dialog.getByRole('textbox', { name: '快捷命令' });
  await input.fill('vol 30');
  await input.press('Enter');
  await expect(dialog.getByRole('status')).toContainText('30%');
  await expect(page.locator('.pb-volume-value')).toHaveText('30%');
  for (const [command, ipc] of [
    ['play', 'player_play'],
    ['pause', 'player_pause'],
    ['next', 'library_queue'],
    ['prev', 'library_queue'],
    ['scan', 'library_scan'],
  ]) {
    await input.fill(command);
    await input.press('Enter');
    await expect
      .poll(() =>
        page.evaluate(
          (name) => (window as any).__mockLog.filter((item: any) => item.cmd === name).length,
          ipc,
        ),
      )
      .toBeGreaterThan(0);
    await expect(dialog.getByRole('button', { name: '执行', exact: true })).toBeDisabled();
  }
  await input.fill('palette cinnabar');
  await input.press('Enter');
  await expect(page.locator('html')).toHaveAttribute('data-palette', 'cinnabar');
  await input.fill('theme dark');
  await input.press('Enter');
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
  await input.fill('vol 101');
  await input.press('Enter');
  await expect(dialog.getByRole('status')).toContainText('0–100');
  await expect(page.locator('.pb-volume-value')).toHaveText('30%');
  await dialog.getByRole('button', { name: '清空日志' }).click();
  await expect(dialog.getByRole('region', { name: '运行日志' })).toContainText('暂无运行日志');
  await page.keyboard.press('Escape');
  await expect(dialog).toHaveCount(0);
  await expect(trigger).toBeFocused();
  await expect(trigger).not.toHaveClass(/active/);
  await expect(trigger).toHaveAttribute('aria-expanded', 'false');
});

test('local status opens Console, errors stay visible and refresh stops on close', async ({
  page,
  context,
}) => {
  await context.grantPermissions(['clipboard-read', 'clipboard-write']);
  const trigger = page.getByRole('button', { name: '本地模式 · 运行控制台' });
  await trigger.click();
  const dialog = page.getByRole('dialog');
  const logs = dialog.getByRole('region', { name: '运行日志' });
  await expect(logs).toContainText('[audio] fixture paused');
  await dialog.getByRole('button', { name: '复制摘要' }).click();
  await expect(dialog.getByRole('status')).toHaveText('已复制诊断摘要与脱敏日志');
  expect(await page.evaluate(() => navigator.clipboard.readText())).toContain('Fixture output');
  await page.evaluate(() => {
    (window as any).__mockFailures = {
      app_console_logs: '日志读取失败',
      player_set_volume: '输出设备不可用',
    };
  });
  await dialog.getByRole('button', { name: '刷新', exact: true }).click();
  await expect(dialog).toContainText('日志：日志读取失败');
  await expect(logs).toContainText('[audio] fixture paused');
  const input = dialog.getByRole('textbox', { name: '快捷命令' });
  await input.fill('vol 30');
  await input.press('Enter');
  await expect(dialog.getByRole('status')).toContainText('命令未完成');
  await expect(page.locator('.pb-volume-value')).toHaveText('70%');
  await input.fill('anything');
  await input.press('Enter');
  await expect(dialog.getByRole('status')).toContainText('可用指令');
  await page.keyboard.press('Escape');
  await expect(trigger).toBeFocused();
  const count = () =>
    page.evaluate(
      () => (window as any).__mockLog.filter((item: any) => item.cmd === 'app_console_logs').length,
    );
  const before = await count();
  await page.waitForTimeout(2200);
  expect(await count()).toBe(before);
});

test('audio properties appear in details and Now Playing without repeated progress requests', async ({
  page,
}) => {
  await page.locator('.sidebar').getByRole('button', { name: /^歌曲/ }).click();
  await page.locator('.track-row').first().click({ button: 'right' });
  await page.getByRole('menuitem', { name: /歌曲详情/ }).click();
  const details = page.getByRole('dialog');
  await expect(details).toContainText('16 bit');
  await expect(details).toContainText('27.8 MB');
  await expect(details).not.toContainText('播放次数');
  await page.keyboard.press('Escape');
  await page
    .locator('.player-bar')
    .getByRole('button', { name: '打开正在播放', exact: true })
    .click();
  const specs = page.getByLabel('专业音频参数');
  await expect(specs).toContainText('FLAC');
  await expect(specs).toContainText('2.0 立体声');
  await expect(specs).toContainText('16 bit');
  await expect(specs).toContainText('872 kbps');
  await expect(specs).toContainText('27.8 MB');
  await expect(specs).not.toContainText(/播放|次/);
  const count = () =>
    page.evaluate(
      () =>
        (window as any).__mockLog.filter((item: any) => item.cmd === 'library_audio_info').length,
    );
  const before = await count();
  await page.waitForTimeout(650);
  expect(await count()).toBe(before);
  await page.evaluate(() => {
    const state = (window as any).__mockSnapshot();
    state.track_id = 'C:\\outside.flac';
    state.current_track = { ...state.current_track, path: state.track_id };
    state.revision++;
  });
  await expect(specs).toHaveCount(0);
});

test('minimal top bar and sidebar Console fit both sizes with keyboard focus restored', async ({
  page,
}, testInfo) => {
  for (const size of [
    { width: 1280, height: 800 },
    { width: 900, height: 640 },
  ]) {
    await page.setViewportSize(size);
    const trigger = page
      .locator('.sidebar-footer')
      .getByRole('button', { name: '控制台', exact: true });
    await expect(
      page.locator('.topbar').getByRole('button', { name: '菜单', exact: true }),
    ).toHaveCount(0);
    for (const selector of ['.search', '.local-status', '.topbar .window-controls']) {
      const bounds = await page.locator(selector).boundingBox();
      expect(bounds).toBeTruthy();
      expect(bounds!.x).toBeGreaterThanOrEqual(0);
      expect(bounds!.x + bounds!.width).toBeLessThanOrEqual(size.width);
    }
    const search = await page.locator('.search').boundingBox();
    const status = await page.locator('.local-status').boundingBox();
    const controls = await page.locator('.topbar .window-controls').boundingBox();
    expect(search!.x + search!.width).toBeLessThanOrEqual(status!.x);
    expect(status!.x + status!.width).toBeLessThanOrEqual(controls!.x);
    await expect(trigger).toBeInViewport({ ratio: 1 });
    const settings = page
      .locator('.sidebar-footer')
      .getByRole('button', { name: '设置', exact: true });
    await expect(settings).toBeInViewport({ ratio: 1 });
    const consoleBounds = await trigger.boundingBox();
    const settingsBounds = await settings.boundingBox();
    expect(consoleBounds!.y + consoleBounds!.height).toBeLessThanOrEqual(settingsBounds!.y);
    await page.screenshot({
      path: testInfo.outputPath(`tools-${size.width}.png`),
      animations: 'disabled',
    });
    await trigger.focus();
    await page.keyboard.press('Enter');
    const dialog = page.getByRole('dialog');
    await expect(dialog).toBeVisible();
    await expect(dialog.getByRole('textbox', { name: '快捷命令' })).toBeInViewport();
    await page.screenshot({
      path: testInfo.outputPath(`console-${size.width}.png`),
      animations: 'disabled',
    });
    await page.keyboard.press('Escape');
    await expect(trigger).toBeFocused();
    await expect(trigger).not.toHaveClass(/active/);
  }
});

test('track changes reject stale audio properties and omit unknown values', async ({ page }) => {
  await page.evaluate(() => {
    (window as any).__mockAudioInfo = {
      1: {
        delay: 850,
        info: {
          format: 'OLD',
          sample_rate: 44100,
          channels: 2,
          bitrate: 872,
          bit_depth: 16,
          file_size: 27800000,
        },
      },
      2: {
        info: {
          format: 'WAV',
          sample_rate: 96000,
          channels: 1,
          bitrate: null,
          bit_depth: 24,
          file_size: 3000000,
        },
      },
    };
  });
  await page
    .locator('.player-bar')
    .getByRole('button', { name: '打开正在播放', exact: true })
    .click();
  await expect
    .poll(() =>
      page.evaluate(() =>
        (window as any).__mockLog.some(
          (item: any) => item.cmd === 'library_audio_info' && item.args.id === 1,
        ),
      ),
    )
    .toBe(true);
  const switchTrack = (index: number) =>
    page.evaluate((i) => {
      const state = (window as any).__mockSnapshot();
      state.current_track = state.queue[i];
      state.track_id = state.current_track.path;
      state.revision++;
    }, index);
  await switchTrack(1);
  const specs = page.getByLabel('专业音频参数');
  await expect(specs).toContainText('WAV');
  await expect(specs).toContainText('24 bit');
  await expect(specs).toContainText('1.0 单声道');
  await expect(specs).not.toContainText('kbps');
  await page.waitForTimeout(950);
  await expect(specs).not.toContainText('OLD');
  await page.evaluate(() => {
    (window as any).__mockAudioInfo[1] = {
      info: {
        format: 'MP3',
        sample_rate: null,
        channels: null,
        bitrate: null,
        bit_depth: null,
        file_size: null,
      },
    };
  });
  await switchTrack(0);
  await expect(specs).toContainText('MP3');
  await expect(specs).not.toContainText('bit');
  await expect(specs).not.toContainText('MB');
  await page.evaluate(() => {
    (window as any).__mockFailures = { library_audio_info: '无法读取音频参数' };
  });
  await switchTrack(1);
  await expect(specs).toHaveCount(0);
  await expect(page.getByRole('alert')).toHaveCount(0);
});

test('sidebar Console preserves the current page and shares active state with the local status shortcut', async ({
  page,
}) => {
  const sidebar = page.locator('.sidebar');
  const trigger = sidebar.getByRole('button', { name: '控制台', exact: true });
  await sidebar.getByRole('button', { name: '设置', exact: true }).click();
  await expect(
    page.locator('#main-content').getByRole('heading', { name: '设置', exact: true }),
  ).toBeVisible();
  await page.getByRole('button', { name: '本地模式 · 运行控制台' }).click();
  await expect(page.getByRole('dialog')).toBeVisible();
  await expect(trigger).toHaveClass(/active/);
  await page.keyboard.press('Escape');
  await expect(trigger).not.toHaveClass(/active/);
  await trigger.click();
  await expect(page.getByRole('dialog')).toBeVisible();
  await page.keyboard.press('Escape');
  await expect(trigger).toBeFocused();
  await expect(
    page.locator('#main-content').getByRole('heading', { name: '设置', exact: true }),
  ).toBeVisible();
});
