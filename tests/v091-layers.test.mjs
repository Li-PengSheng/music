import {test} from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createDismissal,outsideBox} from '../src/lib/utils/modal.ts';
import {escapeLayer} from '../src/lib/utils/layers.ts';
import {visibleRange,scrollToRow} from '../src/lib/utils/virtual-range.ts';

const box={left:420,right:860,top:280,bottom:520};
function harness({dismissible=true,popover=false}={}){
 const node={getBoundingClientRect:()=>box};const inner={};let closed=0;
 const d=createDismissal(node,{canDismiss:()=>dismissible,popoverOpen:()=>popover,dismiss:()=>closed++});
 const at=(target,x,y)=>({target,clientX:x,clientY:y,button:0});
 return {node,inner,d,at,closed:()=>closed};
}

test('modal: backdrop click closes, clicks inside never do',()=>{
 const h=harness();
 h.d.pointerdown(h.at(h.inner,500,300));h.d.click(h.at(h.inner,500,300));
 assert.equal(h.closed(),0,'content click');
 h.d.pointerdown(h.at(h.node,500,300));h.d.click(h.at(h.node,500,300));
 assert.equal(h.closed(),0,'dialog box itself (padding/gap) is inside');
 h.d.pointerdown(h.at(h.node,40,40));h.d.click(h.at(h.node,40,40));
 assert.equal(h.closed(),1,'backdrop');
 assert.equal(outsideBox(box,419,300),true);assert.equal(outsideBox(box,420,300),false);
});

test('modal: drag out of an input or a light-dismissed popover does not close',()=>{
 const h=harness();
 h.d.pointerdown(h.at(h.inner,500,300));h.d.click(h.at(h.node,40,40));
 assert.equal(h.closed(),0,'selection drag that ends on the backdrop');
 const p=harness({popover:true});
 p.d.pointerdown(p.at(p.node,40,40));p.d.click(p.at(p.node,40,40));
 assert.equal(p.closed(),0,'first click only closes the open select/menu');
 const right=harness();
 right.d.pointerdown({...right.at(right.node,40,40),button:2});right.d.click(right.at(right.node,40,40));
 assert.equal(right.closed(),0,'secondary button');
});

test('modal: Escape closes once; IME composition and busy dialogs stay open',()=>{
 const h=harness();
 const key=(extra={})=>({key:'Escape',isComposing:false,defaultPrevented:false,prevented:false,preventDefault(){this.prevented=true;},...extra});
 const ime=key({isComposing:true});h.d.keydown(ime);assert.equal(h.closed(),0);assert.equal(ime.prevented,false);
 const handled=key({defaultPrevented:true});h.d.keydown(handled);assert.equal(h.closed(),0,'a popover inside already handled it');
 const esc=key();h.d.keydown(esc);assert.equal(h.closed(),1);assert.equal(esc.prevented,true,'native cancel suppressed: no double close');
 h.d.keydown({key:'Enter',preventDefault(){}});assert.equal(h.closed(),1);
 const busy=harness({dismissible:false});
 const cancel={prevented:false,preventDefault(){this.prevented=true;}};
 busy.d.cancel(cancel);busy.d.keydown(key());busy.d.pointerdown(busy.at(busy.node,1,1));busy.d.click(busy.at(busy.node,1,1));
 assert.equal(busy.closed(),0);assert.equal(cancel.prevented,true);
});

test('modal component: dialogs share Modal and it restores focus',()=>{
 const modal=readFileSync('src/lib/components/Modal.svelte','utf8');
 assert.match(modal,/showModal\(\)/);
 assert.match(modal,/previous\.focus\(/);
 for(const file of ['src/lib/components/Dialog.svelte','src/lib/TrackDetails.svelte']){
  const source=readFileSync(file,'utf8');
  assert.match(source,/<Modal /,file);
  assert.doesNotMatch(source,/<dialog|showModal/,file+' must not manage its own <dialog>');
 }
});

test('escape closes only the topmost layer: queue above Now Playing above toast',()=>{
 assert.equal(escapeLayer({queue:true,immersive:true,toast:true}),'queue');
 assert.equal(escapeLayer({queue:false,immersive:true,toast:true}),'immersive');
 assert.equal(escapeLayer({queue:false,immersive:false,toast:true}),'toast');
 assert.equal(escapeLayer({queue:false,immersive:false,toast:false}),null);
});

test('queue is an app-level layer drawn above Now Playing',()=>{
 const app=readFileSync('src/App.svelte','utf8');
 const content=app.slice(app.indexOf('<div class="content"'),app.indexOf('</main>'));
 assert.doesNotMatch(content,/<Queue/,'not nested in the page content');
 assert.ok(app.indexOf('<Queue ')>app.indexOf('class="immersive-layer"'),'rendered after the immersive layer');
 const tokens=readFileSync('src/lib/styles/tokens.css','utf8');
 const z=name=>Number(tokens.match(new RegExp(`--z-${name}:(\\d+)`))[1]);
 assert.ok(z('queue')>z('queue-scrim')&&z('queue-scrim')>z('immersive')&&z('immersive')>z('player'));
});

test('song table rows 1/50/200/460/471 stay inside the virtual window',()=>{
 const ROW=56,TOTAL=471,listTop=180,viewport=620,head=36;
 const maxScroll=listTop+TOTAL*ROW+32-viewport;
 for(const n of [1,50,200,460,471]){
  const index=n-1;
  const target=Math.min(maxScroll,Math.max(0,scrollToRow(index,0,viewport,listTop,ROW,head)??0));
  const range=visibleRange(target,viewport,listTop,ROW,TOTAL);
  assert.ok(range.start<=index&&index<range.end,`row ${n} rendered at scrollTop ${target}`);
  assert.ok(listTop+index*ROW+ROW<=target+viewport,`row ${n} bottom inside the viewport`);
 }
 assert.deepEqual(visibleRange(maxScroll,viewport,listTop,ROW,TOTAL).end,TOTAL,'last row rendered at the very bottom');
 assert.deepEqual(visibleRange(0,viewport,listTop,ROW,TOTAL).start,0,'back at the top after a fast scroll');
});
