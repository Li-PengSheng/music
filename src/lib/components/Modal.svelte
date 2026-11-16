<script lang="ts">
  import { Dialog } from 'bits-ui';
  import type { Snippet } from 'svelte';
  import type { HTMLDialogAttributes } from 'svelte/elements';
  import { createDismissal } from '../utils/modal';
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

  let node = $state<HTMLDialogElement | null>(null);
  const previous = document.activeElement;
  const popoverOpen = () =>
    !!document.querySelector('.menu[data-state="open"],.select-panel[data-state="open"]');
  const dismissal = $derived(
    node
      ? createDismissal(node, {
          canDismiss: () => dismissible,
          popoverOpen,
          dismiss: onclose,
        })
      : undefined,
  );
  function nativeDialog(el: HTMLDialogElement) {
    el.showModal();
    return {
      destroy() {
        if (el.open) el.close();
      },
    };
  }
</script>

<Dialog.Root
  open
  onOpenChange={(open) => {
    if (!open && dismissible) onclose();
  }}
>
  <Dialog.Content
    bind:ref={node}
    preventScroll={false}
    preventBodyPointerEvents={false}
    onOpenAutoFocus={(e) => {
      const target = initialFocus ? node?.querySelector<HTMLElement>(initialFocus) : null;
      if (target) {
        e.preventDefault();
        target.focus();
        if (target instanceof HTMLInputElement) target.select();
      }
    }}
    onCloseAutoFocus={(e) => {
      e.preventDefault();
      if (previous instanceof HTMLElement && previous.isConnected)
        previous.focus({ preventScroll: true });
    }}
    onEscapeKeydown={(e) => {
      if (!dismissible || e.isComposing || popoverOpen()) e.preventDefault();
    }}
    onInteractOutside={(e) => e.preventDefault()}
  >
    {#snippet child({ props })}
      <dialog
        {...props}
        {...rest}
        class={['dialog', className]}
        style:contain="none"
        use:nativeDialog
        onpointerdown={(e) => dismissal?.pointerdown(e)}
        onclick={(e) => dismissal?.click(e)}
        oncancel={(e) => e.preventDefault()}
      >
        {@render children()}
      </dialog>
    {/snippet}
  </Dialog.Content>
</Dialog.Root>
