<script lang="ts">
 import {Play,Pause,SkipBack,SkipForward,Shuffle,Repeat,Repeat1,Volume2,Volume1,VolumeX,ListMusic,Maximize2} from 'lucide-svelte';
 import Artwork from '../Artwork.svelte';
 import Slider from '../components/Slider.svelte';
 import IconButton from '../components/IconButton.svelte';
 import {tooltip} from '../components/tooltip';
 import {player,control} from '../stores/player.svelte';
 import {ui} from '../stores/ui.svelte';
 import {play,pause,seek,setVolume,setMode,queueAction,type PlaybackMode} from '../services/music';
 import {duration} from '../utils/format';

 let {artwork,ontogglequeue,queueTrigger=$bindable()}:{artwork:string|null;ontogglequeue:()=>void;queueTrigger?:HTMLButtonElement}=$props();

 let s=$derived(player.snapshot);
 let track=$derived(s?.current_track??null);
 let playing=$derived(s?.playback_state==='playing');
 let mode=$derived<PlaybackMode>(s?.playback_mode??'sequential');
 let hasQueue=$derived(!!s?.queue.length);

 let dragging=$state<number|null>(null);
 let trackId=$derived(s?.track_id);
 $effect(()=>{trackId;dragging=null;});
 let position=$derived(dragging??s?.position_ms??0);
 let total=$derived(s?.duration_ms??0);
 let canSeek=$derived(!!total&&s?.playback_state!=='idle');

 let volumeDraft=$state<number|null>(null);
 let volume=$derived(volumeDraft??(s?Math.round(s.volume*100):70));
 let lastAudible=70;
 $effect(()=>{if(s&&s.volume>0)lastAudible=Math.round(s.volume*100);});

 async function commitVolume(value:number){await control(()=>setVolume(value/100));volumeDraft=null;}
 function toggleMute(){void commitVolume(volume>0?0:lastAudible||70);}
 async function commitSeek(value:number){await control(()=>seek(value));dragging=null;}

 const repeatNext:Record<PlaybackMode,PlaybackMode>={sequential:'repeat_all',shuffle:'repeat_all',repeat_all:'repeat_one',repeat_one:'sequential'};
 let repeatLabel=$derived(mode==='repeat_all'?'列表循环':mode==='repeat_one'?'单曲循环':'循环播放：关');
</script>

<footer class="player-bar" aria-label="播放控制">
 <div class="pb-track">
  <button type="button" class="pb-cover" aria-label="打开正在播放" disabled={!track} onclick={()=>ui.immersive=true}><Artwork src={artwork} alt={track?.album||'专辑封面'}/></button>
  <div class="pb-meta">
   {#if track}
    <button type="button" class="pb-title" onclick={()=>ui.immersive=true} use:tooltip={{text:track.title,overflow:true}}>{track.title}</button>
    <span class="pb-artist" use:tooltip={{text:track.artist,overflow:true}}>{track.artist||'未知艺术家'}</span>
   {:else}
    <span class="pb-title pb-idle">未在播放</span>
    <span class="pb-artist">从音乐库选择一首歌曲</span>
   {/if}
  </div>
 </div>

 <div class="pb-center">
  <div class="pb-controls">
   <IconButton label={mode==='shuffle'?'随机播放：开':'随机播放：关'} active={mode==='shuffle'} aria-pressed={mode==='shuffle'} disabled={!s||player.busy}
    onclick={()=>control(()=>setMode(mode==='shuffle'?'sequential':'shuffle'))}><Shuffle size={17}/></IconButton>
   <IconButton label="上一首" size="lg" disabled={!hasQueue||player.busy} onclick={()=>control(()=>queueAction('previous'))}><SkipBack size={19} fill="currentColor"/></IconButton>
   <button type="button" class="pb-play" aria-label={playing?'暂停':'播放'} use:tooltip={playing?'暂停 (Space)':'播放 (Space)'} disabled={!track||player.busy}
    onclick={()=>control(playing?pause:play)}>{#if playing}<Pause size={18} fill="currentColor"/>{:else}<Play size={18} fill="currentColor" class="pb-play-icon"/>{/if}</button>
   <IconButton label="下一首" size="lg" disabled={!hasQueue||player.busy} onclick={()=>control(()=>queueAction('next'))}><SkipForward size={19} fill="currentColor"/></IconButton>
   <IconButton label={repeatLabel} active={mode==='repeat_all'||mode==='repeat_one'} aria-pressed={mode==='repeat_all'||mode==='repeat_one'} disabled={!s||player.busy}
    onclick={()=>control(()=>setMode(repeatNext[mode]))}>{#if mode==='repeat_one'}<Repeat1 size={17}/>{:else}<Repeat size={17}/>{/if}</IconButton>
  </div>
  <div class="pb-progress">
   <span class="pb-time">{duration(position)}</span>
   <Slider class="pb-seek" label="播放进度" value={position} max={total||1} valuetext={duration(position)+' / '+duration(total)} disabled={!canSeek||player.busy}
    oninput={(v)=>dragging=v} onchange={commitSeek}/>
   <span class="pb-time pb-time-end">{duration(total)}</span>
  </div>
 </div>

 <div class="pb-options">
  <IconButton label="正在播放与歌词 (Ctrl+L)" active={ui.immersive} disabled={!track} onclick={()=>ui.immersive=!ui.immersive}><Maximize2 size={17}/></IconButton>
  <IconButton label="播放队列" active={ui.queueOpen} aria-expanded={ui.queueOpen} aria-controls="playback-queue" bind:element={queueTrigger} onclick={ontogglequeue}><ListMusic size={18}/></IconButton>
  <div class="pb-volume">
   <IconButton label={volume>0?'静音':'取消静音'} disabled={!s||player.busy} onclick={toggleMute}>
    {#if volume===0}<VolumeX size={18}/>{:else if volume<50}<Volume1 size={18}/>{:else}<Volume2 size={18}/>{/if}
   </IconButton>
   <Slider class="pb-volume-slider" label="音量" value={volume} valuetext={volume+'%'} disabled={!s||player.busy}
    oninput={(v)=>volumeDraft=v} onchange={commitVolume}/>
  </div>
 </div>
</footer>
