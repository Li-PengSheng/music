<script lang="ts">
 import {tick} from 'svelte';
 import {TriangleAlert} from 'lucide-svelte';
 import {ui} from '../stores/ui.svelte';
 import {errorText} from '../utils/format';

 let dialog=$state<HTMLDialogElement>();
 let input=$state<HTMLInputElement>();
 let value=$state('');let error=$state('');let busy=$state(false);let touched=$state(false);
 let active=$derived(ui.dialog);
 let previous:Element|null=null;

 $effect(()=>{
  const current=active;const node=dialog;
  if(!current||!node)return;
  previous=document.activeElement;
  value=current.kind==='prompt'?current.value:'';error='';busy=false;touched=false;
  if(!node.open)node.showModal();
  void tick().then(()=>{if(current.kind==='prompt'){input?.focus();input?.select();}else node.querySelector<HTMLButtonElement>('[data-autofocus]')?.focus();});
 });

 function finish(ok:boolean){
  const current=ui.dialog;if(!current||busy)return;
  ui.dialog=null;dialog?.close();current.resolve(ok);
  if(previous instanceof HTMLElement&&previous.isConnected)previous.focus();
 }

 let validation=$derived(active?.kind==='prompt'?active.validate(value):null);

 async function submit(e:SubmitEvent){
  e.preventDefault();
  const current=ui.dialog;if(!current||busy)return;
  if(current.kind==='confirm'){finish(true);return;}
  touched=true;
  if(validation){error=validation;return;}
  busy=true;error='';
  try{await current.submit(value);busy=false;finish(true);}
  catch(err){busy=false;error=errorText(err);void tick().then(()=>input?.focus());}
 }
</script>

{#if active}
 <dialog bind:this={dialog} class="dialog" aria-labelledby="app-dialog-title" aria-describedby={active.kind==='confirm'?'app-dialog-message':undefined} oncancel={(e)=>{e.preventDefault();finish(false);}}>
  <form onsubmit={submit} novalidate>
   <header class="dialog-header">
    {#if active.kind==='confirm'&&active.danger}<span class="dialog-badge danger" aria-hidden="true"><TriangleAlert size={18}/></span>{/if}
    <h2 id="app-dialog-title">{active.title}</h2>
   </header>
   <div class="dialog-body">
    {#if active.kind==='confirm'}
     <p id="app-dialog-message">{active.message}</p>
     {#if active.detail}<p class="dialog-detail">{active.detail}</p>{/if}
    {:else}
     <label class="field-label" for="app-dialog-input">{active.label}</label>
     <input id="app-dialog-input" class="input" bind:this={input} bind:value maxlength="100" autocomplete="off" spellcheck="false"
      aria-invalid={!!(error||(touched&&validation))} aria-describedby="app-dialog-error" oninput={()=>{touched=true;error='';}}/>
     <p id="app-dialog-error" class="field-error" role="alert">{error||(touched?validation??'':'')}</p>
    {/if}
   </div>
   <footer class="dialog-actions">
    <button type="button" class="btn btn-ghost" onclick={()=>finish(false)} disabled={busy} data-autofocus={active.kind==='confirm'&&active.danger?'':undefined}>取消</button>
    <button type="submit" class="btn" class:btn-danger={active.kind==='confirm'&&active.danger} class:btn-primary={!(active.kind==='confirm'&&active.danger)}
     data-autofocus={active.kind==='confirm'&&!active.danger?'':undefined} disabled={busy||(active.kind==='prompt'&&touched&&!!validation)}>
     {busy?'正在保存…':active.confirmLabel}
    </button>
   </footer>
  </form>
 </dialog>
{/if}
