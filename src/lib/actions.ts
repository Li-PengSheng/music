import {
  Play,
  ListEnd,
  ListStart,
  Info,
  Pencil,
  Trash2,
  ListPlus,
  Disc3,
  MicVocal,
  ListMinus,
  Shuffle,
  ArrowUp,
  ArrowDown,
} from 'lucide-svelte';
import {
  playTrack,
  queueAction,
  queueTracks,
  collectTrackIds,
  addToPlaylist,
  removeFromPlaylist,
  setMode,
  createPlaylist,
  renamePlaylist,
  deletePlaylist,
  trackDetails,
  scanFolder,
  scanProgress,
  desktopAvailable,
  type LibraryTrack,
  type LibraryQuery,
  type Playlist,
  type Group,
} from './services/music';
import { perform, control } from './stores/player.svelte';
import { library, refreshSummary } from './stores/library.svelte';
import { go, forgetPlaylist } from './stores/nav.svelte';
import {
  ui,
  confirmAction,
  promptText,
  notify,
  notifyError,
  type MenuItem,
} from './stores/ui.svelte';
import { playlistNameError, suggestPlaylistName } from './utils/playlist-name';
import { LOCAL_COLLECTION, localQuery } from './utils/collections';

export async function playCollection(query: LibraryQuery, replace = true, label = '') {
  try {
    const ids = await collectTrackIds(query);
    if (!ids.length) {
      notify('这里还没有可以播放的歌曲');
      return;
    }
    await perform(
      () => queueTracks(ids, replace),
      replace ? undefined : `已将 ${ids.length} 首歌曲${label ? '（' + label + '）' : ''}加入队列`,
    );
  } catch (e) {
    notifyError(e);
  }
}

export async function importFolder() {
  if (!desktopAvailable) {
    notify('请在 Music Mine 桌面窗口中导入音乐文件夹');
    return;
  }
  try {
    await scanFolder();
    library.progress = await scanProgress();
    if (!library.progress.running) await refreshSummary();
  } catch (e) {
    notifyError(e);
  }
}

export async function openDetails(track: LibraryTrack) {
  try {
    ui.details = await trackDetails(track.id);
  } catch (e) {
    notifyError(e);
  }
}

async function addTo(playlist: Playlist, track: LibraryTrack) {
  try {
    await addToPlaylist(playlist.id, track.id);
    await refreshSummary();
    notify(`已加入「${playlist.name}」`);
  } catch (e) {
    notifyError(e);
  }
}

export function songMenu(
  track: LibraryTrack,
  options: {
    playlistId?: number | null;
    onDetails: () => void;
    order?: { index: number; total: number; moving: boolean; move: (to: number) => unknown };
  },
): MenuItem[] {
  const items: MenuItem[] = [
    { label: '播放', icon: Play, action: () => perform(() => playTrack(track.id)) },
    {
      label: '下一首播放',
      icon: ListStart,
      action: () => perform(() => queueAction('play_next', track.id), '将在当前歌曲之后播放'),
    },
    {
      label: '加入播放队列',
      icon: ListEnd,
      action: () => perform(() => queueAction('add', track.id), '已加入播放队列'),
    },
  ];
  const targets = library.summary.playlists.filter((p) => p.id !== options.playlistId);
  items.push({ kind: 'separator' });
  if (targets.length) {
    items.push({ kind: 'heading', label: '添加到歌单' });
    for (const p of targets.slice(0, 12))
      items.push({ label: p.name, icon: ListPlus, action: () => addTo(p, track) });
  }
  items.push({ label: '新建歌单并添加…', icon: ListPlus, action: () => newPlaylist(track) });
  if (options.playlistId != null) {
    const id = options.playlistId;
    if (options.order) {
      const { index, total, moving, move } = options.order;
      const up = moving || index <= 0 || index >= total;
      const down = moving || index < 0 || index >= total - 1;
      items.push(
        { kind: 'separator' },
        {
          label: '上移一位',
          icon: ArrowUp,
          disabled: up,
          action: () => {
            if (!up) return move(index - 1);
          },
        },
        {
          label: '下移一位',
          icon: ArrowDown,
          disabled: down,
          action: () => {
            if (!down) return move(index + 1);
          },
        },
      );
    }
    items.push(
      { kind: 'separator' },
      {
        label: '从此歌单移除',
        icon: ListMinus,
        action: async () => {
          try {
            await removeFromPlaylist(id, track.id);
            await refreshSummary();
            notify('已从歌单移除，音乐文件不受影响');
          } catch (e) {
            notifyError(e);
          }
        },
      },
    );
  }
  items.push({ kind: 'separator' }, { label: '歌曲详情', icon: Info, action: options.onDetails });
  return items;
}

