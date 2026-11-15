import {test} from 'node:test';
import assert from 'node:assert/strict';
import {createCoverLoader} from '../src/lib/utils/cover-loader.ts';

// Every load() call gets its own manually settled promise; timers run only when advanced.
function harness(options={}){
 let t=0;const timers=[];
 const schedule=(fn,ms)=>{timers.push({at:t+ms,fn});};
 const advance=async ms=>{t+=ms;for(const timer of timers.filter(x=>x.at<=t)){timers.splice(timers.indexOf(timer),1);timer.fn();}await flush();};
 const calls=[];
 const load=key=>()=>new Promise((resolve,reject)=>calls.push({key,resolve,reject}));
 const loader=createCoverLoader({now:()=>t,schedule,retryDelays:[100],timeoutMs:1e9,...options});
 const open=key=>calls.filter(c=>c.key===key);
 return {loader,load,calls,open,advance};
}
const flush=()=>new Promise(r=>setTimeout(r,0));
const sink=()=>{const got=[];return {got,done:o=>got.push(o)};};

test('an old request that succeeds after clear() reaches only its own rows and is not cached',async()=>{
 const {loader,load,open}=harness();
 const old=sink(),fresh=sink();
 loader.request('a',1,load('a'),old.done);
 loader.clear();
 loader.request('a',1,load('a'),fresh.done);
 // The new generation does not join the old in-flight load: a second load for the same key starts.
 assert.equal(open('a').length,2);
 open('a')[0].resolve('data:old');await flush();
 assert.deepEqual(old.got,[{status:'ready',src:'data:old'}]);
 assert.deepEqual(fresh.got,[]);
 assert.equal(loader.peek('a'),undefined);
 open('a')[1].resolve('data:new');await flush();
 assert.deepEqual(fresh.got,[{status:'ready',src:'data:new'}]);
 assert.equal(loader.peek('a').src,'data:new');
 assert.equal(loader.stats.stale,1);
 assert.deepEqual(loader.pending(),{queued:0,running:0,waiting:0});
});

test('an old request finishing after the new one cannot overwrite the new result',async()=>{
 const {loader,load,open}=harness();
 loader.request('a',1,load('a'),()=>{});
 loader.clear();
 const fresh=sink();
 loader.request('a',1,load('a'),fresh.done);
 open('a')[1].resolve(null);await flush();
 open('a')[0].resolve('data:old');await flush();
 assert.deepEqual(fresh.got,[{status:'none',src:null}]);
 assert.deepEqual(loader.peek('a'),{status:'none',src:null,code:undefined});
});

test('an old request that fails after clear() hands its rows to a current request instead of failing them',async()=>{
 const {loader,load,open}=harness();
 const old=sink();
 loader.request('a',1,load('a'),old.done);
 loader.clear();
 open('a')[0].reject('cover-unreadable');await flush();
 assert.deepEqual(old.got,[]);
 assert.equal(open('a').length,2,'a current-generation load replaces the failed old one');
 assert.equal(loader.peek('a'),undefined,'the old failure is not cached');
 open('a')[1].resolve('data:new');await flush();
 assert.deepEqual(old.got,[{status:'ready',src:'data:new'}]);
 assert.equal(loader.peek('a').src,'data:new');
 assert.equal(loader.stats.staleHandedOver,1);
 assert.equal(loader.stats.failed,0);
 assert.deepEqual(loader.pending(),{queued:0,running:0,waiting:0});
});

test('a hand-over joins a current request for the same key rather than starting another',async()=>{
 const {loader,load,open}=harness();
 const old=sink(),fresh=sink();
 loader.request('a',1,load('a'),old.done);
 loader.clear();
 loader.request('a',1,load('a'),fresh.done);
 open('a')[0].reject(new Error('IPC broke'));await flush();
 assert.equal(open('a').length,2);
 open('a')[1].resolve('data:new');await flush();
 assert.equal(old.got[0].src,'data:new');
 assert.equal(fresh.got[0].src,'data:new');
});

