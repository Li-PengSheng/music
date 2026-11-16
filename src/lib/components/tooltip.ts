import { autoUpdate, computePosition, flip, offset, shift } from '@floating-ui/dom';

// One shared tooltip element in the top layer; shown on hover after a short delay and on keyboard focus.
let tip: HTMLDivElement | null = null;
let timer: ReturnType<typeof setTimeout> | undefined;
let owner: HTMLElement | null = null;
let stopPositioning: (() => void) | undefined;

function element() {
  if (!tip) {
    tip = document.createElement('div');
    tip.className = 'tooltip';
    tip.setAttribute('role', 'tooltip');
    tip.id = 'app-tooltip';
    tip.setAttribute('popover', 'manual');
    document.body.append(tip);
  }
  return tip;
}

function show(node: HTMLElement, text: string) {
  const el = element();
  owner = node;
  el.textContent = text;
  try {
    el.showPopover();
  } catch {
    /* already open */
  }
  stopPositioning?.();
  stopPositioning = autoUpdate(node, el, () => {
    void computePosition(node, el, {
      strategy: 'fixed',
      placement: 'top',
      middleware: [offset(8), flip({ padding: 6 }), shift({ padding: 6 })],
    }).then(({ x, y }) => {
      if (owner !== node) return;
      el.style.left = x + 'px';
      el.style.top = y + 'px';
    });
  });
  el.dataset.visible = 'true';
}

function hide(node?: HTMLElement) {
  clearTimeout(timer);
  if (node && owner !== node) return;
  owner = null;
  stopPositioning?.();
  stopPositioning = undefined;
  if (tip) {
    tip.dataset.visible = 'false';
    try {
      tip.hidePopover();
    } catch {
      /* closed */
    }
  }
}

/** Usage: use:tooltip={'播放队列'}; only overflowing text when {text, overflow:true}. */
export function tooltip(
  node: HTMLElement,
  value: string | { text: string; overflow?: boolean } | null,
) {
  let options = typeof value === 'string' ? { text: value } : value;
  const enter = (e: Event) => {
    if (!options?.text) return;
    if (options.overflow && node.scrollWidth <= node.clientWidth) return;
    const focus = e.type === 'focusin';
    if (focus && !node.matches(':focus-visible')) return;
    clearTimeout(timer);
    timer = setTimeout(() => show(node, options!.text), focus ? 150 : 500);
  };
  const leave = () => hide(node);
  node.addEventListener('pointerenter', enter);
  node.addEventListener('focusin', enter);
  node.addEventListener('pointerleave', leave);
  node.addEventListener('focusout', leave);
  node.addEventListener('pointerdown', leave);
  return {
    update(next: typeof value) {
      options = typeof next === 'string' ? { text: next } : next;
      if (owner === node && options?.text && tip) tip.textContent = options.text;
    },
    destroy() {
      hide(node);
      node.removeEventListener('pointerenter', enter);
      node.removeEventListener('focusin', enter);
      node.removeEventListener('pointerleave', leave);
      node.removeEventListener('focusout', leave);
      node.removeEventListener('pointerdown', leave);
    },
  };
}
