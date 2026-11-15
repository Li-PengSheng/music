// Queue cover loading measurements against a running portable exe on the generated corpus.
// Launch first with tests/perf/stage-cover-run.ps1, then:
//   node tests/perf/queue-cover-profile.mjs <label> <scenarios=A,B,C,D,E,F,H> [port=9333]
// Paths and image data never leave the page: rows are identified by the corpus index in the title.
import {writeFileSync,mkdirSync} from 'node:fs';
import {execFileSync} from 'node:child_process';

const [label='run',scenarioArg='A,B,C,D,E,F,H',port='9333']=process.argv.slice(2);
const targets=await (await fetch(`http://127.0.0.1:${port}/json`)).json();
const page=targets.find(t=>t.type==='page');
const ws=new WebSocket(page.webSocketDebuggerUrl);
await new Promise((ok,fail)=>{ws.onopen=ok;ws.onerror=fail;});
let seq=0;const pending=new Map();
// Tauri IPC goes through fetch to http://ipc.localhost/<command>; the Network domain sees every call.
const net=[];const byRequest=new Map();
const rowOf=path=>{const m=/[\\/](\d{3}) [^\\/]*$/.exec(path??'');if(m)return Number(m[1]);const s=/same-name[\\/](\d\d)[\\/]/.exec(path??'');return s?'same'+s[1]:null;};
function onEvent(m){
 const p=m.params;
 if(m.method==='Network.requestWillBeSent'&&p.request.url.startsWith('http://ipc.localhost/')){
  const cmd=decodeURIComponent(p.request.url.slice(21).split('?')[0]);if(!/thumb|artwork/.test(cmd))return;
  let args={};try{args=JSON.parse(p.request.postData??'{}');}catch{}
  const rec={cmd,row:rowOf(args.path),t0:p.timestamp,t1:0,outcome:'pending',bytes:0};net.push(rec);byRequest.set(p.requestId,rec);
 }else if(m.method==='Network.loadingFinished'&&byRequest.has(p.requestId)){
  const rec=byRequest.get(p.requestId);rec.t1=p.timestamp;rec.bytes=p.encodedDataLength;
  send('Network.getResponseBody',{requestId:p.requestId}).then(({body})=>{let v=null;try{v=JSON.parse(body);}catch{}
   const data=typeof v==='string'?v:v?.data;rec.outcome=data?'data':v&&typeof v==='object'?'none:'+(v.status??'?'):'null';rec.source=v?.source;rec.timing=v?.timing;}).catch(()=>{rec.outcome='body?';});
 }else if(m.method==='Network.loadingFailed'&&byRequest.has(p.requestId)){const rec=byRequest.get(p.requestId);rec.t1=p.timestamp;rec.outcome='error';}
}
ws.onmessage=({data})=>{const m=JSON.parse(data);if(m.id&&pending.has(m.id)){const {ok,fail}=pending.get(m.id);pending.delete(m.id);m.error?fail(new Error(m.error.message)):ok(m.result);}else if(m.method)onEvent(m);};
const send=(method,params={})=>new Promise((ok,fail)=>{const id=++seq;pending.set(id,{ok,fail});ws.send(JSON.stringify({id,method,params}));});
async function js(expression){const r=await send('Runtime.evaluate',{expression,awaitPromise:true,returnByValue:true});if(r.exceptionDetails)throw new Error(r.exceptionDetails.exception?.description??r.exceptionDetails.text);return r.result.value;}
const sleep=ms=>new Promise(r=>setTimeout(r,ms));
const q=s=>JSON.stringify(s);

// Process CPU time and working set of this run's exe and its WebView2 children.
function processStats(){
 const script=`$r=Get-CimInstance Win32_Process | ? { $_.Name -eq 'music-mine-desktop.exe' -or ($_.Name -eq 'msedgewebview2.exe' -and $_.CommandLine -like '*mm-cover-corpus*run-${label}*') }; $cpu=($r | % { $_.KernelModeTime + $_.UserModeTime } | Measure-Object -Sum).Sum/10000; $ws=($r | Measure-Object WorkingSetSize -Sum).Sum/1MB; "$([math]::Round($cpu)) $([math]::Round($ws,1)) $($r.Count)"`;
 const [cpuMs,wsMb,procs]=execFileSync('powershell',['-NoProfile','-Command',script],{encoding:'utf8'}).trim().split(' ').map(Number);
 return {cpuMs,wsMb,procs};
}
function cacheFiles(){
 const script=`$d=Join-Path $env:TEMP 'mm-cover-corpus\\run-${label}\\app-data\\caches\\covers'; $f=Get-ChildItem $d -File -ErrorAction SilentlyContinue; "$(($f|?{$_.Extension -eq '.jpg'}).Count) $(($f|?{$_.Extension -eq '.none'}).Count) $(($f|?{$_.Extension -notin '.jpg','.none'}).Count)"`;
 const [jpg,none,other]=execFileSync('powershell',['-NoProfile','-Command',script],{encoding:'utf8'}).trim().split(' ').map(Number);
 return {jpg,none,other};
}

