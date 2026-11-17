import { invoke, isTauri } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import type { Theme, ThemeChoice } from '../utils/theme';
import {
  createCoverLoader,
  type CoverOutcome,
  type CoverPriority,
  type CoverStatus,
  type CoverTicket,
} from '../utils/cover-loader';
export type { CoverOutcome, CoverPriority, CoverStatus, CoverTicket };
export interface Track {
  path: string;
  title: string;
  artist: string;
  album: string;
  duration_ms: number;
}
export interface PlayerSnapshot {
  revision: number;
  metadata_revision?: number;
  track_id: string | null;
  current_track: Track | null;
  playback_state: 'idle' | 'playing' | 'paused';
  position_ms: number;
  duration_ms: number;
  volume: number;
  playback_mode: PlaybackMode;
  lyrics_offset_ms: number;
  output_device_id: string | null;
  error: string | null;
  queue: Track[];
}
interface BridgeReply {
  nonce: string;
  backend: string;
  snapshot: PlayerSnapshot;
}
export const desktopAvailable = isTauri();
export async function verifyConnection() {
  if (!desktopAvailable) throw new Error('浏览器预览 · 请在桌面应用中使用本地播放器');
  const nonce = crypto.randomUUID();
  const reply = await invoke<BridgeReply>('bridge_probe', { nonce });
  if (reply.nonce !== nonce || reply.backend !== 'rust') throw new Error('连接响应校验失败');
  return reply;
}
export const getPlayerState = () => invoke<PlayerSnapshot>('player_get_state');
export const outputDevices = () => invoke<string[]>('player_output_devices');
export const setOutputDevice = (device: string | null) =>
  invoke<PlayerSnapshot>('player_set_output_device', { device });
export const openFile = () => invoke<PlayerSnapshot>('player_open_file');
export const play = () => invoke<PlayerSnapshot>('player_play');
export const pause = () => invoke<PlayerSnapshot>('player_pause');
export const stop = () => invoke<PlayerSnapshot>('player_stop');
export const seek = (positionMs: number) => invoke<PlayerSnapshot>('player_seek', { positionMs });
export const setVolume = (volume: number) =>
  invoke<PlayerSnapshot>('player_set_volume', { volume });
export interface LibraryTrack extends Track {
  id: number;
  album_artist: string;
  genre: string;
  year: number | null;
  track_number: number | null;
  disc_number: number | null;
  added_at: number;
  album_source: string;
}
export interface Group {
  key: string;
  name: string;
  artist: string;
  count: number;
  cover_id: number;
}
export interface Playlist {
  id: number;
  name: string;
  count: number;
}
export interface LibrarySummary {
  tracks: number;
  albums: Group[];
  artists: Group[];
  folders: string[];
  playlists: Playlist[];
}
export interface LibraryQuery {
  search?: string;
  sort?: string;
  offset?: number;
  limit?: number;
  playlist?: number | null;
  album?: string | null;
  artist?: string | null;
  enrichment_ignored?: boolean | null;
}
// Rust still returns stored favourite IDs for database compatibility; the UI no longer reads them.
export interface LibraryPage {
  tracks: LibraryTrack[];
  total: number;
}
export interface ScanProgress {
  running: boolean;
  visited: number;
  imported: number;
  current: string;
  cancelled: boolean;
  errors: string[];
}
export const queryLibrary = (query: LibraryQuery) =>
  invoke<LibraryPage>('library_query', { query });
export const librarySummary = () => invoke<LibrarySummary>('library_summary');
export const removeFolder = (path: string) => invoke<void>('library_remove_folder', { path });
export const scanFolder = () => {
  clearCoverCaches();
  return invoke<void>('library_scan');
};
export const scanProgress = () => invoke<ScanProgress>('library_progress');
export const cancelScan = () => invoke<void>('library_cancel');
export const trackDetails = (id: number) => invoke<LibraryTrack>('library_track', { id });
export const trackByPath = (path: string) =>
  invoke<LibraryTrack | null>('library_track_by_path', { path });
export interface AudioInfo {
  format: string;
  sample_rate: number | null;
  channels: number | null;
  bitrate: number | null;
  bit_depth: number | null;
  file_size: number | null;
}
export const audioInfo = (id: number) => invoke<AudioInfo>('library_audio_info', { id });
export interface TagCandidate {
  source?: 'MusicBrainz' | 'iTunes' | 'LRCLIB' | 'Manual';
  title: string;
  artist: string;
  album: string;
  album_artist: string;
  year: number | null;
  track_number: number | null;
  release_mbid: string | null;
  cover_preview_url: string | null;
  cover_preview: string | null;
  cover_url?: string | null;
  genre?: string | null;
}
export interface LyricsCandidate {
  id: number;
  title: string;
  artist: string;
  album: string;
  preview: string;
  synced_lyrics: string;
}
export interface EnrichmentCandidates {
  title_query: string;
  artist_query: string;
  tags: TagCandidate[];
  lyrics: LyricsCandidate[];
  errors: string[];
}
export interface ApplyEnrichmentRequest {
  track_id: number;
  tags: TagCandidate | null;
  update_library_tags: boolean;
  synced_lyrics: string | null;
  save_lyrics: boolean;
  save_cover: boolean;
  release_mbid: string | null;
  write_audio_tags: boolean;
  overwrite_cover: boolean;
  overwrite_lyrics: boolean;
  dedicated_cover?: boolean;
  cover_url?: string | null;
}
export interface MetadataStatus {
  track_id: number;
  enrichment_ignored: boolean;
  missing_artist: boolean;
  missing_album: boolean;
  missing_cover: boolean | null;
  missing_lyrics: boolean | null;
}
export const metadataTrackStatuses = (trackIds: number[]) =>
  invoke<MetadataStatus[]>('metadata_track_statuses', { trackIds });
