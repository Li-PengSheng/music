<script lang="ts">
 import {tick} from 'svelte';
 import {TriangleAlert} from 'lucide-svelte';
 import Modal from './Modal.svelte';
 import {ui,type DialogState} from '../stores/ui.svelte';
 import {errorText} from '../utils/format';

 let input=$state<HTMLInputElement>();
 let value=$state('');let error=$state('');let busy=$state(false);let touched=$state(false);
 let active=$derived(ui.dialog);

 $effect.pre(()=>{const current=active;if(current){value=current.kind==='prompt'?current.value:'';error='';busy=false;touched=false;}});

 // Cancelling (button, backdrop or Escape) only resolves false; it never runs the submit action.
 function finish(current:DialogState,ok:boolean){
  if(ui.dialog!==current||busy)return;
  ui.dialog=null;current.resolve(ok);
 }

 let validation=$derived(active?.kind==='prompt'?active.validate(value):null);

 async function submit(e:SubmitEvent,current:DialogState){
  e.preventDefault();
  if(busy)return;
  if(current.kind==='confirm'){finish(current,true);return;}
  touched=true;
  if(validation){error=validation;return;}
  busy=true;error='';
  try{await current.submit(value);busy=false;finish(current,true);}
  catch(err){busy=false;error=errorText(err);void tick().then(()=>input?.focus());}
 }
</script>

{#if active}
 {@const current=active}
 {@const danger=current.kind==='confirm'&&current.danger}
 {#key current}
  <Modal onclose={()=>finish(current,false)} dismissible={!busy} initialFocus={current.kind==='prompt'?'#app-dialog-input':'[data-autofocus]'}
   aria-labelledby="app-dialog-title" aria-describedby={current.kind==='confirm'?'app-dialog-message':undefined}>
   <form onsubmit={(e)=>submit(e,current)} novalidate>
    <header class="dialog-header">
     {#if danger}<span class="dialog-badge danger" aria-hidden="true"><TriangleAlert size={18}/></span>{/if}
     <h2 id="app-dialog-title">{current.title}</h2>
    </header>
    <div class="dialog-body">
     {#if current.kind==='confirm'}
      <p id="app-dialog-message">{current.message}</p>
      {#if current.detail}<p class="dialog-detail">{current.detail}</p>{/if}
     {:else}
      <label class="field-label" for="app-dialog-input">{current.label}</label>
      <input id="app-dialog-input" class="input" bind:this={input} bind:value maxlength="100" autocomplete="off" spellcheck="false"
       aria-invalid={!!(error||(touched&&validation))} aria-describedby="app-dialog-error" oninput={()=>{touched=true;error='';}}/>
      <p id="app-dialog-error" class="field-error" role="alert">{error||(touched?validation??'':'')}</p>
     {/if}
    </div>
    <footer class="dialog-actions">
     <button type="button" class="btn btn-ghost" onclick={()=>finish(current,false)} disabled={busy} data-autofocus={danger?'':undefined}>取消</button>
     <button type="submit" class="btn" class:btn-danger={danger} class:btn-primary={!danger}
      data-autofocus={current.kind==='confirm'&&!danger?'':undefined} disabled={busy||(current.kind==='prompt'&&touched&&!!validation)}>
      {busy?'正在保存…':current.confirmLabel}
     </button>
    </footer>
   </form>
  </Modal>
 {/key}
{/if}
