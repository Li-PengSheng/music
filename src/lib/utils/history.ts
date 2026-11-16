/** Back/forward stacks for in-app navigation; equal consecutive entries are not duplicated. */
export function createHistory<T>(
  initial: T,
  same: (a: T, b: T) => boolean = (a, b) => JSON.stringify(a) === JSON.stringify(b),
  limit = 50,
) {
  const back: T[] = [];
  let forward: T[] = [];
  let current = initial;
  return {
    get current() {
      return current;
    },
    get canBack() {
      return back.length > 0;
    },
    get canForward() {
      return forward.length > 0;
    },
    push(next: T) {
      if (same(current, next)) return false;
      back.push(current);
      if (back.length > limit) back.shift();
      forward = [];
      current = next;
      return true;
    },
    replace(next: T) {
      current = next;
    },
    back() {
      const prev = back.pop();
      if (prev === undefined) return false;
      forward.push(current);
      current = prev;
      return true;
    },
    forward() {
      const next = forward.pop();
      if (next === undefined) return false;
      back.push(current);
      current = next;
      return true;
    },
    /** Drops entries matching the predicate, e.g. a deleted playlist. */
    forget(match: (entry: T) => boolean, fallback: T) {
      for (const stack of [back, forward])
        for (let i = stack.length - 1; i >= 0; i--) if (match(stack[i])) stack.splice(i, 1);
      if (match(current)) current = back.pop() ?? fallback;
    },
  };
}