test('old in-flight loads count toward the concurrency limit, and new rows start as soon as they finish',async()=>{
 const {loader,load,calls,open}=harness({concurrency:2});
 loader.request('a',1,load('a'),()=>{});
 loader.request('b',1,load('b'),()=>{});
 loader.clear();
 const c=sink(),d=sink();
 loader.request('c',1,load('c'),c.done);
 loader.request('d',1,load('d'),d.done);
 assert.equal(calls.length,2,'no new load starts while two old ones still run');
 assert.deepEqual(loader.pending(),{queued:2,running:2,waiting:4});
 open('a')[0].resolve('data:a');await flush();
 assert.deepEqual(calls.map(x=>x.key),['a','b','c']);
 open('b')[0].reject('cover-unreadable');await flush();
 assert.deepEqual(calls.map(x=>x.key),['a','b','c','d']);
 open('c')[0].resolve('data:c');open('d')[0].resolve(null);await flush();
 assert.equal(c.got[0].src,'data:c');assert.equal(d.got[0].status,'none');
 assert.equal(loader.stats.maxRunning,2);
 // The old row of b was handed over and still waits for a current load, which starts now.
 assert.equal(open('b').length,2);
 open('b')[1].resolve('data:b');await flush();
 assert.deepEqual([...['a','b','c','d']].map(k=>loader.peek(k)?.src??null),[null,'data:b','data:c',null]);
 assert.equal(loader.peek('d').status,'none');
 assert.deepEqual(loader.pending(),{queued:0,running:0,waiting:0});
});

test('queued and retrying requests survive clear() and still notify their rows',async()=>{
 const {loader,load,open,advance}=harness({concurrency:1});
 const a=sink(),b=sink();
 loader.request('a',1,load('a'),a.done);
 loader.request('b',1,load('b'),b.done);
 open('a')[0].reject('cover-unreadable');await flush();
 // a waits for its retry, b is running; clear() now.
 loader.clear();
 open('b')[0].resolve('data:b');await flush();
 // b started before clear(): its row gets the picture, but the result is not cached for the new generation.
 assert.deepEqual(b.got,[{status:'ready',src:'data:b'}]);
 assert.equal(loader.peek('b'),undefined);
 await advance(100);
 assert.equal(open('a').length,2,'the retry of a still runs after clear()');
 open('a')[1].resolve('data:a');await flush();
 assert.equal(a.got[0].src,'data:a');
 assert.equal(loader.peek('a').src,'data:a');
});

test('after clear() the current track and visible rows still go before preloads',async()=>{
 const {loader,load,calls,open}=harness({concurrency:1});
 loader.request('busy',1,load('busy'),()=>{});
 loader.clear();
 loader.request('pre',2,load('pre'),()=>{});
 loader.request('vis',1,load('vis'),()=>{});
 loader.request('cur',0,load('cur'),()=>{});
 open('busy')[0].resolve('data:x');await flush();
 open('cur')[0].resolve('data:c');await flush();
 open('vis')[0].resolve('data:v');await flush();
 assert.deepEqual(calls.map(x=>x.key),['busy','cur','vis','pre']);
});

test('repeated clear() during long loads keeps the number of loads bounded',async()=>{
 const {loader,load,calls}=harness({concurrency:4});
 for(let round=0;round<10;round++){
  for(let i=0;i<20;i++)loader.request('k'+i,1,load('k'+i),()=>{});
  loader.clear();
 }
 assert.equal(calls.length,4,'nothing finished, so only the first four loads ever started');
 assert.equal(loader.pending().running,4);
 assert.ok(loader.pending().queued<=20);
 for(let n=0;n<40&&calls.some(c=>!c.settled);n++){
  for(const c of calls.filter(c=>!c.settled)){c.settled=true;c.resolve(null);}
  await flush();
 }
 assert.ok(calls.length<=4+20,`${calls.length} loads for 20 keys`);
 assert.deepEqual(loader.pending(),{queued:0,running:0,waiting:0});
});
