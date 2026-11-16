/**
 * Picks a calm ambient colour from RGBA pixels: favours saturated mid-tone pixels,
 * then caps saturation and lightness so lyrics keep their contrast.
 */
export function ambientColor(data: ArrayLike<number>): [number, number, number] | null {
  let r = 0,
    g = 0,
    b = 0,
    weight = 0;
  for (let i = 0; i + 3 < data.length; i += 4) {
    if (data[i + 3] < 128) continue;
    const pr = data[i],
      pg = data[i + 1],
      pb = data[i + 2];
    const max = Math.max(pr, pg, pb),
      min = Math.min(pr, pg, pb);
    const light = (max + min) / 510;
    if (light < 0.08 || light > 0.92) continue;
    const w = 0.15 + (max - min) / 255;
    r += pr * w;
    g += pg * w;
    b += pb * w;
    weight += w;
  }
  if (!weight) return null;
  const [h, s, l] = toHsl(r / weight, g / weight, b / weight);
  return toRgb(h, Math.min(s, 0.42), Math.min(Math.max(l, 0.28), 0.42));
}

function toHsl(r: number, g: number, b: number): [number, number, number] {
  r /= 255;
  g /= 255;
  b /= 255;
  const max = Math.max(r, g, b),
    min = Math.min(r, g, b),
    l = (max + min) / 2;
  if (max === min) return [0, 0, l];
  const d = max - min,
    s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
  const h =
    max === r ? (g - b) / d + (g < b ? 6 : 0) : max === g ? (b - r) / d + 2 : (r - g) / d + 4;
  return [h / 6, s, l];
}

function toRgb(h: number, s: number, l: number): [number, number, number] {
  if (!s) {
    const v = Math.round(l * 255);
    return [v, v, v];
  }
  const q = l < 0.5 ? l * (1 + s) : l + s - l * s,
    p = 2 * l - q;
  const hue = (t: number) => {
    if (t < 0) t += 1;
    if (t > 1) t -= 1;
    if (t < 1 / 6) return p + (q - p) * 6 * t;
    if (t < 1 / 2) return q;
    if (t < 2 / 3) return p + (q - p) * (2 / 3 - t) * 6;
    return p;
  };
  return [hue(h + 1 / 3), hue(h), hue(h - 1 / 3)].map((v) => Math.round(v * 255)) as [
    number,
    number,
    number,
  ];
}
