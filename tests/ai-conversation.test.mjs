import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import ts from 'typescript';
function load() {
  const api = {};
  const code = ts.transpileModule(readFileSync('src/lib/utils/ai-conversation.ts', 'utf8'), {
    compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 },
  }).outputText;
  new Function('require', 'exports', code)(() => ({ AI_TOOLS: [] }), api);
  return api;
}
const config = { enabled: true, has_key: true, revision: 1 };
const call = (id = 'call_1', args = '{"action":"volume","volume":30}') => ({
  id,
  type: 'function',
  function: { name: 'control_playback', arguments: args },
});
test('AI loop preserves assistant tool calls and corresponding tool results before final Chinese reply', async () => {
  const { runAiConversation } = load();
  const inputs = [];
  const events = [];
  let step = 0;
  let volume = 70;
  const d = {
    config: async () => config,
    turn: async (messages) => {
      inputs.push(structuredClone(messages));
      return step++ === 0
        ? {
            message: { role: 'assistant', content: null, tool_calls: [call()] },
            finish_reason: 'tool_calls',
          }
        : { message: { role: 'assistant', content: '音量已设为30%。' }, finish_reason: 'stop' };
    },
    execute: async (name, args) => {
      volume = args.volume;
      return { ok: true, volume_percent: volume };
    },
  };
  const history = await runAiConversation([], '音量调到30', { onEvent: (e) => events.push(e) }, d);
  assert.equal(volume, 30);
  assert.equal(inputs.length, 2);
  assert.equal(inputs[1].at(-2).role, 'assistant');
  assert.equal(inputs[1].at(-1).role, 'tool');
  assert.equal(inputs[1].at(-1).tool_call_id, 'call_1');
  assert.equal(history.at(-1).content, '音量已设为30%。');
  assert.equal(
    events.some((e) => e.role === 'tool' && e.name === 'control_playback'),
    true,
  );
});
test('AI loop stops at five requests and never repeats a tool ID or runs tools from the last round', async () => {
  const { runAiConversation } = load();
  let requests = 0;
  let executed = 0;
  const events = [];
  const d = {
    config: async () => config,
    turn: async () => ({
      message: { role: 'assistant', content: null, tool_calls: [call(`call_${++requests}`)] },
      finish_reason: 'tool_calls',
    }),
    execute: async () => {
      executed++;
      return { ok: true };
    },
  };
  const h = await runAiConversation([], '连续操作', { onEvent: (e) => events.push(e) }, d);
  assert.equal(requests, 5);
  assert.equal(executed, 4);
  assert.match(h.at(-1).content, /上限/);
  requests = 0;
  executed = 0;
  d.turn = async () => {
    requests++;
    return { message: { role: 'assistant', content: null, tool_calls: [call()] } };
  };
  await runAiConversation([], '重复', { onEvent: () => {} }, d);
  assert.equal(executed, 1);
  assert.ok(requests <= 5);
});
test('AI loop does not contact disabled providers, execute after cancellation or accept a changed configuration', async () => {
  const { runAiConversation } = load();
  let called = 0;
  const abort = new AbortController();
  const d = {
    config: async () => ({ ...config, enabled: false }),
    turn: async () => {
      called++;
      return { message: { role: 'assistant', content: 'ok' } };
    },
    execute: async () => {
      called++;
      return { ok: true };
    },
  };
  await assert.rejects(runAiConversation([], '你好', { onEvent: () => {} }, d), /设置/);
  assert.equal(called, 0);
  d.config = async () => config;
  d.turn = async () => {
    abort.abort();
    return { message: { role: 'assistant', content: null, tool_calls: [call()] } };
  };
  await assert.rejects(
    runAiConversation([], '停止', { onEvent: () => {}, signal: abort.signal }, d),
  );
  assert.equal(called, 0);
  let reads = 0;
  d.config = async () => ({ ...config, revision: ++reads });
  d.turn = async () => ({ message: { role: 'assistant', content: null, tool_calls: [call()] } });
  await assert.rejects(runAiConversation([], '新设置', { onEvent: () => {} }, d), /配置/);
  assert.equal(called, 0);
});
test('malformed model arguments receive an error result and history compaction preserves complete tool exchanges', async () => {
  const { runAiConversation } = load();
  let step = 0;
  let executions = 0;
  let payload;
  const d = {
    config: async () => config,
    turn: async (messages) => {
      payload = structuredClone(messages);
      return step++ === 0
        ? { message: { role: 'assistant', content: null, tool_calls: [call('bad', 'not-json')] } }
        : { message: { role: 'assistant', content: '请再试一次。' } };
    },
    execute: async () => {
      executions++;
      return { ok: true };
    },
  };
  const h = await runAiConversation(
    [
      { role: 'user', content: 'a'.repeat(120000) },
      { role: 'assistant', content: '旧回复' },
    ],
    '新问题',
    { onEvent: () => {} },
    d,
  );
  assert.equal(executions, 0);
  assert.equal(h[0].content, '新问题');
  assert.equal(JSON.parse(payload.at(-1).content).ok, false);
  assert.equal(payload[0].role, 'system');
});
test('completed tool effects survive a later request failure and cancellation checkpoints close outstanding calls', async () => {
  const { runAiConversation } = load();
  let saved = [];
  let requests = 0;
  let executed = 0;
  const d = {
    config: async () => config,
    turn: async () => {
      if (requests++) throw Error('AI_NETWORK');
      return { message: { role: 'assistant', content: null, tool_calls: [call()] } };
    },
    execute: async () => ({ ok: true, playlist_id: 7, added: 1 }),
  };
  await assert.rejects(
    runAiConversation(
      [],
      '创建歌单',
      {
        onEvent: () => {},
        onCheckpoint: (h) => {
          saved = h;
        },
      },
      d,
    ),
    /AI_NETWORK/,
  );
  assert.equal(saved.at(-1).role, 'tool');
  assert.equal(JSON.parse(saved.at(-1).content).playlist_id, 7);
  const abort = new AbortController();
  saved = [];
  d.turn = async () => ({
    message: { role: 'assistant', content: null, tool_calls: [call('a'), call('b')] },
  });
  d.execute = async () => {
    executed++;
    abort.abort();
    return { ok: true, playlist_id: 8 };
  };
  await assert.rejects(
    runAiConversation(
      [],
      '两个操作',
      {
        onEvent: () => {},
        signal: abort.signal,
        onCheckpoint: (h) => {
          saved = h;
        },
      },
      d,
    ),
  );
  assert.equal(executed, 1);
  assert.equal(saved.at(-1).tool_call_id, 'b');
  assert.equal(JSON.parse(saved.at(-1).content).error, 'not_executed');
  assert.equal(JSON.parse(saved.at(-2).content).playlist_id, 8);
});