async function install(){
 await send('Network.enable',{maxPostDataSize:4096});
 await js(`(()=>{if(window.__coverProbe)return;const P=window.__coverProbe={imgs:[],frames:[],on:true};
  const imgEvent=type=>e=>{const img=e.target;if(!(img instanceof HTMLImageElement)||!img.closest('.queue'))return;const row=img.closest('.queue-item');P.imgs.push({type,t:performance.now(),title:row?.querySelector('.queue-text strong')?.textContent});};
  document.addEventListener('load',imgEvent('load'),true);document.addEventListener('error',imgEvent('error'),true);
  const loop=t=>{if(P.on)P.frames.push(t);requestAnimationFrame(loop);};requestAnimationFrame(loop);
 })()`);
}
let windowStart=0;
const reset=()=>{windowStart=net.length;return js(`(()=>{const P=window.__coverProbe;P.imgs=[];P.frames=[];return performance.now();})()`);};
function maxInflight(calls){const ev=[];for(const c of calls){ev.push([c.t0,1]);ev.push([c.t1||Infinity,-1]);}ev.sort((a,b)=>a[0]-b[0]||a[1]-b[1]);let n=0,max=0;for(const [,d] of ev){n+=d;max=Math.max(max,n);}return max;}
// Visible queue rows with their corpus index, kind and what they show right now.
const rows=()=>js(`(()=>{const list=document.querySelector('.queue-list');if(!list)return [];const box=list.getBoundingClientRect();
 return [...list.querySelectorAll('.queue-item')].filter(r=>{const b=r.getBoundingClientRect();return b.bottom>box.top+1&&b.top<box.bottom-1;}).map(r=>{const title=r.querySelector('.queue-text strong')?.textContent??'';const m=/Cover (\\d{3}) (\\S+)/.exec(title);const img=r.querySelector('.queue-cover img');
  let h=0;const src=img?.getAttribute('src')??'';for(let i=0;i<src.length;i+=97)h=(h*31+src.charCodeAt(i))|0;
  const cover=r.querySelector('.queue-cover [data-cover]');const art=r.querySelector('.queue-cover [data-image]');
  return {i:m?Number(m[1]):-1,kind:m?.[2]??'?',state:img?(img.classList.contains('loaded')?'shown':'loading'):'placeholder',cover:cover?.dataset.cover,code:cover?.dataset.coverCode,image:art?.dataset.image,hash:img?h:0};});})()`);
// Frontend loader counters (new build only) and Rust cover counters (new build only).
const loaderStats=()=>js(`globalThis.__musicMineCoverStats?JSON.parse(JSON.stringify(globalThis.__musicMineCoverStats)):null`);
const rustStats=()=>js(`window.__TAURI_INTERNALS__.invoke('library_cover_stats').catch(()=>null)`);
function delta(a,b){if(!a||!b)return null;const out={};for(const k of Object.keys(b)){if(typeof b[k]==='number'){const d=k.endsWith('Max')||k.startsWith('max')?b[k]:b[k]-(a[k]??0);if(d)out[k]=Math.round(d);}else if(k==='codes'){const c={};for(const [x,v] of Object.entries(b.codes)){const d=v-(a.codes?.[x]??0);if(d)c[x]=d;}if(Object.keys(c).length)out.codes=c;}}return out;}
const SHOULD_SHOW=new Set(['jpeg-small','jpeg-3000','png-1000','png-3000-rgba','png-noise-13mb','same-name','webp','gif']);

