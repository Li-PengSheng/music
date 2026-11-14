import {test} from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {retainQueue,sameTracks} from '../src/lib/utils/queue.ts';

const tokens=readFileSync('src/lib/styles/tokens.css','utf8');
const track=(path,title='t')=>({path,title,artist:'a',album:'b',duration_ms:1});

test('drawer timing: ~240ms decelerating in, ~180ms accelerating out, no overshoot',()=>{
 const ms=name=>Number(tokens.match(new RegExp(`${name}:\\s*(\\d+)ms`))[1]);
 assert.ok(ms('--dur-drawer-in')>=220&&ms('--dur-drawer-in')<=250);
 assert.ok(ms('--dur-drawer-out')>=160&&ms('--dur-drawer-out')<=200);
 for(const name of ['--ease-enter','--ease-exit']){
  const [x1,y1,x2,y2]=tokens.match(new RegExp(`${name}:\\s*cubic-bezier\\(([^)]+)\\)`))[1].split(',').map(Number);
  for(const v of [x1,y1,x2,y2])assert.ok(v>=0&&v<=1,`${name} control points stay in [0,1]: no spring or bounce`);
 }
});

test('identical queue polls keep the same array, so rows are not re-diffed',()=>{
 const rendered=[track('a','一'),track('b','二')];
 const poll=[track('a','一'),track('b','二')];
 assert.equal(sameTracks(rendered,poll),true);
 assert.equal(retainQueue(rendered,poll),rendered);
 assert.equal(retainQueue(rendered,rendered),rendered);
});

test('a real queue change replaces the array',()=>{
 const rendered=[track('a'),track('b')];
 const renamed=[track('a','新标题'),track('b')];
 const reordered=[track('b'),track('a')];
 const shorter=[track('a')];
 assert.equal(retainQueue(rendered,renamed),renamed);
 assert.equal(retainQueue(rendered,reordered),reordered);
 assert.equal(retainQueue(rendered,shorter),shorter);
 assert.equal(retainQueue([],rendered),rendered);
});

test('queue scrim stays light enough to read the page underneath',()=>{
 const alphas=[...tokens.matchAll(/--scrim-soft:\s*rgba\([^)]*,\s*([.\d]+)\s*\)/g)].map(m=>Number(m[1]));
 assert.ok(alphas.length>=2);
 for(const alpha of alphas)assert.ok(alpha<=.35,String(alpha));
});
