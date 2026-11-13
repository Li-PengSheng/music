import {test} from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {toPercent,toGain,muteTarget,latest,DEFAULT_PERCENT} from '../src/lib/utils/volume.ts';

const read=f=>readFileSync(f,'utf8');

test('percent is a presentation of the same 0.0–1.0 gain Rust uses (no new curve)',()=>{
 for(const p of [0,1,35,50,99,100])assert.equal(toPercent(toGain(p)),p,'round-trips');
 assert.equal(toPercent(.35),35);assert.equal(toPercent(.354),35);assert.equal(toPercent(.355),36);
 assert.equal(toPercent(1.4),100);assert.equal(toPercent(-1),0);assert.equal(toPercent(Number.NaN),0);
 assert.equal(toGain(150),1);assert.equal(toGain(-5),0);
});

test('mute keeps the previous level and unmute restores it',()=>{
 assert.equal(muteTarget(35,35),0,'audible -> mute');
 assert.equal(muteTarget(0,35),35,'muted -> previous level');
 assert.equal(muteTarget(0,0),DEFAULT_PERCENT,'nothing remembered -> sensible default, never stays silent');
});

/** Fake backend: each call resolves only when the test says so, like a slow IPC round-trip. */
function backend(){
 const calls=[];const waiting=[];
 return {calls,send:v=>{calls.push(v);return new Promise(r=>waiting.push(()=>r({volume:v/100})));},
  async flush(){while(waiting.length){waiting.shift()();await new Promise(r=>setImmediate(r));}}};
}

test('slider drags coalesce: one call in flight, the final value always wins',async()=>{
 const b=backend();const results=[];let settled=0;
 const sync=latest(b.send,{result:r=>results.push(r.volume),error:assert.fail,settled:()=>settled++});
 for(const v of [10,20,30,40,55])sync.push(v);
 assert.deepEqual(b.calls,[10],'first value goes out immediately');
 assert.equal(sync.busy,true);
 await b.flush();
 assert.deepEqual(b.calls,[10,55],'intermediate values dropped, last value sent');
 assert.deepEqual(results,[.1,.55],'every reply is applied, ending at the real final volume');
 assert.equal(settled,1);assert.equal(sync.busy,false);
});

test('repeated identical values in a burst are sent once; a later burst can resend',async()=>{
 const b=backend();
 const sync=latest(b.send,{result:()=>{},error:assert.fail,settled:()=>{}});
 sync.push(40);sync.push(40);await b.flush();sync.push(40);await b.flush();
 assert.deepEqual(b.calls,[40,40],'oninput+onchange with the same value collapse; a new interaction is not swallowed');
});

test('a failed call reports the error and still settles so the UI falls back to the real state',async()=>{
 let errors=0,settled=0;
 const sync=latest(()=>Promise.reject(new Error('device lost')),{result:assert.fail,error:()=>errors++,settled:()=>settled++});
 sync.push(30);await new Promise(r=>setImmediate(r));
 assert.equal(errors,1);assert.equal(settled,1);assert.equal(sync.busy,false);
});

test('switching theme never touches volume; volume changes never reload artwork',()=>{
 const prefs=read('src/lib/stores/preferences.ts');
 assert.doesNotMatch(prefs,/setVolume|volume/i,'theme code has no path to the player volume');
 const app=read('src/App.svelte');
 assert.match(app,/let artworkTrackId=\$derived\(snapshot\?\.track_id\);/,'cover effect keys on the track only');
 assert.match(app,/\$effect\(\(\)=>\{const path=artworkTrackId;/);
});

test('player bar shows the real percentage and keeps volume independent of other commands',()=>{
 const bar=read('src/lib/layout/PlayerBar.svelte');
 assert.match(bar,/let volume=\$derived\(volumeDraft\?\?\(s\?toPercent\(s\.volume\):DEFAULT_PERCENT\)\)/);
 assert.match(bar,/settled:\(\)=>\{volumeDraft=null;\}/,'draft is dropped once Rust confirms');
 assert.match(bar,/<span class="pb-volume-value" aria-hidden="true">\{volume\}%<\/span>/);
 assert.match(bar,/valuetext=\{volume===0\?'0%，已静音':volume\+'%'\}/,'screen readers hear the value');
 assert.doesNotMatch(bar,/control\(\(\)=>setVolume/,'volume no longer queues behind the transport lock');
 assert.doesNotMatch(bar,/pb-volume-slider"[^>]*player\.busy/,'slider is not disabled while another command runs');
 assert.match(bar,/oninput=\{changeVolume\} onchange=\{changeVolume\}/);
 const css=read('src/app.css');
 assert.match(css,/\.pb-volume-value\{[^}]*color:var\(--text-2\)[^}]*font-variant-numeric:tabular-nums/);
 assert.match(css,/\.pb-volume \.slider,\.pb-volume-value\{display:none\}/,'narrow windows fall back to the button tooltip');
});
