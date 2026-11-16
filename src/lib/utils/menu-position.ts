export interface Placement {
  left: number;
  top: number;
  maxHeight: number;
}

const clamp = (value: number, min: number, max: number) => Math.max(min, Math.min(value, max));

/** Places a menu of size w×h at a pointer position, flipping left/up near the window edge. */
export function placeAtPoint(
  x: number,
  y: number,
  w: number,
  h: number,
  vw: number,
  vh: number,
  margin = 8,
): Placement {
  const maxHeight = Math.max(80, vh - margin * 2);
  const height = Math.min(h, maxHeight);
  const left = x + w + margin > vw ? x - w : x;
  const top = y + height + margin > vh ? y - height : y;
  return {
    left: clamp(left, margin, Math.max(margin, vw - w - margin)),
    top: clamp(top, margin, Math.max(margin, vh - height - margin)),
    maxHeight,
  };
}

/** Places a menu under an anchor (right-aligned), or above it when there is no room below. */
export function placeAtAnchor(
  r: { left: number; right: number; top: number; bottom: number },
  w: number,
  h: number,
  vw: number,
  vh: number,
  margin = 8,
  gap = 4,
): Placement {
  const maxHeight = Math.max(80, vh - margin * 2);
  const height = Math.min(h, maxHeight);
  const below = r.bottom + gap + height + margin <= vh;
  const above = r.top - gap - height >= margin;
  const top = below || !above ? r.bottom + gap : r.top - gap - height;
  return {
    left: clamp(r.right - w, margin, Math.max(margin, vw - w - margin)),
    top: clamp(top, margin, Math.max(margin, vh - height - margin)),
    maxHeight,
  };
}
