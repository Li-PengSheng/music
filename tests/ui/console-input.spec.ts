import { expect, test, type Page } from '@playwright/test';

async function openConsole(page: Page) {
  await page.goto('/tests/harness/index.html');
  await page.locator('.sidebar').getByRole('button', { name: '控制台', exact: true }).click();
  return page.getByRole('dialog', { name: '运行控制台 (Console)' });
}

test('input modes have distinct labels, fonts, placeholders, badges and button styles in both themes and sizes', async ({
  page,
}) => {
  const model = 'deepseek-chat-' + '很长的本地模型名称'.repeat(15);
  await page.addInitScript((model) => {
    (window as any).__mockAiConfig = { enabled: true, has_key: true, model };
  }, model);
  const dialog = await openConsole(page);
  for (const theme of ['light', 'dark']) {
    await page.evaluate((theme) => {
      document.documentElement.dataset.theme = theme;
    }, theme);
    for (const width of [1280, 900]) {
      await page.setViewportSize({ width, height: width === 900 ? 640 : 800 });
      await dialog.getByRole('button', { name: '运行日志', exact: true }).click();
      const local = dialog.getByRole('textbox', { name: '本地短命令', exact: true });
      await expect(local).toHaveAttribute(
        'placeholder',
        'play · pause · next · prev · vol 30 · theme dark · scan',
      );
      await expect(local).toHaveClass(/mono/);
      await expect(dialog).toContainText('仅执行内置控制指令');
      await expect(dialog.getByRole('button', { name: '执行', exact: true })).toHaveClass(
        /btn-secondary/,
      );
      const mono = await local.evaluate((el) => getComputedStyle(el).fontFamily);
      await dialog.getByRole('button', { name: 'AI 对话', exact: true }).click();
      const ai = dialog.getByRole('textbox', { name: 'AI 对话指令', exact: true });
      await expect(ai).toHaveAttribute(
        'placeholder',
        '直接输入需求，例如：“找几首适合深夜的歌建个歌单并播放”',
      );
      await expect(ai).not.toHaveClass(/mono/);
      expect(await ai.evaluate((el) => getComputedStyle(el).fontFamily)).not.toBe(mono);
      await expect(dialog.getByRole('button', { name: '发送', exact: true })).toHaveClass(
        /btn-primary/,
      );
      await expect(dialog.locator('.console-model')).toHaveText(model);
      await expect(dialog.locator('.console-model')).toHaveAttribute('title', model);
      const expectedBorder = await ai.evaluate((el) => {
        const probe = document.createElement('span');
        probe.style.color = 'var(--accent-line)';
        el.parentElement!.append(probe);
        const value = getComputedStyle(probe).color;
        probe.remove();
        return value;
      });
      await ai.blur();
      await expect(ai).toHaveCSS('border-color', expectedBorder);
      for (const target of [
        ai,
        dialog.locator('.console-model'),
        dialog.getByRole('button', { name: '发送', exact: true }),
      ]) {
        await target.scrollIntoViewIfNeeded();
        const box = await target.boundingBox();
        expect(box!.x).toBeGreaterThanOrEqual(0);
        expect(box!.x + box!.width).toBeLessThanOrEqual(width);
      }
    }
  }
  await dialog
    .locator('.console-command')
    .screenshot({ path: 'test-results/console-input-ai-narrow.png' });
});

test('natural language in logs switches to AI, shows sending state and completes tool execution', async ({
  page,
}) => {
  await page.addInitScript(() => {
    (window as any).__mockAiConfig = { enabled: true, has_key: true, model: 'fixture-chat' };
    (window as any).__mockAiDelay = 500;
    (window as any).__mockAiTurns = [
      {
        message: {
          role: 'assistant',
          content: null,
          tool_calls: [
            {
              id: 'vol',
              type: 'function',
              function: { name: 'control_playback', arguments: '{"action":"volume","volume":40}' },
            },
          ],
        },
      },
      { message: { role: 'assistant', content: '音量已设置为40%。' } },
    ];
  });
  const dialog = await openConsole(page);
  await dialog.getByRole('textbox', { name: '本地短命令', exact: true }).fill('把音量调到40');
  await dialog.getByRole('button', { name: '执行', exact: true }).click();
  await expect(dialog.getByRole('button', { name: 'AI 对话', exact: true })).toHaveAttribute(
    'aria-pressed',
    'true',
  );
  await expect(dialog.getByRole('button', { name: '发送中…', exact: true })).toBeDisabled();
  await expect(dialog.getByRole('textbox', { name: 'AI 对话指令', exact: true })).toBeVisible();
  await expect(dialog.getByRole('log', { name: 'AI 对话' })).toContainText('音量已设置为40%');
  await expect(page.locator('.pb-volume-value')).toHaveText('40%');
  await expect(dialog.getByRole('button', { name: '发送', exact: true })).toBeDisabled();
});

test('disabled AI is explicit and only logs dispatches built-in commands locally', async ({
  page,
}) => {
  const dialog = await openConsole(page);
  await dialog.getByRole('textbox', { name: '本地短命令', exact: true }).fill('你好');
  await dialog.getByRole('button', { name: '执行', exact: true }).click();
  await expect(dialog.locator('.console-model')).toHaveText('未启用');
  await expect(dialog.getByRole('status')).toContainText('设置');
  await dialog.getByRole('textbox', { name: 'AI 对话指令', exact: true }).fill('vol 30');
  await dialog.getByRole('button', { name: '发送', exact: true }).click();
  await expect(page.locator('.pb-volume-value')).toHaveText('70%');
  await dialog.getByRole('button', { name: '运行日志', exact: true }).click();
  await dialog.getByRole('button', { name: '执行', exact: true }).click();
  await expect(page.locator('.pb-volume-value')).toHaveText('30%');
  expect(
    await page.evaluate(
      () => (window as any).__mockLog.filter((x: any) => x.cmd === 'ai_chat_turn').length,
    ),
  ).toBe(0);
});
