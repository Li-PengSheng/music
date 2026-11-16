import { test } from 'node:test';
import assert from 'node:assert/strict';
import { toPercent, toGain, muteTarget, latest } from '../src/lib/utils/volume.ts';

test('percent is a presentation of the same 0.0–1.0 gain Rust uses (no new curve)', () => {
  for (const p of [0, 1, 35, 50, 99, 100]) assert.equal(toPercent(toGain(p)), p, 'round-trips');
  assert.equal(toPercent(0.35), 35);
  assert.equal(toPercent(0.354), 35);
  assert.equal(toPercent(0.355), 36);
  assert.equal(toPercent(1.4), 100);
  assert.equal(toPercent(-1), 0);
  assert.equal(toPercent(Number.NaN), 0);
  assert.equal(toGain(150), 1);
  assert.equal(toGain(-5), 0);
});

test('mute keeps the previous level and unmute restores it', () => {
  assert.equal(muteTarget(35, 35), 0, 'audible -> mute');
  assert.equal(muteTarget(0, 35), 35, 'muted -> previous level');
  assert.equal(muteTarget(0, 0), 20, 'nothing remembered -> sensible default, never stays silent');
});

/** Fake backend: each call resolves only when the test says so, like a slow IPC round-trip. */
function backend() {
  const calls = [];
  const waiting = [];
  return {
    calls,
    send: (v) => {
      calls.push(v);
      return new Promise((r) => waiting.push(() => r({ volume: v / 100 })));
    },
    async flush() {
      while (waiting.length) {
        waiting.shift()();
        await new Promise((r) => setImmediate(r));
      }
    },
  };
}

test('slider drags coalesce: one call in flight, the final value always wins', async () => {
  const b = backend();
  const results = [];
  let settled = 0;
  const sync = latest(b.send, {
    result: (r) => results.push(r.volume),
    error: assert.fail,
    settled: () => settled++,
  });
  for (const v of [10, 20, 30, 40, 55]) sync.push(v);
  assert.deepEqual(b.calls, [10], 'first value goes out immediately');
  assert.equal(sync.busy, true);
  await b.flush();
  assert.deepEqual(b.calls, [10, 55], 'intermediate values dropped, last value sent');
  assert.deepEqual(results, [0.1, 0.55], 'every reply is applied, ending at the real final volume');
  assert.equal(settled, 1);
  assert.equal(sync.busy, false);
});

test('repeated identical values in a burst are sent once; a later burst can resend', async () => {
  const b = backend();
  const sync = latest(b.send, { result: () => {}, error: assert.fail, settled: () => {} });
  sync.push(40);
  sync.push(40);
  await b.flush();
  sync.push(40);
  await b.flush();
  assert.deepEqual(
    b.calls,
    [40, 40],
    'oninput+onchange with the same value collapse; a new interaction is not swallowed',
  );
});

test('a failed call reports the error and still settles so the UI falls back to the real state', async () => {
  let errors = 0,
    settled = 0;
  const sync = latest(() => Promise.reject(new Error('device lost')), {
    result: assert.fail,
    error: () => errors++,
    settled: () => settled++,
  });
  sync.push(30);
  await new Promise((r) => setImmediate(r));
  assert.equal(errors, 1);
  assert.equal(settled, 1);
  assert.equal(sync.busy, false);
});
