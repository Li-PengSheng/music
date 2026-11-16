/** Theme choice shared with public/theme-init.js, which applies it before the first paint. */
export type ThemeChoice = 'dark' | 'light' | 'system';
export type Theme = 'dark' | 'light';
export type Palette = 'celadon' | 'cinnabar' | 'azure' | 'amber';
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

/** Unknown or missing values are dark, so users from before themes existed see no change. */
export function parseTheme(value: unknown): ThemeChoice {
  return value === 'light' || value === 'system' ? value : 'dark';
}
export function resolveTheme(choice: ThemeChoice, systemDark: boolean): Theme {
  return choice === 'system' ? (systemDark ? 'dark' : 'light') : choice;
}
