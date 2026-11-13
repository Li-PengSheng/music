/** Theme choice shared with public/theme-init.js, which applies it before the first paint. */
export type ThemeChoice='dark'|'light'|'system';
export type Theme='dark'|'light';

export const THEME_KEY='music-mine-theme';
export const THEME_COLOR:Record<Theme,string>={dark:'#121414',light:'#F7F8F5'};

/** Unknown or missing values are dark, so users from before themes existed see no change. */
export function parseTheme(value:unknown):ThemeChoice{return value==='light'||value==='system'?value:'dark';}
export function resolveTheme(choice:ThemeChoice,systemDark:boolean):Theme{return choice==='system'?(systemDark?'dark':'light'):choice;}
