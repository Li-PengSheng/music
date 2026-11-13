<script lang="ts">
 import type {Snippet} from 'svelte';
 import type {HTMLButtonAttributes} from 'svelte/elements';
 import {tooltip} from './tooltip';
 let {label,active=false,size='md',children,element=$bindable(),class:className='',...rest}:HTMLButtonAttributes&{
  label:string;active?:boolean;size?:'sm'|'md'|'lg';children:Snippet;element?:HTMLButtonElement;
 }=$props();
</script>

<button bind:this={element} type="button" class={['icon-btn','icon-btn-'+size,className]} class:active aria-label={label} use:tooltip={label} {...rest}>{@render children()}</button>
