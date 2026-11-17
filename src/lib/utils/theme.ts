/** Theme choice shared with public/theme-init.js, which applies it before the first paint. */
export type ThemeChoice = 'dark' | 'light' | 'system';
export type Theme = 'dark' | 'light';
export type Palette = 'celadon' | 'cinnabar' | 'azure' | 'amber';
export type FontPreset = 'harmony' | 'puhuiti' | 'serif' | 'kaiti' | 'system';
export const FONT_KEY = 'music-mine-font';
export const FONTS = [
  { value: 'harmony', label: '鸿蒙黑体（HarmonyOS Sans · 默认现代）' },
  { value: 'puhuiti', label: '阿里普惠体 3.0（现代清晰）' },
  { value: 'serif', label: '思源宋体 VF（中式书卷 · 歌词与标题衬线）' },
  { value: 'kaiti', label: '传统楷书（华文楷体 · 温润手书）' },
  { value: 'system', label: 'Windows 原生（Segoe UI Variable + 微软雅黑）' },
] satisfies { value: FontPreset; label: string }[];

export function parseFont(value: unknown): FontPreset {
  return value === 'puhuiti' || value === 'serif' || value === 'kaiti' || value === 'system'
    ? value
    : 'harmony';
}

export const PALETTE_KEY = 'music-mine-palette';
export const PALETTES = [
  { value: 'celadon', label: '松烟 · 秘色（青瓷）' },
  { value: 'cinnabar', label: '宣纸 · 朱砂（书卷）' },
  { value: 'azure', label: '天青 · 汝窑（霁蓝）' },
  { value: 'amber', label: '琥珀 · 沉香（暖金）' },
];
export function parsePalette(value: unknown): Palette {
  return value === 'cinnabar' || value === 'azure' || value === 'amber' ? value : 'celadon';
}

export const THEME_KEY = 'music-mine-theme';
export const THEME_COLOR: Record<Theme, string> = { dark: '#121414', light: '#F3F5F1' };

/** Missing or invalid choices use light; saved dark/system choices are preserved. */
export function parseTheme(value: unknown): ThemeChoice {
  return value === 'dark' || value === 'system' ? value : 'light';
}
export function resolveTheme(choice: ThemeChoice, systemDark: boolean): Theme {
  return choice === 'system' ? (systemDark ? 'dark' : 'light') : choice;
}
