<script lang="ts">
 import Select from '../components/Select.svelte';
 import { getLyrics, setLyricsOffset, seek, type Lyrics } from '../services/music';
 import { tick, onMount } from 'svelte';
 import {groupLines,activeLine,scrollTarget} from '../utils/lyrics-view';
 import Artwork from '../components/Artwork.svelte';
 import IconButton from '../components/IconButton.svelte';
 import EmptyState from '../components/EmptyState.svelte';
 import { ChevronDown, Music2, LocateFixed } from 'lucide-svelte';
 import {player,control} from '../stores/player.svelte';
 import {ui,notifyError} from '../stores/ui.svelte';
 import {ambientColor} from '../utils/color';

 let {artwork,onclose}:{artwork:string|null;onclose:()=>void}=$props();
 let snapshot=$derived(player.snapshot);

 let lyrics=$state<Lyrics|null>(null); let loading=$state(false); let manual=$state(false); let layout=$state('split'); let container=$state<HTMLDivElement>(); let request=0;
 let trackId=$derived(snapshot?.track_id);
 $effect(()=>{const path=trackId;const ticket=++request;lyrics=null;manual=false;if(path){loading=true;getLyrics().then(l=>{if(ticket===request)lyrics=l;}).catch(notifyError).finally(()=>{if(ticket===request)loading=false;});}});

 // Ambient colour: computed once per song from the already-loaded cover, never per progress tick.
 const ambientCache=new Map<string,string|null>();
 let ambient=$state<string|null>(null);
 $effect(()=>{
  const id=trackId,src=artwork;
  if(!id||!src){ambient=null;return;}
  if(ambientCache.has(id)){ambient=ambientCache.get(id)!;return;}
  let cancelled=false;
  const img=new Image();img.src=src;
  img.decode().then(()=>{
   const canvas=document.createElement('canvas');canvas.width=canvas.height=24;
   const ctx=canvas.getContext('2d',{willReadFrequently:true});if(!ctx)return;
   ctx.drawImage(img,0,0,24,24);
   const rgb=ambientColor(ctx.getImageData(0,0,24,24).data);
   const value=rgb?rgb.join(' '):null;
   ambientCache.set(id,value);if(ambientCache.size>60)ambientCache.delete(ambientCache.keys().next().value!);
   if(!cancelled)ambient=value;
  }).catch(()=>{});
  return ()=>{cancelled=true;};
 });

 let fontSize=$state(22);
 onMount(()=>{try{const saved=Number(localStorage.getItem('lyrics-font-size'));if(saved>=16&&saved<=32)fontSize=saved;}catch{/* optional preference */}});
 function changeSize(value:number){fontSize=value;try{localStorage.setItem('lyrics-font-size',String(value));}catch{/* keep session preference */}}
 let rows=$derived(groupLines(lyrics?.lines??[],lyrics?.synced??false));
 let active=$derived(lyrics?.synced?activeLine(rows,(snapshot?.position_ms??0)-(snapshot?.lyrics_offset_ms??0)):-1);
 function watchSize(node:HTMLDivElement){const observer=new ResizeObserver(()=>{if(manual)return;const line=node.querySelector<HTMLElement>('[data-line="'+active+'"]');if(line)node.scrollTo({top:scrollTarget(line.offsetTop,line.offsetHeight,node.clientHeight,node.scrollHeight),behavior:'instant'});});observer.observe(node);return {destroy(){observer.disconnect();}};}
 let scrollVersion=0;
 $effect(()=>{const index=active;const browsing=manual;const host=container;rows;fontSize;layout;const version=++scrollVersion;
  if(!browsing&&host){void tick().then(()=>{if(version!==scrollVersion)return;const line=host.querySelector<HTMLElement>('[data-line="'+index+'"]');host.scrollTo({top:line?scrollTarget(line.offsetTop,line.offsetHeight,host.clientHeight,host.scrollHeight):0,behavior:ui.reduceMotion?'instant':'smooth'});});}
 });
 async function jump(time:number){await control(()=>seek(Math.min(snapshot?.duration_ms??0,Math.max(0,time+(snapshot?.lyrics_offset_ms??0)))));manual=false;}
</script>
<section style:--lyric-size={fontSize+'px'} style:--ambient={ambient??undefined} class="immersive" class:has-ambient={!!ambient&&!ui.reduceMotion} aria-label="正在播放">
 <header class="immersive-header">
  <IconButton label="收起正在播放 (Esc)" onclick={onclose}><ChevronDown size={20}/></IconButton>
  <span class="immersive-caption">正在播放</span>
  <Select label="正在播放布局" bind:value={layout} options={[{value:'split',label:'封面与歌词'},{value:'cover',label:'大封面'},{value:'lyrics',label:'纯歌词'}]}/>
 </header>
 <div class="immersive-content" class:cover-only={layout==='cover'} class:lyrics-only={layout==='lyrics'}>
  {#if layout!=='lyrics'}<div class="artwork-panel"><div class="large-artwork"><Artwork src={artwork} alt="当前专辑封面"/></div><h1 title={snapshot?.current_track?.title}>{snapshot?.current_track?.title??'未在播放'}</h1><p>{snapshot?.current_track?.artist??'从音乐库选择一首歌曲'}</p>{#if snapshot?.current_track?.album}<small title={snapshot.current_track.album}>{snapshot.current_track.album}</small>{/if}</div>{/if}
  {#if layout!=='cover'}<div class="lyrics-panel">
   <div class="lyric-tools">
    <span class="lyric-source">{lyrics?.source??'本地歌词'}</span>
    <label class="lyric-field">字号 <input class="slider slider-sm" style:--fill={((fontSize-16)/16*100)+'%'} aria-label="歌词字号" type="range" min="16" max="32" step="1" value={fontSize} oninput={(e)=>changeSize(Number(e.currentTarget.value))}/><output>{fontSize}</output></label>
    <label class="lyric-field">偏移 <input class="input input-sm" aria-label="歌词时间偏移（毫秒，正数延后）" type="number" min="-60000" max="60000" step="100" value={snapshot?.lyrics_offset_ms??0} onchange={(e)=>control(()=>setLyricsOffset(Number(e.currentTarget.value)))}/> ms</label>
    {#if manual}<button type="button" class="btn btn-ghost btn-sm" onclick={()=>manual=false}><LocateFixed size={14}/>回到当前行</button>{/if}
   </div>
   <div class="lyric-lines" class:empty-lyrics={!lyrics?.lines.length} bind:this={container} use:watchSize onwheel={()=>manual=true} ontouchmove={()=>manual=true} onpointerdown={()=>manual=true} onkeydown={(e)=>{if(['ArrowDown','ArrowUp','PageDown','PageUp','Home','End'].includes(e.key))manual=true;}} tabindex="0" role="listbox" aria-label="歌词，滚动后可回到当前行">
    {#if loading}<p class="lyric-empty">正在读取歌词…</p>{:else if lyrics?.lines.length}{#each rows as line,i}<button role="option" aria-selected={active===i} data-line={i} class:current-line={active===i} class:past-line={active>i} disabled={!lyrics.synced||snapshot?.playback_state==='idle'} onclick={()=>jump(line.time_ms)}>{#each line.parts as part,j}<span class:lyric-secondary={j>0}>{part||'♪'}</span>{/each}</button>{/each}{:else}<div class="lyric-empty"><EmptyState icon={Music2} title="暂无歌词" message="把同名 .lrc 文件放在歌曲旁边，或使用歌曲内嵌歌词。"/></div>{/if}
   </div>
  </div>{/if}
 </div>
</section>
