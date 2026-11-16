import { expect, test, type Page } from '@playwright/test';

const HARNESS = '/tests/harness/index.html';

async function bootLyrics(page: Page) {
  await page.addInitScript(() => {
    const target = window as Window & {
      __mockLyrics: unknown;
      __lyricsFixture: Record<string, unknown>;
    };
    localStorage.setItem('music-mine-reduce-motion', '1');
    target.__mockLyrics = {
      lines: [
        { time_ms: 0, text: '原文甲\u2009译文甲' },
        { time_ms: 4000, text: '原文乙\u2009译文乙' },
        { time_ms: 8000, text: '原文丙\u2009译文丙' },
      ],
      offset_ms: 0,
      synced: true,
      source: 'fixture.lrc',
    };
    target.__lyricsFixture = {
      position_ms: 4000,
      lyrics_offset_ms: 0,
      playback_state: 'playing',
    };
  });
  await page.goto(`${HARNESS}?window=lyrics`);
}

test('the lyrics query renders only the overlay shell', async ({ page }) => {
  await page.goto(`${HARNESS}?window=lyrics`);
  const overlay = page.getByRole('region', { name: '桌面歌词' });
  await expect(overlay).toBeVisible();
  await expect(page.locator('.sidebar')).toHaveCount(0);
  await expect(page.locator('.player-bar')).toHaveCount(0);
  await expect(page.locator('.queue')).toHaveCount(0);
  await expect(page.locator('body')).toHaveCSS('background-color', 'rgba(0, 0, 0, 0)');

  await page.goto(HARNESS);
  await expect(page.locator('.sidebar')).toBeVisible();
  await expect(page.locator('.desktop-lyrics')).toHaveCount(0);
});

test('bilingual lines follow the same offset as Now Playing', async ({ page }) => {
  await bootLyrics(page);
  await expect(page.locator('.desktop-lyrics')).toHaveClass(/reduce-motion/);
  const current = page.locator('[data-current="true"]');
  await expect(current).toHaveText('原文乙');
  await expect(page.locator('.lyrics-sub')).toHaveText('译文乙');

  await page.evaluate(() => {
    const state = (window as any).__mockSnapshot();
    state.lyrics_offset_ms = 4000;
    state.revision += 1;
  });
  await expect(current).toHaveText('原文甲');
  await expect(page.locator('.lyrics-sub')).toHaveText('译文甲');

  await page.evaluate(() => {
    (window as any).__mockLyrics = null;
    const state = (window as any).__mockSnapshot();
    state.track_id = 'plain';
    state.revision += 1;
    state.current_track = { ...state.current_track, path: 'plain', title: '没有歌词的歌' };
  });
  await expect(page.locator('.lyrics-copy').getByText('没有歌词的歌')).toBeVisible();
  await expect(page.getByText('暂无同步歌词')).toBeVisible();

  await page.evaluate(() => {
    const state = (window as any).__mockSnapshot();
    state.track_id = null;
    state.current_track = null;
    state.playback_state = 'idle';
    state.revision += 1;
  });
  await expect(page.getByText('未在播放')).toBeVisible();
  await expect(page.getByText('暂无同步歌词')).toHaveCount(0);
});

test('overlay controls lock and hide through the desktop command', async ({ page }) => {
  await bootLyrics(page);
  const overlay = page.getByRole('region', { name: '桌面歌词' });
  await overlay.hover();
  await expect(page.getByRole('button', { name: '锁定穿透' })).toBeVisible();
  await page.getByRole('button', { name: '锁定穿透' }).click();
  await page.getByRole('button', { name: '关闭桌面歌词' }).click();
  await expect
    .poll(() =>
      page.evaluate(() =>
        (window as any).__mockLog
          .filter((entry: { cmd: string }) => entry.cmd === 'desktop_lyrics_window')
          .map((entry: { args: { open: boolean | null; locked: boolean | null } }) => entry.args),
      ),
    )
    .toEqual([
      { open: null, locked: true },
      { open: false, locked: null },
    ]);
});

test('compact lyrics fit both window sizes with a floating toolbar in either theme', async ({
  page,
}) => {
  await bootLyrics(page);
  await expect(page.locator('[data-current="true"]')).toHaveText('原文乙');
  for (const theme of ['dark', 'light']) {
    await page.evaluate((theme) => {
      document.documentElement.dataset.theme = theme;
    }, theme);
    for (const size of [
      { width: 640, height: 108 },
      { width: 360, height: 92 },
    ]) {
      await page.setViewportSize(size);
      await page.mouse.move(-10, -10);
      await expect(page.locator('.lyrics-controls')).toHaveCSS('opacity', '0');
      await expect(page.locator('.desktop-lyrics')).toHaveCSS(
        'background-color',
        'rgba(18, 20, 20, 0.8)',
      );
      const centered = await page.locator('.lyrics-copy').boundingBox();
      expect(Math.abs(centered!.y + centered!.height / 2 - size.height / 2)).toBeLessThan(1);
      await page.locator('.desktop-lyrics').hover();
      await expect(page.locator('.lyrics-controls')).toHaveCSS('opacity', '1');
      const shifted = await page.locator('.lyrics-copy').boundingBox();
      expect(shifted!.y - centered!.y).toBeCloseTo(6, 0);
      const translation = await page.locator('.lyric-secondary').boundingBox();
      expect(translation!.y + translation!.height).toBeLessThan(size.height - 5);
      const title = await page.locator('.lyrics-kicker').boundingBox();
      const buttons = await page.locator('.lyrics-buttons').boundingBox();
      expect(
        Math.abs(title!.y + title!.height / 2 - buttons!.y - buttons!.height / 2),
      ).toBeLessThan(1);
      expect(title!.x + title!.width).toBeLessThanOrEqual(buttons!.x);
    }
  }
  await page.mouse.move(-10, -10);
  await page.getByRole('button', { name: '上一首' }).focus();
  await expect(page.locator('.lyrics-controls')).toHaveCSS('opacity', '1');
  await page.evaluate(() => {
    (window as any).__mockLyrics = { lines: [{ time_ms: 0, text: '单行歌词' }], synced: true };
    Object.assign((window as any).__mockSnapshot(), { track_id: 'single', revision: 500 });
  });
  await expect(page.locator('.lyrics-main')).toHaveText('单行歌词');
  await expect(page.locator('.lyrics-main')).toHaveCSS('font-size', '23px');
  await expect(page.locator('.lyric-secondary')).toHaveCount(0);
});

