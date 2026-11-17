import { expect, test } from '@playwright/test';

async function consoleDialog(page: import('@playwright/test').Page) {
  await page.locator('.sidebar').getByRole('button', { name: '控制台', exact: true }).click();
  return page.getByRole('dialog', { name: '运行控制台 (Console)' });
}
test('AI settings default off, save masked credentials and clear them without changing existing settings', async ({
  page,
}) => {
  await page.goto('/tests/harness/index.html');
  await page.locator('.sidebar').getByRole('button', { name: '设置', exact: true }).click();
  const card = page.getByRole('region', { name: 'AI 助手', exact: true });
  const toggle = card.getByRole('switch', { name: '启用 AI 助手' });
  await expect(toggle).not.toBeChecked();
  await expect(card.getByLabel('Base URL', { exact: true })).toHaveValue(
    'https://api.openai.com/v1',
  );
  await card.getByLabel('Base URL', { exact: true }).fill('file:///secret');
  await card.getByRole('button', { name: '保存设置', exact: true }).click();
  await expect(card.getByRole('status')).toContainText('AI_URL_INVALID');
  await card.getByLabel('Base URL', { exact: true }).fill('http://localhost:11434/v1');
  await card.getByLabel('模型名称', { exact: true }).fill('fixture-model');
  const key = card.getByLabel('API Key', { exact: true });
  await expect(key).toHaveAttribute('type', 'password');
  await key.fill('fixture-secret-key');
  await card.getByRole('button', { name: '保存设置', exact: true }).click();
  await expect(card.getByRole('status')).toContainText('已保存');
  await expect(key).toHaveValue('');
  await expect(key).toHaveAttribute('placeholder', /已保存/);
  await toggle.focus();
  await toggle.press('Space');
  await expect(toggle).toBeChecked();
  for (const width of [1280, 900]) {
    await page.setViewportSize({ width, height: width === 900 ? 640 : 800 });
    await key.scrollIntoViewIfNeeded();
    const b = await key.boundingBox();
    expect(b!.x + b!.width).toBeLessThanOrEqual(width);
  }
  await card.screenshot({ path: 'test-results/ai-settings-narrow.png' });
  const logged = await page.evaluate(() => JSON.stringify((window as any).__mockLog));
  expect(logged).not.toContain('fixture-secret-key');
  await card.getByRole('button', { name: '清除密钥', exact: true }).click();
  await expect(toggle).not.toBeChecked();
  await expect(key).not.toHaveAttribute('placeholder', /已保存/);
});
test('local commands remain local and AI-disabled natural language guides users to settings', async ({
  page,
}) => {
  await page.goto('/tests/harness/index.html');
  const dialog = await consoleDialog(page);
  const input = dialog.locator('#console-command');
  await input.fill('vol 30');
  await input.press('Enter');
  await expect(page.locator('.pb-volume-value')).toHaveText('30%');
  await input.fill('vol nope');
  await input.press('Enter');
  await expect(dialog.getByRole('status')).toContainText('音量用法');
  await input.fill('帮我找一些轻音乐');
  await input.press('Enter');
  await expect(dialog.getByRole('status')).toContainText('设置');
  const count = await page.evaluate(
    () => (window as any).__mockLog.filter((x: any) => x.cmd === 'ai_chat_turn').length,
  );
  expect(count).toBe(0);
});
test('Console shows natural question, executed tool results and the final reply', async ({
  page,
}) => {
  await page.addInitScript(() => {
    (window as any).__mockAiConfig = { enabled: true, has_key: true };
    (window as any).__mockAiTurns = [
      {
        message: {
          role: 'assistant',
          content: null,
          tool_calls: [
            {
              id: 'call_1',
              type: 'function',
              function: { name: 'control_playback', arguments: '{"action":"volume","volume":30}' },
            },
            {
              id: 'call_2',
              type: 'function',
              function: { name: 'set_appearance', arguments: '{"palette":"amber","font":"serif"}' },
            },
          ],
        },
        finish_reason: 'tool_calls',
      },
      {
        message: { role: 'assistant', content: '已将音量设为30%，切换琥珀配色和宋体。' },
        finish_reason: 'stop',
      },
    ];
  });
  await page.goto('/tests/harness/index.html');
  const dialog = await consoleDialog(page);
  const input = dialog.locator('#console-command');
  await input.fill('请把音量调到30，并换琥珀配色和宋体');
  await input.press('Enter');
  const chat = dialog.getByRole('log', { name: 'AI 对话' });
  await expect(chat).toContainText('请把音量');
  await expect(chat).toContainText('control_playback');
  await expect(chat).toContainText('"ok":true');
  await expect(chat).toContainText('已将音量设为30%');
  await expect(page.locator('.pb-volume-value')).toHaveText('30%');
  await expect(page.locator('html')).toHaveAttribute('data-palette', 'amber');
  await expect(page.locator('html')).toHaveAttribute('data-font', 'serif');
  await page.setViewportSize({ width: 900, height: 640 });
  await chat.screenshot({ path: 'test-results/ai-chat-narrow.png' });
  await dialog.getByRole('button', { name: '运行日志', exact: true }).click();
  await expect(dialog.getByRole('region', { name: '运行日志' })).toBeVisible();
  await dialog.getByRole('button', { name: 'AI 对话', exact: true }).click();
  await dialog.getByRole('button', { name: '清空对话', exact: true }).click();
  await expect(chat).not.toContainText('请把音量');
});
test('closing Console during a pending AI response prevents later tool execution', async ({
  page,
}) => {
  await page.addInitScript(() => {
    (window as any).__mockAiConfig = { enabled: true, has_key: true };
    (window as any).__mockAiDelay = 500;
    (window as any).__mockAiTurns = [
      {
        message: {
          role: 'assistant',
          content: null,
          tool_calls: [
            {
              id: 'late',
              type: 'function',
              function: { name: 'control_playback', arguments: '{"action":"volume","volume":5}' },
            },
          ],
        },
      },
    ];
  });
  await page.goto('/tests/harness/index.html');
  const dialog = await consoleDialog(page);
  const input = dialog.locator('#console-command');
  await input.fill('稍后调小音量');
  await input.press('Enter');
  await expect
    .poll(() =>
      page.evaluate(
        () => (window as any).__mockLog.filter((x: any) => x.cmd === 'ai_chat_turn').length,
      ),
    )
    .toBe(1);
  await dialog.getByRole('button', { name: '关闭运行控制台', exact: true }).click();
  await page.waitForTimeout(650);
  await expect(page.locator('.pb-volume-value')).toHaveText('70%');
});
test('a successful playlist remains in model context after a later network failure', async ({
  page,
}) => {
  await page.addInitScript(() => {
    (window as any).__mockAiConfig = { enabled: true, has_key: true };
    (window as any).__mockAiTurns = [
      {
        message: {
          role: 'assistant',
          content: null,
          tool_calls: [
            {
              id: 'created',
              type: 'function',
              function: {
                name: 'create_playlist_with_tracks',
                arguments: '{"name":"AI 回归歌单","ids":[1]}',
              },
            },
          ],
        },
      },
      { error: 'AI_NETWORK' },
      { message: { role: 'assistant', content: '歌单已创建，无需重复创建。' } },
    ];
  });
  await page.goto('/tests/harness/index.html');
  const dialog = await consoleDialog(page);
  const input = dialog.locator('#console-command');
  await input.fill('用第一首歌创建歌单');
  await input.press('Enter');
  await expect(dialog.getByRole('status')).toContainText('AI_NETWORK');
  await expect(dialog.getByRole('log', { name: 'AI 对话' })).toContainText('"added":1');
  await input.fill('继续刚才的操作');
  await input.press('Enter');
  await expect(dialog.getByRole('log', { name: 'AI 对话' })).toContainText('无需重复创建');
  expect(await page.evaluate(() => (window as any).__mockAiLastToolIds)).toEqual(['created']);
  await page.keyboard.press('Escape');
  await expect(page.locator('.sidebar').getByRole('button', { name: 'AI 回归歌单' })).toHaveCount(
    1,
  );
});
