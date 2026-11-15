# Task 02 implementation and handoff

Status: implementation prepared; build and desktop acceptance blocked by environment, not passed.

## Structure

The workspace is the application root. `src/App.svelte` owns the lightweight screen layout and local Runes state; `src/app.css` owns theme tokens, local Windows font fallback, responsive layout and generated cover art. `src/lib/services/music.ts` centralizes typed IPC. `src-tauri` contains the native window, commands and in-memory player state. No library, filesystem plugin, audio decoder, AI integration or background server is introduced.

The page contains navigation, album/song/artist views, filtering, temporary favorites, album details, an empty queue, a settings connection screen and playback controls. Every album is clearly identified as demo content. Favorites are session-only. Song/artist views do not claim to query a real library. Playback is not implemented; the play button explains this and does not change state.

The theme is an original estimate (no original screenshot/capture tokens were supplied): charcoal and sage, Segoe UI Variable / Segoe UI / Microsoft YaHei, fixed navigation and bottom controls with a scrollable center. Covers are CSS shapes; no third-party artwork or fonts are downloaded.

## IPC contract

- `bridge_probe(nonce)` returns the same nonce, `backend: rust`, and a snapshot. Frontend checks the random nonce before showing connected status. Browser preview never substitutes a fake success.
- `player_get_state()` returns the Rust-owned idle snapshot.
- `player_set_volume(volume)` validates finite 0..1 input, changes in-memory requested volume and increments revision. There is no audio device yet; this is a state transport test, not audible volume control.
- Snapshot fields follow RFC: revision, track_id, playback_state, position_ms, duration_ms, volume, playback_mode, output_device_id, error. Position and duration are zero; no output device or track is claimed.
- Errors are serialized code/message objects. UI shows a concise message. Frontend prevents older volume replies replacing newer revisions.

Types are temporarily centralized manual definitions as allowed by RFC §11.5. tauri-specta compatibility has not been validated and is not claimed. Add generation once dependencies can be installed; keep the current contract small until then.

## Verification on 2026-11-13

| Requirement | Actual result |
|---|---|
| pnpm install | Failed: Corepack found using existing Node 24.22.1, npm registry download denied with EACCES |
| Offline frontend install | Failed: Svelte Vite plugin missing from npm cache |
| Svelte check / frontend build | Not run: dependencies unavailable |
| cargo check --offline | Failed: Tauri missing from crates.io cache |
| cargo check | Failed: crates.io connection unavailable |
| cargo fmt --check | Passed after formatting |
| Rust tests / Clippy | Not run: dependencies unavailable |
| Windows launch / real IPC | Not run: native executable cannot yet be built |
| rebuild_check | Failed: checker cannot find npm.cmd on its PATH |
| Old project continues running | Not independently verified; no old project or user database was read, modified or started |

No lockfiles are fabricated. Generate and commit pnpm-lock.yaml and src-tauri/Cargo.lock after dependency resolution succeeds. Do not treat this stage as accepted until all requested checks pass.

## Next verification

Make Node and pnpm available to the build process and provide normal registry access, then run `pnpm install`, `pnpm check`, `pnpm build`, `pnpm tauri build --debug --no-bundle`. Run Cargo formatting, Clippy and tests in src-tauri. Launch the built Windows executable, open Settings, press Verify connection, confirm Rust connected and idle snapshot, change volume and confirm revision increases. Close/reopen to confirm there is no persistent state claimed. Browser preview must continue reporting preview status.

The Tauri frontend settings follow the [official Vite integration](https://v2.tauri.app/start/frontend/vite/) and Tailwind uses the [official Vite plugin](https://tailwindcss.com/docs/installation/using-vite). No persistent dev server was started in this task.

## Audio migration preparation

Based solely on the supplied `MIGRATION-AUDIT.md`, not a fresh source inspection:

1. Extract `player.rs` and `logging.rs` algorithms first; preserve Symphonia, cpal and rubato instead of rewriting a decoder. The Player receiver is not Sync. A controller thread should own Player, consume Finished/Failed and expose snapshot/requests to Tauri.
2. Reuse `metadata.rs` tag reading and `lyrics.rs` parsing independently of UI.
3. Later extract queue/import rules and session serialization from egui-dependent code. Do not depend on the old crate wholesale because its entry module imports egui UI.
4. Defer database reuse until a consistent read-only SQLite backup and isolated work copy can be verified. Existing db::open can modify schema and records, so never call it on the original database.

The old AGENTS.md, ARCHITECTURE.md and source were not read because the active clean-room instruction prohibits access to the original. The supplied audit records previous successful old-version tests; those are historical evidence, not a fresh old-version validation.
