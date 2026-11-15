// Samples queue covers and loader counters while a rescan runs (the scan itself is started through the UI).
// node tests/perf/scan-sampler.mjs [seconds=150]
// Starts the scan button click itself; the native folder dialog must then be answered by hand.
const seconds=Number(process.argv[2]??150);
const page=(await (await fetch('http://127.0.0.1:9333/json')).json()).find(t=>t.type==='page');
const ws=new WebSocket(page.webSocketDebuggerUrl);await new Promise(r=>ws.onopen=r);
let id=0;const wait=new Map();ws.onmessage=({data})=>{const m=JSON.parse(data);if(wait.has(m.id)){wait.get(m.id)(m.result);wait.delete(m.id);}};
const send=(method,params={})=>new Promise(r=>{wait.set(++id,r);ws.send(JSON.stringify({id,method,params}));});
const js=async e=>(await send('Runtime.evaluate',{expression:e,returnByValue:true,awaitPromise:true})).result?.value;
const sleep=ms=>new Promise(r=>setTimeout(r,ms));
await js(`[...document.querySelectorAll('.nav-item')].find(e=>e.textContent.includes('设置')).click()`);await sleep(500);
await js(`document.querySelector('.app').classList.contains('queue-open')||document.querySelector('button[aria-label="播放队列"]').click()`);await sleep(600);
await js(`document.querySelector('.queue-list').scrollTop=0`);
const sample=()=>js(`(async()=>{const S=globalThis.__musicMineCoverStats;const p=await window.__TAURI_INTERNALS__.invoke('library_progress');
 const list=document.querySelector('.queue-list');const box=list.getBoundingClientRect();
 const vis=[...list.querySelectorAll('.queue-item')].filter(r=>{const b=r.getBoundingClientRect();return b.bottom>box.top+1&&b.top<box.bottom-1;});
 const kinds=vis.map(r=>{const m=/Cover (\\d{3}) (\\S+)/.exec(r.querySelector('.queue-text strong')?.textContent??'');return m?.[2];});
 const expect=vis.filter((r,i)=>!/no-cover|corrupt/.test(kinds[i]));
 return {running:p.running,visited:p.visited,top:Math.round(list.scrollTop),visible:vis.length,expected:expect.length,
  shown:expect.filter(r=>r.querySelector('img.loaded')).length,blank:expect.filter(r=>!r.querySelector('img')).length,
  loading:vis.filter(r=>r.querySelector('[data-cover="loading"]')).length,failed:vis.filter(r=>r.querySelector('[data-cover="failed"]')).length,
  started:S.started,stale:S.stale,handed:S.staleHandedOver,maxRunning:S.maxRunning,retries:S.retries,failedTotal:S.failed,pending:null};})()`);
const t0=Date.now();let sawScan=false,endAt=0,clicked=false;let last='';const log=[];
let maxBlankDuringScan=0,maxRunningSeen=0;
while(Date.now()-t0<seconds*1000){
 if(!clicked){clicked=true;void js(`document.querySelector('.settings-actions button.btn-primary, button.btn-primary')?.click()`);}
 const s=await sample();
 if(s.running){sawScan=true;maxBlankDuringScan=Math.max(maxBlankDuringScan,s.blank);}
 maxRunningSeen=Math.max(maxRunningSeen,s.maxRunning);
 // Scroll back and forth during the scan so covers are in flight when the scan clears the loader.
 if(s.running)await js(`(()=>{const l=document.querySelector('.queue-list');l.scrollTop=(l.scrollTop+560)%(l.scrollHeight-l.clientHeight);})()`);
 if(sawScan&&!s.running&&!endAt){endAt=Date.now();await js(`document.querySelector('.queue-list').scrollTop=0`);}
 const key=JSON.stringify({...s,top:undefined,visited:undefined});
 if(key!==last){last=key;const line={t:Date.now()-t0,...s};log.push(line);console.log(JSON.stringify(line));}
 if(endAt&&Date.now()-endAt>6000)break;
 await sleep(200);
}
console.log(JSON.stringify({summary:{sawScan,maxBlankDuringScan,final:log.at(-1)}}));
ws.close();
