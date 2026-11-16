/** Rust's player volume is a 0.0–1.0 gain; the UI shows the same value as a whole percentage. */
export const DEFAULT_PERCENT = 70;
export const toPercent = (gain: number) =>
  Number.isFinite(gain) ? Math.round(Math.min(1, Math.max(0, gain)) * 100) : 0;
export const toGain = (percent: number) => Math.min(100, Math.max(0, Math.round(percent))) / 100;

/** Volume a mute toggle should set: mute when audible, otherwise restore the last audible level. */
export function muteTarget(current: number, lastAudible: number) {
  return current > 0 ? 0 : lastAudible > 0 ? lastAudible : DEFAULT_PERCENT;
}

/**
 * Coalesces rapid values (slider drags) into IPC calls: at most one request is in flight,
 * intermediate values are dropped, and the most recent value is always sent last.
 */
export function latest<T, R>(
  send: (value: T) => Promise<R>,
  handlers: { result: (r: R) => void; error: (e: unknown) => void; settled: () => void },
) {
  let pending: { value: T } | null = null;
  let running = false;
  let sent: { value: T } | null = null;
  async function pump() {
    running = true;
    while (pending) {
      const { value } = pending;
      pending = null;
      if (sent && Object.is(sent.value, value)) continue;
      sent = { value };
      try {
        handlers.result(await send(value));
      } catch (e) {
        sent = null;
        handlers.error(e);
      }
    }
    running = false;
    sent = null;
    handlers.settled();
  }
  return {
    push(value: T) {
      pending = { value };
      if (!running) void pump();
    },
    get busy() {
      return running;
    },
  };
}
