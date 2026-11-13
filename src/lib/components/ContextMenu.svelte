<script lang="ts">
 import {tick} from 'svelte';
 import {Check} from 'lucide-svelte';
 import {ui,closeMenu,notifyError,type MenuItem} from '../stores/ui.svelte';
 import {placeAtPoint,placeAtAnchor} from '../utils/menu-position';

 let panel=$state<HTMLDivElement>();
 let menu=$derived(ui.menu);

 $effect(()=>{
  const current=menu;const node=panel;
  if(!current||!node)return;
  node.showPopover?.();
  node.style.left='0px';node.style.top='0px';node.style.maxHeight='';
  const w=node.offsetWidth,h=node.scrollHeight,vw=window.innerWidth,vh=window.innerHeight;
  const at=current.at;
  const p='width' in at?placeAtAnchor(at,w,h,vw,vh):placeAtPoint(at.x,at.y,w,h,vw,vh);
  node.style.left=p.left+'px';node.style.top=p.top+'px';node.style.maxHeight=p.maxHeight+'px';
  void tick().then(()=>items()[0]?.focus({preventScroll:true}));
 });

 $effect(()=>{
  if(!menu)return;
  const outside=(e:Event)=>{if(panel&&!panel.contains(e.target as Node))closeMenu();};
  const scrolled=(e:Event)=>{if(panel&&!panel.contains(e.target as Node))closeMenu();};
  const dismiss=()=>closeMenu();
  document.addEventListener('pointerdown',outside,true);
  document.addEventListener('scroll',scrolled,true);
  window.addEventListener('resize',dismiss);
  window.addEventListener('blur',dismiss);
  document.addEventListener('music-menu-open',dismiss);
  return ()=>{document.removeEventListener('pointerdown',outside,true);document.removeEventListener('scroll',scrolled,true);window.removeEventListener('resize',dismiss);window.removeEventListener('blur',dismiss);document.removeEventListener('music-menu-open',dismiss);};
 });

 const items=()=>Array.from(panel?.querySelectorAll<HTMLButtonElement>('button[role=menuitem]:not(:disabled)')??[]);

 function key(e:KeyboardEvent){
  if(e.isComposing)return;
  const list=items();const index=list.indexOf(document.activeElement as HTMLButtonElement);
  let next=-1;
  if(e.key==='ArrowDown')next=(index+1)%list.length;
  else if(e.key==='ArrowUp')next=(index-1+list.length)%list.length;
  else if(e.key==='Home')next=0;
  else if(e.key==='End')next=list.length-1;
  else if(e.key==='Escape'){e.preventDefault();e.stopPropagation();closeMenu(true);return;}
  else if(e.key==='Tab'){e.preventDefault();closeMenu(true);return;}
  else if(e.key.length===1&&!e.ctrlKey&&!e.altKey){const k=e.key.toLowerCase();next=list.findIndex((b,i)=>i>index&&b.textContent?.trim().toLowerCase().startsWith(k));if(next<0)next=list.findIndex(b=>b.textContent?.trim().toLowerCase().startsWith(k));}
  if(next>=0){e.preventDefault();e.stopPropagation();list[next]?.focus();}
 }

 async function run(item:Extract<MenuItem,{label:string;action:()=>unknown}>){
  closeMenu(true);
  try{await item.action();}catch(e){notifyError(e);}
 }
</script>

{#if menu}
 {#key menu.id}
  <div bind:this={panel} class="menu" popover="manual" role="menu" aria-label={menu.label} tabindex="-1" onkeydown={key} oncontextmenu={(e)=>e.preventDefault()}>
   {#each menu.items as item, i (i)}
    {#if item.kind==='separator'}<div class="menu-separator" role="separator"></div>
    {:else if item.kind==='heading'}<div class="menu-heading" role="presentation">{item.label}</div>
    {:else}
     <button type="button" role="menuitem" class="menu-item" class:danger={item.danger} disabled={item.disabled} tabindex="-1" onclick={()=>run(item)}>
      <span class="menu-icon" aria-hidden="true">{#if item.icon}<item.icon size={16}/>{/if}</span>
      <span class="menu-label">{item.label}</span>
      {#if item.checked}<Check size={15} class="menu-check" aria-label="已选"/>{:else if item.hint}<kbd>{item.hint}</kbd>{/if}
     </button>
    {/if}
   {/each}
  </div>
 {/key}
{/if}
