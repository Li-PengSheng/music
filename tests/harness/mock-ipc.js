// Browser-only layout harness. Loaded exclusively by tests/harness/index.html; the app never imports it.
// It answers Tauri commands with a synthetic in-memory library so lists, dialogs and the queue can be
// inspected in a normal browser. Playback state never advances on its own.
const TOTAL = Number(new URLSearchParams(location.search).get('tracks') ?? 471);
const tracks = Array.from({ length: TOTAL }, (_, i) => ({
  id: i + 1,
  path: `C:\\Fixture\\${String(i + 1).padStart(3, '0')}.flac`,
  title: `测试歌曲 ${i + 1}`,
  artist: `艺术家 ${i % 17}`,
  album: `专辑 ${i % 40}`,
  album_artist: '',
  genre: '',
  year: 2020,
  track_number: (i % 12) + 1,
  disc_number: null,
  duration_ms: 180000 + i * 997,
  added_at: TOTAL - i,
  album_source: 'tag',
}));
const playlists = [
  { id: 7, name: '本地', count: 0 },
  { id: 8, name: '夜晚', count: 2 },
];
const members = { 7: [], 8: [3, 5] };
const playlistSize = Number(new URLSearchParams(location.search).get('playlistTracks'));
if (playlistSize > 0) members[8] = tracks.slice(0, playlistSize).map((t) => t.id);
const folders = ['C:\\Fixture'];
function groups(kind) {
  const map = new Map();
  for (const t of tracks) {
    const name = kind === 'album' ? t.album : t.artist;
    const current = map.get(name);
    if (current) current.count++;
    else
      map.set(name, {
        key: name,
        name,
        artist: kind === 'album' ? t.artist : '',
        count: 1,
        cover_id: t.id,
      });
  }
  return [...map.values()];
}
const queue = tracks.slice(0, 12).map(({ path, title, artist, album, duration_ms }) => ({
  path,
  title,
  artist,
  album,
  duration_ms,
}));
let snapshot = {
  revision: 1,
  track_id: queue[0].path,
  current_track: queue[0],
  playback_state: 'paused',
  position_ms: 42000,
  duration_ms: queue[0].duration_ms,
  volume: 0.7,
  playback_mode: 'sequential',
  lyrics_offset_ms: 0,
  output_device_id: 'Fixture output',
  error: null,
  queue,
};
const lyricsBoot = window.__lyricsPrefs ?? {};
let lyricsOpen = !!lyricsBoot.lyrics_overlay_open;
const prefs = {
  close_to_tray: false,
  queue_open: false,
  theme: null,
  output_device: new URLSearchParams(location.search).get('outputDevice'),
  lyrics_overlay_locked: !!lyricsBoot.lyrics_overlay_locked,
};
if (window.__lyricsFixture) Object.assign(snapshot, window.__lyricsFixture);
window.__mockOutputDevices = ['Fixture output', 'Headphones'];
const log = [];
let consoleLines = ['[12:00:00] [audio] fixture paused', '[12:00:01] [smtc] Fixture'];
window.__mockLog = log;
window.__mockSnapshot = () => snapshot;
function page(q) {
  let list = tracks;
  if (q.playlist != null)
    list = (members[q.playlist] ?? []).map((id) => tracks.find((t) => t.id === id)).filter(Boolean);
  if (q.search) {
    const s = q.search.toLowerCase();
    list = list.filter((t) => (t.title + t.artist + t.album).toLowerCase().includes(s));
  }
  if (q.artist) list = list.filter((t) => t.artist === q.artist);
  if (q.album) list = list.filter((t) => t.album === q.album);
  if (q.playlist == null && q.sort === 'title')
    list = [...list].sort((a, b) => a.title.localeCompare(b.title) || a.id - b.id);
  const offset = q.offset ?? 0,
    limit = q.limit ?? 100;
  return { tracks: list.slice(offset, offset + limit), total: list.length, favorites: [] };
}
const handlers = {
  bridge_probe: (a) => ({ nonce: a.nonce, backend: 'rust', snapshot }),
  player_get_state: () => snapshot,
  player_output_devices: () => window.__mockOutputDevices,
  player_set_output_device: ({ device }) => {
    prefs.output_device = device || null;
    snapshot = {
      ...snapshot,
      revision: snapshot.revision + 1,
      output_device_id: window.__mockOutputDevices.includes(device) ? device : 'Fixture output',
    };
    return snapshot;
  },
  desktop_preferences: (a) => {
    if (a.closeToTray != null) prefs.close_to_tray = a.closeToTray;
    if (a.queueOpen != null) prefs.queue_open = a.queueOpen;
    if (a.theme !== undefined && a.theme !== null) prefs.theme = a.theme;
    return { ...prefs };
  },
  desktop_lyrics_window: (a) => {
    if (a.locked != null) prefs.lyrics_overlay_locked = a.locked;
    if (a.open != null) lyricsOpen = a.open;
    return lyricsOpen;
  },
  media_status: () => 'Fixture',
  app_console_logs: ({ clear }) => {
    if (clear) consoleLines = [];
    return consoleLines;
  },
  library_progress: () => ({
    running: false,
    visited: 0,
    imported: 0,
    current: '',
    cancelled: false,
    errors: [],
  }),
  library_summary: () => ({
    tracks: tracks.length,
    albums: groups('album'),
    artists: groups('artist'),
    folders: [...folders],
    playlists: playlists.map((p) => ({ ...p, count: (members[p.id] ?? []).length })),
  }),
  library_query: (a) => page(a.query),
  library_remove_folder: ({ path }) => {
    const index = folders.indexOf(path);
    if (index < 0) throw '音乐文件夹不存在';
    folders.splice(index, 1);
    const prefix = path.replaceAll('/', '\\').replace(/[\\/]+$/, '') + '\\';
    const deleted = new Set(
      tracks.filter((t) => t.path.replaceAll('/', '\\').startsWith(prefix)).map((t) => t.id),
    );
    for (let i = tracks.length - 1; i >= 0; i--) if (deleted.has(tracks[i].id)) tracks.splice(i, 1);
    for (const id of Object.keys(members)) members[id] = members[id].filter((t) => !deleted.has(t));
    return null;
  },
  library_home: () => ({
    recent: tracks.slice(0, 6),
    artists:
      window.__mockHomeArtists ??
      groups('artist')
        .slice(0, 8)
        .map((g) => [g.name, g.count]),
  }),
  library_cover: () => null,
  library_thumb: () => window.__mockThumb ?? null,
  library_queue_thumb: () => null,
  library_track: (a) => tracks.find((t) => t.id === a.id),
  library_track_by_path: (a) => tracks.find((t) => t.path === a.path) ?? null,
  library_audio_info: async (a) => {
    const custom = window.__mockAudioInfo?.[a.id];
    if (custom?.delay) await new Promise((r) => setTimeout(r, custom.delay));
    return (
      custom?.info ?? {
        format: 'FLAC',
        sample_rate: 44100,
        channels: 2,
        bitrate: 872,
        bit_depth: 16,
        file_size: 27800000,
      }
    );
  },
  library_playlist: (a) => {
    if (a.name) {
      if (playlists.some((p) => p.name === a.name.trim()))
        throw `已存在名为「${a.name.trim()}」的歌单`;
      const id = Math.max(...playlists.map((p) => p.id)) + 1;
      playlists.push({ id, name: a.name.trim(), count: 0 });
      members[id] = [];
      return id;
    }
    members[a.playlistId].push(a.trackId);
    return null;
  },
  library_playlist_edit: (a) => {
    if (a.action === 'reorder') {
      const list = members[a.playlistId];
      if (!list) throw '歌单不存在';
      if (
        !Number.isInteger(a.from) ||
        !Number.isInteger(a.to) ||
        a.from < 0 ||
        a.to < 0 ||
        a.from >= list.length ||
        a.to >= list.length
      )
        throw '歌曲位置超出歌单范围';
      list.splice(a.to, 0, list.splice(a.from, 1)[0]);
      return null;
    }
    if (a.action === 'delete') {
      const i = playlists.findIndex((p) => p.id === a.playlistId);
      if (i >= 0) playlists.splice(i, 1);
      delete members[a.playlistId];
    }
    if (a.action === 'rename') {
      const p = playlists.find((p) => p.id === a.playlistId);
      if (p && a.name) p.name = a.name;
    }
    if (a.action === 'remove_track' && members[a.playlistId])
      members[a.playlistId] = members[a.playlistId].filter((id) => id !== a.trackId);
    return null;
  },
  library_queue_tracks: () => snapshot,
  library_play: () => snapshot,
  library_queue: () => snapshot,
  player_lyrics: () =>
    window.__mockLyrics === undefined
      ? {
          lines: Array.from({ length: 30 }, (_, i) => ({
            time_ms: i * 4000,
            text: `歌词第 ${i + 1} 行`,
          })),
          offset_ms: 0,
          synced: true,
          source: 'fixture.lrc',
        }
      : window.__mockLyrics,
  player_artwork: () => null,
  player_configure: () => snapshot,
  player_play: () => snapshot,
  player_pause: () => snapshot,
  player_stop: () => snapshot,
  player_open_file: () => snapshot,
  player_seek: () => snapshot,
  player_set_volume: async (a) => {
    await new Promise((r) => setTimeout(r, 40));
    snapshot = {
      ...snapshot,
      volume: Math.min(1, Math.max(0, a.volume)),
      revision: snapshot.revision + 1,
    };
    return snapshot;
  },
  library_scan: () => null,
  library_cancel: () => null,
  library_cover_stats: () => ({ requests: 12, memoryHits: 8, diskHits: 2 }),
  library_import: () => null,
};
let callbackId = 0;
const eventHandlers = new Map();
let eventId = 0;
window.__mockEmit = (event, payload) => {
  for (const [id, entry] of eventHandlers) {
    if (entry.event === event) window['_' + entry.handler]?.({ event, id, payload });
  }
};
let maximized = false;
window.isTauri = true;
window.__TAURI_INTERNALS__ = {
  metadata: {
    currentWindow: { label: 'main' },
    currentWebview: { windowLabel: 'main', label: 'main' },
  },
  transformCallback: (cb) => {
    const id = ++callbackId;
    window['_' + id] = cb;
    return id;
  },
  unregisterCallback: (id) => {
    delete window['_' + id];
  },
  invoke: async (cmd, args = {}) => {
    log.push({ cmd, args });
    if (window.__mockFailures?.[cmd]) throw window.__mockFailures[cmd];
    if (cmd === 'plugin:event|listen') {
      const id = ++eventId;
      eventHandlers.set(id, { event: args.event, handler: args.handler });
      return id;
    }
    if (cmd === 'plugin:event|unlisten') {
      eventHandlers.delete(args.eventId);
      return null;
    }
    if (cmd === 'plugin:window|is_maximized') return maximized;
    if (cmd === 'plugin:window|toggle_maximize') {
      maximized = !maximized;
      return null;
    }
    const h = handlers[cmd];
    if (cmd.startsWith('plugin:')) return null;
    if (!h) throw `mock: unhandled command ${cmd}`;
    return structuredClone(await h(args));
  },
};
