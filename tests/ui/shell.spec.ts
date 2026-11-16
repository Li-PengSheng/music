import { expect, test, type Locator, type Page } from '@playwright/test';

const HARNESS = '/tests/harness/index.html';

async function openApp(page: Page) {
  await page.addInitScript(() => {
    const fixed = Date.parse('2026-11-14T15:30:00+08:00');
    const Original = Date;
    function FakeDate(this: Date, ...args: ConstructorParameters<typeof Date>) {
      if (args.length === 0) return new Original(fixed);
      return new Original(...args);
    }
    FakeDate.prototype = Original.prototype;
    Object.assign(FakeDate, { now: () => fixed, parse: Original.parse, UTC: Original.UTC });
    window.Date = FakeDate as DateConstructor;
  });
  await page.goto(HARNESS);
  await expect(page.getByRole('button', { name: 'Music Mine 音乐空间' })).toBeVisible();
  await expect(page.locator('.pb-volume-value')).toHaveText('70%');
  await expect(page.getByRole('alert')).toHaveCount(0);
  await expect(page.getByRole('heading', { name: '下午好' })).toBeVisible();
}

function side(page: Page) {
  return page.locator('.sidebar');
}

function playlists(page: Page) {
  return page.getByRole('list', { name: '我的歌单' });
}

function mainHeading(page: Page, name: string) {
  return page.locator('#main-content').getByRole('heading', { name, exact: true });
}

async function openQueue(page: Page) {
  await page.locator('.player-bar').getByRole('button', { name: '播放队列' }).click();
  const queue = page.getByRole('complementary', { name: '播放队列' });
  await expect(queue).toBeVisible();
  await page.waitForFunction(() => {
    const el = document.querySelector('.queue');
    if (!el) return false;
    const style = getComputedStyle(el);
    return style.visibility === 'visible' && Number(style.opacity) === 1;
  });
  return queue;
}

async function tokenColor(locator: Locator, name: string) {
  return locator.evaluate((el, token) => {
    const probe = document.createElement('div');
    probe.style.background = `var(${token})`;
    el.append(probe);
    const color = getComputedStyle(probe).backgroundColor;
    probe.remove();
    return color;
  }, name);
}

test.beforeEach(async ({ page }) => {
  await openApp(page);
});

test('sidebar opens the main pages', async ({ page }) => {
  await expect(page).toHaveScreenshot('home-dark.png');

  await side(page).getByRole('button', { name: /^歌曲/ }).click();
  await expect(mainHeading(page, '歌曲')).toBeVisible();
  await expect(page.locator('.track-row').first()).toBeVisible();
  await expect(side(page).getByRole('button', { name: /^歌曲/ })).toHaveAttribute(
    'aria-current',
    'page',
  );
  await expect(page).toHaveScreenshot('songs-dark.png');

  const main = page.locator('.main');
  const head = page.locator('.track-head');
  await main.evaluate((el) => {
    el.scrollTop = 900;
  });
  const headBox = await head.boundingBox();
  const mainBox = await main.boundingBox();
  expect(headBox).toBeTruthy();
  expect(mainBox).toBeTruthy();
  expect(headBox!.y).toBeGreaterThanOrEqual(mainBox!.y - 1);
  expect(headBox!.y).toBeLessThan(mainBox!.y + 20);

  await side(page).getByRole('button', { name: /^专辑/ }).click();
  await expect(mainHeading(page, '专辑')).toBeVisible();
  await expect(page.locator('.card').first()).toBeVisible();
  await expect(page).toHaveScreenshot('albums-dark.png');

  await side(page).getByRole('button', { name: '设置' }).click();
  await expect(mainHeading(page, '设置')).toBeVisible();
  await expect(page.getByRole('heading', { name: '外观' })).toBeVisible();
  await expect(page).toHaveScreenshot('settings-dark.png');

  await page.getByRole('button', { name: 'Music Mine 音乐空间' }).click();
  await expect(page.getByRole('heading', { name: '下午好' })).toBeVisible();
});

