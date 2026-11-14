<script lang="ts">
 import Artwork from './Artwork.svelte';
 import { desktopAvailable,trackThumb } from './services/music';
 import { library } from './stores/library.svelte';
 let {id,alt,kind='album',size=112}:{id:number;alt:string;kind?:'album'|'artist';size?:number}=$props();
 let host=$state<HTMLDivElement>();let src=$state<string|null>(null);
 $effect(()=>{
  const requested=id,edge=size,epoch=library.coverEpoch,node=host;
  if(!desktopAvailable||!node)return;
  src=null;
  let disposed=false;
  const observer=new IntersectionObserver(entries=>{
   if(!entries.some(e=>e.isIntersecting))return;
   observer.disconnect();
   void trackThumb(requested,edge).then(value=>{if(!disposed&&requested===id&&epoch===library.coverEpoch)src=value;}).catch(()=>{});
  },{rootMargin:'240px'});
  observer.observe(node);
  return()=>{disposed=true;observer.disconnect();};
 });
</script>
<div class="cover-fill" bind:this={host}><Artwork {src} {alt} {kind}/></div>
