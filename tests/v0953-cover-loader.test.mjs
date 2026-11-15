import {test} from 'node:test';
import assert from 'node:assert/strict';
import {createCoverLoader,coverErrorCode} from '../src/lib/utils/cover-loader.ts';

// Manual clock and timers so retries and timeouts run deterministically.
function harness(options={}){
 let t=0;const timers=[];
 const schedule=(fn,ms)=>{timers.push({at:t+ms,fn});};
 const advance=async ms=>{t+=ms;for(const timer of timers.filter(x=>x.at<=t)){timers.splice(timers.indexOf(timer),1);timer.fn();}await flush();};
 const calls=[];
 const pending=new Map();
 const load=key=>()=>{calls.push(key);return new Promise((resolve,reject)=>pending.set(key,{resolve,reject}));};
 const loader=createCoverLoader({now:()=>t,schedule,...options});
 return {loader,load,calls,pending,advance};
}
const flush=()=>new Promise(r=>setTimeout(r,0));

test('visible and current rows start before preloads; concurrency is bounded',async()=>{
 const {loader,load,calls}=harness({concurrency:2});
 const seen=[];
 loader.request('pre-1',2,load('pre-1'),o=>seen.push(o));
 loader.request('pre-2',2,load('pre-2'),()=>{});
 loader.request('vis',1,load('vis'),()=>{});
 loader.request('cur',0,load('cur'),()=>{});
 // The first two preloads started immediately; the rest wait in priority order.
 assert.deepEqual(calls,['pre-1','pre-2']);
 assert.deepEqual(loader.pending(),{queued:2,running:2,waiting:4});
 assert.equal(loader.stats.maxRunning,2);
});

test('waiting requests are ordered current, visible, preload',async()=>{
 const {loader,load,calls,pending}=harness({concurrency:1});
 loader.request('busy',1,load('busy'),()=>{});
 loader.request('pre',2,load('pre'),()=>{});
 loader.request('vis',1,load('vis'),()=>{});
 loader.request('cur',0,load('cur'),()=>{});
 pending.get('busy').resolve('data:a');await flush();
 pending.get('cur').resolve('data:b');await flush();
 pending.get('vis').resolve(null);await flush();
 assert.deepEqual(calls,['busy','cur','vis','pre']);
});

test('a row that scrolls away cancels its queued request; identical requests share one load',async()=>{
 const {loader,load,calls,pending}=harness({concurrency:1});
 loader.request('busy',1,load('busy'),()=>{});
 const away=loader.request('gone',1,load('gone'),()=>{});
 const a=[],b=[];
 loader.request('same',1,load('same'),o=>a.push(o));
 loader.request('same',1,load('same'),o=>b.push(o));
 away.cancel();
 assert.equal(loader.stats.coalesced,1);
 pending.get('busy').resolve('data:x');await flush();
 pending.get('same').resolve('data:y');await flush();
 assert.deepEqual(calls,['busy','same']);
 assert.equal(a[0].src,'data:y');assert.equal(b[0].src,'data:y');
});

test('fast scrolling through hundreds of rows leaves no backlog in front of the visible rows',async()=>{
 const {loader,load,calls,pending}=harness({concurrency:4});
 for(let i=0;i<300;i++){const ticket=loader.request('row'+i,1,load('row'+i),()=>{});if(i<290)ticket.cancel();}
 // Four requests were already running; the other passed-over rows were cancelled before they started.
 assert.equal(calls.length,4);
 for(const key of [...calls])pending.get(key).resolve(null);await flush();
 assert.deepEqual(calls.slice(4),['row290','row291','row292','row293']);
 assert.ok(loader.pending().queued<=6);
});

test('null is cached as "no cover"; transient errors retry a bounded number of times and are not cached as null',async()=>{
 const {loader,load,calls,pending,advance}=harness({retryDelays:[100,200],failedTtlMs:1000});
 const none=[];
 loader.request('plain',1,load('plain'),o=>none.push(o));
 pending.get('plain').resolve(null);await flush();
 assert.deepEqual(none,[{status:'none',src:null}]);
 assert.equal(loader.peek('plain').status,'none');
 const out=[];
 loader.request('busy',1,load('busy'),o=>out.push(o));
 pending.get('busy').reject('cover-unreadable');await flush();
 assert.equal(out.length,0);
 await advance(100);pending.get('busy').reject('cover-unreadable');await flush();
 await advance(200);pending.get('busy').reject(new Error('IPC broke'));await flush();
 assert.equal(calls.filter(c=>c==='busy').length,3);
 assert.deepEqual(out,[{status:'failed',src:null,code:'ipc-error'}]);
 assert.equal(loader.stats.retries,2);
 // Failures are remembered briefly, then requested again instead of staying "no cover" forever.
 assert.equal(loader.peek('busy').status,'failed');
 await advance(1001);
 assert.equal(loader.peek('busy'),undefined);
});

test('a terminal error is not retried; a hung request times out instead of loading forever',async()=>{
 const {loader,load,calls,pending,advance}=harness({terminal:c=>c==='cover-not-in-library',retryDelays:[],timeoutMs:500});
 const a=[],b=[];
 loader.request('stray',1,load('stray'),o=>a.push(o));
 pending.get('stray').reject('cover-not-in-library');await flush();
 assert.deepEqual(a,[{status:'failed',src:null,code:'cover-not-in-library'}]);
 loader.request('hang',1,load('hang'),o=>b.push(o));
 await advance(500);
 assert.deepEqual(b,[{status:'failed',src:null,code:'cover-timeout'}]);
 assert.equal(loader.stats.timeouts,1);
 assert.equal(calls.length,2);
});

test('finished covers are available synchronously; clear() forgets them',async()=>{
 const {loader,load,pending}=harness();
 loader.request('k',1,load('k'),()=>{});
 pending.get('k').resolve('data:z');await flush();
 const hit=[];loader.request('k',1,load('k'),o=>hit.push(o));
 assert.equal(hit[0].src,'data:z');
 assert.equal(loader.stats.peekHits,1);
 loader.clear();
 assert.equal(loader.peek('k'),undefined);
});

test('the queued set stays bounded by dropping the newest preloads',async()=>{
 const {loader,load}=harness({concurrency:1,maxQueued:3});
 const dropped=[];
 loader.request('run',1,load('run'),()=>{});
 for(let i=0;i<6;i++)loader.request('p'+i,2,load('p'+i),o=>dropped.push([i,o.code]));
 loader.request('v',1,load('v'),()=>{});
 assert.ok(loader.pending().queued<=3);
 assert.ok(dropped.every(([,code])=>code==='dropped'));
 assert.ok(loader.stats.dropped>=3);
});

test('error codes never carry paths',()=>{
 assert.equal(coverErrorCode('cover-file-missing'),'cover-file-missing');
 assert.equal(coverErrorCode('C:\\Music\\a.flac: denied'),'ipc-error');
 assert.equal(coverErrorCode(new Error('boom')),'ipc-error');
});
