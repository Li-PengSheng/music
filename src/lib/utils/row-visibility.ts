// Shared observers per scroll container. `seen` is the row inside the container's box; `near`
// adds a preload margin. The observer root is the container itself: with the implicit viewport
// root a margin does not extend past the container's clipping, so nothing would be preloaded.
export interface RowState { near: boolean; seen: boolean }
type Listener = (state: RowState) => void;
interface Watch { listener: Listener; state: RowState }
interface Root { near: IntersectionObserver; seen: IntersectionObserver; rows: Map<Element, Watch> }

const roots = new WeakMap<Element, Map<string, Root>>();

function rootFor(container: Element, margin: string): Root {
  let byMargin = roots.get(container);
  if (!byMargin) { byMargin = new Map(); roots.set(container, byMargin); }
  let root = byMargin.get(margin);
  if (root) return root;
  const rows = new Map<Element, Watch>();
  const update = (field: keyof RowState) => (entries: IntersectionObserverEntry[]) => {
    for (const entry of entries) {
      const watch = rows.get(entry.target);
      if (!watch || watch.state[field] === entry.isIntersecting) continue;
      watch.state = { ...watch.state, [field]: entry.isIntersecting };
      watch.listener(watch.state);
    }
  };
  root = {
    rows,
    near: new IntersectionObserver(update('near'), { root: container, rootMargin: margin }),
    seen: new IntersectionObserver(update('seen'), { root: container }),
  };
  byMargin.set(margin, root);
  return root;
}

/** Reports when `row` enters or leaves `container`'s box and its preload margin. */
export function watchRow(row: Element, container: Element, margin: string, listener: Listener): () => void {
  const root = rootFor(container, margin);
  root.rows.set(row, { listener, state: { near: false, seen: false } });
  root.near.observe(row);
  root.seen.observe(row);
  return () => {
    root.rows.delete(row);
    root.near.unobserve(row);
    root.seen.unobserve(row);
  };
}
