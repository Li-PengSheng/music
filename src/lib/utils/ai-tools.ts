import {
  librarySummary,
  queryLibrary,
  trackDetails,
  createPlaylist,
  addToPlaylist,
  queueTracks,
  setMode,
  play,
  pause,
  queueAction,
  setVolume,
  type LibraryTrack,
  type PlayerSnapshot,
  type PlaybackMode,
} from '../services/music';
import { player, control } from '../stores/player.svelte';
import { refreshSummary } from '../stores/library.svelte';
import { setTheme, setPalette, setFont } from '../stores/preferences';
import { setDesktopLyricsOpen } from '../stores/lyrics-overlay';
import type { ThemeChoice, Palette, FontPreset } from './theme';

const MODES = ['sequential', 'shuffle', 'repeat_one', 'repeat_all'] as const;
const THEMES = ['dark', 'light', 'system'] as const;
const PALETTES = ['celadon', 'cinnabar', 'azure', 'amber'] as const;
const FONTS = ['harmony', 'puhuiti', 'serif', 'kaiti', 'system'] as const;
const SORTS = ['title', 'artist', 'album', 'added', 'duration'] as const;
const idSchema = {
  type: 'array',
  items: { type: 'integer', minimum: 1 },
  minItems: 1,
  maxItems: 50,
};
function tool(
  name: string,
  description: string,
  properties: Record<string, unknown>,
  required: string[],
) {
  return {
    type: 'function',
    function: {
      name,
      description,
      parameters: { type: 'object', properties, required, additionalProperties: false },
    },
  };
}
export const AI_TOOLS = [
  tool(
    'get_library_overview',
    '查看本地曲库概况，含歌曲数、前30张专辑/艺术家和前100个歌单。不含本地路径。',
    {},
    [],
  ),
  tool(
    'search_library',
    '按关键词查找本地曲目，返回可供播放/创建歌单的真实ID，最多50条。',
    {
      keyword: { type: 'string', minLength: 1, maxLength: 200 },
      sort: { type: 'string', enum: SORTS },
      limit: { type: 'integer', minimum: 1, maximum: 50 },
    },
    ['keyword'],
  ),
  tool(
    'create_playlist_with_tracks',
    '用已检索到的真实曲目ID创建新歌单。不删除现有歌单或文件。',
    { name: { type: 'string', minLength: 1, maxLength: 100 }, ids: idSchema },
    ['name', 'ids'],
  ),
  tool(
    'play_tracks',
    '替换播放队列并播放这些本地曲目，可指定播放模式。',
    { ids: idSchema, mode: { type: 'string', enum: MODES } },
    ['ids'],
  ),
  tool(
    'control_playback',
    '控制播放、暂停、上一首/下一首，或设置0到100的音量。',
    {
      action: { type: 'string', enum: ['play', 'pause', 'previous', 'next', 'volume'] },
      volume: { type: 'number', minimum: 0, maximum: 100 },
    },
    ['action'],
  ),
  tool(
    'set_appearance',
    '切换主题、色系、字体或明确开启/关闭桌面歌词。',
    {
      theme: { type: 'string', enum: THEMES },
      palette: { type: 'string', enum: PALETTES },
      font: { type: 'string', enum: FONTS },
      desktop_lyrics: { type: 'boolean' },
    },
    [],
  ),
];

function defaults() {
  return {
    summary: librarySummary,
    query: queryLibrary,
    track: trackDetails,
    createPlaylist,
    addToPlaylist,
    refresh: refreshSummary,
    queueTracks,
    setMode,
    play,
    pause,
    queueAction,
    setVolume,
    runPlayer: control,
    getPlayer: () => player.snapshot,
    setTheme,
    setPalette,
    setFont,
    lyrics: setDesktopLyricsOpen,
  };
}
export type AiToolDependencies = ReturnType<typeof defaults>;
export type AiToolResult = { ok: boolean; [key: string]: unknown };
function object(input: unknown, keys: string[]): Record<string, unknown> {
  if (
    !input ||
    typeof input !== 'object' ||
    Array.isArray(input) ||
    Object.keys(input).some((k) => !keys.includes(k))
  )
    throw Error('invalid_arguments');
  return input as Record<string, unknown>;
}
function text(value: unknown, max = 200) {
  if (
    typeof value !== 'string' ||
    !value.trim() ||
    value.length > max ||
    /[\u0000-\u001f]/.test(value)
  )
    throw Error('invalid_arguments');
  return value.trim();
}
function ids(value: unknown): number[] {
  if (
    !Array.isArray(value) ||
    !value.length ||
    value.length > 50 ||
    value.some((id) => !Number.isSafeInteger(id) || id <= 0)
  )
    throw Error('invalid_arguments');
  return [...new Set(value)] as number[];
}
function choice<T extends string>(value: unknown, choices: readonly T[]): T {
  if (typeof value !== 'string' || !choices.includes(value as T)) throw Error('invalid_arguments');
  return value as T;
}
const short = (s: string) => s.slice(0, 100);
function project(t: LibraryTrack) {
  return {
    id: t.id,
    title: short(t.title),
    artist: short(t.artist),
    album: short(t.album),
    duration_ms: t.duration_ms,
  };
}
function check(signal?: AbortSignal) {
  if (signal?.aborted) throw Error('cancelled');
}

