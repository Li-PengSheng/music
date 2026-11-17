import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import ts from 'typescript';
test('AI desktop lyrics accepts the native state event emitted before its IPC reply', async () => {
  const api = {};
  const ui = { lyricsOverlayOpen: false, lyricsOverlayLocked: false };
  let resolve;
  const code = ts.transpileModule(readFileSync('src/lib/stores/lyrics-overlay.ts', 'utf8'), {
    compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 },
  }).outputText;
  new Function('require', 'exports', code)(
    (name) =>
      name.includes('music')
        ? {
            desktopAvailable: true,
            desktopLyricsWindow: () =>
              new Promise((r) => {
                resolve = r;
              }),
          }
        : { ui, notifyError: () => {} },
    api,
  );
  const result = api.setDesktopLyricsOpen(true);
  api.applyLyricsOverlay({ open: true, locked: false });
  resolve(true);
  assert.equal(await result, true);
  assert.equal(ui.lyricsOverlayOpen, true);
});