export function albumMenu(album: Group): MenuItem[] {
  const query = { album: album.key, sort: 'album' };
  return [
    { label: '播放专辑', icon: Play, action: () => playCollection(query) },
    {
      label: '加入播放队列',
      icon: ListEnd,
      action: () => playCollection(query, false, album.name),
    },
    { kind: 'separator' },
    { label: '打开专辑', icon: Disc3, action: () => go({ view: 'album', key: album.key }) },
  ];
}

export function artistMenu(artist: Group): MenuItem[] {
  const query = { artist: artist.name, sort: 'album' };
  return [
    { label: '播放全部', icon: Play, action: () => playCollection(query) },
    {
      label: '加入播放队列',
      icon: ListEnd,
      action: () => playCollection(query, false, artist.name),
    },
    { kind: 'separator' },
    {
      label: '查看艺术家',
      icon: MicVocal,
      action: () => go({ view: 'artist', name: artist.name }),
    },
  ];
}

export function playlistMenu(playlist: Playlist): MenuItem[] {
  return [
    {
      label: '播放歌单',
      icon: Play,
      disabled: !playlist.count,
      action: () => playCollection({ playlist: playlist.id }),
    },
    {
      label: '加入播放队列',
      icon: ListEnd,
      disabled: !playlist.count,
      action: () => playCollection({ playlist: playlist.id }, false, playlist.name),
    },
    { kind: 'separator' },
    { label: '重命名…', icon: Pencil, action: () => renamePlaylistDialog(playlist) },
    {
      label: '删除歌单…',
      icon: Trash2,
      danger: true,
      action: () => deletePlaylistDialog(playlist),
    },
  ];
}

/** The built-in collection can be played but never renamed or deleted: it owns no rows and no files. */
export function localMenu(): MenuItem[] {
  const query = localQuery('added', '');
  return [
    {
      label: '播放全部',
      icon: Play,
      disabled: !library.summary.tracks,
      action: () => playCollection(query),
    },
    {
      label: '随机播放',
      icon: Shuffle,
      disabled: !library.summary.tracks,
      action: async () => {
        if (await control(() => setMode('shuffle'))) await playCollection(query);
      },
    },
    {
      label: '加入播放队列',
      icon: ListEnd,
      disabled: !library.summary.tracks,
      action: () => playCollection(query, false, LOCAL_COLLECTION.name),
    },
  ];
}

export async function newPlaylist(track?: LibraryTrack) {
  let created: number | null = null;
  const ok = await promptText({
    title: track ? '新建歌单并添加歌曲' : '新建歌单',
    label: '歌单名称',
    confirmLabel: '创建',
    value: suggestPlaylistName(library.summary.playlists),
    validate: (v) => playlistNameError(v, library.summary.playlists),
    submit: async (v) => {
      created = await createPlaylist(v.trim());
      if (track) await addToPlaylist(created, track.id);
    },
  });
  if (!ok || created == null) return;
  await refreshSummary();
  if (track) notify('已创建歌单并加入歌曲');
  else go({ view: 'playlist', id: created });
}

export async function renamePlaylistDialog(playlist: Playlist) {
  const ok = await promptText({
    title: '重命名歌单',
    label: '歌单名称',
    value: playlist.name,
    confirmLabel: '保存',
    validate: (v) => playlistNameError(v, library.summary.playlists, playlist.id),
    submit: (v) => renamePlaylist(playlist.id, v.trim()),
  });
  if (ok) {
    await refreshSummary();
    notify('歌单已重命名');
  }
}

export async function deletePlaylistDialog(playlist: Playlist) {
  const ok = await confirmAction({
    title: `删除歌单「${playlist.name}」？`,
    message: '只删除歌单，不删除本地音乐文件。歌曲仍保留在音乐库中。',
    detail: playlist.count
      ? `歌单内有 ${playlist.count} 首歌曲。播放队列中的歌曲不受影响，当前播放会继续。`
      : undefined,
    confirmLabel: '删除歌单',
    danger: true,
  });
  if (!ok) return;
  try {
    await deletePlaylist(playlist.id);
    forgetPlaylist(playlist.id);
    notify(`已删除歌单「${playlist.name}」`);
  } catch (e) {
    notifyError(e);
  } finally {
    await refreshSummary();
  }
}