/** All model arguments are untrusted. No dynamic dispatch, shell or arbitrary IPC names. */
export function createAiToolExecutor(d: AiToolDependencies) {
  return async (name: string, input: unknown, signal?: AbortSignal): Promise<AiToolResult> => {
    let playlistId: number | undefined;
    let added = 0;
    try {
      check(signal);
      async function validTracks(values: number[]) {
        for (const id of values) {
          check(signal);
          const track = await d.track(id);
          if (track.id !== id) throw Error('invalid_track');
        }
        check(signal);
      }
      async function transport(action: () => Promise<PlayerSnapshot>) {
        check(signal);
        if (!(await d.runPlayer(action)) || d.getPlayer()?.error) throw Error('playback_failed');
        check(signal);
        const s = d.getPlayer();
        return {
          ok: true,
          status: s?.playback_state,
          volume_percent: Math.round((s?.volume ?? 0) * 100),
        };
      }
      switch (name) {
        case 'get_library_overview': {
          object(input, []);
          const s = await d.summary();
          check(signal);
          return {
            ok: true,
            tracks: s.tracks,
            albums: s.albums.slice(0, 30).map((a) => ({ name: short(a.name), count: a.count })),
            artists: s.artists.slice(0, 30).map((a) => ({ name: short(a.name), count: a.count })),
            playlists: s.playlists
              .slice(0, 100)
              .map((p) => ({ id: p.id, name: short(p.name), count: p.count })),
            playlist_total: s.playlists.length,
          };
        }
        case 'search_library': {
          const a = object(input, ['keyword', 'sort', 'limit']);
          const keyword = text(a.keyword);
          const sort = a.sort === undefined ? 'title' : choice(a.sort, SORTS);
          const limit = a.limit === undefined ? 50 : a.limit;
          if (!Number.isInteger(limit) || Number(limit) < 1 || Number(limit) > 50)
            throw Error('invalid_arguments');
          const found = await d.query({ search: keyword, sort, limit: Number(limit), offset: 0 });
          check(signal);
          return {
            ok: true,
            total: found.total,
            tracks: found.tracks.slice(0, Number(limit)).map(project),
          };
        }
        case 'create_playlist_with_tracks': {
          const a = object(input, ['name', 'ids']);
          const name = text(a.name, 100);
          const values = ids(a.ids);
          await validTracks(values);
          playlistId = await d.createPlaylist(name);
          for (const id of values) {
            check(signal);
            await d.addToPlaylist(playlistId, id);
            added++;
          }
          check(signal);
          return { ok: true, playlist_id: playlistId, name, added };
        }
        case 'play_tracks': {
          const a = object(input, ['ids', 'mode']);
          const values = ids(a.ids);
          const mode = a.mode === undefined ? undefined : choice(a.mode, MODES);
          await validTracks(values);
          return await transport(async () => {
            if (mode) {
              const s = await d.setMode(mode as PlaybackMode);
              if (s.error) return s;
              check(signal);
            }
            return d.queueTracks(values, true);
          });
        }
        case 'control_playback': {
          const a = object(input, ['action', 'volume']);
          const action = choice(a.action, ['play', 'pause', 'previous', 'next', 'volume']);
          if (action === 'volume') {
            if (
              typeof a.volume !== 'number' ||
              !Number.isFinite(a.volume) ||
              a.volume < 0 ||
              a.volume > 100
            )
              throw Error('invalid_arguments');
            const value = a.volume;
            return await transport(() => d.setVolume(value / 100));
          }
          if (a.volume !== undefined) throw Error('invalid_arguments');
          return await transport(
            action === 'play' ? d.play : action === 'pause' ? d.pause : () => d.queueAction(action),
          );
        }
        case 'set_appearance': {
          const a = object(input, ['theme', 'palette', 'font', 'desktop_lyrics']);
          if (!Object.keys(a).length) throw Error('invalid_arguments');
          const theme = a.theme === undefined ? undefined : choice(a.theme, THEMES);
          const palette = a.palette === undefined ? undefined : choice(a.palette, PALETTES);
          const font = a.font === undefined ? undefined : choice(a.font, FONTS);
          if (a.desktop_lyrics !== undefined && typeof a.desktop_lyrics !== 'boolean')
            throw Error('invalid_arguments');
          check(signal);
          // Do the fallible native operation before the local visual changes.
          if (typeof a.desktop_lyrics === 'boolean') {
            await d.lyrics(a.desktop_lyrics);
            check(signal);
          }
          if (theme) d.setTheme(theme as ThemeChoice);
          if (palette) d.setPalette(palette as Palette);
          if (font) d.setFont(font as FontPreset);
          return { ok: true, ...a };
        }
        default:
          return { ok: false, error: 'unknown_tool' };
      }
    } catch (e) {
      // Never return filesystem/DB/native error strings to a model.
      const code =
        e instanceof Error &&
        ['invalid_arguments', 'cancelled', 'invalid_track', 'playback_failed'].includes(e.message)
          ? e.message
          : 'operation_failed';
      return {
        ok: false,
        error: code,
        ...(playlistId === undefined
          ? {}
          : {
              playlist_id: playlistId,
              added,
              detail: '歌单已创建，部分歌曲可能未添加；请勿自动重试创建。',
            }),
      };
    } finally {
      if (playlistId !== undefined) await d.refresh().catch(() => {});
    }
  };
}
export const executeAiTool = (name: string, input: unknown, signal?: AbortSignal) =>
  createAiToolExecutor(defaults())(name, input, signal);
