<script lang="ts">
 import { onMount } from 'svelte';
 import Artwork from './Artwork.svelte';
 import { desktopAvailable,trackThumb } from './services/music';
 let {id,alt,kind='album',size=112}:{id:number;alt:string;kind?:'album'|'artist';size?:number}=$props();
 let host:HTMLDivElement;let src=$state<string|null>(null);
 onMount(()=>{
  if(!desktopAvailable)return;
  let disposed=false;
  const observer=new IntersectionObserver(entries=>{
   if(entries.some(e=>e.isIntersecting)){
    observer.disconnect();
    void trackThumb(id,size).then(value=>{if(!disposed)src=value;}).catch(()=>{});
   }
  },{rootMargin:'240px'});
  observer.observe(host);return()=>{disposed=true;observer.disconnect();};
 });
</script>
<div class="cover-fill" bind:this={host}><Artwork {src} {alt} {kind}/></div>
