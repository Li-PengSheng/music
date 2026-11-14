// Real WebView2 queue-toggle profiler. Drives the running desktop app over CDP and prints per-scenario metrics.
// Start the built exe with WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9333, then:
//   node tests/perf/queue-profile.mjs <label> [scenarios=C,A,B,D] [port=9333]
import {writeFileSync,mkdirSync} from 'node:fs';

const [label='run',scenarioArg='C,A,B,D',port='9333']=process.argv.slice(2);
const targets=await (await fetch(`http://127.0.0.1:${port}/json`)).json();
const page=targets.find(t=>t.type==='page');
if(!page)throw new Error('no page target');
const ws=new WebSocket(page.webSocketDebuggerUrl);
await new Promise((ok,fail)=>{ws.onopen=ok;ws.onerror=fail;});
let seq=0;const pending=new Map();let traceChunks=[];let traceDone=null;
ws.onmessage=({data})=>{
 const msg=JSON.parse(data);
 if(msg.id&&pending.has(msg.id)){const {ok,fail}=pending.get(msg.id);pending.delete(msg.id);msg.error?fail(new Error(msg.error.message)):ok(msg.result);}
 else if(msg.method==='Tracing.dataCollected')traceChunks.push(...msg.params.value);
 else if(msg.method==='Tracing.tracingComplete')traceDone?.();
};
const send=(method,params={})=>new Promise((ok,fail)=>{const id=++seq;pending.set(id,{ok,fail});ws.send(JSON.stringify({id,method,params}));});
async function js(expression){const r=await send('Runtime.evaluate',{expression,awaitPromise:true,returnByValue:true});if(r.exceptionDetails)throw new Error(r.exceptionDetails.exception?.description??r.exceptionDetails.text);return r.result.value;}
const sleep=ms=>new Promise(r=>setTimeout(r,ms));
const q=s=>JSON.stringify(s);
if(process.env.RELOAD){await send('Page.reload');await sleep(4000);}

// In-page probes: frame intervals, long tasks, DOM mutations by region, layout reads and lyric scrolls.
async function installProbe(){
await js(`(()=>{if(window.__probe)return;const p=window.__probe={frames:[],longtasks:[],mut:{lyrics:0,queue:0,other:0},reads:0,scrolls:{instant:0,smooth:0},transitions:0,on:false};
 document.addEventListener('transitionrun',()=>{if(p.on)p.transitions++;},true);
 const loop=t=>{if(p.on)p.frames.push(t);requestAnimationFrame(loop);};requestAnimationFrame(loop);
 try{new PerformanceObserver(l=>{if(p.on)for(const e of l.getEntries())p.longtasks.push(Math.round(e.duration));}).observe({type:'longtask',buffered:false});}catch{}
 new MutationObserver(list=>{if(!p.on)return;for(const m of list){const el=m.target.nodeType===1?m.target:m.target.parentElement;if(el?.closest('.lyric-lines'))p.mut.lyrics++;else if(el?.closest('.queue'))p.mut.queue++;else p.mut.other++;}}).observe(document.body,{subtree:true,childList:true,attributes:true,characterData:true});
 const wrapGet=(proto,key)=>{const d=Object.getOwnPropertyDescriptor(proto,key);Object.defineProperty(proto,key,{...d,get(){if(p.on)p.reads++;return d.get.call(this);}});};
 for(const k of ['offsetTop','offsetHeight','clientHeight','scrollHeight'])wrapGet(Element.prototype.hasOwnProperty(k)?Element.prototype:HTMLElement.prototype,k);
 const gbr=Element.prototype.getBoundingClientRect;Element.prototype.getBoundingClientRect=function(){if(p.on)p.reads++;return gbr.call(this);};
 const st=Element.prototype.scrollTo;Element.prototype.scrollTo=function(o){if(p.on&&this.classList?.contains('lyric-lines'))p.scrolls[o?.behavior==='smooth'?'smooth':'instant']++;return st.apply(this,arguments);};
})()`);
// Isolation experiments: INJECT_CSS adds a temporary stylesheet to the running page (removed when the run ends).
await js(`document.getElementById('__perf_css')?.remove()`);
// DELETE_RULES removes matching rules from the loaded stylesheets for this page session only.
if(process.env.DELETE_RULES)console.log('deleted rules',await js(`(()=>{const re=new RegExp(${q(process.env.DELETE_RULES)});let n=0;const walk=list=>{for(let i=list.length-1;i>=0;i--){const r=list[i];if(r.cssRules&&!r.selectorText)walk(r.cssRules);else if(r.selectorText&&re.test(r.selectorText)){(r.parentRule??r.parentStyleSheet).deleteRule(i);n++;}}};for(const s of document.styleSheets)walk(s.cssRules);return n;})()`));
if(process.env.INJECT_CSS)await js(`(()=>{const s=document.createElement('style');s.id='__perf_css';s.textContent=${q(process.env.INJECT_CSS)};document.head.append(s);})()`);
}
await installProbe();
const click=sel=>js(`(()=>{const el=document.querySelector(${q(sel)});if(!el)throw new Error('missing '+${q(sel)});el.click();return true;})()`);
const clickText=(sel,text)=>js(`(()=>{const el=[...document.querySelectorAll(${q(sel)})].find(e=>e.textContent.includes(${q(text)}));if(!el)throw new Error('missing '+${q(text)});el.click();})()`);
const state=()=>js(`(()=>{const a=document.querySelector('.app');return {w:innerWidth,h:innerHeight,dpr:devicePixelRatio,immersive:!!document.querySelector('.immersive'),queue:a.classList.contains('queue-open'),reduce:a.classList.contains('reduce-motion'),playing:document.querySelector('.pb-play')?.getAttribute('aria-label')==='暂停',lyricRows:document.querySelectorAll('.lyric-lines>button').length,queueRows:document.querySelectorAll('.queue-item').length,theme:document.documentElement.dataset.theme,ambient:!!document.querySelector('.immersive.has-ambient')};})()`);
const QUEUE='button[aria-label="播放队列"]';
async function setQueue(open){const s=await state();if(s.queue!==open){await click(QUEUE);await sleep(700);}}
async function setImmersive(on){const s=await state();if(s.immersive!==on){if(on)await click('button[aria-label="正在播放与歌词 (Ctrl+L)"]');else await click('button[aria-label="收起正在播放 (Esc)"]');await sleep(900);}}
async function setPlaying(on){const s=await state();if(s.playing!==on){await click('.pb-play');await sleep(500);}}
async function setReduce(on){const s=await state();if(s.reduce===on)return;const wasImmersive=s.immersive;if(wasImmersive)await setImmersive(false);
 await clickText('.nav-item','设置');await sleep(500);
 await js(`(()=>{const l=[...document.querySelectorAll('label.switch')].find(e=>e.textContent.includes('减少动态效果'));l.querySelector('input').click();})()`);await sleep(300);
 await clickText('.nav-item','歌曲');await sleep(600);if(wasImmersive)await setImmersive(true);}

