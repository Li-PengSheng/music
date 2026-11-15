<script lang="ts">
 import { untrack } from 'svelte';
 import Artwork from './Artwork.svelte';
 import { desktopAvailable,peekTrackThumb,requestTrackThumb,type CoverStatus,type CoverTicket } from '../services/music';
 import { library } from '../stores/library.svelte';
 let {id,alt,kind='album',size=112}:{id:number;alt:string;kind?:'album'|'artist';size?:number}=$props();
 let host=$state<HTMLDivElement>();
 let shown=$state<{key:string;src:string|null;status:CoverStatus|'loading'}|null>(null);
 $effect(()=>{
  const requested=id,edge=size,node=host,key=requested+'@'+edge;
  void library.coverEpoch;
  if(!desktopAvailable||!node)return;
  // Another cover in this slot drops the old picture; a rescan keeps it until the new result.
  if(untrack(()=>shown?.key!==key))shown={key,src:null,status:'loading'};
  const hit=peekTrackThumb(requested,edge);
  if(hit&&hit.status!=='failed'){shown={key,src:hit.src,status:hit.status};return;}
  let ticket:CoverTicket|null=null,finished=false;
  const done=(o:{status:CoverStatus;src:string|null;code?:string})=>{ticket=null;if(o.code==='dropped')return;finished=true;shown={key,src:o.src,status:o.status};};
  const observer=new IntersectionObserver(entries=>{
   const visible=entries.some(e=>e.isIntersecting);
   if(finished)return;
   if(!visible){ticket?.cancel();ticket=null;}
   else if(!ticket){const next=requestTrackThumb(requested,edge,1,done);if(!finished)ticket=next;}
  },{rootMargin:'240px'});
  observer.observe(node);
  return()=>{observer.disconnect();ticket?.cancel();};
 });
</script>
<div class="cover-fill" bind:this={host} data-cover={shown?.status??'loading'}><Artwork src={shown?.src??null} {alt} {kind}/></div>
