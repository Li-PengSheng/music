import type {Play} from 'lucide-svelte';
import {errorText} from '../utils/format';
import type {LibraryTrack} from '../services/music';

/** Any Lucide icon component (lucide-svelte ships class-style component types). */
export type IconComponent=typeof Play;

export type MenuItem =
 | {kind?:'item';label:string;icon?:IconComponent;action:()=>unknown;danger?:boolean;disabled?:boolean;checked?:boolean;hint?:string}
 | {kind:'separator'}
 | {kind:'heading';label:string};

export interface MenuState {
 id:number;
 label:string;
 items:MenuItem[];
 /** Pointer position, or an anchor rectangle for keyboard/button invocations. */
 at:{x:number;y:number}|DOMRect;
 returnFocus:HTMLElement|null;
}

interface ConfirmState {kind:'confirm';title:string;message:string;detail?:string;confirmLabel:string;danger:boolean;resolve:(ok:boolean)=>void}
interface PromptState {kind:'prompt';title:string;label:string;value:string;confirmLabel:string;validate:(value:string)=>string|null;submit:(value:string)=>Promise<void>;resolve:(ok:boolean)=>void}
export type DialogState = ConfirmState|PromptState;

export const ui=$state({
 menu:null as MenuState|null,
 dialog:null as DialogState|null,
 toast:null as {id:number;message:string;tone:'info'|'error'}|null,
 query:'',
 queueOpen:false,
 immersive:false,
 reduceMotion:false,
 /** Track whose details dialog is open. */
 details:null as LibraryTrack|null,
});

let menuId=0;
export function openMenu(trigger:MouseEvent|HTMLElement,items:MenuItem[],label='操作菜单'){
 let at:MenuState['at'];let returnFocus:HTMLElement|null;
 if(trigger instanceof MouseEvent){
  trigger.preventDefault();
  const target=trigger.currentTarget instanceof HTMLElement?trigger.currentTarget:null;
  // Keyboard-generated contextmenu events (Shift+F10, Menu key) report 0,0.
  at=trigger.clientX||trigger.clientY?{x:trigger.clientX,y:trigger.clientY}:(target?.getBoundingClientRect()??{x:0,y:0});
  returnFocus=target?.matches(':focus-within')?(document.activeElement as HTMLElement):target;
 }else{at=trigger.getBoundingClientRect();returnFocus=trigger;}
 ui.menu={id:++menuId,label,items,at,returnFocus};
}
export function closeMenu(restoreFocus=false){
 const menu=ui.menu;ui.menu=null;
 if(restoreFocus&&menu?.returnFocus?.isConnected)menu.returnFocus.focus();
}

export function confirmAction(options:{title:string;message:string;detail?:string;confirmLabel?:string;danger?:boolean}){
 return new Promise<boolean>(resolve=>{
  ui.dialog?.resolve(false);
  ui.dialog={kind:'confirm',confirmLabel:'确定',danger:false,...options,resolve};
 });
}
export function promptText(options:{title:string;label:string;value?:string;confirmLabel?:string;validate?:(value:string)=>string|null;submit:(value:string)=>Promise<void>}){
 return new Promise<boolean>(resolve=>{
  ui.dialog?.resolve(false);
  ui.dialog={kind:'prompt',value:'',confirmLabel:'保存',validate:()=>null,...options,resolve};
 });
}

let toastId=0;let toastTimer:ReturnType<typeof setTimeout>|undefined;
export function notify(message:string,tone:'info'|'error'='info'){
 if(ui.toast?.message===message&&ui.toast.tone===tone)return;
 ui.toast={id:++toastId,message,tone};
 clearTimeout(toastTimer);
 if(tone==='info')toastTimer=setTimeout(()=>{ui.toast=null;},4000);
}
export function notifyError(e:unknown){notify(errorText(e),'error');}
export function dismissToast(){clearTimeout(toastTimer);ui.toast=null;}