const CATS=['devtools.timeline','disabled-by-default-devtools.timeline','disabled-by-default-devtools.timeline.frame','blink.user_timing','toplevel','v8.execute','benchmark','rail'].join(',');
async function record(fn){
 traceChunks=[];await send('Tracing.start',{categories:CATS,transferMode:'ReportEvents'});
 await sleep(200);await fn();await sleep(200);
 const done=new Promise(r=>traceDone=r);await send('Tracing.end');await done;return traceChunks;
}

async function actions(kind){
 const marks=[];
 const act=async(name,fn,wait)=>{await js(`(()=>{const p=window.__probe;p.frames=[];p.longtasks=[];p.mut={lyrics:0,queue:0,other:0};p.reads=0;p.transitions=0;p.scrolls={instant:0,smooth:0};p.on=true;performance.mark('A:${name}:start');})()`);
  await fn();await sleep(wait);
  const probe=await js(`(()=>{const p=window.__probe;p.on=false;performance.mark('A:${name}:end');return JSON.parse(JSON.stringify(p));})()`);marks.push({name,probe});};
 await act('idle',async()=>{},700);
 for(let i=1;i<=3;i++){await act('open'+i,()=>click(QUEUE),700);await act('close'+i,()=>click(QUEUE),700);}
 if(kind!=='light')await act('rapid',async()=>{for(let i=0;i<8;i++){await click(QUEUE);await sleep(60);}},900);
 // Steady state with the drawer open: what each 200 ms player poll costs while the queue is visible.
 await click(QUEUE);await sleep(800);await act('openIdle',async()=>{},1000);await click(QUEUE);await sleep(700);
 return marks;
}

