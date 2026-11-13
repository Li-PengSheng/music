import {test} from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';

const read=f=>readFileSync(f,'utf8');
const css=read('src/app.css');
const tokens=read('src/lib/styles/tokens.css');
const app=read('src/App.svelte');
const queue=read('src/lib/Queue.svelte');
const rule=selector=>{const i=css.indexOf(selector+'{');assert.ok(i>=0,`missing ${selector}`);return css.slice(i,css.indexOf('}',i));};

test('drawer timing: ~240ms decelerating in, ~180ms accelerating out, no overshoot',()=>{
 const ms=name=>Number(tokens.match(new RegExp(`${name}:(\\d+)ms`))[1]);
 assert.ok(ms('--dur-drawer-in')>=220&&ms('--dur-drawer-in')<=250);
 assert.ok(ms('--dur-drawer-out')>=160&&ms('--dur-drawer-out')<=200);
 for(const name of ['--ease-enter','--ease-exit']){
  const [x1,y1,x2,y2]=tokens.match(new RegExp(`${name}:cubic-bezier\\(([^)]+)\\)`))[1].split(',').map(Number);
  for(const v of [x1,y1,x2,y2])assert.ok(v>=0&&v<=1,`${name} control points stay in [0,1]: no spring or bounce`);
 }
});

test('only transform and opacity animate; pages never change width when the queue toggles',()=>{
 const closed=rule('.queue');
 assert.match(closed,/transform:translateX\(var\(--drawer-shift\)\);opacity:0;visibility:hidden/);
 assert.match(closed,/visibility 0s linear var\(--dur-drawer-out\)/,'hidden only after the exit transition ends');
 for(const block of [closed,rule('.app.queue-open .queue')]){
  const props=[...block.matchAll(/(?:^|,|transition:)\s*([a-z-]+) (?:var|0s)/g)].map(m=>m[1]);
  assert.deepEqual([...new Set(props)].sort(),['opacity','transform','visibility']);
 }
 assert.match(css,/@starting-style\{\.app\.queue-open \.queue\{transform:translateX\(var\(--drawer-shift\)\);opacity:0\}\}/,'first open animates too');
 assert.doesNotMatch(css,/queue-open \.content/,'content grid is not resized');
 assert.match(closed,/top:var\(--topbar-h\);right:0;bottom:var\(--player-h\)/,'between top bar and player bar');
});

test('rapid toggling is pure state: CSS transitions reverse, no timers or remounts',()=>{
 assert.match(app,/let queueMounted=\$state\(false\);/);
 assert.match(app,/\$effect\.pre\(\(\)=>\{if\(ui\.queueOpen\)queueMounted=true;\}\);/,'mounted once, never unmounted');
 assert.match(app,/\{#if queueMounted\}[\s\S]*<Queue open=\{ui\.queueOpen\}/);
 const toggle=app.slice(app.indexOf('async function toggleQueue'),app.indexOf('function keydown'));
 assert.doesNotMatch(toggle,/setTimeout|requestAnimationFrame/);
 assert.doesNotMatch(queue,/onMount|transition:|setTimeout/);
 assert.match(queue,/inert=\{!open\}/,'closed drawer leaves the tab order immediately');
 assert.match(queue,/\$effect\(\(\)=>\{if\(open\)frozen=live;\}\);/,'closed drawer stops re-rendering on player polls');
 assert.doesNotMatch(queue,/getPlayerState|invoke\(/,'opening never refetches the queue');
});

test('Escape, close button and scrim share one close path',()=>{
 assert.match(app,/if\(layer==='queue'\)void toggleQueue\(false\)/);
 assert.match(app,/class="queue-scrim"[^>]*onclick=\{\(\)=>toggleQueue\(false\)\}/);
 assert.match(app,/<Queue open=\{ui\.queueOpen\} onclose=\{\(\)=>toggleQueue\(false\)\}\/>/);
});

test('reduced motion (app setting and Windows) removes the slide',()=>{
 assert.match(css,/@media \(prefers-reduced-motion:reduce\)\{\.queue\{--drawer-shift:0px\}\}/);
 assert.match(css,/\.reduce-motion \.queue\{--drawer-shift:0px\}/);
});

test('narrow-window scrim is light and fades with the drawer',()=>{
 const media=css.slice(css.indexOf('@media (max-width:1199px)'));
 assert.match(media,/\.queue-scrim\{[^}]*background:var\(--scrim-soft\)[^}]*opacity:0;pointer-events:none/);
 assert.match(media,/\.app\.queue-open \.queue-scrim\{opacity:1;pointer-events:auto/);
 for(const alpha of tokens.matchAll(/--scrim-soft:rgba\([^)]*,([.\d]+)\)/g))assert.ok(Number(alpha[1])<=.35);
});
