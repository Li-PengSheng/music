<script lang="ts">
 import {untrack} from 'svelte';
 import {Play,Search,Disc3,MicVocal} from 'lucide-svelte';
 import Cover from '../components/Cover.svelte';
 import EmptyState from '../components/EmptyState.svelte';
 import Select from '../components/Select.svelte';
 import {tooltip} from '../components/tooltip';
 import {library} from '../stores/library.svelte';
 import {ui,openMenu} from '../stores/ui.svelte';
 import {go,memory} from '../stores/nav.svelte';
 import {playCollection,albumMenu,artistMenu} from '../actions';
 import type {Group} from '../services/music';
 import {initial,nameHue} from '../utils/format';

 let {kind}:{kind:'albums'|'artists'}=$props();

 let sort=$state(untrack(()=>memory.get(kind)?.sort??'name'));
 function changeSort(next:string){sort=next;memory.set(kind,{scroll:0,...memory.get(kind),sort:next});}

 let groups=$derived.by(()=>{
  const q=ui.query.trim().toLowerCase();
  const source=kind==='albums'?library.summary.albums:library.summary.artists;
  const list=q?source.filter(g=>(g.name+' '+g.artist).toLowerCase().includes(q)):source.slice();
  return list.sort((a,b)=>sort==='count'?b.count-a.count||a.name.localeCompare(b.name,'zh-CN'):a.name.localeCompare(b.name,'zh-CN'));
 });

 // Render progressively so huge libraries do not mount thousands of cards at once.
 let limit=$state(120);
 function sentinel(node:HTMLElement){
  const observer=new IntersectionObserver(entries=>{if(entries.some(e=>e.isIntersecting))limit+=120;},{rootMargin:'600px'});
  observer.observe(node);return {destroy(){observer.disconnect();}};
 }

 const open=(g:Group)=>kind==='albums'?go({view:'album',key:g.key}):go({view:'artist',name:g.name});
 const play=(g:Group)=>playCollection(kind==='albums'?{album:g.key,sort:'album'}:{artist:g.name,sort:'album'});
 const menu=(e:MouseEvent|HTMLElement,g:Group)=>openMenu(e,kind==='albums'?albumMenu(g):artistMenu(g),g.name+' 操作');
</script>

<header class="page-header">
 <div>
  <h2 class="page-title">{kind==='albums'?'专辑':'艺术家'}</h2>
  <p class="page-meta">{groups.length} {kind==='albums'?'张专辑':'位艺术家'}{#if ui.query.trim()} · 搜索“{ui.query.trim()}”{/if}</p>
 </div>
 <div class="page-actions">
  <span class="toolbar-label">排序</span>
  <Select label="排序方式" value={sort} onchange={changeSort} options={[{value:'name',label:'名称'},{value:'count',label:'歌曲数量'}]}/>
 </div>
</header>

{#if !groups.length}
 {#if ui.query.trim()}<EmptyState icon={Search} title={kind==='albums'?'没有找到匹配的专辑':'没有找到匹配的艺术家'} message="试试其他关键词，或在“歌曲”中搜索。"><button type="button" class="btn btn-ghost" onclick={()=>go({view:'songs'})}>在歌曲中搜索</button></EmptyState>
 {:else if library.loaded}<EmptyState icon={kind==='albums'?Disc3:MicVocal} title={kind==='albums'?'还没有专辑':'还没有艺术家'} message="导入音乐文件夹后会自动整理。"/>{/if}
{:else}
 <div class="card-grid" class:artists={kind==='artists'}>
  {#each groups.slice(0,limit) as g (g.key)}
   <article class="card" oncontextmenu={(e)=>menu(e,g)}>
    <div class="card-art" class:round={kind==='artists'}>
     {#if kind==='albums'}<Cover id={g.cover_id} alt={g.name} size={320}/>
     {:else}<span class="artist-initial" style:--hue={nameHue(g.name)} aria-hidden="true">{initial(g.name)}</span>{/if}
     <button type="button" class="card-open" aria-label={'打开 '+g.name} onclick={()=>open(g)}></button>
     <button type="button" class="card-play" aria-label={'播放 '+g.name} use:tooltip={'播放'} onclick={()=>play(g)}><Play size={18} fill="currentColor"/></button>
    </div>
    <button type="button" class="card-title" onclick={()=>open(g)} use:tooltip={{text:g.name,overflow:true}}>{g.name||'未知'}</button>
    <p class="card-sub" use:tooltip={{text:g.artist,overflow:true}}>{kind==='albums'?(g.artist||'未知艺术家'):`${g.count} 首歌曲`}</p>
   </article>
  {/each}
 </div>
 {#if limit<groups.length}<div class="grid-sentinel" use:sentinel aria-hidden="true"></div>{/if}
{/if}
