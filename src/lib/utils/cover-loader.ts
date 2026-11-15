// Bounded, prioritised cover requests. The loader owns no IPC; callers pass the load function.
// Only rows that ask for a cover are queued, and a row that leaves the screen cancels its request,
// so fast scrolling cannot build an unbounded backlog in front of the rows that are visible now.

/** 0 = current track, 1 = visible row, 2 = preload just outside the list. */
export type CoverPriority = 0 | 1 | 2;
export type CoverStatus = 'ready' | 'none' | 'failed';
export interface CoverOutcome { status: CoverStatus; src: string | null; code?: string }
export interface CoverTicket { setPriority(priority: CoverPriority): void; cancel(): void }
type Done = (outcome: CoverOutcome) => void;
interface Waiter { priority: CoverPriority; done: Done }

interface Job {
  key: string; load: () => Promise<string | null>; priority: CoverPriority; seq: number;
  /** Cache generation the result belongs to. A running job keeps the generation it started in. */
  generation: number;
  waiters: Map<number, Waiter>;
  state: 'queued' | 'running' | 'retry'; attempts: number; queuedAt: number;
}

export interface CoverLoaderOptions {
  concurrency?: number; maxQueued?: number; maxResults?: number;
  /** Delays before each retry of a transient failure. Its length is the retry limit. */
  retryDelays?: number[]; timeoutMs?: number; failedTtlMs?: number;
  /** Errors that will not change by retrying in this session. */
  terminal?: (code: string) => boolean;
  now?: () => number;
  schedule?: (fn: () => void, ms: number) => unknown;
}

export const emptyCoverStats = () => ({
  requested: 0, peekHits: 0, coalesced: 0, started: 0, ready: 0, none: 0, failed: 0, retries: 0,
  cancelled: 0, dropped: 0, timeouts: 0, maxQueued: 0, maxRunning: 0,
  /** Requests that finished after clear(): never cached; failures were handed to a current job. */
  stale: 0, staleHandedOver: 0,
  waitMsTotal: 0, waitMsMax: 0, ipcMsTotal: 0, ipcMsMax: 0, codes: {} as Record<string, number>,
});
export type CoverStats = ReturnType<typeof emptyCoverStats>;

/** Error code from a rejected cover request; never includes a path. */
export function coverErrorCode(error: unknown): string {
  const text = typeof error === 'string' ? error : error instanceof Error ? error.message : '';
  return /^cover-[a-z-]+$/.test(text) ? text : 'ipc-error';
}