export const setEnrichmentIgnored = (trackId: number, ignored: boolean) =>
  invoke<void>('metadata_set_enrichment_ignored', { trackId, ignored });
export const searchMetadataCandidates = (
  trackId: number,
  titleQuery: string | null = null,
  artistQuery: string | null = null,
  forceRefresh = false,
) =>
  invoke<EnrichmentCandidates>('metadata_search_candidates', {
    trackId,
    titleQuery,
    artistQuery,
    forceRefresh,
  });
export const applyMetadataEnrichment = (payload: ApplyEnrichmentRequest) =>
  invoke<LibraryTrack>('metadata_apply_enrichment', { payload });
// Every library cover request goes through one bounded loader: null results mean "no cover" and are
// kept; error codes (file missing or unreadable, IPC failure) get limited retries and are never kept as null.
const covers = createCoverLoader({ terminal: (code) => code === 'cover-not-in-library' });
// Counts and durations only; no paths or image data.
Object.assign(globalThis, { __musicMineCoverStats: covers.stats });
const noTicket: CoverTicket = { setPriority() {}, cancel() {} };
const thumbEdge = (px: number) => Math.max(32, Math.min(960, Math.round(px)));
const thumbKey = (id: number, px: number) => 't:' + id + '@' + thumbEdge(px);
export const peekTrackThumb = (id: number, px: number) => covers.peek(thumbKey(id, px));
/** Small and medium covers. Rust resizes; the page does not receive the original image. */
export function requestTrackThumb(
  id: number,
  px: number,
  priority: CoverPriority,
  done: (outcome: CoverOutcome) => void,
) {
  const edge = thumbEdge(px);
  return covers.request(
    thumbKey(id, px),
    priority,
    () => invoke<string | null>('library_thumb', { id, edge }),
    done,
  );
}
export function trackThumb(id: number, px: number) {
  return new Promise<string | null>((resolve, reject) => {
    requestTrackThumb(id, px, 1, (o) =>
      o.status === 'failed' ? reject(new Error(o.code)) : resolve(o.src),
    );
  });
}
/** Song details use a larger JPEG, still bounded, instead of the embedded original. */
export function trackCover(id: number) {
  return trackThumb(id, 720);
}
export const coverStats = () => invoke<Record<string, number>>('library_cover_stats');
export const homeData = () =>
  invoke<{ recent: LibraryTrack[]; artists: [string, number][] }>('library_home');
export const playTrack = (id: number) => invoke<PlayerSnapshot>('library_play', { id });
export const createPlaylist = (name: string) =>
  invoke<number | null>('library_playlist', { name, playlistId: null, trackId: null }).then(
    (id) => {
      if (id == null) throw new Error('创建歌单失败');
      return id;
    },
  );
export const addToPlaylist = (playlistId: number, trackId: number) =>
  invoke<number | null>('library_playlist', { name: null, playlistId, trackId }).then(() => {});
// Playlist edits address rows by ID only; library tracks and audio files are never touched.
export const renamePlaylist = (playlistId: number, name: string) =>
  invoke<void>('library_playlist_edit', { action: 'rename', playlistId, name, trackId: null });
export const deletePlaylist = (playlistId: number) =>
  invoke<void>('library_playlist_edit', {
    action: 'delete',
    playlistId,
    name: null,
    trackId: null,
  });
export const removeFromPlaylist = (playlistId: number, trackId: number) =>
  invoke<void>('library_playlist_edit', {
    action: 'remove_track',
    playlistId,
    name: null,
    trackId,
  });
export const reorderPlaylist = (playlistId: number, from: number, to: number) =>
  invoke<void>('library_playlist_edit', { action: 'reorder', playlistId, from, to });
export const queueAction = (
  action: 'add' | 'play_next' | 'remove' | 'next' | 'previous' | 'clear',
  id: number | null = null,
  path: string | null = null,
) => invoke<PlayerSnapshot>('library_queue', { action, id, path });
export const queueTracks = (ids: number[], replace: boolean) =>
  invoke<PlayerSnapshot>('library_queue_tracks', { ids, replace });
export async function collectTrackIds(query: LibraryQuery): Promise<number[]> {
  const ids: number[] = [];
  let offset = 0;
  while (true) {
    const page = await queryLibrary({ ...query, offset, limit: 200 });
    ids.push(...page.tracks.map((t) => t.id));
    offset += page.tracks.length;
    if (!page.tracks.length || offset >= page.total || ids.length >= 20000) return ids;
  }
}

