import type { PlayerSnapshot } from '../services/music';
import { notify, notifyError } from './ui.svelte';

/** Read-only copy of Rust's player state; Rust remains the only authority. */
export const player = $state({
  snapshot: null as PlayerSnapshot | null,
  busy: false,
  connection: '尚未验证',
  connected: false,
});

export function accept(next: PlayerSnapshot) {
  const current = player.snapshot;
  if (current && next.revision < current.revision) return;
  player.snapshot = next;
  if (next.error && next.error !== current?.error) notify(next.error, 'error');
}

/** Runs one player command at a time and applies its snapshot. */
export async function control(action: () => Promise<PlayerSnapshot>) {
  if (player.busy) return false;
  player.busy = true;
  try {
    accept(await action());
    return true;
  } catch (e) {
    notifyError(e);
    return false;
  } finally {
    player.busy = false;
  }
}

/** Like control, but not blocked by an in-flight command (library actions). */
export async function perform(action: () => Promise<PlayerSnapshot>, success?: string) {
  try {
    accept(await action());
    if (success) notify(success);
    return true;
  } catch (e) {
    notifyError(e);
    return false;
  }
}
