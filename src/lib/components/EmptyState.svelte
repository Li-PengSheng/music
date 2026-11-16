<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { IconComponent } from '../stores/ui.svelte';
  let {
    icon: Icon,
    title,
    message = '',
    children,
  }: { icon?: IconComponent; title: string; message?: string; children?: Snippet } = $props();
</script>

<div class="empty-state">
  {#if Icon}<span class="empty-icon" aria-hidden="true"><Icon size={26} /></span>{/if}
  <h2>{title}</h2>
  {#if message}<p>{message}</p>{/if}
  {#if children}<div class="empty-actions">{@render children()}</div>{/if}
</div>