/** Samples the visible rows every 50 ms for `ms`, and reports when each first showed its cover. */
async function watch(name,ms,started){
 const first=new Map();let last=[];const t0=Date.now();
 while(Date.now()-t0<ms){last=await rows();const now=await js('performance.now()');for(const r of last)if(r.state==='shown'&&!first.has(r.i))first.set(r.i,now-started);await sleep(50);}
 const probe=await js(`JSON.parse(JSON.stringify(window.__coverProbe))`);
 await sleep(100);
 const calls=net.slice(windowStart);const done=calls.filter(c=>c.t1);
 const visible=last.length;const expected=last.filter(r=>SHOULD_SHOW.has(r.kind));
 const shown=last.filter(r=>r.state==='shown');
 const times=[...first.values()].sort((a,b)=>a-b);
 const missing=expected.filter(r=>r.state!=='shown').map(r=>{const c=calls.filter(c=>c.row===r.i&&c.cmd==='library_queue_thumb');return `${r.i}:${r.kind}:${r.state}:${r.cover?r.cover+(r.code?'/'+r.code:''):'requests='+c.length}`;});
 const noCover=last.filter(r=>!SHOULD_SHOW.has(r.kind)).map(r=>`${r.i}:${r.kind}:${r.cover??r.state}${r.code?'/'+r.code:''}`);
 const stuckLoading=last.filter(r=>r.cover==='loading'||r.state==='loading').length;
 const gaps=probe.frames.slice(1).map((t,i)=>t-probe.frames[i]);
 const durations=done.map(c=>(c.t1-c.t0)*1000).sort((a,b)=>a-b);
 const outcomes={};for(const c of calls)outcomes[c.cmd+' '+c.outcome]=(outcomes[c.cmd+' '+c.outcome]??0)+1;
 const sources={};for(const c of done)if(c.source)sources[c.source]=(sources[c.source]??0)+1;
 const base=calls[0]?.t0??0;
 const detail=last.map(r=>{const c=calls.filter(c=>c.row===r.i&&c.cmd==='library_queue_thumb');return {i:r.i,kind:r.kind,state:r.state,shownMs:first.has(r.i)?Math.round(first.get(r.i)):null,requests:c.map(c=>({at:Math.round((c.t0-base)*1000),ms:c.t1?Math.round((c.t1-c.t0)*1000):null,outcome:c.outcome}))};});
 return {name,detail,visible,expected:expected.length,shown:shown.length,firstShownMs:times[0]!=null?Math.round(times[0]):null,allShownMs:times.length&&shown.length===expected.length?Math.round(times.at(-1)):null,
  missing,noCover,stuckLoading,ipc:calls.length,outcomes,sources,maxInflight:maxInflight(calls),ipcP50:Math.round(durations[Math.floor(durations.length/2)]??0),ipcMax:Math.round(durations.at(-1)??0),
  imgErrors:probe.imgs.filter(e=>e.type==='error').length,maxFrameGap:Math.round(Math.max(0,...gaps)),jank:gaps.filter(g=>g>50).length,rows:last};
}

const QUEUE='button[aria-label="播放队列"]';
const click=sel=>js(`document.querySelector(${q(sel)}).click()`);
const queueOpen=()=>js(`document.querySelector('.app').classList.contains('queue-open')`);
async function setQueue(open){if(await queueOpen()!==open){await click(QUEUE);await sleep(400);}}
async function listTo(top){await js(`document.querySelector('.queue-list').scrollTop=${top}`);}
const results=[];
function log(r){const {rows:_,detail:__,...rest}=r;console.log(JSON.stringify(rest));if(process.env.DETAIL)for(const d of r.detail)console.log('   ',JSON.stringify(d));
 if(process.env.ORDER){const c=net.slice(windowStart).filter(c=>c.cmd==='library_queue_thumb');const b=c[0]?.t0??0;console.log('    order',c.map(x=>`${x.row}@${Math.round((x.t0-b)*1000)}`).join(' '));}
 results.push(r);}
async function measure(name,action,ms=5000){
 const before={proc:processStats(),cache:cacheFiles(),loader:await loaderStats(),rust:await rustStats()};const started=await reset();await action();
 const r=await watch(name,ms,started);r.proc={before:before.proc,after:processStats()};r.cache={before:before.cache,after:cacheFiles()};
 r.loader=delta(before.loader,await loaderStats());r.rust=delta(before.rust,await rustStats());log(r);return r;
}

