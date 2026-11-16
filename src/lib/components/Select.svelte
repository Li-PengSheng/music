<script lang="ts">
  import { Select } from 'bits-ui';
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
  let trigger = $state<HTMLButtonElement | null>(null);
  let opened = $state(false);
  const dialog = $derived(trigger?.closest('dialog') ?? undefined);
</script>

<Select.Root
  type="single"
  bind:value
  bind:open={opened}
  {disabled}
  loop
  allowDeselect={false}
  onValueChange={(next) => onchange?.(next)}
  onOpenChange={(open) => {
    if (open) document.dispatchEvent(new Event('music-menu-open'));
  }}
>
  <Select.Trigger
    class="select-trigger"
    aria-label={label}
    bind:ref={trigger}
    onkeydowncapture={(e) => {
      // Let the IME own composition keys before Bits UI handles list navigation.
      if (e.isComposing) e.stopImmediatePropagation();
    }}
  >
    <span>{options.find((o) => o.value === value)?.label ?? label}</span><ChevronDown size={14} />
  </Select.Trigger>
  <Select.Portal to={dialog}>
    <Select.Content
      class="select-panel"
      aria-label={label}
      align="start"
      sideOffset={4}
      collisionPadding={8}
      collisionBoundary={dialog}
      onEscapeKeydown={(e) => e.stopPropagation()}
      onFocusOutside={(e) => {
        if (e.target !== trigger) opened = false;
      }}
    >
      <Select.Viewport class="select-viewport">
        {#each options as option (option.value)}
          <Select.Item value={option.value} label={option.label}>
            {#snippet child({ props, selected })}
              <button {...props} type="button" tabindex="-1">
                <span>{option.label}</span>{#if selected}<Check size={15} />{/if}
              </button>
            {/snippet}
          </Select.Item>
        {/each}
      </Select.Viewport>
    </Select.Content>
  </Select.Portal>
</Select.Root>
