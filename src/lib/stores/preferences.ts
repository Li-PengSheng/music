import {ui} from './ui.svelte';

const KEY='music-mine-reduce-motion';

export function loadPreferences(){
 try{ui.reduceMotion=localStorage.getItem(KEY)==='1';}catch{/* storage unavailable: keep default */}
}

export function setReduceMotion(on:boolean){
 ui.reduceMotion=on;
 try{localStorage.setItem(KEY,on?'1':'0');}catch{/* keep for this session */}
}

const systemReduced=typeof matchMedia==='function'?matchMedia('(prefers-reduced-motion: reduce)'):null;

/** Transition duration honouring both the in-app setting and the Windows animation preference. */
export function motionMs(ms:number){return ui.reduceMotion||systemReduced?.matches?0:ms;}