export type PlaybackMode = 'sequential' | 'shuffle' | 'repeat_one' | 'repeat_all';
export interface Lyrics {
  lines: { time_ms: number; text: string }[];
  offset_ms: number;
  synced: boolean;
  source: string;
}
export const getLyrics = () => invoke<Lyrics | null>('player_lyrics');
/** `null` is "no cover"; a transient error (file busy or unreadable) is retried once. */
export async function getArtwork() {
  try {
    return await invoke<string | null>('player_artwork');
  } catch {
    covers.stats.retries++;
    await new Promise((r) => setTimeout(r, 800));
    return invoke<string | null>('player_artwork');
  }
}
const configure = (
  mode: PlaybackMode | null,
  offset: number | null,
  path: string | null,
  target: number | null,
) => invoke<PlayerSnapshot>('player_configure', { mode, offset, path, target });
export const setMode = (mode: PlaybackMode) => configure(mode, null, null, null);
export const setLyricsOffset = (offset: number) => configure(null, offset, null, null);
export const reorderQueue = (path: string, target: number) => configure(null, null, path, target);

export const playQueued = (path: string) => configure(null, null, path, null);

export interface DesktopPreferences {
  output_device: string | null;
  close_to_tray: boolean;
  queue_open: boolean;
  theme: string;
  lyrics_overlay_locked: boolean;
}
export const desktopPreferences = (
  closeToTray: boolean | null = null,
  queueOpen: boolean | null = null,
  theme: ThemeChoice | null = null,
) => invoke<DesktopPreferences>('desktop_preferences', { closeToTray, queueOpen, theme });
export interface LyricsOverlayState {
  open: boolean;
  locked: boolean;
}
/** `open` and `locked` are unchanged when null. The boolean is whether the window is visible. */
export const desktopLyricsWindow = (open: boolean | null = null, locked: boolean | null = null) =>
  invoke<boolean>('desktop_lyrics_window', { open, locked });
export const onLyricsOverlay = (handler: (state: LyricsOverlayState) => void) =>
  listen<LyricsOverlayState>('lyrics-overlay', (event) => handler(event.payload));
/** Native title bar colour; `system` hands control back to Windows. */
export const setWindowTheme = (choice: ThemeChoice) =>
  getCurrentWindow().setTheme(choice === 'system' ? null : choice);
export const windowTheme = () => getCurrentWindow().theme();
export const onWindowThemeChanged = (handler: (theme: Theme) => void) =>
  getCurrentWindow().onThemeChanged(({ payload }) => handler(payload));

export const mediaStatus = () => invoke<string>('media_status');
/** Sanitized session logs; clear affects the in-memory buffer only. */
export const appConsoleLogs = (clear = false) => invoke<string[]>('app_console_logs', { clear });

export interface ListenBrainzConfig {
  enabled: boolean;
  username: string | null;
  token_configured: boolean;
  pending_count: number;
}
/** Omitted/null values keep credentials; empty token clears it. No token is returned. */
export const listenbrainzConfig = (
  enabled: boolean | null = null,
  username: string | null = null,
  token: string | null = null,
) => invoke<ListenBrainzConfig>('listenbrainz_config', { enabled, username, token });
export const listenbrainzRecommendations = () =>
  invoke<LibraryTrack[]>('listenbrainz_recommendations');

export interface AiConfig {
  enabled: boolean;
  base_url: string;
  model: string;
  has_key: boolean;
  revision: number;
}
/** null keeps each setting; empty apiKey clears it and disables AI. Key is never returned. */
export const aiConfig = (
  enabled: boolean | null = null,
  baseUrl: string | null = null,
  apiKey: string | null = null,
  model: string | null = null,
) => invoke<AiConfig>('ai_config', { enabled, baseUrl, apiKey, model });
export interface AiToolCall {
  id: string;
  type: 'function';
  function: { name: string; arguments: string };
}
export interface AiMessage {
  role: 'system' | 'user' | 'assistant' | 'tool';
  content: string | null;
  tool_calls?: AiToolCall[];
  tool_call_id?: string;
}
export interface AiTurnResponse {
  message: AiMessage;
  finish_reason: string;
}
export const aiChatTurn = (messages: AiMessage[], tools: unknown[]) =>
  invoke<AiTurnResponse>('ai_chat_turn', { messages, tools });

// Exact library path only. A path that is not a library row fails with `cover-not-in-library`.
const queueKey = (path: string) => 'q:' + path + '@112';
export const peekQueueCover = (path: string): CoverOutcome | undefined =>
  desktopAvailable ? covers.peek(queueKey(path)) : { status: 'none', src: null };
export function requestQueueCover(
  path: string,
  priority: CoverPriority,
  done: (outcome: CoverOutcome) => void,
): CoverTicket {
  if (!desktopAvailable) {
    done({ status: 'none', src: null });
    return noTicket;
  }
  return covers.request(
    queueKey(path),
    priority,
    () => invoke<string | null>('library_queue_thumb', { path, edge: 112 }),
    done,
  );
}
export function clearCoverCaches() {
  covers.clear();
}
