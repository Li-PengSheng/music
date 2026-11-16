<script lang="ts">
  import { DropdownMenu } from 'bits-ui';
  import { Check } from 'lucide-svelte';
  import { ui, closeMenu, notifyError, type MenuItem } from '../stores/ui.svelte';
  let menu = $derived(ui.menu);
  let anchor = $state<HTMLSpanElement | null>(null);
  let panel = $state<HTMLDivElement | null>(null);
  // Preserve the imperative point/rectangle API; Floating UI owns positioning and collisions.
  const rect = $derived(
    menu && ('width' in menu.at ? menu.at : { ...menu.at, width: 0, height: 0 }),
  );
  const portal = $derived(menu?.returnFocus?.closest('dialog') ?? undefined);
  async function run(item: Extract<MenuItem, { label: string; action: () => unknown }>) {
    closeMenu(true);
    try {
      await item.action();
    } catch (e) {
      notifyError(e);
    }
  }
</script>

{#if menu && rect}
  {@const current = menu}
  {#key menu.id}
    <DropdownMenu.Root
      open
      onOpenChange={(open) => {
        if (!open && ui.menu === current) closeMenu();
      }}
    >
      <DropdownMenu.Portal to={portal}>
        <span
          bind:this={anchor}
          aria-hidden="true"
          style:position="fixed"
          style:left={rect.x + 'px'}
          style:top={rect.y + 'px'}
          style:width={rect.width + 'px'}
          style:height={rect.height + 'px'}
          style:pointer-events="none"
        ></span>
        <DropdownMenu.Content
          bind:ref={panel}
          class="menu"
          aria-label={menu.label}
          customAnchor={anchor}
          align={'width' in menu.at ? 'end' : 'start'}
          sideOffset={'width' in menu.at ? 4 : 0}
          collisionPadding={8}
          preventScroll={false}
          onOpenAutoFocus={(e) => {
            e.preventDefault();
            panel?.querySelector<HTMLElement>('.menu-item:not(:disabled)')?.focus();
          }}
          onCloseAutoFocus={(e) => {
            e.preventDefault();
          }}
          onEscapeKeydown={() => closeMenu(true)}
          onkeydowncapture={(e) => {
            if (e.isComposing) e.stopImmediatePropagation();
            else if (e.key === 'Tab') {
              e.preventDefault();
              closeMenu(true);
            }
          }}
          oncontextmenu={(e) => e.preventDefault()}
        >
          {#each menu.items as item, i (i)}
            {#if item.kind === 'separator'}<DropdownMenu.Separator class="menu-separator" />
            {:else if item.kind === 'heading'}<div class="menu-heading" role="presentation">
                {item.label}
              </div>
            {:else}
              <DropdownMenu.Item
                disabled={item.disabled}
                textValue={item.label}
                onSelect={() => run(item)}
              >
                {#snippet child({ props })}
                  <button
                    {...props}
                    type="button"
                    class={['menu-item', item.danger && 'danger']}
                    disabled={item.disabled}
                  >
                    <span class="menu-icon" aria-hidden="true"
                      >{#if item.icon}<item.icon size={16} />{/if}</span
                    >
                    <span class="menu-label">{item.label}</span>
                    {#if item.checked}<Check
                        size={15}
                        class="menu-check"
                        aria-label="已选"
                      />{:else if item.hint}<kbd>{item.hint}</kbd>{/if}
                  </button>
                {/snippet}
              </DropdownMenu.Item>
            {/if}
          {/each}
        </DropdownMenu.Content>
      </DropdownMenu.Portal>
    </DropdownMenu.Root>
  {/key}
{/if}
