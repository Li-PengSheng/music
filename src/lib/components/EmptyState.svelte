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

<style>
  :global {
    /* ============================================================ empty state */
    .empty-state {
      display: flex;
      flex-direction: column;
      align-items: center;
      text-align: center;
      gap: 8px;
      padding: 56px 24px;
      color: var(--text-2);
    }

    .empty-icon {
      display: grid;
      place-items: center;
      width: 56px;
      height: 56px;
      margin-bottom: 6px;
      border-radius: 50%;
      background: var(--surface);
      color: var(--text-3);
    }

    .empty-state h2 {
      color: var(--text);
      font-size: var(--fs-section);
      font-weight: var(--fw-semibold);
    }

    .empty-state p {
      max-width: 420px;
      font-size: var(--fs-meta);
      line-height: 1.6;
    }

    .empty-actions {
      display: flex;
      gap: 8px;
      margin-top: 8px;
    }
  }
</style>
