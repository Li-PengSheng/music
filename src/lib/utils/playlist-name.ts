import { LOCAL_COLLECTION, shadowsBuiltIn } from './collections.ts';

export const PLAYLIST_NAME_MAX = 100;

/** Returns an error message, or null when the name can be saved. Rust repeats the length and duplicate checks. */
export function playlistNameError(
  value: string,
  existing: { id: number; name: string }[],
  selfId: number | null = null,
): string | null {
  const name = value.trim();
  if (!name) return '请输入歌单名称';
  if (name.length > PLAYLIST_NAME_MAX) return `歌单名称最多 ${PLAYLIST_NAME_MAX} 个字符`;
  const lower = name.toLowerCase();
  if (existing.some((p) => p.id !== selfId && p.name.trim().toLowerCase() === lower))
    return `已存在名为「${name}」的歌单`;
  const keepsOwnName =
    selfId != null &&
    existing.some((p) => p.id === selfId && p.name.trim().toLowerCase() === lower);
  if (shadowsBuiltIn(name) && !keepsOwnName)
    return `「${LOCAL_COLLECTION.name}」是系统歌单，会自动显示全部本地音乐`;
  return null;
}

/** Suggests “新建歌单 2” style names that do not collide with existing playlists. */
export function suggestPlaylistName(existing: { name: string }[], base = '新建歌单') {
  const names = new Set(existing.map((p) => p.name.trim().toLowerCase()));
  if (!names.has(base.toLowerCase())) return base;
  for (let i = 2; ; i++) {
    const candidate = `${base} ${i}`;
    if (!names.has(candidate.toLowerCase())) return candidate;
  }
}