test('player bar shows the real volume and does not invent progress', async ({ page }) => {
  const bar = page.locator('.player-bar');
  await expect(bar.getByRole('button', { name: '测试歌曲 1' })).toBeVisible();
  await expect(bar.locator('.pb-time').first()).toHaveText('0:42');
  const slider = bar.getByRole('slider', { name: '音量' });
  await expect(slider).toBeEnabled();
  await expect(slider).toHaveAttribute('aria-valuetext', '70%');

  await slider.fill('15');
  await expect(page.locator('.pb-volume-value')).toHaveText('15%');
  await expect(slider).toHaveAttribute('aria-valuetext', '15%');

  await bar.locator('.pb-volume').getByRole('button').click();
  await expect(page.locator('.pb-volume-value')).toHaveText('0%');
  await expect(slider).toHaveAttribute('aria-valuetext', '0%，已静音');
  await bar.locator('.pb-volume').getByRole('button').click();
  await expect(page.locator('.pb-volume-value')).toHaveText('15%');

  await bar.getByRole('button', { name: '播放', exact: true }).click();
  await page.waitForTimeout(500);
  await expect(bar.locator('.pb-time').first()).toHaveText('0:42');
  await expect(bar.getByRole('button', { name: '播放', exact: true })).toBeVisible();
});

test('queue overlays the page and closes from the button, Escape and the scrim', async ({
  page,
}) => {
  const main = page.locator('.main');
  const before = await main.evaluate((el) => el.getBoundingClientRect().width);
  const queue = await openQueue(page);
  const after = await main.evaluate((el) => el.getBoundingClientRect().width);
  expect(after).toBe(before);
  await expect(queue).toContainText('测试歌曲 1');
  const transition = await page.locator('.queue').evaluate((el) =>
    getComputedStyle(el)
      .transitionProperty.split(',')
      .map((part) => part.trim())
      .sort(),
  );
  expect(transition).toEqual(['opacity', 'transform', 'visibility']);
  await expect(page).toHaveScreenshot('queue-dark.png');

  await queue.getByRole('button', { name: '关闭队列 (Esc)' }).click();
  await expect(page.locator('.app')).not.toHaveClass(/queue-open/);

  await openQueue(page);
  await page.keyboard.press('Escape');
  await expect(page.locator('.app')).not.toHaveClass(/queue-open/);

  await page.setViewportSize({ width: 960, height: 700 });
  await openQueue(page);
  await expect(page.locator('.queue-scrim')).toBeVisible();
  await page.locator('.queue-scrim').click();
  await expect(page.locator('.app')).not.toHaveClass(/queue-open/);
});

