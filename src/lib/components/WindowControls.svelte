<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { Minus, Square, Copy, X } from 'lucide-svelte';
  import { desktopAvailable } from '../services/music';
  import { notifyError } from '../stores/ui.svelte';

  let maximized = $state(false);
  let busy = $state(false);
  onMount(() => {
    if (!desktopAvailable) return;
    const window = getCurrentWindow();
    let disposed = false;
    let unlisten: (() => void) | undefined;
    const refresh = async () => {
      try {
        const value = await window.isMaximized();
        if (!disposed) maximized = value;
      } catch (error) {
        if (!disposed) notifyError(error);
      }
    };
    void refresh();
    void window
      .onResized(() => void refresh())
      .then((stop) => {
        if (disposed) stop();
        else unlisten = stop;
      })
      .catch((error) => {
        if (!disposed) notifyError(error);
      });
    return () => {
      disposed = true;
      unlisten?.();
    };
  });
  async function act(action: 'minimize' | 'toggleMaximize' | 'close') {
    if (!desktopAvailable || busy) return;
    busy = true;
    try {
      const window = getCurrentWindow();
      await window[action]();
      if (action === 'toggleMaximize') maximized = await window.isMaximized();
    } catch (error) {
      notifyError(error);
    } finally {
      busy = false;
    }
  }
</script>

{#if desktopAvailable}
  <div class="window-controls" role="group" aria-label="窗口控制">
    <button type="button" aria-label="最小化" disabled={busy} onclick={() => act('minimize')}
      ><Minus size={14} /></button
    >
    <button
      type="button"
      aria-label={maximized ? '还原窗口' : '最大化'}
      disabled={busy}
      onclick={() => act('toggleMaximize')}
    >
      {#if maximized}<Copy size={13} />{:else}<Square size={12} />{/if}
    </button>
    <button
      type="button"
      class="window-close"
      aria-label="关闭窗口"
      disabled={busy}
      onclick={() => act('close')}><X size={16} /></button
    >
  </div>
{/if}

<style>
  .window-controls {
    display: flex;
    flex: 0 0 auto;
    align-self: stretch;
    align-items: center;
  }
  button {
    width: 40px;
    height: 36px;
    display: grid;
    place-items: center;
    color: var(--text-2);
    border-radius: var(--radius-xs);
  }
  button:hover {
    background: var(--surface-hover);
    color: var(--text);
  }
  button:focus-visible {
    outline: 2px solid var(--focus);
    outline-offset: -2px;
  }
  .window-close:hover {
    background: var(--danger-bg);
    color: var(--on-danger);
  }
</style>