export function createCoverLoader(options: CoverLoaderOptions = {}) {
  const concurrency = options.concurrency ?? 4;
  const maxQueued = options.maxQueued ?? 48;
  const maxResults = options.maxResults ?? 600;
  const retryDelays = options.retryDelays ?? [700, 2500];
  const timeoutMs = options.timeoutMs ?? 15000;
  const failedTtlMs = options.failedTtlMs ?? 30000;
  const terminal = options.terminal ?? (() => false);
  const now = options.now ?? (() => performance.now());
  const schedule = options.schedule ?? ((fn: () => void, ms: number) => setTimeout(fn, ms));
  const results = new Map<string, CoverOutcome & { at: number }>();
  /** Jobs of the current generation, one per key. Requests that outlived clear() are not in here. */
  const jobs = new Map<string, Job>();
  /** The job each waiting ticket currently belongs to; a hand-over moves tickets to another job. */
  const owners = new Map<number, Job>();
  const stats = emptyCoverStats();
  // `running` counts every load still in flight, including ones from before clear(): they still hold
  // an IPC call and a Rust thread, so the concurrency bound covers them too.
  let running = 0, seq = 0, waiterSeq = 0, generation = 0;

  function remember(key: string, outcome: CoverOutcome) {
    results.delete(key);
    results.set(key, { ...outcome, at: now() });
    if (results.size > maxResults) results.delete(results.keys().next().value!);
  }
  /** A finished result, or undefined when the cover still has to be requested. */
  function peek(key: string): CoverOutcome | undefined {
    const hit = results.get(key);
    if (!hit) return undefined;
    if (hit.status === 'failed' && now() - hit.at > failedTtlMs) { results.delete(key); return undefined; }
    results.delete(key); results.set(key, hit);
    return { status: hit.status, src: hit.src, code: hit.code };
  }
  const queued = () => [...jobs.values()].filter(j => j.state === 'queued');
  function release(job: Job) {
    if (jobs.get(job.key) === job) jobs.delete(job.key);
    for (const id of job.waiters.keys()) owners.delete(id);
  }
  function notify(job: Job, outcome: CoverOutcome) {
    const waiters = [...job.waiters.values()];
    release(job);
    for (const { done } of waiters) done(outcome);
  }
  function finish(job: Job, outcome: CoverOutcome) {
    stats[outcome.status]++;
    if (outcome.code) stats.codes[outcome.code] = (stats.codes[outcome.code] ?? 0) + 1;
    remember(job.key, outcome);
    notify(job, outcome);
  }
  function settle(job: Job) {
    job.priority = Math.min(...[...job.waiters.values()].map(w => w.priority), 2) as CoverPriority;
  }
  function createJob(key: string, load: () => Promise<string | null>, priority: CoverPriority): Job {
    const job: Job = { key, load, priority, seq: ++seq, generation, waiters: new Map(), state: 'queued', attempts: 0, queuedAt: now() };
    jobs.set(key, job);
    return job;
  }
  /** Moves the waiters of a request that failed after clear() to the current job for the same key. */
  function handOver(stale: Job) {
    if (stale.waiters.size === 0) return;
    const fresh = jobs.get(stale.key) ?? createJob(stale.key, stale.load, stale.priority);
    for (const [id, waiter] of stale.waiters) { fresh.waiters.set(id, waiter); owners.set(id, fresh); }
    stale.waiters.clear();
    settle(fresh);
    stats.staleHandedOver++;
  }
  function pump() {
    while (running < concurrency) {
      const next = queued().sort((a, b) => a.priority - b.priority || a.seq - b.seq)[0];
      if (!next) return;
      start(next);
    }
  }
  function start(job: Job) {
    job.state = 'running'; job.attempts++; running++; stats.started++;
    stats.maxRunning = Math.max(stats.maxRunning, running);
    const wait = now() - job.queuedAt;
    stats.waitMsTotal += wait; stats.waitMsMax = Math.max(stats.waitMsMax, wait);
    const began = now();
    let timer = false;
    const timeout = new Promise<never>((_, reject) => schedule(() => { timer = true; reject('cover-timeout'); }, timeoutMs));
    Promise.race([job.load(), timeout]).then(src => ({ src, error: null as unknown }), error => ({ src: null, error }))
      .then(({ src, error }) => {
        running--;
        const spent = now() - began;
        stats.ipcMsTotal += spent; stats.ipcMsMax = Math.max(stats.ipcMsMax, spent);
        if (job.generation !== generation) {
          // Started before clear(): the result may predate the library change, so it is never cached.
          // Its own rows still get a picture they asked for; a failure is retried by a current job.
          stats.stale++;
          if (error === null) notify(job, src ? { status: 'ready', src } : { status: 'none', src: null });
          else handOver(job);
          pump();
          return;
        }
        if (error === null) finish(job, src ? { status: 'ready', src } : { status: 'none', src: null });
        else {
          const code = timer && error === 'cover-timeout' ? 'cover-timeout' : coverErrorCode(error);
          if (code === 'cover-timeout') stats.timeouts++;
          const delay = retryDelays[job.attempts - 1];
          if (terminal(code) || delay === undefined || job.waiters.size === 0) finish(job, { status: 'failed', src: null, code });
          else {
            stats.retries++; job.state = 'retry';
            schedule(() => {
              if (jobs.get(job.key) !== job || job.state !== 'retry') return;
              if (job.waiters.size === 0) { release(job); return; }
              job.state = 'queued'; job.queuedAt = now(); pump();
            }, delay);
          }
        }
        pump();
      });
  }
  function request(key: string, priority: CoverPriority, load: () => Promise<string | null>, done: Done): CoverTicket {
    stats.requested++;
    const hit = peek(key);
    if (hit) { stats.peekHits++; done(hit); return { setPriority() {}, cancel() {} }; }
    let job = jobs.get(key);
    if (job) stats.coalesced++;
    else job = createJob(key, load, priority);
    const id = ++waiterSeq;
    job.waiters.set(id, { priority, done });
    owners.set(id, job);
    settle(job);
    const waiting = queued();
    stats.maxQueued = Math.max(stats.maxQueued, waiting.length);
    if (waiting.length > maxQueued) {
      // Over the bound only preloads are dropped, newest first; their rows ask again when visible.
      const victim = waiting.filter(j => j.priority === 2).sort((a, b) => b.seq - a.seq)[0];
      if (victim) { stats.dropped++; notify(victim, { status: 'failed', src: null, code: 'dropped' }); }
    }
    pump();
    return {
      setPriority(next) {
        const owner = owners.get(id);
        const waiter = owner?.waiters.get(id);
        if (!owner || !waiter || waiter.priority === next) return;
        waiter.priority = next; settle(owner); pump();
      },
      cancel() {
        const owner = owners.get(id);
        if (!owner || !owner.waiters.delete(id)) return;
        owners.delete(id);
        stats.cancelled++;
        if (owner.waiters.size === 0 && owner.state === 'queued') release(owner);
        else settle(owner);
      },
    };
  }
  /** Library contents changed: forget results. Rows keep showing what they have until new results arrive.
   *  Loads already running finish in the background but cannot write the new cache; queued and retrying
   *  requests have not read anything yet, so they simply belong to the new generation. */
  function clear() {
    generation++; results.clear();
    for (const job of [...jobs.values()]) {
      if (job.state === 'running') jobs.delete(job.key);
      else job.generation = generation;
    }
  }
  return {
    request, peek, clear, stats,
    pending: () => ({ queued: queued().length, running, waiting: owners.size }),
  };
}
