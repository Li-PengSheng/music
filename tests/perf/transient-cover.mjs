// Transient failure check on the generated corpus only: hides one corpus file, reloads the page, scrolls to its
// queue row and reports the row state over time; then restores the file and checks the cover comes back.
import { renameSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
const row = Number(process.argv[2] ?? 150);
const dir = join(process.env.TEMP, 'mm-cover-corpus', 'music', 'jpeg-small');
const file = readdirSync(dir).find((n) => n.startsWith(String(row).padStart(3, '0') + ' '));
const page = (await (await fetch('http://127.0.0.1:9333/json')).json()).find(
  (t) => t.type === 'page',
);
const ws = new WebSocket(page.webSocketDebuggerUrl);
await new Promise((r) => (ws.onopen = r));
let id = 0;
const wait = new Map();
ws.onmessage = ({ data }) => {
  const m = JSON.parse(data);
  if (wait.has(m.id)) {
    wait.get(m.id)(m.result);
    wait.delete(m.id);
  }
};
const send = (method, params = {}) =>
  new Promise((r) => {
    wait.set(++id, r);
    ws.send(JSON.stringify({ id, method, params }));
  });
const js = async (e) =>
  (await send('Runtime.evaluate', { expression: e, returnByValue: true, awaitPromise: true }))
    .result?.value;
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
async function visit(label, ms) {
  await send('Page.reload');
  for (let i = 0; i < 200; i++) {
    if (
      await js(
        `!!document.querySelector('.queue-list .queue-item')&&!!globalThis.__musicMineCoverStats`,
      )
    )
      break;
    await sleep(50);
  }
  await js(
    `document.querySelector('.app').classList.contains('queue-open')||document.querySelector('button[aria-label="播放队列"]').click()`,
  );
  await sleep(500);
  await js(
    `(()=>{const r=[...document.querySelectorAll('.queue-item')].find(r=>/Cover ${row} /.test(r.textContent));document.querySelector('.queue-list').scrollTop=r.offsetTop-8;})()`,
  );
  const out = [];
  for (let t = 0; t < ms; t += 500) {
    await sleep(500);
    out.push(
      await js(
        `(()=>{const r=[...document.querySelectorAll('.queue-item')].find(r=>/Cover ${row} /.test(r.textContent));const c=r.querySelector('[data-cover]');const S=globalThis.__musicMineCoverStats;return {t:${t + 500},cover:c.dataset.cover,code:c.dataset.coverCode??null,img:!!r.querySelector('img.loaded'),retries:S.retries,failed:S.failed,codes:S.codes};})()`,
      ),
    );
  }
  console.log(label);
  for (const s of out) console.log('  ', JSON.stringify(s));
}
renameSync(join(dir, file), join(dir, file + '.hidden'));
try {
  await visit('hidden', 5000);
} finally {
  renameSync(join(dir, file + '.hidden'), join(dir, file));
}
await visit('restored', 1500);
ws.close();