function analyse(events,marks){
 const main=events.find(e=>e.name==='thread_name'&&e.args?.name==='CrRendererMain'&&events.some(x=>x.pid===e.pid&&x.name==='A:idle:start'));
 const pid=main?.pid,tid=main?.tid;
 const mk=Object.fromEntries(events.filter(e=>e.pid===pid&&e.name?.startsWith('A:')).map(e=>[e.name,e.ts]));
 const thread=events.filter(e=>e.pid===pid&&e.tid===tid&&e.ph==='X').sort((a,b)=>a.ts-b.ts);
 const frames=events.filter(e=>e.name==='PipelineReporter'&&e.ph==='b'&&e.pid===pid);
 const SCRIPT=new Set(['FunctionCall','EvaluateScript','TimerFire','FireAnimationFrame','EventDispatch','RunMicrotasks','V8.Execute','v8.callFunction','FireIdleCallback']);
 return marks.map(({name,probe})=>{
  const from=mk[`A:${name}:start`],to=mk[`A:${name}:end`];
  const inWin=thread.filter(e=>e.ts>=from&&e.ts<=to);
  const sum=n=>{const l=inWin.filter(e=>e.name===n);return {n:l.length,ms:+(l.reduce((s,e)=>s+e.dur,0)/1000).toFixed(1),max:+(Math.max(0,...l.map(e=>e.dur))/1000).toFixed(1)};};
  // Top-level script time only (nested script events are not double-counted); layouts inside script are forced.
  let script=0,forced=0;const stack=[];
  for(const e of inWin){while(stack.length&&stack.at(-1).ts+stack.at(-1).dur<=e.ts)stack.pop();const inScript=stack.some(s=>SCRIPT.has(s.name));
   if(SCRIPT.has(e.name)&&!inScript)script+=e.dur;if((e.name==='Layout'||e.name==='UpdateLayoutTree')&&inScript)forced++;stack.push(e);}
  const tasks=inWin.filter(e=>e.name==='RunTask'||e.name==='ThreadControllerImpl::RunTask');
  const fr=probe.frames;const gaps=fr.slice(1).map((t,i)=>t-fr[i]);
  const states={};for(const f of frames.filter(f=>f.ts>=from&&f.ts<=to)){const s=f.args?.chrome_frame_reporter?.state??'?';states[s]=(states[s]??0)+1;}
  return {name,layout:sum('Layout'),style:sum('UpdateLayoutTree'),paint:sum('Paint'),prePaint:sum('PrePaint'),layerize:sum('Layerize'),commit:sum('Commit'),scriptMs:+(script/1000).toFixed(1),forcedLayouts:forced,
   longTasks:tasks.filter(t=>t.dur>50000).length,maxTaskMs:+(Math.max(0,...tasks.map(t=>t.dur))/1000).toFixed(1),longtaskApi:probe.longtasks,
   rafFrames:fr.length,maxGapMs:+(Math.max(0,...gaps)).toFixed(1),janky:gaps.filter(g=>g>25).length,fps:gaps.length?+(1000/(gaps.reduce((a,b)=>a+b,0)/gaps.length)).toFixed(1):0,
   pipeline:states,mutations:probe.mut,transitions:probe.transitions,layoutReads:probe.reads,lyricScrolls:probe.scrolls};
 });
}

const scenarios={
 C:async()=>{await setImmersive(false);await setPlaying(true);await setReduce(false);},
 A:async()=>{await setReduce(false);await setImmersive(true);await setPlaying(true);},
 B:async()=>{await setReduce(true);await setImmersive(true);await setPlaying(true);},
 D:async()=>{await setReduce(false);await setImmersive(true);await setPlaying(false);},
 // A and B again in the light theme; the stored theme choice is restored at the end of the run.
 AL:async()=>{await setTheme('light');await setReduce(false);await setImmersive(true);await setPlaying(true);},
 BL:async()=>{await setTheme('light');await setReduce(true);await setImmersive(true);await setPlaying(true);},
};
const savedTheme=await js(`localStorage.getItem('music-mine-theme')`);
async function setTheme(choice){if(await js(`document.documentElement.dataset.theme`)===choice)return;await js(`localStorage.setItem('music-mine-theme',${q(choice)})`);await send('Page.reload');await sleep(4000);await installProbe();}
const initial=await state();
console.log('initial',JSON.stringify(initial));
await setQueue(false);
const out={label,initial,results:{}};
mkdirSync('tests/perf/out',{recursive:true});
for(const key of scenarioArg.split(',')){
 await scenarios[key]();await sleep(800);
 const env=await state();let marks;
 const events=await record(async()=>{marks=await actions();});
 writeFileSync(`tests/perf/out/${label}-${key}.trace.json`,JSON.stringify({traceEvents:events}));
 out.results[key]={env,rows:analyse(events,marks)};
 console.log(`\n== ${key}`,JSON.stringify(env));
 for(const r of out.results[key].rows)console.log([r.name.padEnd(6),`style ${r.style.ms}/${r.style.max}`,`layout ${r.layout.ms}`,`paint ${r.paint.ms}`,`script ${r.scriptMs}`,`forced ${r.forcedLayouts}`,`maxTask ${r.maxTaskMs}`,`long ${r.longTasks}`,`fps ${r.fps}`,`gap ${r.maxGapMs}`,`jank ${r.janky}`,`trans ${r.transitions}`,`mut L${r.mutations.lyrics}/Q${r.mutations.queue}/O${r.mutations.other}`,`reads ${r.layoutReads}`,`scroll ${r.lyricScrolls.instant}+${r.lyricScrolls.smooth}`].join(' | '));
}
if(await js(`localStorage.getItem('music-mine-theme')`)!==savedTheme){await js(savedTheme===null?`localStorage.removeItem('music-mine-theme')`:`localStorage.setItem('music-mine-theme',${q(savedTheme)})`);await send('Page.reload');await sleep(4000);await installProbe();}
await setReduce(initial.reduce);await setImmersive(initial.immersive);await setPlaying(initial.playing);await setQueue(initial.queue);
await js(`document.getElementById('__perf_css')?.remove()`);
writeFileSync(`tests/perf/out/${label}.json`,JSON.stringify(out,null,1));
ws.close();