test('a failed lyric read retries on the same track while playback keeps polling', async ({
  page,
}) => {
  await page.addInitScript(() => {
    (window as any).__mockFailures = { player_lyrics: 'temporary lyric read failure' };
  });
  await bootLyrics(page);
  await expect
    .poll(() =>
      page.evaluate(
        () =>
          (window as any).__mockLog.filter((entry: any) => entry.cmd === 'player_get_state').length,
      ),
    )
    .toBeGreaterThan(2);
  await page.evaluate(() => {
    delete (window as any).__mockFailures.player_lyrics;
  });
  await expect(page.locator('[data-current="true"]')).toHaveText('原文乙');
});

test('a pending lyric read cannot stall polling or overwrite the next track', async ({ page }) => {
  await bootLyrics(page);
  await expect(page.locator('[data-current="true"]')).toHaveText('原文乙');
  await page.evaluate(() => {
    const w = window as any;
    const invoke = w.__TAURI_INTERNALS__.invoke;
    w.__TAURI_INTERNALS__.invoke = (cmd: string, args: unknown) => {
      if (cmd === 'player_lyrics' && w.__mockSnapshot().track_id === 'slow') {
        return new Promise((resolve) => {
          w.__resolveSlowLyrics = resolve;
        });
      }
      return invoke(cmd, args);
    };
    Object.assign(w.__mockSnapshot(), { track_id: 'slow', revision: 100 });
  });
  await expect.poll(() => page.evaluate(() => !!(window as any).__resolveSlowLyrics)).toBe(true);
  await page.evaluate(() => {
    const w = window as any;
    w.__mockLyrics = { lines: [{ time_ms: 0, text: '下一首歌词' }], synced: true };
    Object.assign(w.__mockSnapshot(), { track_id: 'next', revision: 101 });
  });
  await expect(page.locator('[data-current="true"]')).toHaveText('下一首歌词');
  await page.evaluate(() =>
    (window as any).__resolveSlowLyrics({
      lines: [{ time_ms: 0, text: '过期歌词' }],
      synced: true,
    }),
  );
  await expect(page.locator('[data-current="true"]')).toHaveText('下一首歌词');
});

test('the player bar and Now Playing toggle and unlock desktop lyrics', async ({ page }) => {
  await page.goto(HARNESS);
  const bar = page.locator('.player-bar');
  const toggle = bar.locator('.desktop-lyrics-toggle');
  await expect(toggle).toHaveAttribute('aria-label', '桌面歌词');
  await expect(toggle).toHaveAttribute('aria-pressed', 'false');
  await toggle.click();
  await expect(toggle).toHaveAttribute('aria-label', '关闭桌面歌词');
  await expect(toggle).toHaveAttribute('aria-pressed', 'true');

  await bar.getByRole('button', { name: '打开正在播放' }).click();
  const nowPlaying = page.getByRole('region', { name: '正在播放' });
  await expect(nowPlaying.locator('.desktop-lyrics-toggle')).toHaveAttribute(
    'aria-label',
    '关闭桌面歌词',
  );
  await nowPlaying.locator('.desktop-lyrics-toggle').click();
  await expect(bar.locator('.desktop-lyrics-toggle')).toHaveAttribute('aria-label', '桌面歌词');
  await expect(bar.locator('.desktop-lyrics-toggle')).toHaveAttribute('aria-pressed', 'false');
});

test('a locked overlay is unlocked from the player bar', async ({ page }) => {
  await page.addInitScript(() => {
    (window as any).__lyricsPrefs = { lyrics_overlay_locked: true, lyrics_overlay_open: true };
  });
  await page.goto(HARNESS);
  const toggle = page.locator('.player-bar').locator('.desktop-lyrics-toggle');
  await expect(toggle).toHaveAttribute('aria-label', '解锁桌面歌词');
  await expect(toggle).toHaveAttribute('aria-pressed', 'true');
  await toggle.click();
  await expect(toggle).toHaveAttribute('aria-label', '关闭桌面歌词');
  await expect(toggle).toHaveAttribute('aria-pressed', 'true');
  await expect
    .poll(() =>
      page.evaluate(() =>
        (window as any).__mockLog.some(
          (entry: { cmd: string; args: { locked: boolean | null } }) =>
            entry.cmd === 'desktop_lyrics_window' && entry.args.locked === false,
        ),
      ),
    )
    .toBe(true);
});
