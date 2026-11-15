<script lang="ts">
 import {diagnostics} from './cover-diagnostics';
 import {Disc3,MicVocal,ListMusic} from 'lucide-svelte';
 let {src=null,alt='专辑封面',kind='album'}:{src?:string|null;alt?:string;kind?:'album'|'artist'|'playlist'}=$props();
 let failed=$state(false);let loaded=$state(false);
 let previous:string|null=null;
 $effect(()=>{const current=src;if(current!==previous){previous=current;failed=false;loaded=false;if(diagnostics)diagnostics.urlChanges++;}});
 function mounted(_node:HTMLImageElement){if(diagnostics)diagnostics.mounts++;}
</script>
<div class="artwork" class:artwork-artist={kind==='artist'} data-image={!src?'empty':failed?'error':loaded?'loaded':'loading'}>
 {#if src&&!failed}<img use:mounted class:loaded onload={()=>{loaded=true;if(diagnostics)diagnostics.loads++;}} {src} {alt} decoding="async" draggable="false" onerror={()=>{failed=true;if(diagnostics)diagnostics.errors++;}}/>
 {:else}<span class="artwork-placeholder" role="img" aria-label={kind==='artist'?'暂无艺术家图片':kind==='playlist'?'歌单':'暂无专辑封面'}>{#if kind==='artist'}<MicVocal/>{:else if kind==='playlist'}<ListMusic/>{:else}<Disc3/>{/if}</span>{/if}
</div>