test('theme switch and a narrower window keep the shell usable', async ({ page }) => {
  await side(page).getByRole('button', { name: '设置' }).click();
  await page.getByRole('button', { name: '主题' }).click();
  await page.getByRole('option', { name: '浅色' }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
  await expect(page.locator('.pb-volume-value')).toHaveText('70%');
  await page.getByRole('button', { name: 'Music Mine 音乐空间' }).click();
  await expect(page).toHaveScreenshot('home-light.png');

  await side(page).getByRole('button', { name: '设置' }).click();
  await page.getByRole('button', { name: '主题' }).click();
  await page.getByRole('option', { name: '深色' }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');

  await page.setViewportSize({ width: 960, height: 700 });
  await expect(page.locator('.pb-volume-value')).toBeHidden();
  await expect(page.locator('.pb-volume').getByRole('button')).toBeVisible();
  await side(page).getByRole('button', { name: /^歌曲/ }).click();
  await expect(page.locator('.track-row').first()).toBeVisible();
  await expect(page.locator('.player-bar')).toBeVisible();
  await expect(page).toHaveScreenshot('songs-narrow.png');

  await page.setViewportSize({ width: 1280, height: 800 });
  await expect(page.locator('.pb-volume-value')).toBeVisible();
});

test('dialogs cancel without changing playlists and confirm a real create', async ({ page }) => {
  const lists = playlists(page);
  await page.getByRole('button', { name: '新建歌单' }).click();
  const dialog = page.getByRole('dialog');
  await expect(dialog.getByRole('heading', { name: '新建歌单' })).toBeVisible();
  const box = await dialog.boundingBox();
  expect(box).toBeTruthy();
  expect(Math.abs(box!.x + box!.width / 2 - 640)).toBeLessThan(2);
  await expect(page).toHaveScreenshot('dialog-dark.png');
  await page.keyboard.press('Escape');
  await expect(dialog).toHaveCount(0);
  await expect(page.getByRole('button', { name: '新建歌单' })).toBeFocused();
  await expect(lists.getByRole('button', { name: /雨天/ })).toHaveCount(0);

  await page.getByRole('button', { name: '新建歌单' }).click();
  await page.getByRole('textbox', { name: '歌单名称' }).fill('雨天');
  await page.getByRole('button', { name: '创建' }).click();
  await expect(lists.getByRole('button', { name: /雨天/ })).toBeVisible();

  await lists.getByRole('button', { name: /夜晚/ }).click({ button: 'right' });
  await page.getByRole('menuitem', { name: '删除歌单…' }).click();
  await expect(page.getByRole('dialog')).toContainText('不删除本地音乐文件');
  await page.getByRole('button', { name: '取消' }).click();
  await expect(lists.getByRole('button', { name: /夜晚/ })).toBeVisible();
});

test('right-click menus differ for songs, the built-in collection, a playlist, an album and the queue', async ({
  page,
}) => {
  await side(page).getByRole('button', { name: /^歌曲/ }).click();
  await page.locator('.track-row', { hasText: '测试歌曲 3' }).click({ button: 'right' });
  await expect(page.getByRole('menuitem', { name: '播放', exact: true })).toBeVisible();
  await page.keyboard.press('Escape');

  const lists = playlists(page);
  await lists
    .getByRole('button', { name: /本地/ })
    .filter({ hasNotText: '自建' })
    .click({ button: 'right' });
  await expect(page.getByRole('menuitem', { name: '播放全部' })).toBeVisible();
  await expect(page.getByRole('menuitem', { name: '随机播放' })).toBeVisible();
  await expect(page.getByRole('menuitem', { name: /重命名|删除/ })).toHaveCount(0);
  await page.keyboard.press('Escape');

  await lists.getByRole('button', { name: /夜晚/ }).click({ button: 'right' });
  await expect(page.getByRole('menuitem', { name: '重命名…' })).toBeVisible();
  await page.keyboard.press('Escape');

  await side(page).getByRole('button', { name: /^专辑/ }).click();
  await page.locator('.card').first().click({ button: 'right' });
  await expect(page.getByRole('menu')).toBeVisible();
  await page.keyboard.press('Escape');

  await openQueue(page);
  await page.locator('.queue-item').nth(2).click({ button: 'right' });
  await expect(page.getByRole('menuitem', { name: '从队列移除' })).toBeVisible();
});

test('hover and selection use the semantic fills, and the queue stacks above Now Playing', async ({
  page,
}) => {
  const songs = side(page).getByRole('button', { name: /^歌曲/ });
  await songs.hover();
  await expect(songs).toHaveCSS('background-color', await tokenColor(songs, '--hover'));
  await songs.click();
  await expect(songs).toHaveCSS('background-color', await tokenColor(songs, '--selected'));

  const row = page.locator('.track-row').nth(1);
  await row.hover();
  await expect(row).toHaveCSS('background-color', await tokenColor(row, '--hover'));
  await row.click();
  await expect(row).toHaveCSS('background-color', await tokenColor(row, '--selected'));

  const border = await page.locator('.search').evaluate((el) => {
    const style = getComputedStyle(el);
    return `${style.borderTopWidth} ${style.borderTopStyle}`;
  });
  expect(border).toBe('1px solid');

  await page.locator('.player-bar').getByRole('button', { name: '打开正在播放' }).click();
  await expect(page.getByRole('region', { name: '正在播放' })).toBeVisible();
  await expect(page).toHaveScreenshot('now-playing-dark.png');
  const queue = await openQueue(page);
  const item = queue.locator('.queue-item').first();
  await expect(item).toHaveCSS('background-color', await tokenColor(item, '--selected'));

  const layers = await page.evaluate(() => {
    const read = (selector: string) =>
      Number(getComputedStyle(document.querySelector(selector)!).zIndex);
    return {
      queue: read('.queue'),
      scrim: read('.queue-scrim'),
      immersive: read('.immersive-layer'),
      player: read('.player-bar'),
    };
  });
  expect(layers.queue).toBeGreaterThan(layers.scrim);
  expect(layers.scrim).toBeGreaterThan(layers.immersive);
  expect(layers.immersive).toBeGreaterThan(layers.player);

  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.reload();
  await expect(page.locator('.pb-volume-value')).toHaveText('70%');
  await openQueue(page);
  await expect
    .poll(() =>
      page
        .locator('.queue')
        .evaluate((el) => getComputedStyle(el).getPropertyValue('--drawer-shift').trim()),
    )
    .toBe('0px');
});