await install();
// A queue of every corpus track (336) in corpus order, so the first screen mixes small and 3000 px JPEGs.
// Rust owns the queue; this only issues the same IPC the UI uses. Library ids follow scan order from 1.
const snap=await js(`window.__TAURI_INTERNALS__.invoke('player_get_state')`);
if((snap.queue?.length??0)<300){
 await js(`window.__TAURI_INTERNALS__.invoke('library_queue_tracks',{ids:Array.from({length:336},(_,i)=>i+1),replace:true})`);
 await sleep(800);
 const scanned=(await js(`window.__TAURI_INTERNALS__.invoke('player_get_state')`)).queue;
 const order=scanned.map((t,k)=>({id:k+1,n:Number(/Cover (\d{3})/.exec(t.title)?.[1]??999)})).sort((a,b)=>a.n-b.n).map(x=>x.id);
 await js(`window.__TAURI_INTERNALS__.invoke('library_queue_tracks',{ids:${q(order)},replace:true})`);
 await sleep(800);
}
console.log('queue length',(await js(`window.__TAURI_INTERNALS__.invoke('player_get_state')`)).queue.length);
await setQueue(false);await sleep(1500);

const scenario={
 A:()=>measure('A cold first open',()=>click(QUEUE),6000),
 B:async()=>{await setQueue(false);await sleep(600);return measure('B warm reopen',()=>click(QUEUE),2500);},
 C:async()=>{await setQueue(true);await listTo(0);await sleep(300);
  const down=await measure('C scroll to end',async()=>{for(let y=0;y<60000;y+=400){await listTo(y);await sleep(40);}},4000);
  await measure('C back to top',async()=>{await listTo(0);},2500);return down;},
 D:async()=>{await setQueue(false);await js(`document.querySelector('button[aria-label="正在播放与歌词 (Ctrl+L)"]')?.click()`);await sleep(900);
  await measure('D lyrics page open',()=>click(QUEUE),3000);await setQueue(false);await js(`document.querySelector('button[aria-label="收起正在播放 (Esc)"]')?.click()`);await sleep(600);},
 E:async()=>{await setQueue(true);await listTo(0);
  return measure('E rapid next x10',async()=>{for(let i=0;i<10;i++){await js(`document.querySelector('button[aria-label="下一首"]')?.click()`);await sleep(150);}},3000);},
 F:async()=>{await setQueue(false);
  const r=await measure('F theme switch then open',async()=>{
   for(const label of ['浅色','深色']){
    await js(`[...document.querySelectorAll('.nav-item')].find(e=>e.textContent.includes('设置')).click()`);await sleep(400);
    await js(`document.querySelector('button.select-trigger[aria-label="主题"]').click()`);await sleep(200);
    await js(`[...document.querySelectorAll('.select-panel>button')].find(b=>b.textContent.includes(${q(label)})).click()`);await sleep(500);
   }
   await click(QUEUE);},3000);
  return r;},
 H:async()=>{await setQueue(true);
  // Row 250 is the first non-JPEG picture; rows 250-299 hold every failure kind of the corpus.
  return measure('H failure kinds',async()=>{await js(`(()=>{const rows=[...document.querySelectorAll('.queue-item')];const r=rows.find(r=>/Cover 250 /.test(r.textContent));document.querySelector('.queue-list').scrollTop=r.offsetTop-8;})()`);},5000);},
 H2:async()=>{await setQueue(true);
  return measure('H2 failure kinds lower',async()=>{await js(`(()=>{const rows=[...document.querySelectorAll('.queue-item')];const r=rows.find(r=>/Cover 270 /.test(r.textContent));document.querySelector('.queue-list').scrollTop=r.offsetTop-8;})()`);},5000);},
 R:()=>measure('R first open after restart',()=>click(QUEUE),4000),
 Q:async()=>{await setQueue(true);await listTo(0);await sleep(300);
  return measure('Q rapid close/reopen x10',async()=>{for(let i=0;i<20;i++){await click(QUEUE);await sleep(60);}},2500);},
 S:async()=>{await setQueue(true);await listTo(0);await sleep(300);return measure('S same rows after recover',async()=>{},1500);},
};
for(const key of scenarioArg.split(','))await scenario[key]();
mkdirSync('tests/perf/out',{recursive:true});
writeFileSync(`tests/perf/out/cover-${label}-${scenarioArg.replace(/,/g,'')}.json`,JSON.stringify(results,null,1));
ws.close();
