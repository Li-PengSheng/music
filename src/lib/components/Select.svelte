<script lang="ts">
  import { tick, onMount } from 'svelte';
  import { ChevronDown, Check } from 'lucide-svelte';
  let {
    label,
    value = $bindable(''),
    options,
    disabled = false,
    onchange,
  }: {
    label: string;
    value?: string;
    options: { value: string; label: string }[];
    disabled?: boolean;
    onchange?: (value: string) => void;
  } = $props();
  const id = $props.id();
  let trigger: HTMLButtonElement;
  let panel: HTMLDivElement;
  let opened = $state(false);
  function close(restore = false) {
    panel?.hidePopover();
    opened = false;
    if (restore) trigger.focus();
  }
  async function open(last = false) {
    document.dispatchEvent(new Event('music-menu-open'));
    panel.showPopover();
    opened = true;
    const r = trigger.getBoundingClientRect();
    panel.style.minWidth = Math.min(Math.max(r.width, 160), window.innerWidth - 16) + 'px';
    panel.style.maxHeight =
      Math.max(80, Math.min(320, Math.max(r.top, window.innerHeight - r.bottom) - 16)) + 'px';
    panel.style.left =
      Math.max(8, Math.min(r.left, window.innerWidth - panel.offsetWidth - 8)) + 'px';
    panel.style.top =
      Math.max(
        8,
        r.bottom + panel.offsetHeight + 8 < window.innerHeight
          ? r.bottom + 4
          : r.top - panel.offsetHeight - 4,
      ) + 'px';
    await tick();
    const buttons = panel.querySelectorAll('button');
    buttons[
      last
        ? buttons.length - 1
        : Math.max(
            0,
            options.findIndex((o) => o.value === value),
          )
    ]?.focus();
  }
  function choose(next: string) {
    value = next;
    onchange?.(next);
    close(true);
  }
  function key(e: KeyboardEvent) {
    if (e.isComposing) return;
    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      close(true);
      return;
    }
    if (e.key === 'Tab') {
      close(true);
      return;
    }
    const buttons = Array.from(panel.querySelectorAll('button'));
    const index = buttons.indexOf(document.activeElement as HTMLButtonElement);
    let next = -1;
    if (e.key === 'ArrowDown') next = (index + 1) % buttons.length;
    else if (e.key === 'ArrowUp') next = (index - 1 + buttons.length) % buttons.length;
    else if (e.key === 'Home') next = 0;
    else if (e.key === 'End') next = buttons.length - 1;
    else if (e.key.length === 1 && !e.ctrlKey && !e.altKey)
      next = options.findIndex(
        (o, i) => i > index && o.label.toLowerCase().startsWith(e.key.toLowerCase()),
      );
    if (next >= 0) {
      e.preventDefault();
      e.stopPropagation();
      buttons[next]?.focus();
    }
  }
  onMount(() => {
    const dismiss = () => close();
    const focus = (e: FocusEvent) => {
      if (opened && !panel.contains(e.target as Node) && e.target !== trigger) close();
    };
    document.addEventListener('focusin', focus);
    document.addEventListener('music-menu-open', dismiss);
    document.addEventListener('scroll', moved, true);
    return () => {
      document.removeEventListener('focusin', focus);
      document.removeEventListener('music-menu-open', dismiss);
      document.removeEventListener('scroll', moved, true);
    };
  });
  function moved(e: Event) {
    if (opened && !panel.contains(e.target as Node)) close();
  }
</script>

<button
  class="select-trigger"
  bind:this={trigger}
  type="button"
  aria-label={label}
  aria-haspopup="listbox"
  aria-controls={id}
  aria-expanded={opened}
  {disabled}
  onclick={() => (opened ? close() : void open())}
  onkeydown={(e) => {
    if (['ArrowDown', 'ArrowUp'].includes(e.key)) {
      e.preventDefault();
      void open(e.key === 'ArrowUp');
    }
  }}
  ><span>{options.find((o) => o.value === value)?.label ?? label}</span><ChevronDown
    size={14}
  /></button
>
<div
  class="select-panel"
  {id}
  bind:this={panel}
  popover="auto"
  role="listbox"
  aria-label={label}
  tabindex="-1"
  onkeydown={key}
  ontoggle={(e) => {
    opened = e.newState === 'open';
  }}
>
  {#each options as option}<button
      type="button"
      role="option"
      aria-selected={value === option.value}
      tabindex="-1"
      onclick={() => choose(option.value)}
      ><span>{option.label}</span>{#if value === option.value}<Check size={15} />{/if}</button
    >{/each}
</div>

<svelte:window onresize={() => close()} onscroll={moved} />
