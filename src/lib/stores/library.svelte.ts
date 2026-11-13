import {desktopAvailable,librarySummary,scanProgress,type LibrarySummary,type ScanProgress} from '../services/music';
import {notifyError} from './ui.svelte';

export const library=$state({
 summary:{tracks:0,albums:[],artists:[],folders:[],playlists:[]} as LibrarySummary,
 loaded:false,
 /** Bumped after any change to tracks, favourites or playlists so open lists reload. */
 revision:0,
 progress:null as ScanProgress|null,
});

export async function refreshSummary(){
 if(!desktopAvailable)return;
 try{library.summary=await librarySummary();library.loaded=true;library.revision++;}
 catch(e){notifyError(e);}
}

export function changed(){library.revision++;}

/** Polls scan progress and refreshes the library when a scan finishes. */
export function watchScans(){
 if(!desktopAvailable)return ()=>{};
 let disposed=false;let timer:ReturnType<typeof setTimeout>;
 async function poll(){
  try{
   const next=await scanProgress();
   const was=library.progress?.running;
   library.progress=next;
   if(was&&!next.running)await refreshSummary();
  }catch(e){if(!disposed)notifyError(e);}
  if(!disposed)timer=setTimeout(poll,library.progress?.running?500:2000);
 }
 void poll();
 return ()=>{disposed=true;clearTimeout(timer);};
}
