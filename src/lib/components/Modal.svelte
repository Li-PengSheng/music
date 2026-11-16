<script lang="ts">
  import { onMount, tick, type Snippet } from 'svelte';
  import type { HTMLDialogAttributes } from 'svelte/elements';
  import { createDismissal } from '../utils/modal';

  /** One modal dialog in the browser top layer: centred, backdrop click and Escape close it, focus returns afterwards. */
  let {
    onclose,
    dismissible = true,
    initialFocus,
    class: className = '',
    children,
    ...rest
  }: Omit<HTMLDialogAttributes, 'class' | 'onclose' | 'children'> & {
    onclose: () => void;
    dismissible?: boolean;
    initialFocus?: string;
    class?: string;
    children: Snippet;
  } = $props();

  let node: HTMLDialogElement;
  let dismissal: ReturnType<typeof createDismissal> | undefined;
  let destroyed = false;
  const popoverOpen = () =>
    !!document.querySelector('.menu:popover-open,.select-panel[data-state="open"]');

  onMount(() => {
    const previous = document.activeElement;
    dismissal = createDismissal(node, {
      canDismiss: () => dismissible,
      popoverOpen,
      dismiss: () => onclose(),
    });
    node.showModal();
    void tick().then(() => {
      const target = initialFocus ? node.querySelector<HTMLElement>(initialFocus) : null;
      target?.focus();
      if (target instanceof HTMLInputElement) target.select();
    });
    return () => {
      destroyed = true;
      if (node.open) node.close();
      if (previous instanceof HTMLElement && previous.isConnected)
        previous.focus({ preventScroll: true });
    };
  });

  // Chromium may force-close a dialog on a repeated Escape without firing cancel; keep state in sync.
  function closed() {
    if (destroyed) return;
    if (dismissible) onclose();
    else node.showModal();
  }
</script>

<dialog
  bind:this={node}
  class={['dialog', className]}
  {...rest}
  onpointerdown={(e) => dismissal?.pointerdown(e)}
  onclick={(e) => dismissal?.click(e)}
  onkeydown={(e) => {
    if (!popoverOpen()) dismissal?.keydown(e);
  }}
  oncancel={(e) => dismissal?.cancel(e)}
  onclose={closed}
>
  {@render children()}
</dialog>
