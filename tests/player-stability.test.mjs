import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync, unlinkSync } from 'node:fs';
import { compileModule } from 'svelte/compiler';
import { execFileSync } from 'node:child_process';

test('real Svelte effects keep cover stable through progress, pause, volume and reject stale results', () => {
  const app = readFileSync('src/App.svelte', 'utf8');
  const start = app.search(/let artwork\s*=\s*\$state/);
  const end = app.search(/async\s+function\s+connect\s*\(/);
  assert.ok(start >= 0 && end > start, 'cover effect is present in App');
  const declaration = app.slice(start, end).replace(/<string\s*\|\s*null>/g, '');
  const code = `import {flushSync} from 'svelte';
 let snapshot=$state(null); const desktopAvailable=true;const diagnostics=null; let pending=[]; let calls=0; const getArtwork=()=>{calls++;return new Promise(r=>pending.push(r));};
 export async function run(){let dispose= $effect.root(()=>{${declaration}
 globalThis.read=()=>({artwork,calls});});
 const update=(v)=>flushSync(()=>snapshot=v);
 update({track_id:'a'});pending.shift()('cover-a');await Promise.resolve();flushSync();
 for(let i=0;i<1500;i++)update({track_id:'a',position_ms:i*200,volume:i%100,playback_state:i%2?'playing':'paused'});
 if(calls!==1||globalThis.read().artwork!=='cover-a')throw Error('progress reloaded cover '+calls);
 update({track_id:'b'});const old=pending.shift(); update({track_id:'c'});pending.shift()('cover-c');await Promise.resolve();old('cover-b');await Promise.resolve();flushSync();
 if(globalThis.read().artwork!=='cover-c'||calls!==3)throw Error('stale cover');dispose(); }
 `;
  const path = 'tests/.cover-runtime.mjs';
  try {
    writeFileSync(
      path,
      compileModule(code, { filename: 'cover.svelte.ts', generate: 'client' }).js.code +
        '\nawait run();',
    );
    execFileSync(process.execPath, ['--conditions=browser', path], { stdio: 'pipe' });
  } finally {
    try {
      unlinkSync(path);
    } catch {}
  }
});

test('player bar never resets the cover or the seek draft on progress-only snapshots', () => {
  const bar = readFileSync('src/lib/layout/PlayerBar.svelte', 'utf8');
  // Seek draft resets only when the track identity changes, not on each 200 ms snapshot.
  assert.match(
    bar,
    /let\s+trackId\s*=\s*\$derived\(\s*s\s*\?\.\s*track_id\s*\)\s*;?\s*\$effect\(\s*\(\s*\)\s*=>\s*\{\s*trackId\s*;\s*dragging\s*=\s*null\s*;?\s*\}\s*\)\s*;?/,
  );
  // The bar renders the App-level artwork and does not request covers itself.
  assert.doesNotMatch(bar, /getArtwork|trackCover/);
  const now = readFileSync('src/lib/views/NowPlaying.svelte', 'utf8');
  assert.doesNotMatch(
    now,
    /getArtwork\(/,
    'Now Playing reuses the App cover instead of a second IPC request',
  );
});
